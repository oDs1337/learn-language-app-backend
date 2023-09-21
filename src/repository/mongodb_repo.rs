use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error},
    results::{InsertOneResult},
    sync::{Client, Collection},
};

use crate::models::word_model::Word;

pub struct MongoRepo{
    col: Collection<Word>,
}

impl MongoRepo {
    pub fn init() -> Self{
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("swedish_words_db");
        let col: Collection<Word> = db.collection("swedish_words");
        MongoRepo{col}
    }

    pub fn add_word(&self, add_word: Word) -> Result<InsertOneResult, Error> {
        let new_doc = Word {
            id: None,
            word: add_word.word,
            word_plural: add_word.word_plural,
            picture_url: add_word.picture_url,
        };
        let word = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error adding word");
        Ok(word)
    }
}