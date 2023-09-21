use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Word {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub word_single_indefinite: String,
    pub word_single_definite: String,
    pub word_plural_indefinite: String,
    pub word_plural_definite: String,
    pub word_plural_genitive: String,
    pub picture_url: String,
}
