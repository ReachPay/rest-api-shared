use my_http_server::{RequestClaim, RequestCredentials};
use rust_extensions::date_time::DateTimeAsMicroseconds;

use super::TokenKey;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionClaim {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub expires: i64,
    #[prost(repeated, string, tag = "3")]
    pub ip: Vec<::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionToken {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub expires: i64,
    #[prost(string, tag = "3")]
    pub ip: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub claims: Vec<SessionClaim>,
}

impl RequestCredentials for SessionToken {
    fn get_id(&self) -> &str {
        &self.user_id
    }

    fn get_claims(&self) -> Option<Vec<RequestClaim>> {
        if self.claims.len() == 0 {
            return None;
        }

        let mut result = Vec::new();

        for claim in &self.claims {
            let item = RequestClaim {
                id: claim.id.as_str(),
                expires: DateTimeAsMicroseconds::new(claim.expires),
                allowed_ips: if claim.ip.len() == 0 {
                    None
                } else {
                    Some(&claim.ip)
                },
            };

            result.push(item);
        }

        Some(result)
    }
}

impl SessionToken {
    pub fn new(
        user_id: String,
        expires: DateTimeAsMicroseconds,
        ip: String,
        claims: Vec<SessionClaim>,
    ) -> Self {
        SessionToken {
            user_id,
            expires: expires.unix_microseconds,
            ip,
            claims,
        }
    }

    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }

    pub fn receive_user_id(self) -> String {
        self.user_id
    }

    pub fn get_expires_microseconds(&self) -> i64 {
        self.expires
    }

    pub fn into_token(&self, session_encryption_key: &TokenKey) -> String {
        let mut token_payload = Vec::new();
        prost::Message::encode(self, &mut token_payload).unwrap();

        let ciphertext = session_encryption_key
            .aes_key
            .encrypt(token_payload.as_slice());

        base64::encode(ciphertext)
    }

    pub fn parse_from_token(
        token_as_str: &str,
        session_encryption_key: &TokenKey,
    ) -> Option<SessionToken> {
        let encoded_token = base64::decode(token_as_str);

        if encoded_token.is_err() {
            return None;
        }

        let result = session_encryption_key
            .aes_key
            .decrypt(encoded_token.unwrap().as_slice());

        if result.is_err() {
            return None;
        }

        let result: Result<SessionToken, prost::DecodeError> =
            prost::Message::decode(result.unwrap().as_slice());

        if result.is_err() {
            return None;
        }

        Some(result.unwrap())
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_encrypt_decrypt() {
        use super::*;

        let token_key = TokenKey::new(b"012345678901234567890123456789012345678901234567");

        let session_token = SessionToken::new(
            "user_id".to_string(),
            DateTimeAsMicroseconds::now(),
            "127.0.0.1".to_string(),
            vec![],
        );

        let token_as_str = session_token.into_token(&token_key);

        let session_token_from_token = SessionToken::parse_from_token(&token_as_str, &token_key);

        print!("{:?}", session_token_from_token);

        assert_eq!(session_token, session_token_from_token.unwrap());
    }
}
