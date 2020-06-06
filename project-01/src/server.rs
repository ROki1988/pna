use crate::command::{Request, Response};
use crate::engine::KvsEngine;
use crate::error::KvsErrorKind;
use crate::Result;
use slog::*;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::str::FromStr;

/// Key Value Store server
pub struct KvsServer<E: KvsEngine> {
    engine: E,
}

impl<E: KvsEngine> KvsServer<E> {
    /// Create Kvs Server
    pub fn new(engine: E) -> Self {
        Self { engine }
    }

    /// Execute KvsServer
    pub fn run<A: ToSocketAddrs>(mut self, addr: A, logger: Logger) -> Result<()> {
        let listener = TcpListener::bind(addr)?;

        for stream in listener.incoming() {
            match stream {
                Ok(xs) => {
                    debug!(logger, "accept connection from {}", xs.peer_addr().unwrap());
                    self.handle_stream(xs, &logger);
                }
                Err(e) => {
                    warn!(logger, "{}", e);
                }
            }
        }
        info!(logger, "stopping");
        Ok(())
    }

    fn handle_stream(&mut self, mut xs: TcpStream, logger: &Logger) {
        let mut buff = String::new();
        let mut reader = BufReader::new(xs.try_clone().unwrap());
        let h = reader
            .read_line(&mut buff)
            .map_err(Into::into)
            .and_then(|_| {
                debug!(logger, "accept message {:?}", buff);
                Request::from_str(buff.as_str())
            })
            .map(|request| {
                debug!(logger, "parsed request {:?}", request);
                self.process(request)
            })
            .and_then(|response| {
                let s = response.to_string();
                debug!(logger, "response {:?}", s);
                xs.write_all(s.as_bytes()).map_err(Into::into)
            })
            .and_then(|_| xs.flush().map_err(Into::into));
        match h {
            Ok(_) => {
                debug!(logger, "success request/response.");
            }
            Err(e) => {
                error!(logger, "fail process request: {}", e);
            }
        }
    }

    fn process(&mut self, request: Request) -> Response {
        match request {
            Request::Get { key } => self.engine.get(key).map_or_else(
                |x| Response::Error {
                    message: x.to_string(),
                },
                |x| {
                    x.map_or_else(
                        || Response::Error {
                            message: KvsErrorKind::KeyNotFound.to_string(),
                        },
                        |value| Response::String { value },
                    )
                },
            ),
            Request::Set { key, value } => self.engine.set(key, value).map_or_else(
                |x| Response::Error {
                    message: x.to_string(),
                },
                |_| Response::String {
                    value: "".to_string(),
                },
            ),
            Request::Remove { key } => self.engine.remove(key).map_or_else(
                |x| Response::Error {
                    message: x.to_string(),
                },
                |_| Response::String {
                    value: "".to_string(),
                },
            ),
        }
    }
}
