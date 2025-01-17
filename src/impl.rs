use crate::proto::geometric::geometric_server::Geometric;
use crate::proto::geometric::{
    AreaCircleMessage, AreaSquareMessage, FloatResponse, IntegerResponse,
};
use tonic::{Request, Response, Status};

pub struct GeometricImpl;

#[tonic::async_trait]
impl Geometric for GeometricImpl {
    async fn area_square(
        &self,
        request: Request<AreaSquareMessage>,
    ) -> Result<Response<IntegerResponse>, Status> {
        let AreaSquareMessage { base } = request.into_inner();

        let res = base * base;

        let response = IntegerResponse { value: res };
        Ok(Response::new(response))
    }

    async fn area_circle(
        &self,
        request: Request<AreaCircleMessage>,
    ) -> Result<Response<FloatResponse>, Status> {
        let AreaCircleMessage { radius } = request.into_inner();

        let radius = radius as f32;
        let res = std::f32::consts::PI * radius * radius;

        let response = FloatResponse { value: res };
        Ok(Response::new(response))
    }
}
