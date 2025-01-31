use crate::proto::geometric::geometric_client::GeometricClient;
pub use crate::proto::geometric::{AreaCircleMessage, AreaSquareMessage, ResultResponse};
use tonic::transport::Channel;
use tonic::Request;

mod proto;

#[derive(Clone)]
pub struct GeometricGrpcInterface {
    client: GeometricClient<Channel>,
}

impl GeometricGrpcInterface {
    pub async fn new(addr: &'static str, port: u16) -> Self {
        let channel = Channel::from_shared(format!("http://{addr}:{port}"))
            .unwrap()
            .connect()
            .await
            .unwrap();
        Self {
            client: GeometricClient::new(channel),
        }
    }

    pub async fn area_square(&mut self, message: AreaSquareMessage) -> Option<ResultResponse> {
        self.client
            .area_square(Request::new(message))
            .await
            .map(tonic::Response::into_inner)
            .ok()
    }

    pub async fn area_circle(&mut self, message: AreaCircleMessage) -> Option<ResultResponse> {
        self.client
            .area_circle(Request::new(message))
            .await
            .map(tonic::Response::into_inner)
            .ok()
    }
}
