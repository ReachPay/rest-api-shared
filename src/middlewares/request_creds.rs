use std::sync::Arc;

use service_sdk::my_http_server;
use service_sdk::my_http_server::{RequestClaim, RequestCredentials};
use service_sdk::rust_extensions::date_time::DateTimeAsMicroseconds;

use super::SessionEntity;

pub struct TradingPlatformRequestCredentials {
    pub session_entity: Arc<SessionEntity>,
}

impl TradingPlatformRequestCredentials {
    pub fn new(session_entity: Arc<SessionEntity>) -> Self {
        Self { session_entity }
    }
}

impl RequestCredentials for TradingPlatformRequestCredentials {
    fn get_id(&self) -> &str {
        &self.session_entity.client_id
    }

    fn get_claims<'s>(&'s self) -> Option<Vec<my_http_server::RequestClaim<'s>>> {
        if self.session_entity.claims.len() == 0 {
            return None;
        }

        let mut result = Vec::with_capacity(self.session_entity.claims.len());

        for session_claim in &self.session_entity.claims {
            result.push(RequestClaim {
                id: &session_claim.name,
                expires: DateTimeAsMicroseconds::new(session_claim.expires),
                allowed_ips: None,
            });
        }

        Some(result)
    }
}
