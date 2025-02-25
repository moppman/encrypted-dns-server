use crate::config::*;
use crate::errors::*;
use crate::varz::*;

use futures::prelude::*;
use hyper::header::CONTENT_TYPE;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, StatusCode};
use prometheus::{self, Encoder, TextEncoder};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;

async fn handle_client_connection(
    req: Request<Body>,
    varz: Varz,
    path: Arc<String>,
) -> Result<Response<Body>, Error> {
    let mut buffer = vec![];
    if req.uri().path() != path.as_str() {
        let response = Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())?;
        return Ok(response);
    }
    let StartInstant(start_instant) = varz.start_instant;
    let uptime = start_instant.elapsed().as_secs();
    varz.uptime.set(uptime as f64);
    let client_queries = varz.client_queries_udp.get() + varz.client_queries_tcp.get();
    varz.client_queries.set(client_queries);
    let cached_queries = varz.client_queries_cached.get();
    let cache_hit_ratio = if client_queries <= 0.0 {
        0.0
    } else {
        cached_queries as f64 * 100.0 / client_queries as f64
    };
    varz.cache_hit_ratio.set(cache_hit_ratio);
    let metric_families = prometheus::gather();
    let encoder = TextEncoder::new();
    encoder.encode(&metric_families, &mut buffer)?;
    let response = Response::builder()
        .header(CONTENT_TYPE, encoder.format_type())
        .body(buffer.into())?;
    Ok(response)
}

#[allow(unreachable_code)]
pub async fn prometheus_service(
    varz: Varz,
    metrics_config: MetricsConfig,
    runtime: Arc<Runtime>,
) -> Result<(), Error> {
    let path = Arc::new(metrics_config.path);
    let mut stream = TcpListener::bind(metrics_config.listen_addr).await?;
    loop {
        let (client, _client_addr) = stream.accept().await?;
        let path = path.clone();
        let varz = varz.clone();
        let service =
            service_fn(move |req| handle_client_connection(req, varz.clone(), path.clone()));
        let connection = Http::new().serve_connection(client, service);
        runtime.spawn(connection.map(|_| {}));
    }
    Ok(())
}
