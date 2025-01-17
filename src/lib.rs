use crate::proto::geometric::geometric_client::GeometricClient;
use crate::proto::geometric::{AreaCircleMessage, AreaSquareMessage};
use tonic::transport::Channel;
use tonic::Request;

mod proto;

pub struct GeometricClientImpl {
    pub addr: String,
    pub port: u16,
}

impl GeometricClientImpl {
    pub async fn area_square(&self, base: u64) -> Option<u64> {
        let channel = Channel::from_shared(format!("http://{}:{}", self.addr, self.port))
            .unwrap()
            .connect()
            .await
            .unwrap();
        let mut client = GeometricClient::new(channel);

        let area_square_message = AreaSquareMessage { base };

        client
            .area_square(Request::new(area_square_message))
            .await
            .map(|x| x.into_inner().value)
            .ok()
    }

    pub async fn area_circle(&self, radius: u64) -> Option<f32> {
        let channel = Channel::from_shared(format!("http://{}:{}", self.addr, self.port))
            .unwrap()
            .connect()
            .await
            .unwrap();
        let mut client = GeometricClient::new(channel);

        let area_circle_message = AreaCircleMessage { radius };

        client
            .area_circle(Request::new(area_circle_message))
            .await
            .map(|x| x.into_inner().value)
            .ok()
    }
}
