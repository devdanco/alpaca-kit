use http::{header, Request};
use crate::client::Client;
use crate::endpoint::Endpoint;
use crate::error::ApiError;
use crate::query;
use crate::query::Query;

/// A query modifier that returns the raw data from the endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Raw<E> {
    endpoint: E,
}

/// Return the raw data from the endpoint.
pub fn raw<E>(endpoint: E) -> Raw<E> {
    Raw {
        endpoint,
    }
}

impl<E, C> Query<Vec<u8>, C> for Raw<E>
    where
        E: Endpoint,
        C: Client,
{
    fn query(&self, client: &C) -> Result<Vec<u8>, ApiError<C::Error>> {
        let mut url = self
            .endpoint
            .url_base()
            .endpoint_for(client, &self.endpoint.endpoint())?;
        self.endpoint.parameters().add_to_url(&mut url);

        let req = Request::builder()
            .method(self.endpoint.method())
            .uri(query::url_to_http_uri(url));
        let (req, data) = if let Some((mime, data)) = self.endpoint.body()? {
            let req = req.header(header::CONTENT_TYPE, mime);
            (req, data)
        } else {
            (req, Vec::new())
        };
        let rsp = client.rest(req, data)?;
        let status = rsp.status();
        if !status.is_success() {
            let v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
                v
            } else {
                return Err(ApiError::server_error(status, rsp.body()));
            };
            return Err(ApiError::Alpaca {
                msg: v,
            });
        } else if status == http::StatusCode::MOVED_PERMANENTLY {
            return Err(ApiError::moved_permanently(
                rsp.headers().get(http::header::LOCATION),
            ));
        }

        Ok(rsp.into_body().as_ref().into())
    }
}

