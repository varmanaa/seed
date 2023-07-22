use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unable to build connection pool")]
    Build(#[from] deadpool_postgres::BuildError),
    #[error("Provided time component is out of range")]
    ComponentRange(#[from] time::error::ComponentRange),
    #[error("{0}")]
    Custom(String),
    #[error("Unable to make deserialize response body")]
    DeserializeBody(#[from] twilight_http::response::DeserializeBodyError),
    #[error("Environment variable is not set")]
    EnvironmentVariable(#[from] std::env::VarError),
    #[error("Unable to make HTTP request to Discord")]
    Http(#[from] twilight_http::error::Error),
    #[error("Unable to validate message")]
    MessageValidation(#[from] twilight_validate::message::MessageValidationError),
    #[error("Unable to parse integer")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Unable to parse interaction options")]
    Parse(#[from] twilight_interactions::error::ParseError),
    #[error("Unable to retrieve object from pool")]
    PoolObject(#[from] deadpool_postgres::PoolError),
    #[error("Unable to fetch members from gateway")]
    Send(#[from] twilight_gateway::error::SendError),
    #[error("Unable to fetch recommended number of shards to use")]
    StartRecommended(#[from] twilight_gateway::stream::StartRecommendedError),
    #[error("TokioPostgres error")]
    TokioPostgres(#[from] tokio_postgres::Error),
}
