use rocket_contrib::json::Json;
use rocket::request::Form;
use config::DbConn;
use jwt::UserToken;
use rocket::response::status;
use models::response::Response;
use services::user_service;
use rocket::http::Status;
use std::fs::File;
use rocket::Data;

#[derive(FromForm)]
pub struct InfoImg {
    pub ex: String,
}

// #[post("/upload-img-user?<info..>", data = "<paste>")]
// pub fn load_user_imagen(info: Form<InfoImg>, paste: Data ) -> Result<JsonValue, std::io::Error> {
#[post("/upload-img-user?<info..>", data = "<paste>")]
pub fn load_user_imagen(info: Form<InfoImg>,
                        paste: Data,
                        token: Result<UserToken,status::Custom<Json<Response>>>,
                        conn: DbConn) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    let response = user_service::update_user_img(&token.unwrap().user, &info.ex,paste, conn );

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

#[get("/user-img")]
pub fn retrieve_user_img(
        token: Result<UserToken,status::Custom<Json<Response>>>,
        conn: DbConn
    ) -> Option<File> {
    if token.is_err() {
        println!("toke {:?}", token);
        return None
    }

    if let Some(img) = user_service::get_imagen_path(&token.unwrap().user, conn) {
        print!("{}", img);
        return File::open(&img).ok()
    }

    None
}
