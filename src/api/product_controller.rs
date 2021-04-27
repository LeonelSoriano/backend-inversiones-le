/*use config::DbConn;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use models::response::Response;



#[post("/", format = "json", data = "<user>")]
pub fn create_token(
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
*/
