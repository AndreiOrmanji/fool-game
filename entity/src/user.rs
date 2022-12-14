//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.3

use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "user"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub login: String,
    pub password: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: TimeDateTimeWithTimeZone,
    pub uuid: Option<Uuid>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<TimeDateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Login,
    Password,
    CreatedAt,
    Uuid,
    UpdatedAt,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i64;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Game,
    Player,
    UserToken,
    Room,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Login => ColumnType::String(None).def(),
            Self::Password => ColumnType::String(None).def(),
            Self::CreatedAt => ColumnType::TimestampWithTimeZone.def(),
            Self::Uuid => ColumnType::Uuid.def().null(),
            Self::UpdatedAt => ColumnType::TimestampWithTimeZone.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Game => Entity::has_many(super::game::Entity).into(),
            Self::Player => Entity::has_many(super::player::Entity).into(),
            Self::UserToken => Entity::has_many(super::user_token::Entity).into(),
            Self::Room => Entity::has_many(super::room::Entity).into(),
        }
    }
}

impl Related<super::game::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Game.def()
    }
}

impl Related<super::player::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Player.def()
    }
}

impl Related<super::user_token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserToken.def()
    }
}

impl Related<super::room::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Room.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    /// Create a new ActiveModel with default values. Also used by `Default::default()`.
    fn new() -> Self {
        Self {
            uuid: Set(Some(Uuid::new_v4())),
            ..ActiveModelTrait::default()
        }
    }

    /// Will be called before saving
    fn before_save(self, insert: bool) -> Result<Self, DbErr> {
        if insert || !self.is_changed() {
            Ok(self)
        } else {
            let mut new_self = self.clone();

            new_self.updated_at = Set(Some(TimeDateTimeWithTimeZone::now_utc()));

            Ok(new_self)
        }
    }
}

impl Hash for Model {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.login.hash(state);
        self.uuid.hash(state);
    }
}
