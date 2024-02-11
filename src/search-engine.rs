// Search engine implementation
use serde::{Serialize, Deserialize};
use tantivy::{schema::*, Index, doc, Document as TantivyDocument, query::QueryParser, collector::TopDocs, TantivyError};

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub title: String,
    pub body: String,
}

pub fn create_index() -> Result<Index, TantivyError> {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);
    let schema = schema_builder.build();

    let index = Index::create_in_ram(schema.clone());

    let mut index_writer = index.writer(50_000_000)?;
    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();
    let doc = doc!(title => "Example Title", body => "This is the body of the document.");
    index_writer.add_document(doc)?;
    index_writer.commit()?;

    Ok(index)
}

pub fn search_index(index: &Index, query_str: &str) -> Result<(), TantivyError> {
    let reader = index.reader()?;
    let searcher = reader.searcher();

    let schema = index.schema();
    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();
    let query_parser = QueryParser::for_index(&index, vec![title, body]);

    let query = query_parser.parse_query(query_str)?;
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    for (_, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        // For a more user-friendly display, consider implementing custom formatting.
        println!("Document found: {:?}", retrieved_doc);
    }

    Ok(())
}

