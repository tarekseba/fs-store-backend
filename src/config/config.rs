use dotenvy::dotenv;

#[derive(Debug)]
pub struct Config {
    server_port: u16,
    server_addr: String,
    db_url: String,
    pool_size: u32,
}

impl Config {
    pub fn load_config() -> Self {
        dotenv().ok();
        Config {
            server_port: std::env::var("SERVER_PORT")
                .expect("SERVER_PORT not found in .env")
                .parse()
                .expect("Wrong value for SERVER_PORT"),
            server_addr: std::env::var("SERVER_ADDR").expect("SERVER_ADDR not found in .env"),
            db_url: std::env::var("DATABASE_URL").expect("DATABASE_URL not found in .env"),
            pool_size: std::env::var("POOL_SIZE")
                .unwrap_or("10".to_owned())
                .parse()
                .expect("wrong POOL_SIZE provided in .env"),
        }
    }

    pub fn get_srv_addr(&self) -> &str {
        &self.server_addr
    }

    pub fn get_srv_port(&self) -> &u16 {
        &self.server_port
    }

    pub fn get_db_url(&self) -> &str {
        &self.db_url
    }

    pub fn get_pool_size(&self) -> &u32 {
        &self.pool_size
    }
}
