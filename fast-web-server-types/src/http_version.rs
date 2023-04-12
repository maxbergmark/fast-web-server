

#[derive(Debug, Default, Clone, PartialEq)]
pub enum HttpVersion {
    HTTP1_0,
    #[default]
    HTTP1_1,
}

impl HttpVersion {
    pub fn to_string(&self) -> String {
        match self {
            HttpVersion::HTTP1_0 => "HTTP/1.0",
            HttpVersion::HTTP1_1 => "HTTP/1.1",
        }.to_string()
    }

    pub fn from_string(s: &String) -> Self {
        match s.as_str() {
            "HTTP/1.0" => Self::HTTP1_0,
            "HTTP/1.1" => Self::HTTP1_1,
            _ => unimplemented!(),
        }
    }
}

impl From<HttpVersion> for Vec<u8> {
    fn from(http_version: HttpVersion) -> Self {
        http_version.to_string().into_bytes()
    }
}