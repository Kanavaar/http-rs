pub type HeaderMap = std::collections::HashMap<String, String>;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Headers(HeaderMap);

impl std::fmt::Display for Headers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut headers = String::new();
        for (header, value) in self.get_map() {
            headers.push_str(format!("{header}: {value}\r\n").as_str());
        }
        write!(f, "{}", headers)
    }
}

impl Headers {
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
    pub fn remove(&mut self, header: impl Into<String>) {
        self.0.remove(&header.into());
    }
}