use rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionToken {
    #[prost(string, tag = "1")]
    user_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    expires: i64,
}

impl SessionToken {
    pub fn new(user_id: String, expires: DateTimeAsMicroseconds) -> Self {
        SessionToken {
            user_id,
            expires: expires.unix_microseconds,
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

    pub fn parse_from_header(payload: &[u8]) -> Option<SessionToken> {
        let result: Result<SessionToken, prost::DecodeError> = prost::Message::decode(payload);

        if result.is_err() {
            return None;
        }

        Some(result.unwrap())
    }
}
