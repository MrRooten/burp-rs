use std::fs;
use std::path::Path;

use colored::Colorize;
use log::{Level, Log};

pub struct Logger;
pub static mut logs: Vec<String> = vec![];

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let mut log = Option::<String>::default();
            if record.level() == log::Level::Debug {
                log = Some(format!("{} - {}", record.level().to_string().blue(), record.args()));
            } else if record.level() == log::Level::Error {
                log = Some(format!("{} - {}", record.level().to_string().red(), record.args()));
            } else if record.level() == log::Level::Warn {
                log = Some(format!("{} - {}", record.level().to_string().yellow(), record.args()));
            } else if record.level() == log::Level::Info {
                log = Some(format!("{} - {}", record.level().to_string().green(), record.args()));
            } 
            unsafe {
                //logs.push(log.unwrap_or_default());
            }
        }
    }

    fn flush(&self) {
        todo!()
    }
}

use log::LevelFilter;

use crate::st_error;

use super::STError;
static LOGGER: Logger = Logger;
pub fn init() -> Result<(), STError> {
    //let path = Path::new("./logs/");
    //let metadata = path.metadata().unwrap();
    //if path.exists() {
    //    if !metadata.is_dir() {
    //        return Err(STError::new("Can not create log dir because there is a ./logs file"));
    //    }

    //    let dir = fs::create_dir_all("./logs/");
    //    match dir {
    //        Ok(o) => {

    //        },
    //        Err(e) => {
    //            return Err(st_error!(e));
    //        }
    //    }
    //}

    let result = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info));

    match result {
        Ok(o) => {
            return Ok(());
        }
        Err(e) => {
            return Err(st_error!(e));
        }
    }
}
