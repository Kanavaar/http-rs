pub type HeaderMap = std::collections::HashMap<String, String>;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Header(HeaderMap);

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut headers = String::new();
        for (header, value) in self.get_map() {
            headers.push_str(format!("{header}: {value}\r\n").as_str());
        }
        write!(f, "{}", headers)
    }
}

impl From<HeaderMap> for Header {
    fn from(value: HeaderMap) -> Self {
        Self(value)
    }
}

impl Header {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_map(&self) -> &HeaderMap {
        &self.0
    }

    pub fn get_mut_map(&mut self) -> &mut HeaderMap {
        &mut self.0
    }

    pub fn insert(&mut self, header: impl Into<String>, value: impl Into<String>) {
        self.0.insert(header.into(), value.into());
    }

    /// Removes the entry of Header and value returning neither
    /// Different from std, as std's `HashMap::remove` return the
    /// value of the deleted key
    /// to get that behavior use `get_mut_map`
    /// ```
    /// use http_rs::header::Header;
    /// 
    /// let mut header = Header::new();
    /// header.insert("Key", "Value");
    /// let mut map = header.get_mut_map();
    /// let value = map.remove("Key").unwrap(); // "Value"
    /// ```
    pub fn remove(&mut self, header: impl Into<String>) {
        self.0.remove(&header.into());
    }
}
