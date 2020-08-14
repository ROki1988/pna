use crate::command::{Request, Response};
use crate::error::{KvsError, KvsErrorKind};
use crate::Result;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{Shutdown, TcpStream, ToSocketAddrs};
use std::str::FromStr;

/// Key value store client
pub struct KvsClient {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl KvsClient {
    /// Connect to `addr` to access `KvsServer`.
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let tcp_reader = TcpStream::connect(addr)?;
        let tcp_writer = tcp_reader.try_clone()?;
        Ok(KvsClient {
            reader: BufReader::new(tcp_reader),
            writer: BufWriter::new(tcp_writer),
        })
    }

    /// Get the value of a given key from the server.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        let request = Request::Get { key };
        self.communicate(&request).map_or_else(
            |e| {
                if e.is_key_not_found() {
                    Ok(None)
                } else {
                    Err(e)
                }
            },
            |x| match x {
                Response::String { value } => Ok(Some(value)),
                _ => Err(KvsError::from(KvsErrorKind::UnknownCommand(x.to_string()))),
            },
        )
    }

    /// Set the value of a string key in the server.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let request = Request::Set { key, value };
        let _response = self.communicate(&request)?;
        Ok(())
    }

    /// Remove a string key in the server.
    pub fn remove(&mut self, key: String) -> Result<()> {
        let request = Request::Remove { key };
        let _response = self.communicate(&request)?;
        Ok(())
    }

    fn communicate(&mut self, request: &Request) -> Result<Response> {
        let send = request.to_string();
        self.writer.write_all(send.as_bytes())?;
        self.writer.flush()?;
        let mut receive = String::new();
        self.reader.read_line(&mut receive)?;
        Response::from_str(receive.as_str()).and_then(|x| match x {
            Response::Error { message } => {
                let e = KvsErrorKind::from_str(message.as_str())?;
                Err(KvsError::from(e))
            }
            other => Ok(other),
        })
    }

    /// shutdown tcp connection
    pub fn close(self) -> Result<()> {
        self.reader.into_inner().shutdown(Shutdown::Both)?;
        Ok(())
    }
}
