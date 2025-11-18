use crate::{db::MongoRepo, models::user::User};
use axum::{Json, extract::State};
use futures::stream::TryStreamExt;
use mongodb::bson::doc;

pub async fn create_user(State(repo): State<MongoRepo>, Json(user): Json<User>) -> Json<User> {
    let mut new_user = user.clone();
    new_user.id = None; // MongoDB generate ObjectId
    let insert_result = repo
        .user_collection
        .insert_one(new_user.clone(), None)
        .await
        .expect("Failed to insert user");

    new_user.id = insert_result.inserted_id.as_object_id();
    Json(new_user)
}

pub async fn get_users(State(repo): State<MongoRepo>) -> Json<Vec<User>> {
    let cursor = repo
        .user_collection
        .find(None, None)
        .await
        .expect("Failed to fetch users");

    let users: Vec<User> = cursor.try_collect().await.expect("Failed to collect users");
    Json(users)
}

pub async fn get_user_by_id(
    State(repo): State<MongoRepo>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Option<Json<User>> {
    let obj_id = mongodb::bson::oid::ObjectId::parse_str(&id).ok()?;
    let user = repo
        .user_collection
        .find_one(doc! { "_id": obj_id }, None)
        .await
        .expect("Failed to fetch user")?;
    Some(Json(user))
}
