use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::str::FromStr;
use std::fmt::Debug;

pub struct LoaderError {
    message: String
}

impl From<Error> for LoaderError {
    fn from(err: Error) -> Self {
        LoaderError {
            message: err.to_string()
        }
    }
}

impl From<LoaderError> for String {
    fn from(err: LoaderError) -> Self {
        err.message
    }
}

pub fn load_data<T: FromStr>(file_name: &str) -> Result<Vec<T>, LoaderError>
    where T::Err: Debug
{
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let result = contents.lines().into_iter()
        .map(|item| item.parse())
        .filter(|item| item.is_ok())
        .map(|item| item.unwrap())
        .collect();

    Ok(result)
}