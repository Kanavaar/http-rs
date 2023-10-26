#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Http,
    Https,
    None,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let protocol = match self {
            Self::Http => "http://",
            Self::Https => "https://",
            Self::None => "",
        };

        write!(f, "{}", protocol)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Url {
    protocol: Protocol,
    host: String,
    path: String,
}

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.protocol, self.host, self.path)
    }
}

impl Url {
    pub fn new(url: &str) -> Self {
        let mut protocol = Protocol::None;

        if url.contains("https://") {
            protocol = Protocol::Https;
        } else if url.contains("http://") {
            protocol = Protocol::Http
        }

        // This will turl into the path when draining
        let mut url_no_proto = String::new();
        match protocol {
            Protocol::Http => url_no_proto.push_str(&url[7..]),
            Protocol::Https => url_no_proto.push_str(&url[8..]),
            Protocol::None => url_no_proto.push_str(url),
        }

        let path_offset = url_no_proto.find('/').unwrap_or(url_no_proto.len());
        // url_no_proto now only contains the path
        let host: String = url_no_proto.drain(..path_offset).collect();

        if url_no_proto.is_empty() {
            url_no_proto.push('/');
        }

        Self {
            protocol,
            host,
            path: url_no_proto,
        }
    }

    pub fn protocol(&self) -> Protocol {
        self.protocol
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn  test_new_url() {
        let url = Url::new("https://example.com/no_there/lol/index.html");
        assert_eq!(url.to_string(), String::from("https://example.com/no_there/lol/index.html"));
        assert_eq!(url.protocol(), Protocol::Https);
        assert_eq!(url.host(), String::from("example.com"));
        assert_eq!(url.path(), String::from("/no_there/lol/index.html"));

    }
}