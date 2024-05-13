use std::borrow::Cow;
use http::Method;
use crate::endpoint::Endpoint;
use typed_builder::TypedBuilder;

/// Query information about the API calling user.
#[derive(Debug, Clone, TypedBuilder)]
pub struct Asset {
    symbol_or_asset_id: String
}


impl Endpoint for Asset {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("assets/{}", self.symbol_or_asset_id).into()
    }
}
