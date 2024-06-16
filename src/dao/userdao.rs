use deadpool_postgres::Pool;
use tokio_postgres::Error;
use std::sync::Arc;

pub async fn get_user(pool: Arc<Pool>, code: &str) -> Result<String, Error> {
    let client = pool.get().await.unwrap();
//    client.execute("INSERT INTO users (name, age) VALUES ($1, $2)", &[&name, &age]).await?;

    let stmt = client.prepare_cached("SELECT 'value from Db..'").await?;


    let rows = client.query(&stmt, &[]).await.unwrap();
    let value: String = rows[0].get(0);

    Ok(value)
}