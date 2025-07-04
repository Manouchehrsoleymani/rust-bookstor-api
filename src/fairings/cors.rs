/*
*  Bookstor API Project
*
* @author Manouchehr Soleymani
* @link https://github.com/Manouchehrsoleymani/rust-bookstor-api
*/

use rocket::{fairing::{Fairing, Info, Kind}, http::Header, Request, Response};


pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers",
            kind: Kind::Response,
        }
    }
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>){
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, PATCH, DELETE, OPTIONS",
    ));
        response.set_header(Header::new("Access-Control-Allow-Headers","*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials","true"));

    }
}

#[options("/<_..>")]
pub fn options() -> &'static str {
    ""
}