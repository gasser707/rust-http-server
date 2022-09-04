use super::StatusCode;
use std::io::{Result as IOResult, Write};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    net::TcpStream,
};
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    // write to tcp stream
    pub fn send(&self, stream: &mut impl Write) -> IOResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}

// write to formatter

// impl Display for Response {
//     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
//         let body = match  &self.body {
//             Some(b) => b ,
//             None => "",
//         };
//         write!(
//             f,
//             "HTTP/1.1 {} {}\r\n\r\n{}",
//             self.status_code,
//             self.status_code.reason_phrase(),
//             body
//         )
//     }
// }
