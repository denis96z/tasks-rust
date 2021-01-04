use super::student::Student;
use serde_yaml as yaml;

pub fn load_students(filename: &str) -> InputResult<Vec<Student>> {
    let s = match std::fs::read_to_string(filename) {
        Ok(s) => s,
        Err(err) => {
            return Err(InputError::IO(err));
        }
    };
    match yaml::from_str(&s) {
        Ok(v) => Ok(v),
        Err(err) => Err(InputError::YAML(err)),
    }
}

pub fn save_students(filename: &str, students: &[Student]) -> OutputResult<()> {
    let s = match yaml::to_string(students) {
        Ok(s) => s,
        Err(err) => {
            return Err(OutputError::YAML(err));
        }
    };
    match std::fs::write(filename, s) {
        Ok(_) => Ok(()),
        Err(err) => Err(OutputError::IO(err)),
    }
}

pub type InputResult<T> = Result<T, InputError>;
pub type OutputResult<T> = Result<T, OutputError>;

pub type InputError = Error;
pub type OutputError = Error;

use std::io::Error as ErrIO;
use yaml::Error as ErrYAML;

#[derive(Debug)]
pub enum Error {
    IO(ErrIO),
    YAML(ErrYAML),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::IO(err) => err.fmt(f),
            Self::YAML(err) => err.fmt(f),
        }
    }
}
