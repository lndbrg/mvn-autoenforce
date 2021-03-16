use core::cmp::{Eq, Ord, Ordering};
use core::fmt::{Display, Error, Formatter};
use core::result::Result;

use regex::{escape, Match, Regex};
use version_compare::Version;

use crate::dependency::version::create_version;

mod version;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Dependency<'a> {
    group_id: &'a str,
    artifact_id: &'a str,
    version: Version<'a>,
}

impl<'a> Dependency<'a> {
    pub fn from(coordinates_str: &'a str) -> Dependency<'a> {
        let coordinates: Vec<&str> = coordinates_str.split(':').collect();
        Dependency {
            group_id: coordinates[0],
            artifact_id: coordinates[1],
            version: create_version(coordinates[2]),
        }
    }
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

#[cfg(test)]
mod tests {
    use version_compare::Version;

    use crate::dependency::Dependency;

    #[test]
    fn dependency_from_should_parse_dependency_correctly() {
        assert_eq!(
            Dependency::from("com.h2database:h2:1.4.190"),
            Dependency {
                group_id: "com.h2database",
                artifact_id: "h2",
                version: Version::from("1.4.190").unwrap(),
            }
        )
    }
}
