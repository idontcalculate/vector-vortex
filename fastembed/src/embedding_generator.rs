// Assume FlagEmbedding is part of a crate that's been added to Cargo.toml
use fastembed::FlagEmbedding; // Import the embedding model
use crate::hnsw_index::VectorIndex; // The HNSW index
use std::sync::Arc;

pub struct EmbeddingGenerator {
    model: FlagEmbedding, // The embedding model
}

impl EmbeddingGenerator {
    pub fn new() -> Self {
        // Initialize the FlagEmbedding model
        let model = FlagEmbedding::try_new(Default::default()).unwrap(); // Handle errors as appropriate
        Self { model }
    }

    pub fn generate_embeddings(&self, texts: &[String]) -> Vec<Arc<Vec<f32>>> {
        texts.iter()
            .map(|text| {
                // Generate embeddings for each text
                let embedding = self.model.passage_embed(text, Some(1)).unwrap(); // Handle errors as appropriate
                Arc::new(embedding)
            })
            .collect()
    }

    pub fn create_and_populate_index(&self, passages: &[String], passage_ids: &[usize], max_elements: usize, m: usize, ef_construction: usize, ef_search: usize) -> VectorIndex {
        let embeddings = self.generate_embeddings(passages);

        let mut vector_index = VectorIndex::new(max_elements, m, ef_construction, ef_search);
        
        for (id, embedding) in passage_ids.iter().zip(embeddings.iter()) {
            vector_index.add_vector(*id, embedding.clone()).unwrap(); // Handle errors as appropriate
        }

        vector_index
    }
}
