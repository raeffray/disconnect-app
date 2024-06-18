use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

pub type DbPool = Arc<r2d2::Pool<ConnectionManager<PgConnection>>>;

pub fn create_pool() -> DbPool {
    
    dotenv().ok();

    let db_url: String = env::var("DATABASE_URL").expect("DB_NAME not configured");
    let manager = ConnectionManager::<PgConnection>::new(
        db_url
    );
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    Arc::new(pool)
}
