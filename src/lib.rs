#[derive(Debug, PartialEq)]
pub enum Error {
    CurlError(curl::Error),
}

impl From<curl::Error> for Error {
    fn from(err: curl::Error) -> Self {
        Self::CurlError(err)
    }
}

mod internal {
    use curl;

    pub fn init(api_key: &str, url: &str) -> Result<curl::easy::Easy, super::Error> {
        let mut easy = curl::easy::Easy::new();
        easy.url(url)?;
        let mut headers = curl::easy::List::new();
        headers.append(&format!("Authorization: Bearer {}", api_key))?;
        headers.append("Content-Type: application/json")?;
        headers.append("Accept: text/event-stream")?;
        easy.http_headers(headers)?;
        Ok(easy)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn init_handle() {
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not defined");
        let result = crate::internal::init(&api_key, "https://api.openai.com/v1/models");
        assert!(result.is_ok());
    }
}