use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, doc},
    results::{InsertOneResult},
    Client, Collection,
};

use crate::models::url_model::Url;

pub struct MongoRepo {
    col: Collection<Url>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let database_url = match env::var("DATABASE_URL") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let database_name = match env::var("MONGO_DB") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };


        let client = Client::with_uri_str(database_url).await.unwrap();
        let db = client.database(&database_name);
        let col: Collection<Url> = db.collection("Url");
        MongoRepo { col }
    }

    pub async fn create_url(&self, new_url: Url) -> Result<InsertOneResult, Error> {
        let new_doc = Url {
            id: None,
            short_url: new_url.short_url,
            destination: new_url.destination,
        };

        let url = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating new url");
        Ok(url)
    }

    pub async fn get_url(&self, short_url: &String) -> Result<Url, Error> {
        let url_detail = self
            .col
            .find_one(doc! {"short_url": short_url}, None)
            .await
            .ok()
            .expect("Error when retrieve short_url");
        Ok(url_detail.unwrap())
    }
}
