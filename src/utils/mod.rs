use std::error::Error;

pub mod error;
#[derive(Debug,Default)]
pub struct STError {
    detail  : String,
    err     : Option<Box<dyn Error>>
}