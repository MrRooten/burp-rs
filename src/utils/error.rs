use std::{error::{Error}, fmt};

use super::STError;

#[macro_export]
macro_rules! st_error {
    (  $e:expr  ) => {
        {
            STError::from(Box::new($e))
        }
    };
    
}

impl STError {
    pub fn new(msg: &str) -> STError{
        STError {
            detail: msg.to_string(),
            ..Default::default()
        }
    }

    pub fn from(err: Box<dyn Error + Send>) -> STError{
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