use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use super::book::Book;

pub struct Bookshelf {
    // NOTE: 相対パスでモジュールを参照（crate root からの絶対パスでの参照方法もある）
    books: Vec<Book>,
    matcher: SkimMatcherV2,
}

impl Default for Bookshelf {
    fn default() -> Self {
        Self::new()
    }
}

impl Bookshelf {
    pub fn new() -> Self {
        let matcher = SkimMatcherV2::default();
        Self {
            books: Vec::new(),
            matcher,
        }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn search_books(&self, title_query: &str) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|book| self.matcher.fuzzy_match(&book.title, title_query).is_some())
            .collect()
    }

    pub fn remove_book(&self, _book: Book) -> Option<Book> {
        todo!("Implement `Bookshelf::remove_book`");
    }

    pub fn take_all_books(&mut self) -> Vec<Book> {
        todo!("Implement `Bookshelf::take_all_books`");
    }
}

#[cfg(test)]
mod tests {
    use super::{Book, Bookshelf};

    #[test]
    fn should_be_fuzzy_searched_books() {
        let book1 = Book::new("すごいぞChatGPT！AIを使って学ぼうRust！", "山田太郎");
        let book2 = Book::new("Pythonプログラミング入門", "田中花子");

        let mut shelf = Bookshelf::default();
        shelf.add_book(book1);
        shelf.add_book(book2);

        let found_books = shelf.search_books("chatgpt");
        println!("{:?}", found_books);
    }
}
