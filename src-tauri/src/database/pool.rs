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

    async fn init_schema(_pool: &PgPool) -> Result<(), sqlx::Error> {
        // Schema initialization is handled by migrations
        Ok(())
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
