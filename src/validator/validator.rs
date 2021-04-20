use rocket::response::status;
use rocket_contrib::json::Json;
use rocket::http::Status;
use models::response::Response;
use super::serde_types::{Invalid, MultipleError};


pub type ValidatorResult = std::result::Result<(), Invalid>;

pub type Result = std::result::Result<(), MultipleError>;

/// Trait that can be used for accepting types that can be validated
pub trait Accord {
    fn validate(&self) -> Result;
}

pub fn rocket_invalid_data_response(multiple_error: MultipleError) -> status::Custom<Json<Response>> {
        let r = Response{
            message: String::from("Invalid Data"),
            data: Some(serde_json::to_value(multiple_error).unwrap()),
        };

        return status::Custom(
            Status::from_code(400).unwrap(),
            Json(r)
        );
}


#[macro_export]
macro_rules! rules {
    ( $a:expr, [ $( $b:expr ),* ] ) => {{
        let invalids = [$($b(&$a)),*]
           .iter()
           .cloned()
           .filter_map(move |r| r.err())
           .collect::<Vec<_>>();
        if invalids.len() > 0 {
            Err(Error(invalids))
        } else {
            Ok(())
        }
    }};
    ( $( $a:expr => $b:expr => [ $( $c:expr ),* ] ),* ) => {{
        use validator::serde_types::{MultipleError, MultipleInvalid};
		let multiple_invalids = vec![$(MultipleInvalid {
                tag: $a.to_string(),
                invalids: [$($c(&$b)),*]
                    .iter()
                    .cloned()
                    .filter_map(move |r| r.err())
                    .collect::<Vec<_>>()
            }),*]
            .iter()
            .cloned()
            .filter(|m| m.invalids.len() > 0)
            .collect::<Vec<_>>();
        if multiple_invalids.len() > 0 {
            Err(MultipleError(multiple_invalids))
        } else {
            Ok(())
        }
    }};
}



