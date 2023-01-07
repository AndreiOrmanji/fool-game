use crate::{m20221021_000001_create_table_user::User, DEFAULT_CURRENT_TIMESTAMP};
use sea_orm_migration::prelude::*;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Card::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Card::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Card::Rank).char().char_len(1).null())
                    .col(ColumnDef::new(Card::Suit).char().char_len(1).null())
                    .to_owned(),
            )
            .await?;

        let mut cards_insert_query = Query::insert();
        cards_insert_query
            .into_table(Card::Table)
            .columns([Card::Id, Card::Rank, Card::Suit]);

        let shoe = playin_cards::gen_shoe(1, false);
        for i in 0..shoe.len() {
            if let playin_cards::Card::Regular { rank, suit } = shoe.get(i).unwrap() {
                cards_insert_query.values_panic([
                    ((i as i64) + 1).into(),
                    format!("{}", rank).into(),
                    format!("{}", suit).into(),
                ]);
            }
        }

        manager.exec_stmt(cards_insert_query).await?;

        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Game::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Game::Uuid).uuid().null())
                    .col(ColumnDef::new(Game::FoolUserId).big_integer().null())
                    .col(
                        ColumnDef::new(Game::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra(DEFAULT_CURRENT_TIMESTAMP.into()),
                    )
                    .col(
                        ColumnDef::new(Game::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Game::FinishedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_game_fool_user")
                            .from(Game::Table, Game::FoolUserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Room::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Room::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Room::AdminUserId).big_integer().null())
                    .col(ColumnDef::new(Room::CurrentGameId).big_integer().null())
                    .col(ColumnDef::new(Room::Name).string().string_len(255).null())
                    .col(ColumnDef::new(Room::Uuid).uuid().null())
                    .col(
                        ColumnDef::new(Room::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra(DEFAULT_CURRENT_TIMESTAMP.into()),
                    )
                    .col(
                        ColumnDef::new(Room::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_room_admin_user")
                            .from(Room::Table, Room::AdminUserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_room_current_game")
                            .from(Room::Table, Room::CurrentGameId)
                            .to(Game::Table, Game::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await?;

        let mut rooms_insert_query = Query::insert();
        rooms_insert_query.into_table(Room::Table).columns([
            Room::Id,
            Room::Name,
            Room::Uuid,
            Room::CreatedAt,
        ]);

        for i in 0..10 {
            let room_number = (i as i64) + 1;
            rooms_insert_query.values_panic([
                room_number.into(),
                format!("Room {}", room_number).into(),
                Uuid::new_v4().into(),
                OffsetDateTime::now_utc().into(),
            ]);
        }

        manager.exec_stmt(rooms_insert_query).await?;

        manager
            .create_table(
                Table::create()
                    .table(Player::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Player::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Player::UserId).big_integer().null())
                    .col(ColumnDef::new(Player::GameId).big_integer().null())
                    .col(ColumnDef::new(Player::Uuid).uuid().null())
                    .col(
                        ColumnDef::new(Player::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra(DEFAULT_CURRENT_TIMESTAMP.into()),
                    )
                    .col(
                        ColumnDef::new(Player::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_player_user")
                            .from(Player::Table, Player::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_player_game")
                            .from(Player::Table, Player::GameId)
                            .to(Game::Table, Game::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PlayerCard::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .col(PlayerCard::PlayerId)
                            .col(PlayerCard::CardId),
                    )
                    .col(
                        ColumnDef::new(PlayerCard::PlayerId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PlayerCard::CardId).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_playercard_player")
                            .from(PlayerCard::Table, PlayerCard::PlayerId)
                            .to(Player::Table, Player::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_playercard_card")
                            .from(PlayerCard::Table, PlayerCard::CardId)
                            .to(Card::Table, Card::Id)
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
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(PlayerCard::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(Player::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(Room::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(Game::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(Card::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Room {
    Table,
    Id,
    Name,
    AdminUserId,
    CurrentGameId,
    Uuid,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Game {
    Table,
    Id,
    Uuid,
    FinishedAt,
    FoolUserId,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Player {
    Table,
    Id,
    UserId,
    GameId,
    Uuid,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum PlayerCard {
    Table,
    PlayerId,
    CardId,
}

#[derive(Iden)]
enum Card {
    Table,
    Id,
    Rank,
    Suit,
}
