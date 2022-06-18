use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct UnparseableVersionError {
    version_string: String,
}

impl Error for UnparseableVersionError {}

impl From<&str> for UnparseableVersionError {
    fn from(version_string: &str) -> Self {
        Self {
            version_string: version_string.to_string(),
        }
    }
}

impl Display for UnparseableVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to parse version from: '{}'",
            &self.version_string
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum DependencyParseError {
    CoordinateError(String),
    VersionError(String, String, UnparseableVersionError),
}

impl Error for DependencyParseError {}

impl Display for DependencyParseError {
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
