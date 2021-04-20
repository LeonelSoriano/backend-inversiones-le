#![allow(clippy::all)]

use diesel::prelude::*;
use diesel::PgConnection;
use schema::{unit, business};
use schema::people::dsl::*;


pub struct Business {
    pub id: i32,
    pub name: String,
    pub sign: String,
}

#[derive(Queryable, Debug)]
pub struct Unit {
    pub id: i32,
    pub name: String,
    pub sign: String,
}

pub struct Params {}

impl Params {

    pub fn findAllUnit (conn: &PgConnection) -> Vec<Unit> {
        unit.order(id.asc()).load::<Unit>(conn).unwrap()
    }

    pub fn findAllBusiness () -> Vec<Business> {
        business.order(id.asc()).load::<Unit>(conn).unwrap()
    }
}
