use crate::{
    domain::{
        input::post_in_dto::PostInDto,
        output::{new_post::NewPost, post::Post},
    },
    schema::{posts, posts::dsl::*},
};
use diesel::{pg::PgConnection, prelude::*};

pub fn new_post(conn: &mut PgConnection, dto: PostInDto) -> Result<Post, diesel::result::Error> {
    let new_post = NewPost {
        title: &dto.title.as_str(),
        body: &dto.body,
        slug: &dto.slug,
        published: dto.published,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
}

pub fn get_all_posts(conn: &mut PgConnection) -> Result<Vec<Post>, diesel::result::Error> {
    posts.load::<Post>(conn)
}

pub fn get_post_by_id(conn: &mut PgConnection, _id: i32) -> Post {
    posts
        .find(_id)
        .get_result::<Post>(conn)
        .expect("Error loading post")
}

pub fn by_ids(conn: &mut PgConnection, _ids: Vec<i32>) -> Result<Vec<Post>, diesel::result::Error> {
    posts.filter(id.eq_any(_ids)).load::<Post>(conn)
}

pub fn update_post(
    conn: &mut PgConnection,
    _id: i32,
    dto: PostInDto,
) -> Result<Post, diesel::result::Error> {
    let change = NewPost {
        title: &dto.title.as_str(),
        body: &dto.body,
        slug: &dto.slug,
        published: dto.published,
    };

    diesel::update(posts.find(_id))
        .set(&change)
        .get_result(conn)
}

pub fn delete_post(conn: &mut PgConnection, _id: i32) -> Result<bool, diesel::result::Error> {
    match diesel::delete(posts.find(_id)).execute(conn) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
