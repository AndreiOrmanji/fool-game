use entity::user::{
    ActiveModel as UserActiveModel, Column, Entity as UserEntity, Model as UserModel,
};
use sea_orm::{
    entity::*,
    prelude::*, // ActiveModelBehavior, ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    ActiveValue,
    DatabaseConnection,
    DbErr,
};

pub async fn find_by_uuid(db: &DatabaseConnection, uuid: Uuid) -> Result<Option<UserModel>, DbErr> {
    let user = UserEntity::find()
        .filter(Column::Uuid.eq(uuid))
        .one(db)
        .await?;

    Ok(user)
}

pub async fn create_user(
    db: &DatabaseConnection,
    username: &str,
    password: &str,
) -> Result<UserModel, DbErr> {
    let mut u = UserActiveModel::new();
    u.login = Set(username.into());
    u.password = Set(password.into());

    let inserted_user = u.insert(db).await?;

    Ok(inserted_user)
}

pub async fn update_user(
    db: &DatabaseConnection,
    user: UserModel,
    username: &Option<String>,
    password: &Option<String>,
) -> Result<UserModel, DbErr> {
    if username.is_none() && password.is_none() {
        return Ok(user);
    }

    let mut at_least_one_changed = false;
    let mut u = user.into_active_model();

    if let Some(new_username) = username {
        let old_login_matches_new = match &u.login {
            ActiveValue::Set(old_login_set) => old_login_set == new_username,
            ActiveValue::Unchanged(old_login_unchanged) => old_login_unchanged == new_username,
            ActiveValue::NotSet => false,
        };

        if !old_login_matches_new {
            at_least_one_changed = true;
            u.login = Set(new_username.into());
        }
    }

    if let Some(new_password) = password {
        let old_password_matches_new = match &u.password {
            ActiveValue::Set(old_password_set) => old_password_set == new_password,
            ActiveValue::Unchanged(old_password_unchanged) => {
                old_password_unchanged == new_password
            }
            ActiveValue::NotSet => false,
        };

        if !old_password_matches_new {
            at_least_one_changed = true;
            u.password = Set(new_password.into());
        }
    }

    if at_least_one_changed {
        Ok(u.update(db).await?)
    } else {
        Ok(u.try_into_model()?)
    }
}
