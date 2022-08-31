use dotenv::dotenv;
use redis::{self, Commands};
use std::env;

pub fn connect() -> redis::Connection {
    dotenv().ok();
    let redis_host = env::var("REDIS_HOST").expect("Missing redis host");
    let redis_port = env::var("REDIS_PORT")
        .expect("Missing redis port")
        .parse::<u32>()
        .expect("Failed to convert redist host to integer");
    let redis_password = env::var("REDIS_PASS").expect("Missing redis password");
    let conn_url = format!("redis://:{}@{}:{}", redis_password, redis_host, redis_port);
    redis::Client::open(conn_url)
        .expect("Invalid redis connection url")
        .get_connection()
        .expect("Failed to connect to redis")
}

pub fn test_redis(con: &mut redis::Connection) {
    let _: () = con
        .set("my_key", 42)
        .expect("Failed to set value in redis test");
    let x: u32 = con
        .get("my_key")
        .expect("Failed to get value in redis test");
}
