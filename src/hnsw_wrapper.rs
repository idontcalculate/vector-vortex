// hnsw_wrapper.rs
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use hnsw_wrapper::HnswWrapper; // Import the HnswWrapper struct
use fastembed::{FlagEmbedding, InitOptions, EmbeddingModel};
use std::error::Error;

#[get("/fetch_data")]
async fn fetch_data() -> impl Responder {
    // Implement your logic to fetch data via API here
    // This function should return fetched data as an HTTP response
    HttpResponse::Ok().body("Data fetched successfully!")
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the embedding model
    let model = FlagEmbedding::try_new(InitOptions {
        model_name: EmbeddingModel::BGEBaseEN,
        show_download_message: true,
        ..Default::default()
    })?;

    // Initialize the HNSW wrapper
    let mut hnsw = HnswWrapper::new();

    HttpServer::new(|| {
        App::new()
            .service(fetch_data)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
