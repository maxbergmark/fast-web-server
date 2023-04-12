use std::collections::HashMap;



#[derive(Debug, PartialEq)]
pub struct RequestTarget {
    pub uri: String,
    pub request_params: HashMap<String, String>,
}

impl RequestTarget {
    pub fn new(s: &String) -> Result<Self, String> {
        let parts: Vec<String> = s.split("?").map(str::to_string).collect();
        match &parts[..] {
            [uri] => Ok(Self  {
                uri: uri.to_owned(),
                request_params: HashMap::default(),
            }),
            [uri, params] => {
                Self::with_request_params(uri, params)

            },
            _ => Err(String::from("Could not parse uri+params"))
        }
    }

    fn with_request_params(uri: &String, params: &String) -> Result<Self, String> {
        let mut request_params = HashMap::new();
        for param in params.split("&") {
            let pair: Vec<String> = param.split("=").map(str::to_string).collect();
            match &pair[..] {
                [key, value] => request_params.insert(key.to_owned(), value.to_owned()),
                _ => return Err(String::from("Could not parse request param")),
            };
        }
        Ok(Self {
            uri: uri.to_owned(),
            request_params,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_uri_only() {
        let s = String::from("/path/to/resource");
        let rt = RequestTarget::new(&s).unwrap();
        assert_eq!(rt.uri, "/path/to/resource");
        assert!(rt.request_params.is_empty());
    }

    #[test]
    fn test_new_with_uri_and_params() {
        let s = String::from("/path/to/resource?param1=value1&param2=value2");
        let rt = RequestTarget::new(&s).unwrap();
        assert_eq!(rt.uri, "/path/to/resource");
        assert_eq!(rt.request_params.get("param1"), Some(&String::from("value1")));
        assert_eq!(rt.request_params.get("param2"), Some(&String::from("value2")));
    }

    #[test]
    fn test_new_with_invalid_input() {
        // input with no URI or query parameters
        let s = String::from("");
        let _err: Result<RequestTarget, String> = Err(String::from("test"));
        assert!(matches!(RequestTarget::new(&s), _err));

        // input with invalid query parameters
        let s = String::from("/path/to/resource?");
        assert!(matches!(RequestTarget::new(&s), _err));

        let s = String::from("/path/to/resource?key=");
        assert!(matches!(RequestTarget::new(&s), _err));

        let s = String::from("/path/to/resource?=value");
        assert!(matches!(RequestTarget::new(&s), _err));
    }
}