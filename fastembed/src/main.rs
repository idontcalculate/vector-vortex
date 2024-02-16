// main.rs
mod fastembed_model;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the model
    let model = fastembed_model::init_model_custom()?;

    // Define passages
    let passages = vec![
        "This is the first passage. It contains provides more context for retrieval.".to_string(),
        "Here's the second passage, which is longer than the first one. It includes additional information.".to_string(),
        "And this is the third passage, the longest of all. It contains several sentences and is meant for more extensive testing.".to_string(),
    ];

    // Generate embeddings for the passages
    let passage_embeddings = fastembed_model::generate_passage_embeddings(&model, &passages)?;

    // Print embeddings length and dimension
    println!("Passage embeddings length: {}", passage_embeddings.len());
    println!("Passage embedding dimension: {}", passage_embeddings[0].len());

    // Define a query
    let query = "What is the answer to this generic question?";

    // Generate embedding for the query
    let query_embedding = fastembed_model::generate_query_embedding(&model, &query)?;

    // Print query embedding dimension
    println!("Query embedding dimension: {}", query_embedding.len());

    Ok(())
}
