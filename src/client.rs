use std::error::Error;
use url::Url;
use crate::endpoint::UrlBase;
use crate::error::ApiError;
use http::request::Builder as RequestBuilder;
use http::Response;
use bytes::Bytes;

/// A trait representing a client which can communicate with a GitLab instance via REST.
pub trait RestClient {
    /// The errors which may occur for this client.
    type Error: Error + Send + Sync + 'static;

    /// Get the URL for a REST v4 endpoint for the client.
    ///
    /// This method adds the hostname for the client's target instance.
    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>>;

    /// Get the URL for an instance endpoint for the client.
    fn instance_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>> {
        let _ = endpoint;
        Err(ApiError::unsupported_url_base(UrlBase::Instance))
    }
}

/// A trait representing a client which can communicate with a GitLab instance.
pub trait Client: RestClient {
    /// Send a REST query.
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}
