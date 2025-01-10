use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Servers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Servers::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Servers::Name).string().not_null())
                    .col(ColumnDef::new(Servers::Host).string().not_null())
                    .col(ColumnDef::new(Servers::Port).integer().not_null())
                    .col(ColumnDef::new(Servers::GroupId).integer())
                    .col(
                        ColumnDef::new(Servers::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Servers::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Servers::LastCheck).timestamp())
                    .col(
                        ColumnDef::new(Servers::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_server_group")
                            .from(Servers::Table, Servers::GroupId)
                            .to(ServerGroups::Table, ServerGroups::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Servers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Servers {
    Table,
    Id,
    Name,
    Host,
    Port,
    GroupId,
    CreatedAt,
    UpdatedAt,
    LastCheck,
    IsActive,
}

#[derive(DeriveIden)]
enum ServerGroups {
    Table,
    Id,
}
