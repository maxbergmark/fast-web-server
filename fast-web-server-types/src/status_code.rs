

#[derive(Debug, Default, Clone)]
pub enum StatusCode {
    #[default]
    Code200,
    Code404,
}



impl StatusCode {
    pub const fn to_string(&self) -> &'static str {
        match self {
            StatusCode::Code200 => "200 OK",
            StatusCode::Code404 => "404 Not Found",
        }
    }
}


use std::borrow::Cow;

impl From<StatusCode> for Cow<'static, [u8]> {
    fn from(status_code: StatusCode) -> Self {
        status_code.to_string().as_bytes().into()
    }
}

impl Into<Vec<u8>> for StatusCode {
    fn into(self) -> Vec<u8> {
        self.to_string().as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::StatusCode;

    #[test]
    fn ok() {
        assert_eq!("200 OK", StatusCode::Code200.to_string());
    }
    
    #[test]
    fn to_vec() {
        let expected = "200 OK".as_bytes().to_vec();
        let actual: Vec<u8> = StatusCode::Code200.into();
        assert_eq!(expected, actual)
    }
}