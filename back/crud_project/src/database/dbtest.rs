use sqlx::mysql::MySqlPool;

pub async fn dbtest() -> MySqlPool {
    let pool = MySqlPool::connect("mysql://dundun:dundun@localhost:3305/madb").await.unwrap();
   pool
}