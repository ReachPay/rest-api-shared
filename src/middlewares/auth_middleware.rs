use std::collections::HashMap;

use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpServerMiddleware, HttpServerRequestFlow,
};
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::session_token::SessionToken;

const AUTH_HEADER: &str = "authorization";
pub const KV_USER_ID: &str = "USER_ID";

pub struct AuthMiddleware {
    ignore_paths: Option<HashMap<String, String>>,
}

impl AuthMiddleware {
    pub fn new() -> Self {
        Self { ignore_paths: None }
    }

    pub fn path_is_ignored(&self, path: &str) -> bool {
        if let Some(ref items) = self.ignore_paths {
            return items.contains_key(path);
        }

        return false;
    }
}

#[async_trait::async_trait]
impl HttpServerMiddleware for AuthMiddleware {
    async fn handle_request(
        &self,
        ctx: &mut HttpContext,
        get_next: &mut HttpServerRequestFlow,
    ) -> Result<HttpOkResult, HttpFailResult> {
        let path = ctx.request.get_path_lower_case();
        if self.path_is_ignored(path) {
            return get_next.next(ctx).await;
        }

        match ctx.request.get_headers().get(AUTH_HEADER) {
            Some(header) => {
                if let Some(session_token) = SessionToken::parse_from_header(header.as_bytes()) {
                    let now = DateTimeAsMicroseconds::now();

                    if session_token.get_expires_microseconds() >= now.unix_microseconds {
                        return Err(HttpFailResult::as_unauthorized(
                            "Token is expired".to_string().into(),
                        ));
                    }

                    ctx.request.set_key_value(
                        KV_USER_ID.to_string(),
                        session_token.receive_user_id().into_bytes(),
                    );
                    return get_next.next(ctx).await;
                } else {
                    return Err(HttpFailResult::as_unauthorized(
                        "Invalid token".to_string().into(),
                    ));
                }
            }
            None => {
                return Err(HttpFailResult::as_unauthorized(
                    "Token is missing".to_string().into(),
                ));
            }
        }
    }
}
