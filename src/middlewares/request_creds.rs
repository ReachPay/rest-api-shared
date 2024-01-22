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
        &self.session_entity.trader_id
    }

    fn get_claims<'s>(&'s self) -> Option<Vec<my_http_server::RequestClaim<'s>>> {
        let session_claims = self.session_entity.claims.as_ref()?;

        let mut result = Vec::new();

        for session_claim in session_claims {
            result.push(RequestClaim {
                id: &session_claim.name,
                expires: DateTimeAsMicroseconds::new(session_claim.expires),
                allowed_ips: None,
            });
        }

        Some(result)
    }
}
