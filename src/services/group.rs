use sea_orm::*;
use crate::db::entities::server_group::{self, Entity as ServerGroup, Model as ServerGroupModel};
use crate::error::Result;

pub struct GroupService {
    db: DatabaseConnection,
}

impl GroupService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_group(&self, name: String) -> Result<ServerGroupModel> {
        let group = server_group::ActiveModel {
            name: Set(name),
            ..Default::default()
        };

        let result = ServerGroup::insert(group)
            .exec_with_returning(&self.db)
            .await?;

        Ok(result)
    }

    pub async fn list_groups(&self) -> Result<Vec<ServerGroupModel>> {
        let groups = ServerGroup::find()
            .order_by_asc(server_group::Column::Name)
            .all(&self.db)
            .await?;

        Ok(groups)
    }

    pub async fn delete_group(&self, id: i32) -> Result<bool> {
        let result = ServerGroup::delete_by_id(id)
            .exec(&self.db)
            .await?;

        Ok(result.rows_affected > 0)
    }
} 