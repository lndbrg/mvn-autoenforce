use core::cmp::{Eq, Ord, Ordering};
use core::fmt::{Display, Error, Formatter};
use core::result::Result;
use std::convert::TryFrom;

use regex::{escape, Match, Regex};

use crate::dependency::errors::DependencyParseError;
use crate::dependency::errors::DependencyParseError::{CoordinateError, VersionError};
use crate::dependency::version::Version;

pub(crate) mod errors;
mod version;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Dependency<'a> {
    group_id: &'a str,
    artifact_id: &'a str,
    version: Version<'a>,
}

impl<'a> Eq for Dependency<'a> {}

impl<'a> Ord for Dependency<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.group_id
            .cmp(other.group_id)
            .then_with(|| self.artifact_id.cmp(other.artifact_id))
            .then_with(|| self.version.partial_cmp(&other.version).unwrap())
    }
}

impl<'a> TryFrom<&'a str> for Dependency<'a> {
    type Error = DependencyParseError;

    fn try_from(coordinate_string: &'a str) -> Result<Self, Self::Error> {
        let coordinates: Vec<&str> = coordinate_string.split(':').collect();
        match coordinates[..] {
            [group_id, artifact_id, version_string] if !version_string.trim().is_empty() => {
                Version::try_from(version_string)
                    .map(|version| Self {
                        group_id,
                        artifact_id,
                        version,
                    })
                    .map_err(|e| VersionError(group_id.to_owned(), artifact_id.to_owned(), e))
            }
            _ => Err(CoordinateError(coordinate_string.to_owned())),
        }
    }
}

impl<'a> Display for Dependency<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            r#"
    <dependency>
        <groupId>{}</groupId>
        <artifactId>{}</artifactId>
        <version>{}</version>
    </dependency>"#,
            self.group_id,
            self.artifact_id,
            self.version.as_str()
        )
    }
}

pub fn max_by_dep<'a>(dependency: Dependency<'a>, output: &'a str) -> Option<Dependency<'a>> {
    let version_regex = Regex::new(
        format!(
            "{}:{}:(\\S+)",
            escape(dependency.group_id),
            escape(dependency.artifact_id)
        )
        .as_ref(),
    )
    .unwrap();

    version_regex
        .captures_iter(output)
        /*
        Translate all captures found, but skip the first as that is the full match of the
        regex, not just the capture group of versions we are looking for.
        */
        .flat_map(|captures| captures.iter().skip(1).flatten().collect::<Vec<Match>>())
        .map(|m| m.as_str())
        .map(Version::try_from)
        //TODO: Remove the flatten 👇 and handle error
        .flatten()
        .max_by(Ord::cmp)
        .map(|version| Dependency {
            version,
            ..dependency
        })
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::dependency::errors::DependencyParseError::{CoordinateError, VersionError};
    use crate::dependency::errors::UnparseableVersionError;
    use crate::dependency::version::Version;
    use crate::dependency::Dependency;

    #[test]
    fn dependency_from_should_parse_dependency_correctly() {
        assert_eq!(
            Dependency::try_from("com.h2database:h2:1.4.190").unwrap(),
            Dependency {
                group_id: "com.h2database",
                artifact_id: "h2",
                version: Version::try_from("1.4.190").unwrap(),
            }
        )
    }
    #[test]
    fn dependency_from_with_missing_parts_should_result_in_coordinate_error() {
        assert_eq!(
            Dependency::try_from("com.h2database:h2"),
            Err(CoordinateError("com.h2database:h2".to_string()))
        )
    }
    #[test]
    fn dependency_from_with_missing_version_should_result_in_coordinate_error() {
        assert_eq!(
            Dependency::try_from("com.h2database:h2:"),
            Err(CoordinateError("com.h2database:h2:".to_string()))
        );
        assert_eq!(
            Dependency::try_from("com.h2database:h2: "),
            Err(CoordinateError("com.h2database:h2: ".to_string()))
        )
    }

    #[test]
    fn dependency_from_with_broken_version_should_result_in_version_error() {
        assert_eq!(
            Dependency::try_from("com.h2database:h2:broken"),
            Err(VersionError(
                "com.h2database".to_string(),
                "h2".to_string(),
                UnparseableVersionError::from("broken")
            ))
        )
    }
}
