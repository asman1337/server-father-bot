use sea_orm::*;
use crate::db::entities::server::{self, Entity as Server, Model as ServerModel};
use crate::error::Result;

pub struct ServerService {
    db: DatabaseConnection,
}

impl ServerService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn add_server(
        &self,
        name: String,
        host: String,
        port: i32,
        group_id: Option<i32>,
    ) -> Result<ServerModel> {
        let server = server::ActiveModel {
            name: Set(name),
            host: Set(host),
            port: Set(port),
            group_id: Set(group_id),
            is_active: Set(true),
            ..Default::default()
        };

        let result = Server::insert(server)
            .exec_with_returning(&self.db)
            .await?;

        Ok(result)
    }

    pub async fn remove_server(&self, id: i32) -> Result<bool> {
        let result = Server::delete_by_id(id)
            .exec(&self.db)
            .await?;

        Ok(result.rows_affected > 0)
    }

    pub async fn get_server(&self, id: i32) -> Result<Option<ServerModel>> {
        let server = Server::find_by_id(id)
            .one(&self.db)
            .await?;

        Ok(server)
    }

    pub async fn list_servers(&self) -> Result<Vec<ServerModel>> {
        let servers = Server::find()
            .order_by_asc(server::Column::Name)
            .all(&self.db)
            .await?;

        Ok(servers)
    }

    pub async fn update_server_status(&self, id: i32, is_active: bool) -> Result<bool> {
        let server = server::ActiveModel {
            id: Set(id),
            is_active: Set(is_active),
            ..Default::default()
        };

        let result = Server::update(server)
            .exec(&self.db)
            .await?;

        Ok(result.rows_affected > 0)
    }
} 