use http::{HeaderMap, HeaderValue};
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthError {
    #[error("header value error: {}", source)]
    HeaderValue {
        #[from]
        source: http::header::InvalidHeaderValue,
    },
}

type AuthResult<T> = Result<T, AuthError>;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Auth {
    SecretTokens(String, String),
}

impl Auth {
    pub fn set_header<'a>(
        &self,
        headers: &'a mut HeaderMap<HeaderValue>,
    ) -> AuthResult<&'a mut HeaderMap<HeaderValue>> {
        match self {
            Auth::SecretTokens(api_key, secret_key) => {
                let mut api_key_header_value = HeaderValue::from_str(api_key)?;
                let mut secret_key_header_value = HeaderValue::from_str(secret_key)?;
                let accept_header_value = HeaderValue::from_str("application/json")?;
                api_key_header_value.set_sensitive(true);
                secret_key_header_value.set_sensitive(true);
                headers.insert("APCA-API-KEY-ID", api_key_header_value);
                headers.insert("APCA-API-SECRET-KEY", secret_key_header_value);
                headers.insert("accept", accept_header_value);
            }
            _ => {}
        }

        Ok(headers)
    }
}
