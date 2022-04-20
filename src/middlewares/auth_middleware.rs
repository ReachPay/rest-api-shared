use std::collections::HashMap;

use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpServerMiddleware, HttpServerRequestFlow,
};
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::session_token::SessionToken;

const AUTH_HEADER: &str = "authorization";
pub const KV_USER_ID: &str = "USER_ID";

pub struct AuthMiddleware {
    token_key: Vec<u8>,
    ignore_full_paths: Option<HashMap<String, String>>,
    ignore_start_path: Option<Vec<String>>,
}

impl AuthMiddleware {
    pub fn new() -> Self {
        Self {
            token_key: comb_the_key(get_token_key().as_slice()),
            ignore_full_paths: None,
            ignore_start_path: None,
        }
    }

    pub fn new_with_default_paths_to_ignore() -> Self {
        let mut result = Self::new();
        result.add_start_path_to_ignore("/swagger");
        result
    }

    pub fn path_is_ignored(&self, path: &str) -> bool {
        if let Some(ref items) = self.ignore_full_paths {
            return items.contains_key(path);
        }

        if let Some(ref items) = self.ignore_start_path {
            for item in items {
                if path.starts_with(item) {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn add_full_path_to_ignore(&mut self, path: &str) {
        if self.ignore_full_paths.is_none() {
            self.ignore_full_paths = Some(HashMap::new());
        }

        self.ignore_full_paths
            .as_mut()
            .unwrap()
            .insert(path.to_string(), path.to_string());
    }

    pub fn add_start_path_to_ignore(&mut self, path: &str) {
        if self.ignore_start_path.is_none() {
            self.ignore_start_path = Some(Vec::new());
        }

        self.ignore_start_path
            .as_mut()
            .unwrap()
            .push(path.to_string());
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
                if let Some(session_token) = SessionToken::parse_from_header(
                    header.as_bytes().to_vec(),
                    self.token_key.clone(),
                ) {
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

fn comb_the_key(token_key: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(32);

    let mut i: usize = 0;
    while result.len() < result.capacity() {
        result.push(token_key[i]);
        i += 1;

        if i >= token_key.len() {
            i = 0;
        }
    }

    result
}

fn get_token_key() -> Vec<u8> {
    match std::env::var("TOKEN_KEY") {
        Ok(result) => result.into_bytes(),
        Err(_) => {
            panic!("Can not read environment variable TOKEN_KEY");
        }
    }
}
