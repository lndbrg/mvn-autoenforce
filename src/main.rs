extern crate atty;
extern crate regex;

use std::env;
use std::io;
use std::io::Read;

use atty::Stream;
use regex::Regex;

use crate::dependency::{max_by_dep, Dependency};
use crate::iter::SortedByExt;

mod dependency;
mod iter;

fn parse(input: &str) -> Vec<Dependency> {
    let upper_bounds =
        Regex::new("Require upper bound dependencies error for (.*) paths to dependency are:")
            .unwrap();

    upper_bounds
        .captures_iter(input)
        .map(|cap| Dependency::from(cap.get(1).unwrap().as_str()))
        .flat_map(|dep| max_by_dep(dep, input))
        .sorted_by(Ord::cmp)
        .collect()
}

fn main() {
    if env::args().any(|arg| arg.eq(&String::from("-v")) || arg.eq(&String::from("--version"))) {
        const NAME: &str = env!("CARGO_PKG_NAME");
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        println!("{} {}", NAME, VERSION);
        return;
    }

    if atty::is(Stream::Stdin) {
        eprintln!(
            "Stdin is a terminal, you should pipe the output of mvn validate to this program"
        );
        return;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_should_return_vec_of_deps_on_validate_failed_input() {
        let failed = include_str!("../test/fixtures/fail-one-regular.out");
        let deps = parse(failed);
        assert_eq!(
            deps,
            vec![Dependency::from(
                "org.jenkins-ci.plugins.workflow:workflow-api:2.32"
            )]
        )
    }

    #[test]
    fn parse_should_return_vec_of_deps_on_validate_failed_with_managed_input() {
        let failed = include_str!("../test/fixtures/fail-managed.out");
        let deps = parse(failed);
        assert_eq!(deps, vec![Dependency::from("com.h2database:h2:1.4.190"),])
    }

    #[test]
    fn parse_should_return_vec_of_deps_on_validate_failed_with_multiple_bound_concflicts_input() {
        let failed = include_str!("../test/fixtures/fail-multiple.out");
        let deps = parse(failed);
        assert_eq!(
            deps,
            vec![
                Dependency::from("org.codehaus.groovy:groovy-all:2.4.12"),
                Dependency::from("org.slf4j:jcl-over-slf4j:1.7.26"),
                Dependency::from("org.slf4j:log4j-over-slf4j:1.7.26"),
                Dependency::from("org.slf4j:slf4j-jdk14:1.7.26")
            ]
        )
    }

    #[test]
    fn parse_should_return_empty_vec_on_validate_successful_input() {
        let success = include_str!("../test/fixtures/success.out");
        let deps = parse(success);
        assert_eq!(deps, vec![])
    }
}
