use multiply::multiply;
use crate::proto::geometric::geometric_server::Geometric;
use crate::proto::geometric::{
    AreaCircleMessage, AreaSquareMessage, ResultResponse,
};
use tonic::{Request, Response, Status};

pub struct GeometricImpl;

#[tonic::async_trait]
impl Geometric for GeometricImpl {
    async fn area_square(
        &self,
        request: Request<AreaSquareMessage>,
    ) -> Result<Response<ResultResponse>, Status> {
        let AreaSquareMessage { base } = request.into_inner();

        let res = multiply(base, base);

        let response = ResultResponse { value: res };
        Ok(Response::new(response))
    }

    async fn area_circle(
        &self,
        request: Request<AreaCircleMessage>,
    ) -> Result<Response<ResultResponse>, Status> {
        let AreaCircleMessage { radius } = request.into_inner();

        let mut res = multiply(std::f32::consts::PI, radius);
        res = multiply(res, radius);

        let response = ResultResponse { value: res };
        Ok(Response::new(response))
    }
}
