// api_client_test.rs
// tests/api_client_test.rs
use actix_web::{web, App, HttpServer};
use serde_json::Value;

#[actix_rt::test]
async fn test_fetch_posts() {
    // Start the Actix web server in the background
    let server = HttpServer::new(|| {
        App::new().route("/posts", web::get().to(handler))
    })
    .bind("127.0.0.1:8080") // Bind to localhost on port 8080
    .expect("Failed to bind to port 8080")
    .run();

    // Make an HTTP GET request to the local Actix web server
    let response = reqwest::get("http://127.0.0.1:8080/posts")
        .await
        .expect("Failed to send GET request");

    // Assert that the response status code is 200 OK
    assert_eq!(response.status().as_u16(), 200);

    // Parse the response body as JSON
    let json: Value = response
        .json()
        .await
        .expect("Failed to parse response body as JSON");

    // Assert that the response body is not empty
    assert!(!json.is_null());
}

async fn handler() -> web::Json<Value> {
    // Simulate fetching posts from a database or external API
    let posts = vec![
        json!({"id": 1, "title": "Post 1", "body": "This is the body of post 1"}),
        json!({"id": 2, "title": "Post 2", "body": "This is the body of post 2"}),
        // Add more posts here if needed
    ];

    web::Json(json!(posts))
}

