use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpPath, HttpServerMiddleware,
    HttpServerRequestFlow,
};
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::session_token::{SessionToken, TokenKey};

const AUTH_HEADER: &str = "authorization";
pub const KV_USER_ID: &str = "USER_ID";

pub struct AuthMiddleware {
    token_key: TokenKey,
    ignore_full_paths: Option<Vec<HttpPath>>,
    ignore_start_path: Option<Vec<HttpPath>>,
}

impl AuthMiddleware {
    pub fn new(token_key: TokenKey) -> Self {
        Self {
            token_key,
            ignore_full_paths: None,
            ignore_start_path: None,
        }
    }

    pub fn new_with_default_paths_to_ignore(token_key: TokenKey) -> Self {
        let mut result = Self::new(token_key);
        result.add_start_path_to_ignore("/swagger");
        result
    }

    pub fn path_is_ignored(&self, http_path: &HttpPath) -> bool {
        if let Some(ref items) = self.ignore_full_paths {
            for full_path_to_ignore in items {
                if http_path.is_the_same_to(full_path_to_ignore) {
                    return true;
                }
            }
        }

        if let Some(ref items) = self.ignore_start_path {
            for start_path_to_ignore in items {
                if http_path.is_starting_with(start_path_to_ignore) {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn add_full_path_to_ignore(&mut self, path: &str) {
        if self.ignore_full_paths.is_none() {
            self.ignore_full_paths = Some(Vec::new());
        }

        self.ignore_full_paths
            .as_mut()
            .unwrap()
            .push(HttpPath::from_str(path));
    }

    pub fn add_start_path_to_ignore(&mut self, path: &str) {
        if self.ignore_start_path.is_none() {
            self.ignore_start_path = Some(Vec::new());
        }

        self.ignore_start_path
            .as_mut()
            .unwrap()
            .push(HttpPath::from_str(path));
    }
}

#[async_trait::async_trait]
impl HttpServerMiddleware for AuthMiddleware {
    type TRequestCredentials = SessionToken;
    async fn handle_request(
        &self,
        ctx: &mut HttpContext<SessionToken>,
        get_next: &mut HttpServerRequestFlow<SessionToken>,
    ) -> Result<HttpOkResult, HttpFailResult> {
        if self.path_is_ignored(&ctx.request.http_path) {
            return get_next.next(ctx).await;
        }

        match ctx.request.get_headers().get(AUTH_HEADER) {
            Some(header) => {
                if let Some(session_token) = SessionToken::parse_from_token(
                    std::str::from_utf8(header.as_bytes()).unwrap(),
                    &self.token_key,
                ) {
                    let now = DateTimeAsMicroseconds::now();

                    if now.unix_microseconds >= session_token.get_expires_microseconds() {
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
