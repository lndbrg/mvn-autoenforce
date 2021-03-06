mod version;

use core::cmp::{Eq, Ord, Ordering};
use core::fmt::{Display, Error, Formatter};
use core::result::Result;

use crate::dependency::version::create_version;
use regex::Regex;
use version_compare::Version;

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
            "\\S{}:{}:(\\S+)",
            dependency.group_id, dependency.artifact_id
        )
        .as_ref(),
    )
    .unwrap();

    version_regex
        .captures_iter(output)
        .map(|v| Dependency {
            version: create_version(v.get(1).unwrap().as_str()),
            ..dependency
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
