use service_sdk::my_http_server::{HttpContext, HttpFailResult};

pub trait RequestExtensions {
    fn get_user_id(&self) -> Result<&str, HttpFailResult>;
}

impl RequestExtensions for HttpContext {
    fn get_user_id(&self) -> Result<&str, HttpFailResult> {
        match &self.credentials {
            Some(value) => {
                return Ok(value.get_id());
            }
            None => Err(HttpFailResult::as_unauthorized(
                "User id is not found".to_string().into(),
            )),
        }
    }
}
