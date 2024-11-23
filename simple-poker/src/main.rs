#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    suit: Suit,
    rank: i32,
}

fn main() {
    let suit = Suit::Club;
    let rank = 1;

    // 構造体のインスタンスを生成
    // NOTE: フィールド名と変数名が同じ場合、短く書ける
    let card = Card { suit, rank };

    // NOTE: 構造体の中身の表示には {:?} or {:#?} を指定する必要がある
    println!("{:?}", card);
}
