use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "servers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub host: String,
    pub port: i32,
    pub group_id: Option<i32>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub last_check: Option<DateTime>,
    pub is_active: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::server_group::Entity",
        from = "Column::GroupId",
        to = "super::server_group::Column::Id"
    )]
    ServerGroup,
}

impl Related<super::server_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ServerGroup.def()
    }
}

impl ActiveModelBehavior for ActiveModel {} 