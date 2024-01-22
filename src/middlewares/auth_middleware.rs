use my_http_server::*;
use service_sdk::{my_http_server, my_no_sql_sdk::reader::MyNoSqlDataReaderTcp};
use std::sync::Arc;

use super::{GetSessionToken, SessionEntity, TradingPlatformRequestCredentials};

pub struct AuthMiddleware {
    sessions_reader: Arc<MyNoSqlDataReaderTcp<SessionEntity>>,
}

impl AuthMiddleware {
    pub fn new(sessions_reader: Arc<MyNoSqlDataReaderTcp<SessionEntity>>) -> Self {
        Self { sessions_reader }
    }
}

#[async_trait::async_trait]
impl HttpServerMiddleware for AuthMiddleware {
    async fn handle_request(
        &self,
        ctx: &mut HttpContext,
        get_next: &mut HttpServerRequestFlow,
    ) -> Result<HttpOkResult, HttpFailResult> {
        let session_token = ctx.get_session_token();

        if session_token.is_none() {
            return get_next.next(ctx).await;
        }

        let token_entity = self
            .sessions_reader
            .get_entity(&SessionEntity::get_pk(), session_token.unwrap())
            .await;

        if token_entity.is_none() {
            return get_next.next(ctx).await;
        }

        ctx.credentials = Some(Box::new(TradingPlatformRequestCredentials::new(
            token_entity.unwrap(),
        )));

        get_next.next(ctx).await
    }
}
