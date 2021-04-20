use config::DbConn;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use models::response::Response;
use models::user::{LoginDTO, UserDTO};
use services::account_service;
use validator::validator::{Accord, Result as AccordResult, rocket_invalid_data_response};
use validator::validations::{max, required, email};

#[post("/signup", format = "json", data = "<user>")]
pub fn signup(user: Json<UserDTO>, conn: DbConn) -> status::Custom<Json<Response>> {
    if let Err(multiple_error) = user.validate() {
        return rocket_invalid_data_response(multiple_error);
    }

    let response = account_service::signup(user.0, conn);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

#[post("/login", format = "json", data = "<login>")]
pub fn login(login: Json<LoginDTO>, conn: DbConn) -> status::Custom<Json<Response>> {
    let response = account_service::login(login.0, conn);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    pub name: String,
    pub email: String,
    pub age: i8,
}

impl Accord for Account {
    fn validate(&self) -> AccordResult {
        rules!{
            "name" => self.name => [max(10)],
            "email" => self.email => [email()],
            "age" => self.email => [max(1)]
        }
    }
}

#[get("/hola")]
pub fn res_hola() -> status::Custom<Json<Response>> {
    //validator::hola();
    let account = Account {
        name: "hola mundo son con 10 LDLDLDLDL".to_string(),
        email: "sampe@algo".to_string(),
        age: 1,
    };

    if let Err(multiple_error) = account.validate() {
        println!("Errors as json: {}",
                 serde_json::to_string(&multiple_error).unwrap());

        return rocket_invalid_data_response(multiple_error);
    }

    let r = Response{
        message: String::from("hola"),
        data: None,
    };

    status::Custom(
        Status::from_code(200).unwrap(),
        Json(r)
    )
}
