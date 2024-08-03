use serde::Deserialize;

mod address;
mod backlog;
mod keep_alive;
mod max_connection_rate;
mod max_connections;
mod mode;
mod num_workers;
mod timeout;
#[cfg(feature = "openssl")]
mod tls;

#[cfg(feature = "openssl")]
pub use self::tls::Tls;
pub use self::{
    address::Address, backlog::Backlog, keep_alive::KeepAlive,
    max_connection_rate::MaxConnectionRate, max_connections::MaxConnections, mode::Mode,
    num_workers::NumWorkers, timeout::Timeout,
};

/// Settings types for Actix Web.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ActixSettings {
    /// List of addresses for the server to bind to.
    pub hosts: Vec<Address>,

    /// Marker of intended deployment environment.
    pub mode: Mode,

    /// True if the `Compress` middleware should be enabled.
    pub enable_compression: bool,

    /// True if the [`Logger`](actix_web::middleware::Logger) middleware should be enabled.
    pub enable_log: bool,

    /// The number of workers that the server should start.
    pub num_workers: NumWorkers,

    /// The maximum number of pending connections.
    pub backlog: Backlog,

    /// The per-worker maximum number of concurrent connections.
    pub max_connections: MaxConnections,

    /// The per-worker maximum concurrent TLS connection limit.
    pub max_connection_rate: MaxConnectionRate,

    /// Server keep-alive preference.
    pub keep_alive: KeepAlive,

    /// Timeout duration for reading client request header.
    pub client_timeout: Timeout,

    /// Timeout duration for connection shutdown.
    pub client_shutdown: Timeout,

    /// Timeout duration for graceful worker shutdown.
    pub shutdown_timeout: Timeout,

    /// TLS (HTTPS) configuration.
    #[cfg(feature = "openssl")]
    pub tls: Tls,
}
