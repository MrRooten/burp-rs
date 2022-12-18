use std::{
    collections::HashMap,
    fs::File,
    os::unix::{
        net::UnixStream,
        prelude::{AsRawFd, FromRawFd},
    },
    sync::mpsc::{Receiver, Sender, self},
    thread::spawn,
};

use hyper::Method;
use log::error;
use rutie::{types::RawFd, Thread};

use crate::{librs::http::utils::{HttpRequest, HttpResponse}, utils::STError};
use std::io::Write;
#[derive(Debug)]
pub struct HttpRequestWrapper {
    method: Method,
    fd: RawFd,
    fd2: RawFd,
    request: HttpRequest,
}

static mut REQUEST_SENDER: Option<Sender<HttpRequestWrapper>> = None;
static mut REQUEST_RECEIVER: Option<Receiver<HttpRequestWrapper>> = None;

static mut RESPONSE_HASHMAP: Option<HashMap<i32, Option<HttpResponse>>> = None;

fn get_response(fd: i32) -> Result<HttpResponse, STError> {
    unsafe {
        let map = match &mut RESPONSE_HASHMAP {
            Some(s) => s,
            None => {
                return Err(STError::new("Request Sender is not initialize"));
            }
        };

        let resp = match map.remove(&fd) {
            Some(s) => {
                match s  {
                    Some(ok) => {
                        return Ok(ok);
                    },
                    None => {
                        return Err(STError::new("Not exist response"));
                    }
                }
            },
            None => {
                return Err(STError::new("Not exist fd"));
            }
        };
    }
}

fn take_space(fd: i32) {
    unsafe {
        let map = match &mut RESPONSE_HASHMAP {
            Some(s) => s,
            None => {
                return ;
            }
        };
        map.insert(fd, None);
    }
}
pub fn send_request(method: &Method, request: &HttpRequest) -> Result<HttpResponse, STError> {
    let (unix_socket, unix_socket2) = UnixStream::pair().unwrap();
    let sender: &Sender<HttpRequestWrapper>;
    unsafe {
        sender = match &REQUEST_SENDER {
            Some(s) => s,
            None => {
                return Err(STError::new("Request Sender is not initialize"));
            }
        };
    }

    let fd = unix_socket2.as_raw_fd();
    let fd2 = unix_socket.as_raw_fd();
    take_space(fd);
    let request = HttpRequestWrapper{
        method: method.clone(),
        fd,
        fd2,
        request: request.clone(),
    }; 
    //println!("Send request");
    match sender.send(request) {
        Ok(o) => {},
        Err(e) => {
            error!("{}", e);
        }
    };
    //println!("wait fd");
    
    Thread::wait_fd(fd);
    //println!("wait done");
    
    
    let resp = get_response(fd);
    //let _ = unix_socket.shutdown(Shutdown::Both);
    //let _ = unix_socket2.shutdown(Shutdown::Both);
    resp
}

pub fn rb_http_thread() {
    let (tx, rx) = mpsc::channel::<HttpRequestWrapper>();
    unsafe {
        REQUEST_SENDER = Some(tx);
        REQUEST_RECEIVER = Some(rx);
        RESPONSE_HASHMAP = Some(HashMap::new());
    }
    spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on( async move {
        let receiver = unsafe {
            match &REQUEST_RECEIVER {
                Some(s) => s,
                None => {
                    return;
                }
            }
        };

        loop {
            
            let request = receiver.recv().unwrap();
            
            let r = tokio::spawn(async move {
                println!("Send request");
                let resp = HttpRequest::send_async(request.method, &request.request).await;
                let resp = match resp {
                    Ok(s) => s,
                    Err(e) => {
                        return;
                    }
                };
                unsafe {
                    let map = match &mut RESPONSE_HASHMAP {
                        Some(s) => s,
                        None => {
                            return;
                        }
                    };

                    map.insert(request.fd, Some(resp));
                    println!("Insert to taht");
                    let mut f = File::from_raw_fd(request.fd2);
                    write!(&mut f, "Hello, world!").unwrap();
                }
            });
        }
        });
    });
}
