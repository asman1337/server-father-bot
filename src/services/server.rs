use crate::db::entities::{prelude::*, server};
use crate::error::Result;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

#[derive(Clone)]
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
    ) -> Result<server::Model> {
        let server = server::ActiveModel {
            name: Set(name),
            host: Set(host),
            port: Set(port),
            group_id: Set(group_id),
            ..Default::default()
        };

        let server = server.insert(&self.db).await?;
        Ok(server)
    }

    pub async fn get_server(&self, id: i32) -> Result<Option<server::Model>> {
        let server = Server::find_by_id(id).one(&self.db).await?;
        Ok(server)
    }

    pub async fn list_servers(&self) -> Result<Vec<server::Model>> {
        let servers = Server::find().all(&self.db).await?;
        Ok(servers)
    }

    pub async fn list_servers_by_group(&self, group_id: i32) -> Result<Vec<server::Model>> {
        let servers = Server::find()
            .filter(server::Column::GroupId.eq(Some(group_id)))
            .all(&self.db)
            .await?;
        Ok(servers)
    }

    pub async fn remove_server(&self, id: i32) -> Result<bool> {
        let server = match self.get_server(id).await? {
            Some(server) => server,
            None => return Ok(false),
        };

        let server: server::ActiveModel = server.into();
        server.delete(&self.db).await?;
        Ok(true)
    }

    pub async fn assign_to_group(&self, server_id: i32, group_id: i32) -> Result<bool> {
        let server = match self.get_server(server_id).await? {
            Some(server) => server,
            None => return Ok(false),
        };

        let mut server: server::ActiveModel = server.into();
        server.group_id = Set(Some(group_id));
        server.update(&self.db).await?;
        Ok(true)
    }
}
