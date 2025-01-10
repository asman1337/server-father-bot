use sea_orm_migration::prelude::*;

mod m20240301_000001_create_server_groups;
mod m20240301_000002_create_servers;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240301_000001_create_server_groups::Migration),
            Box::new(m20240301_000002_create_servers::Migration),
        ]
    }
}
