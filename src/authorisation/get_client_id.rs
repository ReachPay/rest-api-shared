use my_http_server::{HttpContext, HttpFailResult};

use crate::session_token::SessionToken;

pub trait GetClientId {
    fn get_client_id(&self) -> Result<&str, HttpFailResult>;
}

impl GetClientId for HttpContext<SessionToken> {
    fn get_client_id(&self) -> Result<&str, HttpFailResult> {
        if let Some(session_token) = &self.credentials {
            return Ok(&session_token.user_id);
        }

        return Err(HttpFailResult::as_unauthorized(
            "Can not get client Id. Looks like request is unathorised"
                .to_string()
                .into(),
        ));
    }
}
