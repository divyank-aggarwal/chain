#[derive(Debug)]
pub enum ChainErrors {
    RedisNotFound,
    RedisOther(String),
    DatabaseOther(String),
    ConversionError(String),
}
