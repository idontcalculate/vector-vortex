// vector_store.rs
use std::collections::HashMap;
use std::sync::Arc;
use crate::hnsw_wrapper::HnswWrapper;

/// A simple structure to represent a high-dimensional vector.
pub struct Vector {
    pub id: String,
    pub values: Arc<Vec<f32>>,
}

/// A basic in-memory vector store.
pub struct VectorStore {
    vectors: HashMap<String, Vector>, // Store the entire Vector to access both ID and values.
    hnsw: HnswWrapper,
}

impl VectorStore {
    /// VectorStore plus HNSW wrapper.
    pub fn new(hnsw: HnswWrapper) -> Self {
        Self {
            vectors: HashMap::new(),
            hnsw,
        }
    }

    /// Adds a vector to the store and updates the ANN index.
    pub fn add_vector(&mut self, id: String, values: Vec<f32>) {
        let arc_values = Arc::new(values);
        self.hnsw.add_vector(id.clone(), Arc::clone(&arc_values)); // Update the ANN index
        let vector = Vector { id, values: arc_values };
        self.vectors.insert(vector.id.clone(), vector); // Store the vector
    }

    /// Retrieves a vector by its ID.
    pub fn get_vector(&self, id: &str) -> Option<&Vector> {
        self.vectors.get(id).map(|v| &v.values)
    }

    /// Searches for the k nearest neighbors to a query vector using the HNSW index.
   
    pub fn search_nearest_neighbors(&self, query: &[f32], k: usize) -> Vec<(String, f32)> {
        let mut neighbors: Vec<(String, f32)> = self.vectors.iter()
            .map(|(id, vec)| (id.clone(), cosine_similarity(query, &vec.values)))
            .collect();

        // Sort by cosine similarity in descending order (highest similarity first)
        neighbors.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Take the top k most similar vectors
        neighbors.truncate(k);
        neighbors
    }
}

/// Calculates the cosine similarity between two vectors.
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
    dot_product / (norm_a * norm_b)
}

