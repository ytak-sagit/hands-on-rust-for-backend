pub struct Book {
    title: String,
    author: String,
}

impl Book {
    fn new(title: &str, author: &str) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
        }
    }
}
