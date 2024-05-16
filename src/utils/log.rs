use colored::Colorize;
use log::{Level, Log};

pub struct Logger;
pub static mut LOGS: Vec<String> = vec![];
pub static mut LEVEL: Level = Level::Info;
impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        unsafe { metadata.level() <= LEVEL }
    }

    fn log(&self, record: &log::Record) {
        //println!("{:?}",record);
        if self.enabled(record.metadata()) {
            let mut log = Option::<String>::default();
            if record.level() == log::Level::Debug {
                log = Some(format!(
                    "{} {} {}",
                    record.level().to_string().bright_blue(),
                    record.file().unwrap_or(""),
                    record.args()
                ));
            } else if record.level() == log::Level::Error {
                log = Some(format!(
                    "{} {} {}",
                    record.level().to_string().red(),
                    record.file().unwrap_or(""),
                    record.args()
                ));
            } else if record.level() == log::Level::Warn {
                log = Some(format!(
                    "{} {} {}",
                    record.level().to_string().yellow(),
                    record.file().unwrap_or(""),
                    record.args()
                ));
            } else if record.level() == log::Level::Info {
                log = Some(format!(
                    "{} {} {}",
                    record.level().to_string().green(),
                    record.file().unwrap_or(""),
                    record.args()
                ));
            }
            unsafe {
                let log = match log {
                    Some(log) => log,
                    None => "".to_string(),
                };
                if record.target().starts_with("burp_rs") {
                    LOGS.push(log);
                }
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

    let result = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug));

    match result {
        Ok(o) => {
            Ok(())
        }
        Err(e) => {
            Err(st_error!(e))
        }
    }
}

//Avoid big cost of fmt 
pub fn can_debug() -> bool {
    unsafe { if LEVEL >= Level::Debug {
        return true;
    } }

    false
}
