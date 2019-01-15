extern crate itertools;
extern crate regex;
extern crate version_compare;

use std::cmp::Ordering;
use std::env;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::io;
use std::io::Read;

use itertools::Itertools;
use regex::Regex;
use version_compare::Version;
use version_compare::VersionPart;

struct Dependency<'a> {
    group_id: &'a str,
    artifact_id: &'a str,
    version: Version<'a>,
}

impl<'a> PartialEq for Dependency<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.group_id == other.group_id
            && self.artifact_id == other.artifact_id
            && self.version == other.version
    }
}

impl<'a> Eq for Dependency<'a> {}

impl<'a> PartialOrd for Dependency<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

fn max_by_dep<'a>(dependency: Dependency<'a>, output: &'a str) -> Option<Dependency<'a>> {
    let version_regex =
        Regex::new(format!("\\S{}:{}:(\\S+)", dependency.group_id, dependency.artifact_id).as_ref())
            .unwrap();

    version_regex
        .captures_iter(output)
        .map(|v| Dependency {
            version: create_version(v.get(1).unwrap().as_str()),
            ..dependency
        }).max_by(Ord::cmp)
}

fn parse_dependency(dependency: &str) -> Dependency {
    let coordinates: Vec<&str> = dependency.split(":").collect();
    Dependency {
        group_id: coordinates[0],
        artifact_id: coordinates[1],
        version: create_version(coordinates[2]),
    }
}

fn create_version(version_string: &str) -> Version {
    let initial_version = Version::from(version_string)
        .expect(format!("Unparseable version '{}'", version_string).as_str());
    Version::from_parts(version_string,
                        initial_version.parts()
                            .iter()
                            .map(explode_part)
                            .flatten()
                            .collect_vec())
}

fn explode_part<'a>(version_part: &VersionPart<'a>) -> Vec<VersionPart<'a>> {
    match version_part {
        VersionPart::Number(val) => { vec![VersionPart::Number(*val)] }
        VersionPart::Text(val) => {
            let split: Vec<&str> = val.split("-").collect();
            split.iter().map(|part| match part.parse::<i32>() {
                Ok(number) => { VersionPart::Number(number) }
                Err(_) => { VersionPart::Text(part) }
            }).collect_vec()
        }
    }
}


fn main() -> io::Result<()> {
    if env::args()
        .find(|arg| arg.eq(&String::from("-v")) || arg.eq(&String::from("--version")))
        .is_some() {
        const NAME: &'static str = env!("CARGO_PKG_NAME");
        const VERSION: &'static str = env!("CARGO_PKG_VERSION");
        return Ok(println!("{} {}", NAME, VERSION));
    }

    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    match handle.read_to_string(&mut buffer) {
        Err(err) => panic!("Failed to read from stdin {}", err),
        Ok(_) => {
            let upper_bounds = Regex::new(
                "Require upper bound dependencies error for (.*) paths to dependency are:",
            ).unwrap();

            // TODO: more efficient parsing, using nom?
            upper_bounds
                .captures_iter(buffer.as_str())
                .map(|cap| parse_dependency(cap.get(1).unwrap().as_str()))
                .flat_map(|dep| max_by_dep(dep, buffer.as_str()))
                .sorted_by(Ord::cmp)
                .into_iter()
                .for_each(|dep| println!("{}", dep));
            Ok(())
        }
    }
}
