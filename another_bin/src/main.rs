use my_library::library::{book::Book, bookshelf::Bookshelf};

fn main() {
    let book1 = Book::new("すごいぞChatGPT！AIを使って学ぼうRust！", "山田太郎");
    let book2 = Book::new("Pythonプログラミング入門", "田中花子");

    let mut shelf = Bookshelf::default();
    shelf.add_book(book1);
    shelf.add_book(book2);

    let found_books = shelf.search_books("python");
    println!("{:?}", found_books);
}
