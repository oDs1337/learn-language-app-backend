use crate::{models::word_model::Word, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[post("/add_word", data = "<new_word>")]
pub fn add_word(
    db: &State<MongoRepo>,
    new_word: Json<Word>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Word {
        id: None,
        word: new_word.word.to_owned(),
        word_plural: new_word.word_plural.to_owned(),
        picture_url: new_word.picture_url.to_owned(),
    };
    let word_detail = db.add_word(data);
    match word_detail {
        Ok(word) => Ok(Json(word)),
        Err(_) => Err(Status::InternalServerError),
    }
}