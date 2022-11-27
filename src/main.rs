use crate::core::AppState;
use actix_web::{
    http::StatusCode, middleware, web, App, HttpResponse, HttpResponseBuilder, HttpServer,
    Responder,
};
use anyhow::Result as AnyhowResult;
use log::debug;
use sea_orm::{prelude::Uuid, ConnectOptions, Database};
use serde_json::json;
use std::{convert::Infallible, env, time::Duration};
use tracing_subscriber::filter::LevelFilter;

mod core;
mod game;
mod user;

#[actix_web::main]
async fn main() -> AnyhowResult<()> {
    dotenv::from_filename(".env").ok();
    dotenv::from_filename(".env.local").ok();

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .with_test_writer()
        .init();

    println!(
        "{:?}",
        env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file")
    );

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_max_connections = env::var("DATABASE_MAX_CONNECTIONS")
        .expect("DATABASE_MAX_CONNECTIONS is not set in .env file")
        .parse::<u32>()
        .unwrap();
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT")
        .expect("PORT is not set in .env file")
        .parse::<u16>()
        .expect("PORT should be a u16");
    let server_url = format!("{}:{}", host, port);

    log::info!("using database at: {}", &db_url);

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(db_max_connections)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Trace)
        .idle_timeout(Duration::from_secs(1));

    let db_pool = Database::connect(opt).await?;

    let app_state = web::Data::new(AppState::new(db_pool));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .route(
                "/help",
                web::get().to(|| async {
                    Ok::<_, Infallible>(HttpResponse::Ok().body(r#"Learn Rust project"#))
                }),
            )
            .route("/", web::get().to(test_card))
            .service(
                web::scope("/users")
                    .configure(user::user_handlers_config)
            )
            .default_service(web::route().to(HttpResponse::MethodNotAllowed))
    });

    let s = server.bind(&server_url)?;

    println!("Starting server at {}", server_url);
    s.run().await?;

    Ok(())
}

async fn test_card() -> impl Responder {
    let u = json!(
        {
            "uuid_str" : "f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4"
        }
    );

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
