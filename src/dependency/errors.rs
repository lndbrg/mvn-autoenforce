use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct UnparseableVersionError<'a> {
    version_string: &'a str,
}

impl Error for UnparseableVersionError<'_> {}

impl<'a, 'b> From<&'a str> for UnparseableVersionError<'b>
where
    'a: 'b,
{
    fn from(version_string: &'a str) -> UnparseableVersionError<'b> {
        Self { version_string }
    }
}

impl Display for UnparseableVersionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to parse version from: '{}'",
            &self.version_string
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum DependencyParseError<'a> {
    CoordinateError(&'a str),
    VersionError(&'a str, &'a str, UnparseableVersionError<'a>),
}

impl Error for DependencyParseError<'_> {}

impl Display for DependencyParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DependencyParseError::CoordinateError(coords) => {
                write!(
                    f,
                    "Failed to parse dependency coordinates from: '{}'",
                    coords
                )
            }
            DependencyParseError::VersionError(group_id, artifact_id, version_error) => {
                write!(
                    f,
                    "Failed to parse version for coordinates '{}:{}': {}",
                    group_id, artifact_id, version_error
                )
            }
        }
    }
}
