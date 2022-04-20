pub struct TokenKey {
    pub key: String,
}

impl TokenKey {
    pub fn new() -> Self {
        let result = std::env::var("TOKEN_KEY");

        if result.is_err() {
            panic!("TOKEN_KEY environment variable is not set");
        }

        Self::from_string_token(result.unwrap().as_str())
    }

    pub fn from_string_token(token_key: &str) -> Self {
        if token_key.len() != 32 {
            panic!("TOKEN_KEY is not 32 characters long");
        }
        Self {
            key: token_key.to_string(),
        }
    }
}
