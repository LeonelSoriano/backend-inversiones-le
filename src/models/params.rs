#![allow(clippy::all)]

use diesel::prelude::*;
use diesel::PgConnection;
use schema::{unit, business};
use schema::unit::dsl::*;
use schema::business::dsl::*;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Business {
    pub id: i64,
    pub name: String,
    pub sign: String,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Unit {
    pub id: i64,
    pub name: String,
    pub sign: String,
}

pub struct Params {}

impl Params {
    pub fn find_all_unit (conn: &PgConnection) -> Vec<Unit> {
        unit.order(unit::id.asc()).load::<Unit>(conn).unwrap()
    }

    pub fn find_all_business (conn: &PgConnection) -> Vec<Business> {
        business.order(business::id.asc()).load::<Business>(conn).unwrap()
    }
}
