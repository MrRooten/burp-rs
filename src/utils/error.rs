use std::{error::{Error, self}, fmt};

use super::STError;


impl STError {
    pub fn new(msg: &str) -> STError{
        STError {
            detail: msg.to_string(),
            ..Default::default()
        }
    }

    pub fn from(err: Box<dyn Error>) -> STError{
        let mut result = STError {
            detail: "".to_string(),
            ..Default::default()
        };
        result.err = Some(err);
        result
    }
}

impl fmt::Display for STError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.detail)
    }
}

impl Error for STError {
    fn description(&self) -> &str {
        &self.detail
    }


}