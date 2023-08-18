use crate::schema::posts;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, AsChangeset, Deserialize, Serialize, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub slug: String,
    pub published: bool,
}
