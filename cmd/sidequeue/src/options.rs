use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct SideQueueOptions {
    #[structopt(flatten)]
    pub api: APIServiceOptions,
}

#[derive(Clone, Debug, StructOpt)]
pub struct APIServiceOptions {
    #[structopt(short="a", long, default_value="127.0.0.1:7777", parse(from_str = parse_socket_addr))]
    /// Specify the IP:Port for Safety rules. If this is not defined, SafetyRules will run in its
    /// default configuration.
    pub address: SocketAddr,
}

impl Default for APIServiceOptions {
    fn default() -> APIServiceOptions {
        APIServiceOptions {
            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7777),
        }
    }
}

/// Helper functions go here
fn parse_socket_addr(src: &str) -> SocketAddr {
    src.parse::<SocketAddr>().unwrap()
}
