use aes::{
    cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt},
    Aes256,
};

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

    pub fn as_token(&self, cipher: &Aes256) -> String {
        let mut token_payload = Vec::new();
        prost::Message::encode(self, &mut token_payload).unwrap();

        let mut block = GenericArray::from_iter(token_payload);

        cipher.encrypt_block(&mut block);

        base64::encode(block.iter().as_slice())
    }

    pub fn parse_from_token(token_as_str: &str, cipher: &Aes256) -> Option<SessionToken> {
        let token_as_vec = base64::decode(token_as_str);

        if token_as_vec.is_err() {
            return None;
        }

        let token_as_vec = token_as_vec.unwrap();

        let mut block = GenericArray::from_iter(token_as_vec);

        cipher.decrypt_block(&mut block);

        let result: Result<SessionToken, prost::DecodeError> =
            prost::Message::decode(block.iter().as_slice());

        if result.is_err() {
            return None;
        }

        Some(result.unwrap())
    }
}

#[cfg(test)]
mod test {

    use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
    use aes::Aes256;

    #[test]
    fn test_encrypt_decrypt() {
        let key = GenericArray::from_slice([0u8; 32].as_slice());

        let mut block = GenericArray::from([42u8; 16]);

        let cipher = Aes256::new(&key);

        let block_copy = block.clone();

        // Encrypt block in-place
        cipher.encrypt_block(&mut block);

        println!("{:?}", block);

        // And decrypt it back
        cipher.decrypt_block(&mut block);

        println!("{:?}", block);
        assert_eq!(block, block_copy);
    }
}
