use crate::core::AppState;
use actix_web::middleware;
use actix_web::{get, web::ServiceConfig};
use actix_web::{http::StatusCode, web, HttpResponse, HttpResponseBuilder, Responder};
use log::debug;
use migration::{Migrator, MigratorTrait};
use sea_orm::prelude::Uuid;
use sea_orm::SqlxPostgresConnector;
use serde_json::json;
use shuttle_service::ShuttleActixWeb;
// use shuttle_shared_db::Postgres;
use shuttle_aws_rds::Postgres;
use shuttle_secrets::{SecretStore, Secrets};
use sqlx::PgPool;
use std::{convert::Infallible, env};

mod core;
mod game;
mod user;

#[get("/hello")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[shuttle_service::main]
async fn actix_web(
    #[Postgres] pool: PgPool,
    #[Secrets] secret_store: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Sync + Send + Clone + 'static> {
    let db_pool = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);

    Migrator::up(&db_pool, None).await.unwrap();

    let app_state = web::Data::new(AppState::new(db_pool));

    Ok(move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("")
                .wrap(middleware::Logger::default())
                .app_data(app_state.clone())
                .service(hello_world)
                .route(
                    "/help",
                    web::get().to(|| async {
                        Ok::<_, Infallible>(HttpResponse::Ok().body(r#"Learn Rust project"#))
                    }),
                )
                .route("/", web::get().to(test_card))
                .service(web::scope("/users").configure(user::user_handlers_config))
                .default_service(web::route().to(HttpResponse::MethodNotAllowed)),
        );
    })
}

async fn test_card() -> impl Responder {
    let u = json!(
        {
            "uuid_str" : "f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4"
        }
    );
    for (key, value) in env::vars() {
        println!("{key}: {value}");
        tracing::info!("{key}: {value}");
    }

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct Q {
        uuid_str: Uuid,
    }

    let qqq: Q = serde_json::from_str(u.to_string().as_str()).unwrap();
    debug!("{:?} v: {}\n", &qqq, qqq.uuid_str.get_version_num());

    let s = playin_cards::gen_shoe(1, false);
    let b: String = s.iter().map(|c| format!("{}", c)).collect();
    HttpResponseBuilder::new(StatusCode::OK)
        .reason("who knows")
        .body(format!("{}\n", b))
}
