use models::response::{Response, ResponseWithStatus};
use models::user::User;
use models::user::UserOutput;
use rocket::http::Status;
use config::DbConn;
use constants::message_constants;
use models::user::UserUpdateDto;
use rocket::Data;
use std::fs;
use std::path::Path;
use until;

const DEFAULT_PATH_IMAGEN: &str = "img/";


pub fn get_user_by_token(username: &str, conn: DbConn) -> ResponseWithStatus {
    if let Some(user) = User::find_user_by_username(username, &conn) {
        let user_output = UserOutput {
            address: user.address,
            address_number: user.address_number,
            birthday: user.birthday,
            city: user.city,
            email: user.email,
            gender: user.gender,
            id: user.id,
            name: user.name,
            phone: user.phone,
            surname: user.surname,
            username: user.username,
            zip: user.zip,
        };
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_OK),
                data: Some(serde_json::to_value(user_output).unwrap()),
            },
        }
    } else  {
        ResponseWithStatus {
            status_code: Status::Forbidden.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_USER_NOT_EXIST),
                data: None,
            },
        }
    }
}

pub fn update_user_img(username: &str, ex: &str, paste: Data, conn: DbConn) -> ResponseWithStatus {

    let user_imagen = User::find_user_by_username(username, &conn).unwrap().img;

    let img_name = until::get_random_name();

    let name_new_img = format!("{}.{}", img_name, ex);

    if User::update_user_img(username, &name_new_img,&conn) {

        if user_imagen.is_some() {
            let validate_user_imagen: String = user_imagen.unwrap();
            let path_img = format!("{}{}",DEFAULT_PATH_IMAGEN, validate_user_imagen);
            match fs::remove_file(path_img) {
                Ok(()) => {info!("user imagen remove succesfully")}
                Err(_) => {error!("Cant remove user imagen")}
            }
        }

        let new_img_path = format!("{}{}",DEFAULT_PATH_IMAGEN, name_new_img);
        match paste.stream_to_file(Path::new(&new_img_path)) {
            Ok(_) => {info!("create new user imagen")}
            Err(_) => {error!("Cant create new user img")}
        }

        /* let data = r#"
        {
            "msg": "ok"
        }"#;

        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_OK),
                data: Some(serde_json::from_str(data).unwrap()),
            },
        }
        */
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_OK),
                data: None,
            },
        }
    } else  {
        ResponseWithStatus {
            status_code: Status::Forbidden.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_IMG_CANT_SAVE),
                data: None,
            },
        }
    }
}

pub fn get_imagen_path(username: &str, conn: DbConn) -> Option<String> {
    if let Some(user) = User::find_user_by_username(username, &conn) {
        if let Some(img) = user.img {
            let img_path = format!("{}{}", DEFAULT_PATH_IMAGEN, img);
            return Some(img_path)
        }

        return None
    }
    None
}


pub fn update_user_by_token(username: &str, input: UserUpdateDto,conn: DbConn) -> ResponseWithStatus {
    if let Some(user) = User::find_user_by_username(username, &conn) {

        if User::update_user_by_token(user.id, input, &conn) {
            ResponseWithStatus {
                status_code: Status::Ok.code,
                response: Response {
                    message: String::from(message_constants::MESSAGE_OK),
                    data: Some(serde_json::to_value(true).unwrap()),
                },
            }
        } else {

            ResponseWithStatus {
                status_code: Status::Forbidden.code,
                response: Response {
                    message: String::from(message_constants::MESSAGE_USER_NOT_EXIST),
                    data: None,
                },
            }

        }

    } else  {
        ResponseWithStatus {
            status_code: Status::Forbidden.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_USER_NOT_EXIST),
                data: None,
            },
        }
    }
}
