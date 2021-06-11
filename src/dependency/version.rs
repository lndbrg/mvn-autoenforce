use core::result::Result::Ok;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::ops::Deref;

use version_compare::version::Version as InnerVersion;
use version_compare::version_part::VersionPart;

use crate::dependency::errors::UnparseableVersionError;

/*
This is a bit ugly, but we create a wrapper struct for the foreign `Version` (called InnerVersion)
struct in order to be able to implement the `TryFrom` trait. Because Version does not implement copy
or clone we wrap it in and implement deref to be able to get to the InnerVersion.

It's extremely unlikely to actually get am error from the `Version::try_from` as we have very well
defined regexes that should make sure we never even call try_from unless we have output from maven
that matches a coordinate string, but if someone for some reason has managed to create an
alphabetical version and gotten it uploaded somewhere we will fail to parse it.
*/
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Version<'a> {
    inner: InnerVersion<'a>,
}

impl<'a> Eq for Version<'a> {}

impl<'a> Ord for Version<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.partial_cmp(&other.inner).unwrap()
    }
}

/*
Implement `Deref` to be able to call methods on the `InnerVersion`.
 */
impl<'a> Deref for Version<'a> {
    type Target = InnerVersion<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> TryFrom<&'a str> for Version<'a> {
    type Error = UnparseableVersionError;

    fn try_from(version_string: &'a str) -> Result<Self, Self::Error> {
        let initial_version = InnerVersion::from(version_string)
            .ok_or_else(|| UnparseableVersionError::from(version_string))?;

        Ok(Version {
            inner: InnerVersion::from_parts(
                version_string,
                initial_version
                    .parts()
                    .iter()
                    .flat_map(explode_part)
                    .collect(),
            ),
        })
    }
}

fn explode_part<'a>(version_part: &VersionPart<'a>) -> Vec<VersionPart<'a>> {
    match version_part {
        VersionPart::Number(val) => vec![VersionPart::Number(*val)],
        VersionPart::Text(val) => val
            .split('-')
            .map(|part| match part.parse::<i32>() {
                Ok(number) => VersionPart::Number(number),
                Err(_) => VersionPart::Text(part),
            })
            .collect(),
    }
}
