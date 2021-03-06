extern crate atty;
extern crate regex;
extern crate version_compare;

use std::env;
use std::io;
use std::io::Read;

use atty::Stream;
use regex::Regex;

use crate::dependency::{max_by_dep, parse_dependency, Dependency};
use crate::iter::SortedByExt;

mod dependency;
mod iter;

fn parse(input: &str) -> Vec<Dependency> {
    let upper_bounds =
        Regex::new("Require upper bound dependencies error for (.*) paths to dependency are:")
            .unwrap();

    upper_bounds
        .captures_iter(input)
        .map(|cap| parse_dependency(cap.get(1).unwrap().as_str()))
        .flat_map(|dep| max_by_dep(dep, input))
        .sorted_by(Ord::cmp)
        .collect()
}

fn main() -> io::Result<()> {
    if env::args().any(|arg| arg.eq(&String::from("-v")) || arg.eq(&String::from("--version"))) {
        const NAME: &str = env!("CARGO_PKG_NAME");
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        println!("{} {}", NAME, VERSION);
        return Ok(());
    }

    if atty::is(Stream::Stdin) {
        eprintln!(
            "Stdin is a terminal, you should pipe the output of mvn validate to this program"
        );
        return Ok(());
    }

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();

    match handle.read_to_string(&mut buffer) {
        Err(err) => panic!("Failed to read from stdin {}", err),
        Ok(_) => parse(buffer.as_str())
            .iter()
            .for_each(|dep| println!("{}", dep)),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use version_compare::Version;

    use super::*;

    #[test]
    fn parse_should_return_vec_of_deps_on_validate_failed_input() {
        let failed = include_str!("../test/fixtures/fail.out");
        let deps = parse(failed);
        assert_eq!(
            deps,
            vec![Dependency {
                group_id: "org.jenkins-ci.plugins.workflow",
                artifact_id: "workflow-api",
                version: Version::from("2.32").unwrap(),
            }]
        )
    }

    #[test]
    fn parse_should_return_empty_vec_on_validate_successful_input() {
        let success = include_str!("../test/fixtures/success.out");
        let deps = parse(success);
        assert_eq!(deps, vec![])
    }
}
