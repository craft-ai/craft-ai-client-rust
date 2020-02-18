use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
  BadToken(String, Option<Box<dyn error::Error + 'static>>),
  NetworkError(String, Box<dyn error::Error + 'static>),
  InternalError(String, Box<dyn error::Error + 'static>),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Error::BadToken(s, _) => write!(f, "Bad token - {}", s),
      Error::NetworkError(s, e) => write!(f, "Network error - {} ({})", s, e.to_string()),
      Error::InternalError(s, e) => write!(f, "Internal error - {} ({})", s, e.to_string()),
    }
  }
}

impl error::Error for Error {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    match self {
      Error::BadToken(_, Some(e)) => Some(e.as_ref()),
      Error::NetworkError(_, e) => Some(e.as_ref()),
      Error::InternalError(_, e) => Some(e.as_ref()),
      _ => None,
    }
  }
}
