pub struct TokenKey {
    pub aes_key: encryption::aes::AesKey,
}

impl TokenKey {
    pub fn new(key: &[u8]) -> Self {
        TokenKey {
            aes_key: encryption::aes::AesKey::new(key),
        }
    }
}
