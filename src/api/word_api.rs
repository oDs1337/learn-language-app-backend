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
        word_single_indefinite: new_word.word_single_indefinite.to_owned(),
        word_single_definite: new_word.word_single_definite.to_owned(),
        word_plural_indefinite: new_word.word_plural_indefinite.to_owned(),
        word_plural_definite: new_word.word_plural_definite.to_owned(),
        word_plural_genitive: new_word.word_plural_genitive.to_owned(),
        picture_url: new_word.picture_url.to_owned(),
    };
    let word_detail = db.add_word(data);
    match word_detail {
        Ok(word) => Ok(Json(word)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/get_word/<path>")]
pub fn get_word(db: &State<MongoRepo>, path: String) -> Result<Json<Word>, Status>{
    let id = path;
    if id.is_empty(){
        return Err(Status::BadRequest);
    };
    let word_detail = db.get_word(&id);
    match word_detail {
        Ok(word) => Ok(Json(word)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/get_all_words")]
pub fn get_all_words(db: &State<MongoRepo>) -> Result<Json<Vec<Word>>, Status>{
    let word_detail = db.get_all_words();
    match word_detail {
        Ok(word) => Ok(Json(word)),
        Err(_) => Err(Status::InternalServerError),
    }
}
