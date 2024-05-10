use http::Uri;
use url::Url;
use crate::client::Client;
use crate::error::ApiError;

pub fn url_to_http_uri(url: Url) -> Uri {
    url.as_str()
        .parse::<Uri>()
        .expect("failed to parse a url::Url as an http::Uri")
}


/// A trait which represents a query which may be made to a Alpaca client.
pub trait Query<T, C>
    where
        C: Client,
{
    /// Perform the query against the client.
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}
