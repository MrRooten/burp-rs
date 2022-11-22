use std::fs;
use std::path::Path;

use log::{Level, Log};

pub struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {
        todo!()
    }
}

use log::LevelFilter;

use super::STError;
static LOGGER: Logger = Logger;
pub fn init() -> Result<(), STError> {
    let path = Path::new("./logs/");
    let metadata = path.metadata().unwrap();
    if path.exists() {
        if !metadata.is_dir() {
            return Err(STError::new("Can not create log dir because there is a ./logs file"));
        }

        let dir = fs::create_dir_all("./logs/");
        match dir {
            Ok(o) => {

            },
            Err(e) => {
                return Err(STError::from(Box::new(e)));
            }
        }
    }

    let result = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info));

    match result {
        Ok(o) => {
            return Ok(());
        }
        Err(e) => {
            return Err(STError::from(Box::new(e)));
        }
    }
}
