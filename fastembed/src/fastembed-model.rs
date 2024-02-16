use fastembed::{FlagEmbedding, InitOptions, EmbeddingModel};

// Function to initialize the FlagEmbedding model with default options
pub fn init_model_default() -> Result<FlagEmbedding, Box<dyn std::error::Error>> {
    let model = FlagEmbedding::try_new(Default::default())?;
    Ok(model)
}

// Function to initialize the FlagEmbedding model with custom options
pub fn init_model_custom() -> Result<FlagEmbedding, Box<dyn std::error::Error>> {
    let model = FlagEmbedding::try_new(InitOptions {
        model_name: EmbeddingModel::BGEBaseEN,
        show_download_message: false,
        ..Default::default()
    })?;
    Ok(model)
}

// Function to generate embeddings for passages
pub fn generate_passage_embeddings(model: &FlagEmbedding, passages: &[String]) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
    let embeddings = model.passage_embed(passages, Some(1))?;
    Ok(embeddings)
}

// Function to generate an embedding for a single query
pub fn generate_query_embedding(model: &FlagEmbedding, query: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let query_embedding = model.query_embed(query)?;
    Ok(query_embedding)
}
