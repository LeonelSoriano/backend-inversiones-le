use config::DbConn;
use jwt::UserToken;
use rocket_contrib::json::Json;
use rocket::response::status;
use models::response::Response;
use rocket::http::Status;
use services::param_service;

#[get("/units")]
pub fn find_all_units(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    conn: DbConn,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    let response = param_service::get_all_type_units(conn);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}
