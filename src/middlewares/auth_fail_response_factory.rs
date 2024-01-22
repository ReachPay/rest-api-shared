service_sdk::macros::use_my_http_server!();
use my_http_server::controllers::documentation::{
    data_types::HttpDataType, out_results::HttpResult,
};

use super::{UnauthorizedReasonCode, UnauthorizedRequestResponse};

pub struct AuthFailResponseFactory;

impl my_http_server::controllers::AuthErrorFactory for AuthFailResponseFactory {
    fn get_not_authenticated(&self) -> my_http_server::HttpFailResult {
        return UnauthorizedRequestResponse::new(
            UnauthorizedReasonCode::InvalidSessionToken,
            "Session token is either invalid or not presented".to_string(),
            None,
        );
    }

    fn get_not_authorized(&self, claim_name: String) -> my_http_server::HttpFailResult {
        return UnauthorizedRequestResponse::new(
            UnauthorizedReasonCode::Unauthorzed,
            "Authorization required".to_string(),
            Some(claim_name),
        );
    }
    fn get_global_http_fail_result_types(&self) -> Option<Vec<HttpResult>> {
        let http_object_structure = UnauthorizedRequestResponse::get_http_data_structure();
        Some(vec![HttpResult {
            http_code: 401,
            nullable: false,
            description: "Unauthorized".to_string(),
            data_type: HttpDataType::Object(http_object_structure),
        }])
    }
}
