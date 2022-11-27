pub use sea_orm_migration::prelude::*;

mod m20221021_000001_create_table_user;
mod m20221025_000002_create_table_room;

const DEFAULT_CURRENT_TIMESTAMP: &str = "DEFAULT CURRENT_TIMESTAMP";

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221021_000001_create_table_user::Migration),
            Box::new(m20221025_000002_create_table_room::Migration),
        ]
    }
}
