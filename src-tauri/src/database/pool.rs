use crate::config::DatabaseConfig;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(config: DatabaseConfig) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await?;

        // Initialize database with tables
        Self::init_schema(&pool).await?;

        Ok(Database { pool })
    }

    async fn init_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id SERIAL PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) UNIQUE NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(pool)
        .await?;

        // Create index on email
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_email ON users(email)")
            .execute(pool)
            .await?;

        Ok(())
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
