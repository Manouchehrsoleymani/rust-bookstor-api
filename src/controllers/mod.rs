/*
*  Bookstor API Project
*
* @author Manouchehr Soleymani
* @link https://github.com/Manouchehrsoleymani/rust-bookstor-api
*/

use rocket::http::Status;

pub mod auth;
pub mod authors;
pub mod books;

#[derive(Responder)]
pub struct SuccessResponse<T> (pub (Status,T));

#[derive(Responder)]
pub struct ErrorResponse(pub (Status,String));


pub type Response<T> = Result<SuccessResponse<T>, ErrorResponse>;