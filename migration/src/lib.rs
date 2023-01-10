pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230110_123833_create_user;
mod m20230110_123842_create_link;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230110_123833_create_user::Migration),
            Box::new(m20230110_123842_create_link::Migration),
        ]
    }
}
