use regex::Regex;

use validator::validator::ValidatorResult;
use validator::serde_types::Invalid;

/// Enforce that a `String` is maximum `max` characters long.
pub fn max(max: usize) -> Box<dyn Fn(&String) -> ValidatorResult> {
    Box::new(move |s: &String| {
        if s.len() <= max {
            Ok(())
        } else {
            Err(Invalid {
                msg: "Must not contain more characters than %1.".to_string(),
                args: vec![max.to_string()],
                human_readable: format!("Must contain less than {} characters", max)
            })
        }
    })
}

// Enforce that a `String` is minimum `min` characters long.
pub fn min(min: usize) -> Box<dyn Fn(&String) -> ValidatorResult> {
    Box::new(move |s: &String| {
        if s.len() >= min {
            Ok(())
        } else {
            Err(Invalid {
                msg: "Must contain more than %1 characters".to_string(),
                args: vec![min.to_string()],
                human_readable: format!("Must contain more than {} characters", min)
            })
        }
    })
}


/// required.
pub fn required() -> Box<dyn Fn(&String) -> ValidatorResult> {
    Box::new(move |s: &String| {
        if s.len() > 0 {
            Ok(())
        } else {
            Err(Invalid {
                msg: "this val is required".to_string(),
                args: vec![],
                human_readable: format!("required but get '{}'", s)
            })
        }
    })
}

// email
pub fn email() -> Box<dyn Fn(&String) -> ValidatorResult> {
    Box::new(move |s: &String| {
        let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();

        if email_regex.is_match(s) == true {
            Ok(())
        } else {
            Err(Invalid {
                msg: "this val its a not email".to_string(),
                args: vec![],
                human_readable: format!("email but get '{}'", s)
            })
        }
    })
}
