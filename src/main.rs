mod vector_store;
mod hnsw_wrapper;
mod operations; // Assuming you have an operations module, if not, this can be removed.

use vector_store::{VectorStore, Vector};
use hnsw_wrapper::HnswWrapper;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the HNSW wrapper (assuming HnswWrapper::new() is implemented).
    let hnsw_wrapper = HnswWrapper::new();

    // Create a new VectorStore with the HNSW wrapper.
    let mut vector_store = VectorStore::new(hnsw_wrapper);

    // Add some vectors to the store.
    vector_store.add_vector("vector1".to_string(), vec![0.1, 0.2, 0.3, 0.4]);
    vector_store.add_vector("vector2".to_string(), vec![0.5, 0.6, 0.7, 0.8]);
    // Add more vectors as needed.

    // Search for the k nearest neighbors to a query vector.
    let query_vector = vec![0.15, 0.25, 0.35, 0.45]; // This is an example query vector.
    let k = 2; // Number of nearest neighbors to find.
    let nearest_neighbors = vector_store.search_nearest_neighbors(&query_vector, k);

    // Display the search results.
    println!("Nearest Neighbors:");
    for (id, similarity) in nearest_neighbors {
        println!("ID: {}, Similarity: {}", id, similarity);
    }

    Ok(())
}



