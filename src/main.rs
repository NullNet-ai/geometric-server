use crate::proto::geometric::geometric_server::GeometricServer;
use crate::r#impl::GeometricImpl;
use std::net::ToSocketAddrs;
use tonic::transport::Server;

mod r#impl;
mod proto;

#[tokio::main]
async fn main() {
    let addr = "localhost:50051"
        .to_string()
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    Server::builder()
        .add_service(GeometricServer::new(GeometricImpl))
        .serve(addr)
        .await
        .unwrap();
}
