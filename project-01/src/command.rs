use crate::error::{KvsError, KvsErrorKind};
use crate::Result;
use std::iter::Iterator;
use std::str::FromStr;

#[derive(Debug)]
pub enum Request {
    Get { key: String },
    Set { key: String, value: String },
    Remove { key: String },
}

#[derive(Debug)]
pub enum Response {
    String { value: String },
    Error { message: String },
}

impl ToString for Response {
    fn to_string(&self) -> String {
        match self {
            Response::String { value } => format!("+{}\r\n", value),
            Response::Error { message } => format!("-{}\r\n", message),
        }
    }
}

impl FromStr for Response {
    type Err = KvsError;

    fn from_str(s: &str) -> Result<Self> {
        let first = s
            .chars()
            .next()
            .ok_or_else(|| KvsError::from(KvsErrorKind::InvalidArgument))?;
        match first {
            '+' => Ok(Response::String {
                value: s
                    .trim_end_matches("\r\n")
                    .trim_start_matches("+")
                    .to_string(),
            }),
            '-' => Ok(Response::Error {
                message: s
                    .trim_end_matches("\r\n")
                    .trim_start_matches("-")
                    .to_string(),
            }),
            _ => Err(KvsError::from(KvsErrorKind::InvalidArgument)),
        }
    }
}

impl ToString for Request {
    fn to_string(&self) -> String {
        match self {
            Request::Get { key } => format!("GET {key}\r\n", key = key),
            Request::Set { key, value } => {
                format!("SET {key} {value}\r\n", key = key, value = value)
            }
            Request::Remove { key } => format!("REMOVE {key}\r\n", key = key),
        }
    }
}

impl FromStr for Request {
    type Err = KvsError;

    fn from_str(s: &str) -> Result<Self> {
        let mut xs = s.trim_end_matches("\r\n").splitn(3, " ");
        let command: &str = xs
            .next()
            .ok_or_else(|| KvsError::from(KvsErrorKind::InvalidArgument))?;

        match command {
            "GET" => {
                let key = xs
                    .next()
                    .ok_or_else(|| KvsError::from(KvsErrorKind::InvalidArgument))?
                    .to_string();
                Ok(Request::Get { key })
            }
            "SET" => {
                let key = xs
                    .next()
                    .ok_or_else(|| KvsError::from(KvsErrorKind::InvalidArgument))?
                    .to_string();
                let value = xs
                    .next()
                    .ok_or_else(|| KvsError::from(KvsErrorKind::InvalidArgument))?
                    .to_string();
                Ok(Request::Set { key, value })
            }
            "REMOVE" => {
                let key = xs
                    .next()
                    .ok_or_else(|| KvsError::from(KvsErrorKind::InvalidArgument))?
                    .to_string();
                Ok(Request::Remove { key })
            }
            _ => Err(KvsError::from(KvsErrorKind::InvalidArgument)),
        }
    }
}

mod tests {

    #[test]
    fn get_request_from_to() {
        use crate::command::Request;
        use std::str::FromStr;

        let input = "GET TEST\r\n";
        let to = Request::from_str(input).unwrap();
        let from = to.to_string();
        assert_eq!(input, from.as_str());
    }

    #[test]
    fn set_request_from_to() {
        use crate::command::Request;
        use std::str::FromStr;

        let input = "SET TEST 1\r\n";
        let to = Request::from_str(input).unwrap();
        let from = to.to_string();
        assert_eq!(input, from.as_str());
    }
}
