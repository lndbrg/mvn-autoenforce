use core::cmp::{Eq, Ord, Ordering};
use core::fmt::{Display, Error, Formatter};
use core::result::Result;

use regex::{escape, Match, Regex};
use version_compare::Version;

use crate::dependency::version::create_version;

mod version;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Dependency<'a> {
    pub group_id: &'a str,
    pub artifact_id: &'a str,
    pub version: Version<'a>,
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
        .flat_map(|captures| {
            /*
            Captures does not have an into_iter method so we get the wrong type of reference, we
            take a roundtrip through a vec that can give us the correct type of reference and we
            no longer need to worry about borrowed values.

            We translate all captures found, but skip the first as that is the full match of the
            regex, not just the capture group of versions we are looking for.
            */
            captures
                .iter()
                .skip(1)
                .filter_map(|x| x)
                .collect::<Vec<Match>>()
                .into_iter()
                .map(|m| Dependency {
                    version: create_version(m.clone().as_str()),
                    ..dependency
                })
        })
        .max_by(Ord::cmp)
}

pub fn parse_dependency(dependency: &str) -> Dependency {
    let coordinates: Vec<&str> = dependency.split(':').collect();
    Dependency {
        group_id: coordinates[0],
        artifact_id: coordinates[1],
        version: create_version(coordinates[2]),
    }
}
