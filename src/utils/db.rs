use crate::config::Config;
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct AppData {
    pub pg_pool: Pool<ConnectionManager<PgConnection>>,
}

pub fn create_conn_pool(config: &Config) -> AppData {
    let manager: ConnectionManager<PgConnection> =
        ConnectionManager::<PgConnection>::new(config.get_db_url());
    let pool: Pool<ConnectionManager<PgConnection>> = Pool::builder()
        .max_size(*config.get_pool_size())
        .build(manager)
        .expect("Failed to connect to database!");
    AppData { pg_pool: pool }
}
