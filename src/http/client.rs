use crate::http::{config::HttpClientConfig, error::HttpError};
use bytes::{Bytes, BytesMut};
use http::Method;
use http::Request;
use http::Uri;
use http::header;
use http_body_util::BodyExt;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper_rustls::HttpsConnectorBuilder;
use hyper_util::{
    client::legacy::{Client, connect::HttpConnector},
    rt::{TokioExecutor, TokioTimer},
};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::time::Duration;

type RequstBody = Full<Bytes>;

type HyperClient = Client<hyper_rustls::HttpsConnector<HttpConnector>, RequstBody>;

#[derive(Clone)]
pub struct AsyncHttpClient {
    inner: HyperClient,
    config: HttpClientConfig,
}

impl AsyncHttpClient {
    pub fn new(config: HttpClientConfig) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let mut connector = HttpConnector::new();

        connector.enforce_http(false);

        connector.set_connect_timeout(Some(config.connect_timeout));

        let connector = HttpsConnectorBuilder::new()
            .with_native_roots()?
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .wrap_connector(connector);

        let inner = Client::builder(TokioExecutor::new())
            .pool_timer(TokioTimer::new())
            .pool_idle_timeout(config.pool_idle_timeout)
            .pool_max_idle_per_host(config.max_idle_per_host)
            .build::<_, RequstBody>(connector);

        Ok(Self { inner, config })
    }

    async fn execute(&self, request: Request<RequstBody>) -> Result<Bytes, HttpError> {
        let client = self.inner.clone();

        let max_response_bytes = self.config.max_response_bytes;

        let read_idle_timeout = self.config.read_idle_timeout;

        let operation = async move {
            let response = client.request(request).await?;

            let status = response.status();

            let body =
                collect_body(response.into_body(), max_response_bytes, read_idle_timeout).await?;

            if !status.is_success() {
                return Err(HttpError::Status {
                    status,
                    body: String::from_utf8_lossy(&body).into_owned(),
                });
            }

            Ok(body)
        };

        tokio::time::timeout(self.config.request_timeout, operation)
            .await
            .map_err(|_| HttpError::Timeout)?
    }

    pub async fn get_json<T>(&self, url: &str) -> Result<T, HttpError>
    where
        T: DeserializeOwned,
    {
        let uri: Uri = url.parse()?;

        let request = Request::builder()
            .method(Method::GET)
            .uri(uri)
            .header(header::ACCEPT, "application/json")
            .body(Full::new(Bytes::new()))?;

        let body = self.execute(request).await?;

        Ok(serde_json::from_slice(&body)?)
    }

    pub async fn post_json<B, T>(&self, url: &str, request_body: &B) -> Result<T, HttpError>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let uri: Uri = url.parse()?;

        let json = serde_json::to_vec(request_body)?;

        let request = Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header(header::ACCEPT, "application/json")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Full::new(Bytes::from(json)))?;

        let body = self.execute(request).await?;

        Ok(serde_json::from_slice(&body)?)
    }

    pub async fn get_text(&self, url: &str) -> Result<String, HttpError> {
        let uri: Uri = url.parse()?;

        let request = Request::builder()
            .method(Method::GET)
            .uri(uri)
            .body(Full::new(Bytes::new()))?;

        let body = self.execute(request).await?;

        String::from_utf8(body.to_vec()).map_err(|err| HttpError::Body(err.to_string()))
    }
}

async fn collect_body(
    mut body: Incoming,
    max_size: usize,
    read_idle_timeout: Duration,
) -> Result<bytes::Bytes, HttpError> {
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
