use std::{io::BufRead, error::Error};
use thiserror::Error;

use crate::{request_type::RequestType, request_target::RequestTarget, http_version::HttpVersion};


#[derive(Debug, PartialEq)]
pub struct StartLine {
    pub request_type: RequestType,
    pub request_target: RequestTarget,
    pub http_version: HttpVersion,
}

impl StartLine {
    pub fn new(reader: &mut dyn BufRead) -> Result<Self, Box<dyn Error>> {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        line = line.trim().to_string();

        let parts: Vec<String> = line.split(" ").map(str::to_string).collect();
        match &parts[..] {
            [request_type, request_target, http_version] => Ok(Self {
                request_type: RequestType::from_string(request_type)?,
                request_target: RequestTarget::new(request_target)?,
                http_version: HttpVersion::from_string(http_version),
            }),
            _ => Err(StartLineError(line.clone(), line.len()).into()),
        }
    }
}
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Could not parse start line {0} of length {1}")]
struct StartLineError(String, usize);

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    #[test]
    fn test_start_line_new_valid_input() {
        let input = "GET /index.html HTTP/1.1\n".as_bytes();
        let mut reader = BufReader::new(input);

        let expected = StartLine {
            request_type: RequestType::GET,
            request_target: RequestTarget::new(&String::from("/index.html")).unwrap(),
            http_version: HttpVersion::HTTP1_1,
        };

        let actual = StartLine::new(&mut reader).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_start_line_new_invalid_input() {
        let input = "INVALID\n".as_bytes();
        let mut reader = BufReader::new(input);

        let expected = StartLineError("INVALID".to_string(), 7);
        let actual = StartLine::new(&mut reader).unwrap_err();
        assert_eq!(actual.to_string(), expected.to_string());
    }
}