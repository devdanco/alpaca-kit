use std::borrow::Cow;
use derive_builder::Builder;
use http::Method;
use crate::endpoint::Endpoint;

/// Query information about the API calling user.
#[derive(Debug, Clone, Copy, Builder)]
pub struct TradingAccount {}


impl TradingAccount {
    /// Create a builder for the endpoint.
    pub fn builder() -> TradingAccountBuilder {
        TradingAccountBuilder::default()
    }
}


impl Endpoint for TradingAccount {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "account".into()
    }
}
