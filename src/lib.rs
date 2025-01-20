use crate::proto::geometric::geometric_client::GeometricClient;
pub use crate::proto::geometric::{AreaCircleMessage, AreaSquareMessage, ResultResponse};
use tonic::transport::Channel;
use tonic::Request;

mod proto;

pub struct GeometricClientImpl {
    pub addr: String,
    pub port: u16,
}

impl GeometricClientImpl {
    pub async fn area_square(&self, message: AreaSquareMessage) -> Option<ResultResponse> {
        let channel = Channel::from_shared(format!("http://{}:{}", self.addr, self.port))
            .unwrap()
            .connect()
            .await
            .unwrap();
        let mut client = GeometricClient::new(channel);

        client
            .area_square(Request::new(message))
            .await
            .map(tonic::Response::into_inner)
            .ok()
    }

    pub async fn area_circle(&self, message: AreaCircleMessage) -> Option<ResultResponse> {
        let channel = Channel::from_shared(format!("http://{}:{}", self.addr, self.port))
            .unwrap()
            .connect()
            .await
            .unwrap();
        let mut client = GeometricClient::new(channel);

        client
            .area_circle(Request::new(message))
            .await
            .map(tonic::Response::into_inner)
            .ok()
    }
}
