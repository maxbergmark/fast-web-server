use crate::{http_version::HttpVersion, status_code::StatusCode};



#[derive(Debug, Default, Clone)]
pub struct StatusLine {
    pub protocol: HttpVersion,
    pub status_code: StatusCode,

}

impl Into<Vec<u8>> for StatusLine {
    fn into(self) -> Vec<u8> {
        let mut protocol: Vec<u8> = self.protocol.into();
        let mut status_code: Vec<u8> = self.status_code.into();
        let size = protocol.len() + status_code.len() + 3;
        let mut buf = Vec::with_capacity(size);

        buf.append(&mut protocol);
        buf.push(b' ');
        buf.append(&mut status_code);
        buf.extend(b"\r\n");
        buf
    }
}

#[cfg(test)]
mod tests {
    use crate::StatusLine;
    use crate::StatusCode;
    use crate::HttpVersion;

    #[test]
    fn to_vec() {
        let status_line = StatusLine { protocol: HttpVersion::HTTP1_0, status_code: StatusCode::Code200 };
        let expected = "HTTP/1.0 200 OK\r\n".as_bytes().to_vec();
        let actual: Vec<u8> = status_line.into();
        assert_eq!(expected, actual);
    }
}
