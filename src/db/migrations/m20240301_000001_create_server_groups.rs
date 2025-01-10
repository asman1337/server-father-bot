use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ServerGroups::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ServerGroups::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ServerGroups::Name).string().not_null())
                    .col(
                        ColumnDef::new(ServerGroups::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(ServerGroups::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ServerGroups::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ServerGroups {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
} 