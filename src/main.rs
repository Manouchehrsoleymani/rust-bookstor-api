/*
*  Bookstor API Project
*
* @author Manouchehr Soleymani
* @link https://github.com/Manouchehrsoleymani/rust-bookstor-api
*/

#[macro_use] 
extern crate rocket;
use sea_orm_migration::MigratorTrait;

use crate::migrator::Migrator;
mod migrator;
mod db;

pub struct AppConfig{
    db_host:String,
    db_port:String,
    db_username:String,
    db_password:String,
    db_name:String,
}
impl Default for AppConfig{
    fn default() -> Self {
        Self {
            db_name: std::env::var("BOOKSTOR_DB_NAME").unwrap_or("bookstore".to_string()),
            db_username: std::env::var("BOOKSTOR_DB_USER").unwrap_or("root".to_string()),
            db_password: std::env::var("BOOKSTOR_DB_PASS").unwrap_or("my_secret".to_string()),
            db_host: std::env::var("BOOKSTOR_DB_HOST").unwrap_or("localhost".to_string()),
            db_port: std::env::var("BOOKSTOR_DB_PORT").unwrap_or("3306".to_string()),

        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    let config=AppConfig::default();
    let db=match db::connect(&config).await {
        Ok(db)=> db,
        Err(err)=> panic!("{}",err),
        
    };
    match Migrator::up(&db, None).await {
        Ok(_)=>(),
        Err(err)=>panic!("{}",err)
    };
    rocket::build().mount("/", routes![index])
}
