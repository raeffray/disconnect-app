use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;
use std::sync::Arc;
use std::env;

pub type DbPool = Arc<Pool>;

pub fn create_pool() -> DbPool {
    let mut cfg = Config::new();

    cfg.host = Some("localhost".to_string());
    cfg.port = Some(5432);
    cfg.dbname = Some("disconnect-app-storage".to_string());
    cfg.user = Some("disconnect-app".to_string());
    cfg.password = Some("qwaszx12".to_string());
    
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    
    Arc::new(pool)
}
