/*
*  Bookstor API Project
*
* @author Manouchehr Soleymani
* @link https://github.com/Manouchehrsoleymani/rust-bookstor-api
*/

#[macro_use] 
extern crate rocket;
use rocket::{http::Status};
use sea_orm_migration::MigratorTrait;

use crate::{ controllers::{ErrorResponse, Response, SuccessResponse}, migrator::Migrator};
mod migrator;
mod db;
mod controllers;

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
fn index() -> Response<String> {
    Ok(SuccessResponse((Status::Ok,"Hello World!".to_string())))
}

#[launch]
async fn rocket() -> _ {
    let config=AppConfig::default();
    
    let db = match db::setup_db(&config).await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };
    // let db=match db::connect(&config).await {
    //     Ok(db)=> db,
    //     Err(err)=> panic!("{}",err),
        
    // };
    match Migrator::up(&db, None).await {
        Ok(_)=>(),
        Err(err)=>panic!("{}",err)
    };
    rocket::build()
    .mount("/", routes![index])
    .mount("/auth", routes![
        controllers::auth::signin,
        controllers::auth::signup
    ])
    .mount("/authors", routes![
        controllers::authors::index,
        controllers::authors::create,
        controllers::authors::show,
        controllers::authors::update,
        controllers::authors::delete
    ])
    .mount("/books", routes![
        controllers::books::index,
        controllers::books::create,
        controllers::books::show,
        controllers::books::update,
        controllers::books::delete
    ])

}
