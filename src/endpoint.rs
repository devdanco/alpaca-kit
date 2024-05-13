use std::borrow::Cow;
use http::{self, header, Method, Request};
use serde::de::DeserializeOwned;
use url::Url;
use crate::client::{Client, RestClient};
use crate::error::{ApiError, BodyError};
use crate::params::QueryParams;
use crate::query;
use crate::query::Query;

/// URL bases for endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum UrlBase {
    /// An endpoint uses the API v4 URL prefix.
    ApiV2
}

impl UrlBase {
    /// Get the endpoint for a given URL base.
    pub fn endpoint_for<C>(&self, client: &C, endpoint: &str) -> Result<Url, ApiError<C::Error>>
        where
            C: RestClient,
    {
        match self {
            UrlBase::ApiV2 => client.rest_endpoint(endpoint)
        }
    }
}



/// A trait for providing the necessary information for a single REST API endpoint.
pub trait Endpoint {
    /// The HTTP method to use for the endpoint.
    fn method(&self) -> Method;
    /// The path to the endpoint.
    fn endpoint(&self) -> Cow<'static, str>;

    /// The URL base of the API endpoint.
    fn url_base(&self) -> UrlBase {
        UrlBase::ApiV2
    }

    /// Query parameters for the endpoint.
    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    /// The body for the endpoint.
    ///
    /// Returns the `Content-Encoding` header for the data as well as the data itself.
    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        Ok(None)
    }
}

impl<E> Endpoint for &E
    where
        E: Endpoint,
{
    fn method(&self) -> Method {
        (*self).method()
    }

    fn endpoint(&self) -> Cow<'static, str> {
        (*self).endpoint()
    }

    fn url_base(&self) -> UrlBase {
        (*self).url_base()
    }

    fn parameters(&self) -> QueryParams {
        (*self).parameters()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        (*self).body()
    }
}

impl<E, T, C> Query<T, C> for E
    where
        E: Endpoint,
        T: DeserializeOwned,
        C: Client,
{
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let mut url = self.url_base().endpoint_for(client, &self.endpoint())?;
        self.parameters().add_to_url(&mut url);

        let req = Request::builder()
            .method(self.method())
            .uri(query::url_to_http_uri(url));
        let (req, data) = if let Some((mime, data)) = self.body()? {
            let req = req.header(header::CONTENT_TYPE, mime);
            (req, data)
        } else {
            (req, Vec::new())
        };
        let rsp = client.rest(req, data)?;
        let status = rsp.status();
        let v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
            v
        } else {
            return Err(ApiError::server_error(status, rsp.body()));
        };
        if !status.is_success() {
            return Err(ApiError::server_error(status, rsp.body()));
        } else if status == http::StatusCode::MOVED_PERMANENTLY {
            return Err(ApiError::moved_permanently(
                rsp.headers().get(header::LOCATION),
            ));
        }
        serde_json::from_value::<T>(v).map_err(ApiError::data_type::<T>)
    }
}


