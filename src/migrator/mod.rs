/*
*  Bookstor API Project
*
* @author Manouchehr Soleymani
* @link https://github.com/Manouchehrsoleymani/rust-bookstor-api
*/
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_table;
mod m20250623_204658_create_author_table;
mod m20250623_210217_create_book_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user_table::Migration),
            Box::new(m20250623_204658_create_author_table::Migration),
            Box::new(m20250623_210217_create_book_table::Migration),
        ]
    }
}
