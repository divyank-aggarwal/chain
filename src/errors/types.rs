#[derive(Debug)]
pub enum ChainErrors {
    RedisNotFound,
    RedisOther(String),
    DatabaseOther(String),
    ConversionError(String),
}

impl From<sqlx::Error> for ChainErrors {
    fn from(y: sqlx::Error) -> Self {
        ChainErrors::DatabaseOther(format!("{:?}", y))
    }
}
