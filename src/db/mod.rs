use sea_orm::*;
use sea_orm_migration::MigratorTrait;

pub mod entities;
pub mod migrations;

#[derive(Debug, Clone)]
pub struct Database {
    pub connection: DatabaseConnection,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, DbErr> {
        let connection = sea_orm::Database::connect(database_url).await?;
        
        // Run migrations
        migrations::Migrator::up(&connection, None).await?;
        
        Ok(Self { connection })
    }
} 