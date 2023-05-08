use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpPath, HttpServerMiddleware,
    HttpServerRequestFlow,
};
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::session_token::{SessionToken, TokenKey};

use super::UnathorizedRequestResponse;

const AUTH_HEADER: &str = "authorization";

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
    async fn handle_request(
        &self,
        ctx: &mut HttpContext,
        get_next: &mut HttpServerRequestFlow,
    ) -> Result<HttpOkResult, HttpFailResult> {
        if self.path_is_ignored(&ctx.request.http_path) {
            return get_next.next(ctx).await;
        }

        match ctx.request.get_header(AUTH_HEADER) {
            Some(header) => {
                if let Some(session_token) = SessionToken::parse_from_token(
                    std::str::from_utf8(extract_token(header.as_bytes())).unwrap(),
                    &self.token_key,
                ) {
                    let now = DateTimeAsMicroseconds::now();

                    if now.unix_microseconds >= session_token.get_expires_microseconds() {
                        return Err(UnathorizedRequestResponse::new(
                            super::UnauthorizedReasonCode::SessionTokenIsExpired,
                            "Session token is expired".to_string(),
                            None,
                        ));
                    }

                    ctx.credentials = Some(Box::new(session_token));
                    return get_next.next(ctx).await;
                } else {
                    return Err(UnathorizedRequestResponse::new(
                        super::UnauthorizedReasonCode::InvalidSessionToken,
                        "Invalid session token".to_string(),
                        None,
                    ));
                }
            }
            None => {
                return Err(UnathorizedRequestResponse::new(
                    super::UnauthorizedReasonCode::InvalidSessionToken,
                    "No session token found".to_string(),
                    None,
                ));
            }
        }
    }
}

fn extract_token(src: &[u8]) -> &[u8] {
    if src[6] == b' ' {
        return &src[7..];
    }
    src
}

#[cfg(test)]
mod tests {
    use crate::middlewares::auth_middleware::extract_token;

    #[test]
    fn test_extract_token() {
        let src = b"Bearer 1234567890";
        let result = extract_token(src);
        assert_eq!(result, b"1234567890");
    }

    #[test]
    fn test_extract_token_without_bearer() {
        let src = b"1234567890";
        let result = extract_token(src);
        assert_eq!(result, b"1234567890");
    }
}
