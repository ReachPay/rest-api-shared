pub mod authorization;
pub mod lang_id;
pub mod middlewares;
pub mod request_extensions;
pub mod shared_contracts;

#[cfg(feature = "auth-middleware")]
mod configure_rest_api_server;
#[cfg(feature = "auth-middleware")]
pub use configure_rest_api_server::*;

#[cfg(not(feature = "auth-middleware"))]
mod configure_rest_api_server_with_no_auth_middleware;
#[cfg(not(feature = "auth-middleware"))]
pub use configure_rest_api_server_with_no_auth_middleware::*;
pub mod http_fields;
