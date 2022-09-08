use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

pub struct DbState {
    pub db_pool: PgPool,
}

pub fn create_connection_string() -> String {
    dotenv().ok();
    let port = env::var("DATABASE_PORT")
        .expect("database port not found in env")
        .parse::<u32>()
        .expect("database port could not be parsed into integer");
    let host = env::var("DATABASE_HOST").expect("database host not found in env");
    let pass = env::var("DATABASE_PASS").expect("database password not found in env");
    let db_name = env::var("DATABASE_NAME").expect("database name not found in env");
    let user = env::var("DATABASE_USER").expect("database user not found in env");
    format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, db_name)
}
