#![allow(dead_code)]
#![allow(unused_variables)]
use std::error::Error;

use colored::Colorize;

pub mod error;
pub mod config;
pub mod log;
#[derive(Debug,Default)]
pub struct STError {
    detail  : String,
    err     : Option<Box<dyn Error>>
}

pub fn banner() {
    let ptr = Box::into_raw(Box::new(123));
    let rand = ptr as usize;
    let mut line1 = r#"   
       ____     _   _    ____      ____         ____     ____     
    U | __")uU |"|u| |U |  _"\ u U|  _"\ u   U |  _"\ u / __"| u  
     \|  _ \/ \| |\| | \| |_) |/ \| |_) |/U  u\| |_) |/<\___ \/   
      | |_) |  | |_| |  |  _ <    |  __/  /___\|  _ <   u___) |   
      |____/  <<\___/   |_| \_\   |_|    |__"__|_| \_\  |____/>>  
     _|| \\_ (__) )(    //   \\_  ||>>_        //   \\_  )(  (__) 
    (__) (__)    (__)  (__)  (__)(__)__)      (__)  (__)(__)     "#.red();

    let a = rand % 7;
    if a == 0 {
        line1 = line1.bright_red();
    }
    else if a == 1 {
        line1 = line1.bright_blue();
    }
    else if a == 2 {
        line1 = line1.bright_cyan();
    }
    else if a == 3 {
        line1 = line1.bright_green();
    }
    else if a == 4 {
        line1 = line1.bright_purple();
    }
    else if a == 5 {
        line1 = line1.bright_white();
    }
    else if a == 6 {
        line1 = line1.bright_yellow();
    }
    println!("{}\n",line1);

}