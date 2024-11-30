// NOTE: mod 定義時、ブロックではなく ; で終わらせることで、同名のファイル/ディレクトリを module として読み込む
pub mod library;

fn _function_1() {
    let _shelf = crate::library::bookshelf::Bookshelf::new();
}

fn _function_2() {
    use library::bookshelf;
    let _shelf = bookshelf::Bookshelf::new();
}
