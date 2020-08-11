use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub struct SideQueueOptions {
    pub api: APIServiceOptions,
}

#[derive(Clone, Debug)]
pub struct APIServiceOptions {
    pub address: SocketAddr,
}

impl Default for APIServiceOptions {
    fn default() -> APIServiceOptions {
        APIServiceOptions {
            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7777),
        }
    }
}
