use crate::command::{Request, Response};
use crate::engine::KvsEngine;
use crate::error::KvsErrorKind;
use crate::thread_pool::ThreadPool;
use crate::Result;
use slog::*;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;

/// Key Value Store server
pub struct KvsServer<E: KvsEngine, T: ThreadPool> {
    engine: E,
    thread_pool: T,
}

impl<E: KvsEngine, T: 'static + ThreadPool> KvsServer<E, T>
where
    E: Send + Sync + 'static,
    T: std::marker::Send,
{
    /// Create Kvs Server
    pub fn new(engine: E, thread_pool: T) -> Self {
        Self {
            engine,
            thread_pool,
        }
    }

    /// Execute KvsServer
    pub fn run<A: ToSocketAddrs>(self, addr: A, logger: Logger) -> Result<Shutdown> {
        let stop = Arc::new(AtomicBool::new(false));
        let should_shutdown = Arc::clone(&stop);
        let listener = TcpListener::bind(addr)?;
        let thread = std::thread::spawn(move || {
            for stream in listener.incoming().take_while(|_| !stop.load(Ordering::Relaxed)) {
                match stream {
                    Ok(xs) => {
                        debug!(logger, "accept connection from {}", xs.peer_addr().unwrap());
                        let e = self.engine.clone();
                        let l = logger.clone();
                        self.thread_pool.spawn(move || handle_stream(e, xs, &l));
                    }
                    Err(e) => {
                        warn!(logger, "{}", e);
                    }
                }
            }
            info!(logger, "stopping worker thread");
            self.thread_pool.shutdown();
            info!(logger, "stop receive thread");

        });
        Ok(Shutdown { should_shutdown, thread })
    }
}

pub struct Shutdown {
    should_shutdown: Arc<AtomicBool>,
    thread: JoinHandle<()>,
}

impl Shutdown {
    pub fn do_shutdown(self) -> Result<()> {
        self.should_shutdown.store(true, Ordering::Relaxed);
        Ok(())
    }
}

fn handle_stream<E: KvsEngine>(engine: E, mut xs: TcpStream, logger: &Logger) {
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
            process(engine, request)
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

fn process<E: KvsEngine>(engine: E, request: Request) -> Response {
    match request {
        Request::Get { key } => engine.get(key).map_or_else(
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
        Request::Set { key, value } => engine.set(key, value).map_or_else(
            |x| Response::Error {
                message: x.to_string(),
            },
            |_| Response::String {
                value: "".to_string(),
            },
        ),
        Request::Remove { key } => engine.remove(key).map_or_else(
            |x| Response::Error {
                message: x.to_string(),
            },
            |_| Response::String {
                value: "".to_string(),
            },
        ),
    }
}
