pub use crate::http::request;

use super::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::str;


pub struct Request<'buf> {
    pub path: &'buf str,
    pub query_string: Option<&'buf str>,
    pub method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf > {
    type Error = ParseError;

    // GET /about?param1=name&param2=age HTTP/1.1 
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        /*         
        match str::from_utf8(&buf) {
            Ok(request) => {},
            Err(_) => (),
        } 
        
        match str::from_utf8(&buf).or(Err(ParseError::InvalidEncoding)) {
            Ok(request) => {},
            Err(e) => return Err(e),
        }
        */       
        let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
        
        let (method, request) = split_words(request).ok_or(ParseError::InvalidRequest)?; 
        let (mut path, request) = split_words(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = split_words(request).ok_or(ParseError::InvalidRequest)?; 

        let method: Method = method.parse()?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let mut query: Option<&str> = None;
        if let Some(i) = path.find('?') {
            path = &path[..i]; 
            query = Some(&path[i + 1 ..]); 
        }
        
        Ok(Request { path: (path), query_string: (query), method: (method) })

    }
}

// Split a String Slice into 2 slices first slice contains first word and rest of the str
fn split_words(request: &str) -> Option<(&str, &str)> {
    for (i, val) in request.chars().enumerate() {
        if val == ' ' || val == '\n' || val == '\r' {
            return Some((&request[..i], &request[i+1..]));
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidMethod,
    InvalidProtocol,
    InvalidEncoding,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request", 
            Self::InvalidMethod => "Invalid Method", 
            Self::InvalidProtocol => "Invalid Protocol", 
            Self::InvalidEncoding => "Invalid Encoding", 
        }
    }    
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {
    
}