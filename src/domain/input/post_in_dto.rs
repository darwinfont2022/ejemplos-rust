use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct PostInDto {
    pub title: String,
    pub body: String,
    pub slug: String,
    pub published: bool,
}
