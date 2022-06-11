use anyhow;
use std::num;

#[derive(Debug, PartialEq)]
pub enum VersionError {
    InvalidVersion(String),
    UnexpectedError(String),
    UnsupportedVersion(String),
    ParsingError(num::ParseIntError),
    BumpError(BumpError),
}

impl From<num::ParseIntError> for VersionError {
    fn from(err: num::ParseIntError) -> VersionError {
        VersionError::ParsingError(err)
    }
}

impl From<BumpError> for VersionError {
    fn from(err: BumpError) -> VersionError {
        VersionError::BumpError(err)
    }
}

#[derive(Debug)]
pub enum BumpError {
    AnyError(anyhow::Error),
    MissingBumpScript,
    InvalidOperation(String),
}

impl PartialEq for BumpError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (BumpError::AnyError(_), _) => false,
            (_, BumpError::AnyError(_)) => false,
            (BumpError::MissingBumpScript, BumpError::MissingBumpScript) => true,
            (BumpError::InvalidOperation(m1), BumpError::InvalidOperation(m2)) => m1 == m2,
            _ => false,
        }
    }
}

impl From<anyhow::Error> for BumpError {
    fn from(err: anyhow::Error) -> BumpError {
        BumpError::AnyError(err)
    }
}
