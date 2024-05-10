use std::any;
use std::error::Error;
use thiserror::Error;
use crate::auth::AuthError;
use crate::endpoint::UrlBase;

/// Errors which may occur when creating form data.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BodyError {
    /// Body data could not be serialized from form parameters.
    #[error("failed to URL encode form parameters: {}", source)]
    UrlEncoded {
        /// The source of the error.
        #[from]
        source: serde_urlencoded::ser::Error,
    },
    /// Body data could not be serialized to JSON from form parameters.
    #[error("failed to JSON encode form parameters: {}", source)]
    JsonEncoded {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },
}


/// Errors which may occur when using API endpoints.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ApiError<E>
    where
        E: Error + Send + Sync + 'static,
{
    /// The client encountered an error.
    #[error("client error: {}", source)]
    Client {
        /// The client error.
        source: E,
    },
    /// Authentication failed.
    #[error("failed to authenticate: {}", source)]
    Auth {
        /// The source of the error.
        #[from]
        source: AuthError,
    },
    /// The URL failed to parse.
    #[error("failed to parse url: {}", source)]
    UrlParse {
        /// The source of the error.
        #[from]
        source: url::ParseError,
    },
    /// Body data could not be created.
    #[error("failed to create form data: {}", source)]
    Body {
        /// The source of the error.
        #[from]
        source: BodyError,
    },
    /// Alpaca returned an error without JSON information.
    #[error("alpaca internal server error {}", status)]
    AlpacaService {
        /// The status code for the return.
        status: http::StatusCode,
        /// The error data from Alpaca.
        data: Vec<u8>,
    },

    /// JSON deserialization from GitLab failed.
    #[error("could not parse JSON response: {}", source)]
    Json {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },
    /// The resource has been moved permanently.
    #[error("moved permanently to: {}", location.as_ref().map(AsRef::as_ref).unwrap_or("<UNKNOWN>"))]
    MovedPermanently {
        /// The new location for the resource.
        location: Option<String>,
    },
    /// Alpaca returned an error message.
    #[error("alpaca server error: {}", msg)]
    Alpaca {
        /// The error message from Alpaca.
        msg: String,
    },
    /// Failed to parse an expected data type from JSON.
    #[error("could not parse {} data from JSON: {}", typename, source)]
    DataType {
        /// The source of the error.
        source: serde_json::Error,
        /// The name of the type that could not be deserialized.
        typename: &'static str,
    },
    /// The client does not understand how to use an endpoint for the given URL base.
    #[error("unsupported URL base: {:?}", url_base)]
    UnsupportedUrlBase {
        /// The URL base that is not supported.
        url_base: UrlBase,
    },
}

impl<E> ApiError<E>
    where
        E: Error + Send + Sync + 'static,
{
    /// Create an API error in a client error.
    pub fn client(source: E) -> Self {
        ApiError::Client {
            source,
        }
    }

    /// Wrap a client error in another wrapper.
    pub fn map_client<F, W>(self, f: F) -> ApiError<W>
        where
            F: FnOnce(E) -> W,
            W: Error + Send + Sync + 'static,
    {
        match self {
            Self::Client {
                source,
            } => ApiError::client(f(source)),
            Self::UrlParse {
                source,
            } => {
                ApiError::UrlParse {
                    source,
                }
            },
            Self::Auth {
                source,
            } => {
                ApiError::Auth {
                    source,
                }
            },
            Self::Body {
                source,
            } => {
                ApiError::Body {
                    source,
                }
            },
            Self::Json {
                source,
            } => {
                ApiError::Json {
                    source,
                }
            },
            Self::MovedPermanently {
                location,
            } => {
                ApiError::MovedPermanently {
                    location,
                }
            },
            Self::AlpacaService {
                status,
                data,
            } => {
                ApiError::AlpacaService {
                    status,
                    data,
                }
            }

            Self::DataType {
                source,
                typename,
            } => {
                ApiError::DataType {
                    source,
                    typename,
                }
            },
            Self::UnsupportedUrlBase {
                url_base,
            } => {
                ApiError::UnsupportedUrlBase {
                    url_base,
                }
            },
            Self::Alpaca {
                msg
            } => {
                ApiError::Alpaca {
                    msg,
                }
            }
        }
    }

    pub(crate) fn moved_permanently(raw_location: Option<&http::HeaderValue>) -> Self {
        let location = raw_location.map(|v| String::from_utf8_lossy(v.as_bytes()).into());
        Self::MovedPermanently {
            location,
        }
    }

    pub(crate) fn data_type<T>(source: serde_json::Error) -> Self {
        ApiError::DataType {
            source,
            typename: any::type_name::<T>(),
        }
    }

    pub(crate) fn server_error(status: http::StatusCode, body: &bytes::Bytes) -> Self {
        Self::AlpacaService {
            status,
            data: body.into_iter().copied().collect(),
        }
    }


    pub(crate) fn unsupported_url_base(url_base: UrlBase) -> Self {
        Self::UnsupportedUrlBase {
            url_base,
        }
    }
}

