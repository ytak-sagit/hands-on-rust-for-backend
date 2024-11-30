// NOTE: mod 定義時、ブロックではなく ; で終わらせることで、同名のファイル/ディレクトリを module として読み込む
pub mod library;

fn function_1() {
    let shelf = crate::library::bookshelf::Bookshelf::new();
}

fn function_2() {
    use library::bookshelf;
    let shelf = bookshelf::Bookshelf::new();
}
