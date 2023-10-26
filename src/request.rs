#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    components: RequestComponents,
    body: String,
}


#[derive(Clone, Debug, PartialEq, Eq)]
struct RequestComponents {
    method: crate::Method,
    url: crate::url::Url,
    headers: crate::header::Headers,
}
