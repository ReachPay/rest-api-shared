use service_sdk::my_http_server::{HttpContext, HttpRequestHeaders};

const AUTH_HEADER: &str = "authorization";

pub trait GetSessionToken {
    fn get_session_token(&self) -> Option<&str>;
}

impl GetSessionToken for HttpContext {
    fn get_session_token(&self) -> Option<&str> {
        let auth_header = self.request.get_headers().try_get_case_sensitive(AUTH_HEADER)?;

        let token = extract_token(auth_header.value)?;

        match std::str::from_utf8(token) {
            Ok(result) => Some(result),
            Err(_) => None,
        }
    }
}

fn extract_token(src: &[u8]) -> Option<&[u8]> {
    if src.len() < 6 {
        return None;
    }

    if src[6] == b' ' {
        return Some(&src[7..]);
    }
    Some(src)
}
