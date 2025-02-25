use coarsetime::Instant;
use prometheus::{Counter, Gauge, Histogram};
use std::sync::Arc;

pub struct StartInstant(pub Instant);

pub struct Inner {
    pub start_instant: StartInstant,
    pub uptime: Gauge,
    pub cache_hit_ratio: Gauge,
    pub client_queries: Gauge,
    pub client_queries_udp: Counter,
    pub client_queries_tcp: Counter,
    pub client_queries_cached: Counter,
    pub client_queries_expired: Counter,
    pub client_queries_offline: Counter,
    pub client_queries_errors: Counter,
    pub client_queries_blocked: Counter,
    pub inflight_udp_queries: Gauge,
    pub inflight_tcp_queries: Gauge,
    pub upstream_errors: Counter,
    pub upstream_sent: Counter,
    pub upstream_received: Counter,
    pub upstream_response_sizes: Histogram,
}

pub type Varz = Arc<Inner>;

pub fn new() -> Varz {
    Arc::new(Inner::new())
}

impl Inner {
    pub fn new() -> Inner {
        Inner {
            start_instant: StartInstant::default(),
            uptime: register_gauge!(opts!(
                "encrypted_dns_uptime",
                "Uptime",
                labels! {"handler" => "all",}
            ))
            .unwrap(),
            client_queries: register_gauge!(opts!(
                "encrypted_dns_client_queries",
                "Number of client queries received",
                labels! {"handler" => "all",}
            ))
            .unwrap(),
            cache_hit_ratio: register_gauge!(opts!(
                "encrypted_dns_cache_hit_ratio",
                "Cache hit ratio",
                labels! {"handler" => "all",}
            ))
            .unwrap(),
            client_queries_udp: register_counter!(opts!(
                "encrypted_dns_client_queries_udp",
                "Number of client queries received \
                 using UDP",
                labels! {"handler" => "all",}
            ))
            .unwrap(),
            client_queries_tcp: register_counter!(opts!(
                "encrypted_dns_client_queries_tcp",
                "Number of client queries received \
                 using TCP",
                labels! {"handler" => "all",}
            ))
            .unwrap(),
            client_queries_cached: register_counter!(opts!(
                "encrypted_dns_client_queries_cached",
                "Number of client queries sent from \
                 the cache",
                labels! {"handler" => "all",}
            ))
            .unwrap(),
            client_queries_expired: register_counter!(opts!(
                "encrypted_dns_client_queries_expired",
                "Number of expired client queries",
                labels! {"handler" => "all",}
            ))
            .unwrap(),
            client_queries_offline: register_counter!(opts!(
                "encrypted_dns_client_queries_offline",
                "Number of client queries answered \
                 while upstream resolvers are \
                 unresponsive",
                labels! {"handler" => "all",}
            ))
            .unwrap(),
            client_queries_errors: register_counter!(opts!(
                "encrypted_dns_client_queries_errors",
                "Number of bogus client queries",
                labels! {"handler" => "all",}
            ))
            .unwrap(),
            client_queries_blocked: register_counter!(opts!(
                "encrypted_dns_client_queries_blocked",
                "Number of blocked client queries",
                labels! {"handler" => "all",}
            ))
            .unwrap(),
            inflight_udp_queries: register_gauge!(opts!(
                "encrypted_dns_inflight_udp_queries",
                "Number of UDP queries currently waiting for a response",
                labels! {"handler" => "all",}
            ))
            .unwrap(),
            inflight_tcp_queries: register_gauge!(opts!(
                "encrypted_dns_inflight_tcp_queries",
                "Number of TCP queries currently waiting for a response",
                labels! {"handler" => "all",}
            ))
            .unwrap(),
            upstream_errors: register_counter!(opts!(
                "encrypted_dns_upstream_errors",
                "Number of bogus upstream servers responses",
                labels! {"handler" => "all",}
            ))
            .unwrap(),
            upstream_sent: register_counter!(opts!(
                "encrypted_dns_upstream_sent",
                "Number of upstream servers queries sent",
                labels! {"handler" => "all",}
            ))
            .unwrap(),
            upstream_received: register_counter!(opts!(
                "encrypted_dns_upstream_received",
                "Number of upstream servers responses received",
                labels! {"handler" => "all",}
            ))
            .unwrap(),
            upstream_response_sizes: register_histogram!(histogram_opts!(
                "encrypted_dns_upstream_response_sizes",
                "Response size in bytes",
                vec![64.0, 128.0, 192.0, 256.0, 512.0, 1024.0, 2048.0]
            ))
            .unwrap(),
        }
    }
}

impl Default for Inner {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for StartInstant {
    fn default() -> StartInstant {
        StartInstant(Instant::now())
    }
}
