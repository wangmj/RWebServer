#[derive(Debug,Default)]
pub enum HttpMethod{
    Get,
    Post,
    Put,
    Delete,
    Options,
    Other(String)
}

pub trait ToHttpMethod{
    fn to_http_method(&self)->HttpMethod;
}
impl ToHttpMethod for String {
    fn to_http_method(&self)->HttpMethod {
        match self.to_lowercase().as_str() {
            "get"=>HttpMethod::Get,
            "post"=>HttpMethod::Post,
            "put"=>HttpMethod::Put,
            "options"=>HttpMethod::Options,
            "delete"=>HttpMethod::Delete,
            _=>HttpMethod::Other(self.clone())
        }
    }
}
impl ToHttpMethod for &str{
    fn to_http_method(&self)->HttpMethod {
        match self.to_lowercase().as_str() {
            "get"=>HttpMethod::Get,
            "post"=>HttpMethod::Post,
            "put"=>HttpMethod::Put,
            "options"=>HttpMethod::Options,
            "delete"=>HttpMethod::Delete,
            _=>HttpMethod::Other(self.to_string())
        }
    }
}