use crate::auth::{Auth, AuthError};
use reqwest::blocking::Client;
use std::fmt;
use std::fmt::Debug;
use bytes::Bytes;
use thiserror::Error;
use url::{ParseError, Url};
use log::debug;
use crate::error::ApiError;
use http::{Response as HttpResponse};
use http::request::Builder;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AlpacaError {
    #[error("failed to parse url: {}", source)]
    UrlParse {
        #[from]
        source: url::ParseError,
    },
}

impl From<reqwest::Error> for AlpacaError {
    fn from(error: reqwest::Error) -> Self {
        // Convert reqwest::Error into AlpacaError here
        // You may need to inspect `error` to decide how to handle it
        // For example:
        AlpacaError::UrlParse {
            source: ParseError::EmptyHost,
        }
    }
}
type AlpacaResult<T> = Result<T, AlpacaError>;

#[derive(Clone)]
pub struct Alpaca {
    /// The client to use for API calls.
    client: Client,
    /// The base URL to use for API calls.
    rest_url: Url,
    /// The authentication information to use when communicating with Alpaca.
    auth: Auth,
}

impl Alpaca {
    pub fn new<Host, ApiKey, ApiSecret>(
        host: Host,
        api_key: ApiKey,
        secret_key: ApiSecret,
    ) -> AlpacaResult<Self>
    where
        Host: AsRef<str>,
        ApiKey: Into<String>,
        ApiSecret: Into<String>,
    {
        let rest_url = Url::parse(&format!("https://{}/v2/", host.as_ref()))?;
        let auth = Auth::SecretTokens(api_key.into(), secret_key.into());
        let client = Client::new();

        let api = Alpaca {
            client,
            rest_url,
            auth,
        };

        Ok(api)
    }

    /// Perform a REST query with a given auth.
    fn rest_auth(
        &self,
        mut request: Builder,
        body: Vec<u8>,
        auth: &Auth,
    ) -> Result<HttpResponse<Bytes>, ApiError<<Self as crate::client::RestClient>::Error>> {
        let call = || -> Result<_, RestError> {
            println!("Am I here");
            auth.set_header(request.headers_mut().unwrap())?;
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;
            let rsp = self.client.execute(request)?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
            }
            Ok(http_rsp.body(rsp.bytes()?)?)
        };
        call().map_err(ApiError::client)
    }

}

impl Debug for Alpaca {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Alpaca")
            .field("rest_url", &self.rest_url)
            .finish()
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RestError {
    #[error("error setting auth header: {}", source)]
    AuthError {
        #[from]
        source: AuthError,
    },
    #[error("communication with gitlab: {}", source)]
    Communication {
        #[from]
        source: reqwest::Error,
    },
    #[error("`http` error: {}", source)]
    Http {
        #[from]
        source: http::Error,
    },
}



impl crate::client::RestClient for Alpaca {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>> {
        println!("{}", endpoint);
        debug!(target: "gitlab", "REST api call {}", endpoint);
        Ok(self.rest_url.join(endpoint)?)
    }
}

impl crate::client::Client for Alpaca {
    fn rest(
        &self,
        request: Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, ApiError<Self::Error>> {
        println!("{:#?}", request);
        self.rest_auth(request, body, &self.auth)
    }
}
