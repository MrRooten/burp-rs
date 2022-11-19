use std::error::Error;

pub mod error;
pub mod config;
#[derive(Debug,Default)]
pub struct STError {
    detail  : String,
    err     : Option<Box<dyn Error>>
}