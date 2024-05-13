use std::borrow::Cow;
use derive_builder::Builder;
use http::Method;
use crate::endpoint::Endpoint;

/// Query information about the API calling user.
#[derive(Debug, Clone, Copy, Builder)]
pub struct OptionsContract {}


impl OptionsContract {
    /// Create a builder for the endpoint.
    pub fn builder() -> OptionsContractBuilder {
        OptionsContractBuilder::default()
    }
}


impl Endpoint for OptionsContract {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "options/contracts".into()
    }
}
