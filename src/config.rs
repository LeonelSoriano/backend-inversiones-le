use api::account_controller::*;
use api::address_book_controller::*;
use api::user_controller::*;
use api::img_controller::*;
use api::params_api_controller::*;
use diesel::pg::PgConnection;
use rocket::Rocket;
use rocket::{Request, Response};
use rocket::response::content::Html;
use dotenv::dotenv;
use rocket::fairing::{Fairing, Info, Kind, AdHoc};
use rocket::http::{Header, ContentType, Method, Status};
use std::io::Cursor;

embed_migrations!();

#[database("postgres_database")]
pub struct DbConn(PgConnection);


pub struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {

        if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON) {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new("Access-Control-Allow-Methods", "POST, PUT,GET, OPTIONS"));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new("Response"));
            response.set_status(Status::new(200, "No Content"));
        }
       response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
    }
}


/*
#[derive(Default)]
pub struct SerdeError();

impl Fairing for SerdeError {
    fn info(&self) -> Info {
        Info {
            name: "For Serde error",
            kind: Kind::Response
        }
    }

    fn on_launch(&self, rocket: &Rocket) {
        println!("4");
    }

    fn on_request(&self, request: &mut Request, data: &Data) {
         println!("Hello1");
        println!("{:?}", request);
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        println!("Hello");
        println!("{:?}", response);
    }
}*/


#[catch(404)]
fn not_found(request: &Request<'_>) -> Html<String> {
    let html = match request.format() {
        Some(ref mt) if !mt.is_json() && !mt.is_plain() => {
            format!("<p>Sorry, '{}' is an invalid path!</p>",
                 request.uri())
        }
        _ => format!("<p>Sorry2, '{}' is an invalid path! Try \
                 /hello/&lt;name&gt;/&lt;age&gt; instead.</p>",
                 request.uri())
    };

    Html(html)
}



pub fn rocket() -> (Rocket, Option<DbConn>) {
    dotenv().ok();

    let rocket = rocket::ignite()
        .attach(DbConn::fairing())
        .attach(CORS())
        //.attach(SerdeError::default())
        .attach(AdHoc::on_attach("Database Migrations", |rocket| {
            let conn = DbConn::get_one(&rocket).expect("database connection");
            match embedded_migrations::run(&*conn) {
                Ok(()) => Ok(rocket),
                Err(e) => {
                    error!("Failed to run database migrations: {:?}", e);
                    Err(rocket)
                }
            }
        }))
        .mount("/api/img", routes![load_user_imagen, retrieve_user_img])
        .mount("/api/auth", routes![login, signup, res_hola])
        .mount("/api/params", routes![find_all_units])
        .mount(
            "/api/address-book",
            routes![find_all, find_by_id, query, insert, update, delete],
        )
        .mount("/api/user", routes![get_user_by_token, update_user_by_token])
        .register(catchers![not_found]);

    let conn = if cfg!(test) {
        DbConn::get_one(&rocket)
    } else {
        None
    };

    (rocket, conn)
}
