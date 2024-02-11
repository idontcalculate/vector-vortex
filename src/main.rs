mod data_loader;
mod vector_store;
mod search_engine;
mod utils;

use clap::{App, Arg};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Vector Vortex Search Engine")
        .version("1.0")
        .author("tensorrist")
        .about("Performs a search with the given query")
        .arg(Arg::new("query")
             .short('q')
             .long("query")
             .value_name("QUERY")
             .help("The search query to execute")
             .takes_value(true)
             .required(true))
        .get_matches();

    let query = matches.value_of("query").unwrap();

    let index = search_engine::create_index()?;
    search_engine::search_index(&index, query)?;

    Ok(())
}
