use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;
use std::error::Error;

// Define a struct representing a document
#[derive(Debug, Serialize, Deserialize)]
struct Document {
    // Define the structure of the document
    // Adjust fields based on the actual data structure
    id: u64,
    title: String,
    content: String,
}

async fn fetch_documents() -> Result<Vec<Document>, Box<dyn Error>> {
    // Make HTTP request to fetch documents from the API
    let response = reqwest::get("https://example.com/api/documents")
        .await?
        .json::<Vec<Document>>()
        .await?;
    Ok(response)
}

async fn index() -> impl Responder {
    // Fetch documents and return as JSON response
    match fetch_documents().await {
        Ok(documents) => HttpResponse::Ok().json(documents),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
