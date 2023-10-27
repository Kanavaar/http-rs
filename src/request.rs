use crate::{url::Url, Method};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    components: RequestComponents,
    body: String,
}

impl Request {
    pub fn builder() -> RequestBuilder {
        RequestBuilder::default()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RequestBuilder {
    components: Option<RequestComponents>,
    body: Option<String>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn method(mut self, method: Method) -> Self {
        let components = match self.components {
            None => RequestComponents {
                method,
                ..Default::default()
            },
            Some(component) => RequestComponents {
                method,
                ..component
            },
        };

        self.components = Some(components);

        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        let url = url.into();
        let url = Url::new(url.as_str());
        let components = match self.components {
            None => RequestComponents {
                url,
                ..Default::default()
            },
            Some(component) => RequestComponents { url, ..component },
        };

        self.components = Some(components);

        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RequestComponents {
    method: Method,
    url: crate::url::Url,
    headers: crate::header::Headers,
}

impl Default for RequestComponents {
    fn default() -> Self {
        Self {
            method: Method::Get,
            url: Url::new(""),
            headers: Default::default(),
        }
    }
}
