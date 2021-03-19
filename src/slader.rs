use std::vec;

pub struct Textbook {
    title: String,
    image: String,
    isbn: String,
}

impl Textbook {
    pub fn new(title: String, image: String,
        isbn: String) -> Textbook {
        Textbook { title: title, image: image, isbn: isbn }
    }
}

pub async fn search_textbooks(query: &str) -> Result<Vec<Textbook>,
    Box<dyn std::error::Error>> {
    let mut results: Vec<Textbook> = vec![];

    results.push(Textbook::new(String::from("Hello"),
        String::from("world!"), String::from("112358")));

    Ok(results)
}
