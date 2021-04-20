use config::DbConn;
use constants::message_constants;
use jwt;
use models::response::{Response, ResponseWithStatus};
use models::user::{LoginDTO, User, UserDTO};
use rocket::http::Status;

pub fn signup(user: UserDTO, conn: DbConn) -> ResponseWithStatus {
    if let Some(_) = User::find_user_by_email(&user.email, &conn) {
        return ResponseWithStatus {
            status_code: Status::Forbidden.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_EMAIL_EXIST),
                data: None,
            },
        };
    }

    if let Some(_) = User::find_user_by_username(&user.username, &conn) {
        return ResponseWithStatus {
            status_code: Status::Forbidden.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_USERNAME_EXIST),
                data: None,
            },
        };
    }

    if User::signup(user, &conn) {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_SIGNUP_SUCCESS),
                data: None,
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_SIGNUP_FAILED),
                data: None,
            },
        }
    }
}

pub fn login(login: LoginDTO, conn: DbConn) -> ResponseWithStatus {
    if let Some(result) = User::login(login, &conn) {
        let gender = if result.gender == true {"male".to_string()} else {"female".to_string()};
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_LOGIN_SUCCESS),
                data: Some(serde_json::to_value(json!({
                    "type": "Bearer",
                    "address": result.address,
                    "username": result.username,
                    "birthday": result.birthday.format("%Y-%m-%d").to_string(),
                    "email": result.email,
                    "surname": result.surname,
                    "name": result.name,
                    "phone": result.phone,
                    "gender": gender,
                    "token": jwt::generate_token(result)
                }))
                    .unwrap()),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_LOGIN_FAILED),
                data: None,
            },
        }
    }
}
