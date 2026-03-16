use sqlx::{PgPool, Row};
use dotenvy::dotenv;
use std::env;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        dotenv().ok();
        
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        
        let pool = PgPool::connect(&database_url).await?;
        
        Ok(Database { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

// Database initialization function
pub async fn init_database() -> Result<Database, sqlx::Error> {
    let db = Database::new().await?;
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db.pool)
        .await?;
    
    Ok(db)
}