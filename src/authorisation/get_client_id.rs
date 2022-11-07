use my_http_server::{HttpContext, HttpFailResult};

pub trait GetClientId {
    fn get_client_id(&self) -> Result<&str, HttpFailResult>;
}

impl GetClientId for HttpContext {
    fn get_client_id(&self) -> Result<&str, HttpFailResult> {
        if let Some(session_token) = &self.credentials {
            return Ok(&session_token.get_id());
        }

        return Err(HttpFailResult::as_unauthorized(
            "Can not get client Id. Looks like request is unathorised"
                .to_string()
                .into(),
        ));
    }
}
