use anyhow::Result;
use bytes::{Bytes, BytesMut};
use http_body_util::{BodyExt, Full, Limited};
use hyper::{Method, Request, Uri, body::Incoming, header};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use hyper_util::{
    client::legacy::{Client, connect::HttpConnector},
    rt::{TokioExecutor, TokioTimer},
};
use serde::Serialize;
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Invalid URI: {0}")]
    InvalidUri(#[from] http::uri::InvalidUri),

    #[error("Request build error: {0}")]
    RequestBuild(#[from] http::Error),

    #[error("HTTP transport error: {0}")]
    Transport(#[from] hyper_util::client::legacy::Error),

    #[error("Response body error: {0}")]
    Body(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("HTTP status error: {status}")]
    Status {
        status: http::StatusCode,
        body: String,
    },

    #[error("Request timeout")]
    Timeout,

    #[error("Read idle timeout")]
    ReadIdleTimeout,

    #[error("Response body exceeds configured limit")]
    BodyToolarge,
}

fn ensure_success(status: http::StatusCode, body: &Bytes) -> Result<(), HttpError> {
    if !status.is_success() {
        return Err(HttpError::Status {
            status,
            body: String::from_utf8_lossy(body).into_owned(),
        });
    }
    Ok(())
}

async fn collect_body(
    mut body: Incoming,
    max_size: usize,
    read_idle_timeout: Duration,
) -> Result<Bytes, HttpError> {
    let mut output = BytesMut::new();
    loop {
        let frame = tokio::time::timeout(read_idle_timeout, body.frame())
            .await
            .map_err(|_| HttpError::ReadIdleTimeout)?;
        let Some(frame) = frame else {
            break;
        };

        let frame = frame.map_err(|error| HttpError::Body(error.to_string()))?;

        if let Some(data) = frame.data_ref() {
            if output.len() + data.len() > max_size {
                return Err(HttpError::BodyToolarge);
            }

            output.extend_from_slice(data);
        }
    }
    Ok(output.freeze())
}

type RequestBody = Full<Bytes>;

type HyperClient = Client<HttpsConnector<HttpConnector>, RequestBody>;

fn create_client() -> Result<HyperClient> {
    let mut connector = HttpConnector::new();
    connector.enforce_http(false);

    connector.set_connect_timeout(Some(Duration::from_secs(3)));

    let connector = HttpsConnectorBuilder::new()
        .with_native_roots()?
        .https_or_http()
        .enable_http1()
        .enable_http2()
        .wrap_connector(connector);

    let client = Client::builder(TokioExecutor::new())
        .pool_timer(TokioTimer::new())
        .pool_idle_timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(10)
        .build::<_, RequestBody>(connector);

    Ok(client)
}

async fn exp_get() -> Result<()> {
    let client = create_client()?;

    // 构建URL
    let mut uri = url::Url::parse("https://jsonplaceholder.typicode.com/todos")?;

    uri.query_pairs_mut()
        .append_pair("userId", "1")
        .append_pair("_limit", "3");

    let uri: Uri = uri.as_str().parse()?;

    let request = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .header(header::ACCEPT, "application/json")
        .body(Full::new(Bytes::new()))?;

    let response = client.request(request).await?;

    println!("Status: {}", response.status());

    println!("Headers: {:#?}", response.headers());

    // 限制响应大小
    let limit = Limited::new(response.into_body(), 2 * 1024 * 1024);

    let body = limit
        .collect()
        .await
        .map_err(|e| anyhow::anyhow!(e))?
        .to_bytes();

    println!("Body: {}", String::from_utf8_lossy(&body));
    Ok(())
}

#[derive(Debug, Serialize)]
struct CreateTodo {
    #[serde(rename = "userId")]
    user_id: u32,
    title: String,
    completed: bool,
}

async fn exp_post() -> Result<()> {
    let client = create_client()?;

    let payload = CreateTodo {
        user_id: 1,
        title: "Build Hyper Agent Runtime".to_string(),
        completed: false,
    };

    let json = serde_json::to_vec(&payload)?;

    // 构建URL
    let mut uri = url::Url::parse("https://jsonplaceholder.typicode.com/todos")?;

    uri.query_pairs_mut()
        .append_pair("userId", "1")
        .append_pair("_limit", "3");

    let uri: Uri = uri.as_str().parse()?;

    let request = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header(header::ACCEPT, "application/json")
        .body(Full::new(Bytes::from(json)))?;

    let response = client.request(request).await?;

    println!("Status: {}", response.status());

    let body = response.into_body().collect().await?.to_bytes();

    println!("Body: {}", String::from_utf8_lossy(&body));

    Ok(())
}

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

// ======================  AsyncHttpClient Start ======================

// ======================  AsyncHttpClient End ========================

#[tokio::main]
async fn main() -> Result<()> {
    exp_get().await?;
    // exp_post().await?;
    Ok(())
}
