use sea_orm_migration::prelude::*;

#[actix_rt::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}
