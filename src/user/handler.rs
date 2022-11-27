use actix_web::{post, put, web, HttpResponse};
use sea_orm::{prelude::Uuid, DbErr};

use serde::Deserialize;

use crate::{
    core::{AppState, ErrorResponder},
    user::repository,
};

// this function could be located in a different module
pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(store_user).service(update_user);
}

#[derive(Deserialize)]
pub struct NewUser {
    username: String,
    password: String,
}

#[post("/sign-up")]
async fn store_user(
    app_state: web::Data<AppState>,
    new_user: web::Json<NewUser>,
) -> Result<HttpResponse, ErrorResponder> {
    repository::create_user(
        app_state.get_db_conn(),
        &new_user.username,
        &new_user.password,
    )
    .await?;

    Ok(HttpResponse::Created().finish())
}

#[derive(Deserialize)]
pub struct UpdateUser {
    username: Option<String>,
    password: Option<String>,
}

#[put("/{user_uuid}")]
async fn update_user(
    app_state: web::Data<AppState>,
    user_uuid: web::Path<Uuid>,
    update_user: web::Json<UpdateUser>,
) -> Result<HttpResponse, ErrorResponder> {
    let q = std::time::Instant::now();

    let db = app_state.get_db_conn();
    let uuid = user_uuid.into_inner();
    let map_err_closure = |e: DbErr| -> ErrorResponder { e.into() };

    let existing_user = repository::find_by_uuid(db, uuid)
        .await
        .map_err(map_err_closure)?;

    let q2 = std::time::Instant::now();
    println!("select and deserialize: {:?}", q.elapsed());
    match existing_user {
        Some(user) => {
            let updated_user =
                repository::update_user(db, user, &update_user.username, &update_user.password)
                    .await
                    .map_err(map_err_closure)?
                    ;
            println!(
                "update and deserialize updated: {:?}. \nFinal: {:?}",
                q2.elapsed(),
                q.elapsed()
            );
            Ok(HttpResponse::Ok().json(updated_user))
            // Ok(HttpResponse::Ok().json(updated_user))
        }
        None => Ok(HttpResponse::NotFound().finish()),
    }
}
