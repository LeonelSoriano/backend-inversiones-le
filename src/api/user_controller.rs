use config::DbConn;
use jwt::UserToken;
use rocket_contrib::json::Json;
use rocket::response::status;
use models::response::Response;
use services::user_service;
use rocket::http::Status;
use models::user::UserUpdateDto;

#[get("/")]
pub fn get_user_by_token(
        token: Result<UserToken,status::Custom<Json<Response>>>,
        conn: DbConn) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let response = user_service::get_user_by_token(&token.unwrap().user, conn);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

#[put("/", format = "json", data = "<user>")]
pub fn update_user_by_token(
        user: Json<UserUpdateDto>,
        token: Result<UserToken,status::Custom<Json<Response>>>,
        conn: DbConn) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }

    let response = user_service::update_user_by_token(&token.unwrap().user, user.0, conn);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}
