use crate::{status_line::StatusLine, HttpHeaders};



#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status_line: StatusLine,
    pub headers: HttpHeaders,
    pub body: String,
}

impl HttpResponse {

    pub fn from_body(body: String) -> Self {
        let mut headers = HttpHeaders::default();
        headers.insert(String::from("Content-Length"), body.len().to_string());
        headers.insert(String::from("Connection"), String::from("close"));
        Self {
            status_line: Default::default(),
            headers: headers,
            body,
        }
    }
}

impl From<HttpResponse> for Vec<u8> {
    fn from(http_response: HttpResponse) -> Self {
        let mut status_line: Vec<u8> = http_response.status_line.into();
        let mut headers: Vec<u8> = http_response.headers.into();
        let body = http_response.body.as_bytes();

        let size = status_line.len() + headers.len() + 4 + body.len();
        let mut buf = Vec::with_capacity(size);

        buf.append(&mut status_line);
        buf.append(&mut headers);
        buf.extend(b"\r\n");
        buf.extend_from_slice(body);
        buf
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_empty() {
        let response = HttpResponse::from_body(String::from(""));
        let expected = "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Length: 0\r\n\r\n".as_bytes().to_vec();
        let actual: Vec<u8> = response.into();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_into() {
        let response = HttpResponse::from_body("test".to_string());
        let expected = "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Length: 4\r\n\r\ntest".as_bytes().to_vec();
        let actual: Vec<u8> = response.into();
        assert_eq!(expected, actual);
    }
}