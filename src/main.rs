extern crate atty;
extern crate regex;

use std::convert::TryFrom;
use std::env;
use std::io;
use std::io::Read;

use atty::Stream;
use regex::Regex;

use crate::dependency::errors::DependencyParseError;
use crate::dependency::{max_by_dep, Dependency};
use crate::iter::SortedByExt;

mod dependency;
mod iter;

fn parse(input: &str) -> Result<Vec<Dependency>, DependencyParseError> {
    let upper_bounds =
        Regex::new("Require upper bound dependencies error for (\\S+) paths to dependency are:")
            .unwrap();

    let dependencies = upper_bounds
        .captures_iter(input)
        .flat_map(|cap| cap.iter().nth(1).flatten())
        .map(|m| m.as_str())
        .map(Dependency::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    let dependencies_by_max = dependencies
        .into_iter()
        .map(|dep| max_by_dep(dep, input))
        .collect::<Result<Vec<Dependency>, _>>()?;

    Ok(dependencies_by_max
        .into_iter()
        .sorted_by(Ord::cmp)
        .collect())
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

    let mut buffer = String::new();

    match io::stdin().lock().read_to_string(&mut buffer) {
        Err(err) => eprintln!("Failed to read from stdin {}", err),
        Ok(_) => match parse(buffer.as_str()) {
            Err(e) => eprintln!("{}", e),
            Ok(deps) => deps.iter().for_each(|dep| println!("{}", dep)),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_should_return_vec_of_deps_on_validate_failed_input() {
        let failed = include_str!("../test/fixtures/fail-one-regular.out");
        let deps = parse(failed).unwrap();
        assert_eq!(
            deps,
            vec![
                Dependency::try_from("org.jenkins-ci.plugins.workflow:workflow-api:2.32").unwrap()
            ]
        )
    }

    #[test]
    fn parse_should_return_vec_of_deps_on_validate_failed_with_managed_input() {
        let failed = include_str!("../test/fixtures/fail-managed.out");
        let deps = parse(failed).unwrap();
        assert_eq!(
            deps,
            vec![Dependency::try_from("com.h2database:h2:1.4.190").unwrap()]
        )
    }

    #[test]
    fn parse_should_return_vec_of_deps_on_validate_failed_with_multiple_bound_concflicts_input() {
        let failed = include_str!("../test/fixtures/fail-multiple.out");
        let deps = parse(failed).unwrap();
        assert_eq!(
            deps,
            vec![
                Dependency::try_from("org.codehaus.groovy:groovy-all:2.4.12").unwrap(),
                Dependency::try_from("org.slf4j:jcl-over-slf4j:1.7.26").unwrap(),
                Dependency::try_from("org.slf4j:log4j-over-slf4j:1.7.26").unwrap(),
                Dependency::try_from("org.slf4j:slf4j-jdk14:1.7.26").unwrap(),
            ]
        )
    }

    #[test]
    fn parse_should_return_empty_vec_on_validate_successful_input() {
        let success = include_str!("../test/fixtures/success.out");
        let deps = parse(success).unwrap();
        assert_eq!(deps, vec![])
    }
}
