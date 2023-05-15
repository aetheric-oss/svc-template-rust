//! Client Library: Client Functions, Structs, Traits
/// gRPC object traits to provide wrappers for grpc functions
#[tonic::async_trait]
pub trait Client<T>
where
    Self: Sized + super::Client<T> + super::ClientConnect<T>,
    T: Send + Clone,
{
    /// The type expected for ReadyRequest structs.
    type ReadyRequest;
    /// The type expected for ReadyResponse structs.
    type ReadyResponse;

    /// Wrapper for is_ready function.
    async fn is_ready(
        &self,
        request: tonic::Request<Self::ReadyRequest>,
    ) -> Result<tonic::Response<Self::ReadyResponse>, tonic::Status>;
}
