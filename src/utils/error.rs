use std::fmt::Display;

#[derive(Debug)]
pub enum Error
{
    InvalidRefference(String),
}

pub type Result<T> = std::result::Result<T, Error>;
