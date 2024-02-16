// hnsw_index.rs

use hnsw_rs::{Hnsw, Point, Distance};
use std::sync::Arc;

pub struct VectorIndex {
    pub hnsw: Hnsw<f32, Distance>, // The actual HNSW index.
}

impl VectorIndex {
    // Create a new VectorIndex with the specified parameters.
    pub fn new(max_elements: usize, m: usize, ef_construction: usize, ef_search: usize) -> Self {
        let hnsw = Hnsw::<f32, Distance>::new(max_elements, m, ef_construction, ef_search);
        Self { hnsw }
    }

    // Add a vector to the HNSW index.
    pub fn add_vector(&mut self, id: usize, data: Vec<f32>) -> Result<(), String> {
        let point = Point::new(id, Arc::new(data));
        self.hnsw.insert(point);
        Ok(())
    }

    // Search for k nearest neighbors to the given vector.
    pub fn search(&self, data: Vec<f32>, k: usize) -> Vec<(usize, f32)> {
        let query_point = Point::new(0, Arc::new(data)); // ID is not important for query
        self.hnsw.search(&query_point, k)
            .into_iter()
            .map(|neighbor| (neighbor.id, neighbor.distance))
            .collect()
    }
}
