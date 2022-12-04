use super::{UnathorizedRequestResponse, UnauthorizedReasonCode};

pub struct AuthFailResponseFactory;

impl my_http_server_controllers::controllers::AuthErrorFactory for AuthFailResponseFactory {
    fn get_not_authenticated(&self) -> my_http_server::HttpFailResult {
        return UnathorizedRequestResponse::new(
            UnauthorizedReasonCode::InvalidSessionToken,
            "Session token is either invalid or not presented".to_string(),
            None,
        );
    }

    fn get_not_authorized(&self, claim_name: String) -> my_http_server::HttpFailResult {
        return UnathorizedRequestResponse::new(
            UnauthorizedReasonCode::Unauthorzed,
            "Authorization required".to_string(),
            Some(claim_name),
        );
    }
}
