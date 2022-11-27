use crate::DEFAULT_CURRENT_TIMESTAMP;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Login).string().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra(DEFAULT_CURRENT_TIMESTAMP.into()),
                    )
                    .col(ColumnDef::new(User::Uuid).uuid().null())
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserToken::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserToken::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserToken::UserId).big_integer().not_null())
                    .col(ColumnDef::new(UserToken::Token).string().not_null())
                    .col(
                        ColumnDef::new(UserToken::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra(DEFAULT_CURRENT_TIMESTAMP.into()),
                    )
                    .col(
                        ColumnDef::new(UserToken::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_user_token_user")
                            .from(UserToken::Table, UserToken::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().if_exists().table(UserToken::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(User::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Login,
    Password,
    Uuid,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum UserToken {
    Table,
    Id,
    UserId,
    Token,
    CreatedAt,
    UpdatedAt,
}
