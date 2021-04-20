use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use diesel::PgConnection;
use schema::users;
use schema::users::dsl::*;
use uuid::Uuid;
use models::login_history::LoginHistory;
use jwt::UserToken;
use log::error;
use validator::validator::{Accord, Result as AccordResult};
use chrono::NaiveDateTime;
use chrono::DateTime;
use chrono::offset::Utc;
use serde::Serialize;

use validator;

#[derive( Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub address: String,
    pub address_number: String,
    pub birthday: NaiveDateTime,
    pub city: String,
    pub email: String,
    pub gender: bool,
    pub img: Option<String>,
    pub password: String,
    pub phone: String,
    pub username: String,
    pub name: String,
    pub surname: String,
    pub zip: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub login_session: String,
}

#[derive( Serialize)]
pub struct UserOutput {
    pub id: i32,
    pub address: String,
    pub address_number: String,
    pub birthday: NaiveDateTime,
    pub city: String,
    pub email: String,
    pub gender: bool,
    pub phone: String,
    pub username: String,
    pub name: String,
    pub surname: String,
    pub zip: String,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserCreate {
    pub address: String,
    pub address_number: String,
    pub birthday: NaiveDateTime,
    pub city: String,
    pub email: String,
    pub gender: bool,
    pub password: String,
    pub phone: String,
    pub username: String,
    pub name: String,
    pub surname: String,
    pub zip: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct UserDTO {
    pub address: String,
    pub address_number: String,
    pub birthday: NaiveDateTime,
    pub city: String,
    pub email: String,
    pub gender: bool,
    pub phone: String,
    pub username: String,
    pub name: String,
    pub surname: String,
    pub password: String,
    pub zip: String,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct UserImgInsert {
    pub img: Option<String>,
    pub id: i32,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct UserUpdateDto {
    pub address: String,
    pub address_number: String,
    pub birthday: NaiveDateTime,
    pub city: String,
    pub email: String,
    pub gender: bool,
    pub phone: String,
    pub username: String,
    pub name: String,
    pub surname: String,
    pub zip: String,
}

impl Accord for UserDTO {
    fn validate(&self) -> AccordResult {
        rules!{
            "email" => self.email => [validator::validations::email()],
            "username" => self.username => [validator::validations::min(6),
                validator::validations::max(255)],
            "address" => self.address => [validator::validations::min(10),
                validator::validations::max(50)],
            "phone" => self.phone => [validator::validations::min(9),
                validator::validations::max(11)],
            "name" => self.name => [validator::validations::min(4),
                validator::validations::max(60)],
            "surname" => self.surname => [validator::validations::min(4),
                validator::validations::max(60)]
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub username_or_email: String,
    pub password: String,
}

pub struct LoginInfoDTO {
    pub address: String,
    pub username: String,
    pub login_session: String,
    pub birthday: NaiveDateTime,
    pub email: String,
    pub surname: String,
    pub name: String,
    pub phone: String,
    pub gender: bool,
}

impl User {
    pub fn signup(user: UserDTO, conn: &PgConnection) -> bool {
        let hashed_pwd = hash(&user.password, DEFAULT_COST).unwrap();

        let user = UserCreate {
            password: hashed_pwd,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            address: user.address,
            address_number: user.address_number,
            birthday: user.birthday,
            city: user.city,
            email: user.email,
            gender: user.gender,
            phone: user.phone,
            username: user.username,
            name: user.name,
            surname: user.surname,
            zip: user.zip,
        };
        let r = diesel::insert_into(users)
            .values(&user)
            .execute(conn)
            .is_ok();
        r
    }

    pub fn find_user_by_email(em: &str, conn: &PgConnection) -> Option<User> {
        let result_user = users.filter(email.eq(em)).get_result::<User>(conn);
        if let Ok(user) = result_user {
            Some(user)
        } else {
            None
        }
    }


    pub fn login(login: LoginDTO, conn: &PgConnection) -> Option<LoginInfoDTO> {
        let user_to_verify = match users
            .filter(username.eq(&login.username_or_email))
            .or_filter(email.eq(&login.username_or_email))
            .get_result::<User>(conn) {
                Ok(user)  => user,
                Err(e) => { error!("user not found {}", e); return None;},
            };

        if !user_to_verify.password.is_empty()
            && verify(&login.password, &user_to_verify.password).unwrap()
        {
            if let Some(login_history) = LoginHistory::create(&user_to_verify.username, conn) {
                if !LoginHistory::save_login_history(login_history, conn) {
                    return None;
                }
                let login_session_str = User::generate_login_session();
                User::update_login_session_to_db(&user_to_verify.username, &login_session_str, conn);
                Some(LoginInfoDTO {
                    username: user_to_verify.username,
                    login_session: login_session_str,
                    address: user_to_verify.address,
                    birthday: user_to_verify.birthday,
                    email: user_to_verify.email,
                    surname: user_to_verify.surname,
                    name: user_to_verify.name,
                    phone: user_to_verify.phone,
                    gender: user_to_verify.gender,
                })
            } else {
                None
            }
        } else {
            None
        }
    }


    pub fn is_valid_login_session(user_token: &UserToken, conn: &PgConnection) -> bool {
        users
            .filter(username.eq(&user_token.user))
            .filter(login_session.eq(&user_token.login_session))
            .get_result::<User>(conn)
            .is_ok()
    }

    pub fn find_user_by_username(un: &str, conn: &PgConnection) -> Option<User> {
        let result_user = users.filter(username.eq(un)).get_result::<User>(conn);
        if let Ok(user) = result_user {
            Some(user)
        } else {
            None
        }
    }

    pub fn update_user_img(user_name_input: &str , input_img: &str, conn: &PgConnection) -> bool {
        diesel::update(users.filter(username.eq(user_name_input)))
            .set(img.eq(input_img))
            .get_result::<User>(conn).is_ok()
    }


    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_simple().to_string()
    }

    pub fn update_login_session_to_db(un: &str, login_session_str: &str, conn: &PgConnection) -> bool {
        if let Some(user) = User::find_user_by_username(un, conn) {
            diesel::update(users.find(user.id))
            .set(login_session.eq(login_session_str.to_string()))
            .execute(conn)
            .is_ok()
        } else {
            false
        }
    }

    pub fn update_user_by_token(i: i32, user_update: UserUpdateDto, conn: &PgConnection) -> bool {
        diesel::update(users.find(i))
            .set(&user_update)
            .execute(conn)
            .is_ok()
    }
}
