// Importing necessary dependencies for the Actix Web server and MongoDB interaction
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use mongodb::{Client, Collection};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;
use futures_util::stream::TryStreamExt;

// Item struct for handling item data
#[derive(Serialize, Deserialize)]
struct Item {
    name: String,         
    description: String,  
}

// DeleteRequest struct to handle incoming delete requests
#[derive(Deserialize)]
struct DeleteRequest {
    name: String,       
}

// Enum for API response
enum ApiResponse {
    Success(String),
    Created(String),
    NotFound(String),
    InternalServerError(String),
}


impl ApiResponse {
    // This function converts the ApiResponse enum into an actual HTTP response
    fn to_http_response(&self) -> HttpResponse {
        match self {
            ApiResponse::Success(msg) => HttpResponse::Ok().body(msg.clone()),
            ApiResponse::Created(msg) => HttpResponse::Created().body(msg.clone()),
            ApiResponse::NotFound(msg) => HttpResponse::NotFound().body(msg.clone()),
            ApiResponse::InternalServerError(msg) => HttpResponse::InternalServerError().body(msg.clone()),
        }
    }
}

// This function connects to MongoDB using the URI defined in the .env file
async fn connect_to_mongodb() -> mongodb::error::Result<Collection<Item>> {
    dotenv().ok();
    let db_uri = env::var("MONGO_URI").expect("MONGO_URI must be set in the .env file");
    let client = Client::with_uri_str(&db_uri).await?;
    let db = client.database("rust_api_db"); // Database name
    Ok(db.collection::<Item>("items")) // Collection name with type `Item`
}

// Function to create a new item (POST)
async fn create_item(item: web::Json<Item>) -> impl Responder {
    let collection = connect_to_mongodb().await.unwrap();
    let new_item = item.into_inner();

    // Trying to  insert the new item into the MongoDB collection
    match collection.insert_one(new_item, None).await {
        Ok(_) => ApiResponse::Created("Item created successfully".to_string()).to_http_response(),
        Err(_) => ApiResponse::InternalServerError("Failed to create item".to_string()).to_http_response(),
    }
}

// Function to read all items (GET)
async fn get_items() -> impl Responder {
    let collection = connect_to_mongodb().await.unwrap();
    let mut cursor = collection.find(None, None).await.unwrap();

    let mut items = Vec::new();
    while let Ok(Some(item)) = cursor.try_next().await {
        items.push(item);
    }

    HttpResponse::Ok().json(items)
}

// Function to update an item (PUT)
async fn update_item(item: web::Json<Item>) -> impl Responder {
    let collection = connect_to_mongodb().await.unwrap();

    // Try updating the item in the collection by matching its name
    match collection.update_one(
        doc! { "name": &item.name },
        doc! { "$set": { "description": &item.description } },
        None
    ).await {
        Ok(result) if result.matched_count > 0 => ApiResponse::Success("Item updated successfully".to_string()).to_http_response(),
        Ok(_) => ApiResponse::NotFound("Item not found".to_string()).to_http_response(),
        Err(_) => ApiResponse::InternalServerError("Failed to update item".to_string()).to_http_response(),
    }
}

// Function to delete an item (DELETE)
async fn delete_item(item: web::Json<DeleteRequest>) -> impl Responder {
    let collection = connect_to_mongodb().await.unwrap();

    // Try deleting the item based on its name
    match collection.delete_one(doc! { "name": &item.name }, None).await {
        Ok(result) if result.deleted_count > 0 => ApiResponse::Success("Item deleted successfully".to_string()).to_http_response(),
        Ok(_) => ApiResponse::NotFound("Item not found".to_string()).to_http_response(),
        Err(_) => ApiResponse::InternalServerError("Failed to delete item".to_string()).to_http_response(),
    }
}

// Main function to set up Actix-web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    println!("Server starting...");
    HttpServer::new(|| {
        App::new()
            .route("/create", web::post().to(create_item))
            .route("/get", web::get().to(get_items))
            .route("/update", web::put().to(update_item))
            .route("/delete", web::delete().to(delete_item))
    })
    .bind("127.0.0.1:8080")? // Server binds on localhost and port 8080
    .run()
    .await
}

