use crate::proto::geometric::geometric_server::Geometric;
use crate::proto::geometric::{AreaCircleMessage, AreaSquareMessage, ResultResponse};
use multiply::multiply;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_area_square() {
        let geometric_impl = GeometricImpl;
        let message = AreaSquareMessage { base: 3.0 };
        let request = Request::new(message);

        let response = geometric_impl.area_square(request).await.unwrap();
        assert_eq!(response.into_inner().value, 9.0);
    }

    #[tokio::test]
    async fn test_area_circle() {
        let geometric_impl = GeometricImpl;
        let message = AreaCircleMessage { radius: 2.0 };
        let request = Request::new(message);

        let response = geometric_impl.area_circle(request).await.unwrap();
        assert_eq!(response.into_inner().value, 12.566371);
    }
}
