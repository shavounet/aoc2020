use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::fmt::{Debug, Display, Formatter};
use std::error::Error;
use std::num::ParseIntError;
use crate::day8::ExitCode;


pub fn load_data<T: FromStr>(file_name: &str, split_pattern: &str) -> Result<Vec<T>, LoadError>
    where T::Err: Error
{
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let result = contents.split(split_pattern).into_iter()
        .map(|item| item.parse::<T>())
        .filter(|item| item.is_ok())
        .collect::<Result<Vec<T>, T::Err>>()?;

    Ok(result)
}

#[derive(Debug)]
pub struct GenericError {
    message: String
}

impl GenericError {
    pub fn new(message: String) -> Self {
        GenericError { message }
    }
    pub fn throw<O>(message: &str) -> Result<O, Self> {
        Err(GenericError { message: message.to_string() })
    }
}

impl From<LoadError> for GenericError
{
    fn from(err: LoadError) -> Self {
        GenericError {
            message: err.to_string()
        }
    }
}

impl From<ParseIntError> for GenericError
{
    fn from(err: ParseIntError) -> Self {
        GenericError {
            message: err.to_string()
        }
    }
}


impl From<regex::Error> for GenericError
{
    fn from(err: regex::Error) -> Self {
        GenericError {
            message: err.to_string()
        }
    }
}

impl From<ExitCode> for GenericError
{
    fn from(err: ExitCode) -> Self {
        match err {
            ExitCode::EndOfProgram => GenericError::new("End of program".to_string()),
            ExitCode::Error(generic_error) => generic_error
        }
    }
}

impl From<std::io::Error> for GenericError
{
    fn from(err: std::io::Error) -> Self {
        GenericError {
            message: err.to_string()
        }
    }
}

impl Display for GenericError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for GenericError {}

pub struct LoadError {
    message: String
}

impl<T> From<T> for LoadError
    where T: Error
{
    fn from(err: T) -> Self {
        LoadError {
            message: err.to_string()
        }
    }
}

impl ToString for LoadError {
    fn to_string(&self) -> String {
        self.message.clone()
    }
}
