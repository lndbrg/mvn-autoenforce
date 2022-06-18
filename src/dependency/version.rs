use core::result::Result::{Err, Ok};
use version_compare::version::Version;
use version_compare::version_part::VersionPart;

pub fn create_version(version_string: &str) -> Version {
    let initial_version = Version::from(version_string)
        .unwrap_or_else(|| panic!("Unparseable version '{}'", version_string));
    Version::from_parts(
        version_string,
        initial_version
            .parts()
            .iter()
            .flat_map(explode_part)
            .collect(),
    )
}

fn explode_part<'a>(version_part: &VersionPart<'a>) -> Vec<VersionPart<'a>> {
    match version_part {
        VersionPart::Number(val) => {
            vec![VersionPart::Number(*val)]
        }
        VersionPart::Text(val) => {
            let split: Vec<&str> = val.split('-').collect();
            split
                .iter()
                .map(|part| match part.parse::<i32>() {
                    Ok(number) => VersionPart::Number(number),
                    Err(_) => VersionPart::Text(part),
                })
                .collect()
        }
    }
}
