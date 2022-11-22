#![allow(dead_code)]
#![allow(unused_variables)]
use std::error::Error;

pub mod error;
pub mod config;
pub mod log;
#[derive(Debug,Default)]
pub struct STError {
    detail  : String,
    err     : Option<Box<dyn Error>>
}