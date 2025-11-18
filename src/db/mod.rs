use crate::models::user::User;
use dotenvy::dotenv;
use mongodb::{Client, Collection, options::ClientOptions};
use std::env;

#[derive(Clone)]
pub struct MongoRepo {
    pub user_collection: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok(); // Load .env

        let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
        let client_options = ClientOptions::parse(&uri)
            .await
            .expect("Failed to parse MongoDB URI");
        let client = Client::with_options(client_options).expect("Failed to connect to MongoDB");

        let db = client.database("corequarry");
        let user_collection = db.collection::<User>("users");

        MongoRepo { user_collection }
    }
}
