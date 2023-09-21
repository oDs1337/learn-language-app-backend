use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
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
            word_single_indefinite: add_word.word_single_indefinite,
            word_single_definite: add_word.word_single_definite,
            word_plural_indefinite: add_word.word_plural_indefinite,
            word_plural_definite: add_word.word_plural_definite,
            word_plural_genitive: add_word.word_plural_genitive,
            picture_url: add_word.picture_url,
        };
        let word = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error adding word");
        Ok(word)
    }

    pub fn get_word(&self, id: &String) -> Result<Word, Error>{
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let word_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting word");
        Ok(word_detail.unwrap())
    }

    pub fn get_all_words(&self) -> Result<Vec<Word>, Error>{
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of users");
        let words = cursors.map(|doc| doc.unwrap()).collect();
        Ok(words)
    }
}