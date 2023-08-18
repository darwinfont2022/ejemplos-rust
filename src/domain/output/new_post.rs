use crate::schema::posts;

use diesel::prelude::*;

#[derive(Insertable, AsChangeset, Debug)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str,
    pub published: bool,
}
