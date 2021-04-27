use models::response::{Response, ResponseWithStatus};
use rocket::http::Status;
use config::DbConn;
use models::params::Params;
use constants::message_constants;

pub fn get_all_type_units(conn: DbConn) -> ResponseWithStatus {
    ResponseWithStatus {
        status_code: Status::Ok.code,
        response: Response {
            message: String::from(message_constants::MESSAGE_OK),
            data: Some(serde_json::to_value(Params::find_all_unit(&conn)).unwrap()),
        },
    }
}
