use sqlx::PgPool;
use std::env;

pub async fn establish_connection() -> PgPool {
    let postgres_user = env::var("POSTGRES_USER").expect("DATABASE_URL must be set");
    let postgres_password = env::var("POSTGRES_PASSWORD").expect("DATABASE_URL must be set");
    let postgres_db = env::var("POSTGRES_DB").expect("DATABASE_URL must be set");
    let postgres_host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
    let postgres_port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());

    let database_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        postgres_user, postgres_password, postgres_host, postgres_port, postgres_db
    );
    PgPool::connect(&database_url).await.expect("Failed to connect to database")
}