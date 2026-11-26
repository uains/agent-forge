use std::time::Duration;

// ======================  HttpClientConfig Start ======================

#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    pub connect_timeout: Duration,
    pub request_timeout: Duration,
    pub read_idle_timeout: Duration,
    pub pool_idle_timeout: Duration,
    pub max_idle_per_host: usize,
    pub max_response_bytes: usize,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            connect_timeout: Duration::from_secs(3),
            request_timeout: Duration::from_secs(30),
            read_idle_timeout: Duration::from_secs(5),
            pool_idle_timeout: Duration::from_secs(30),
            max_idle_per_host: 10,
            max_response_bytes: 2 * 1024 * 1024,
        }
    }
}

// ======================  HttpClientConfig End ========================
