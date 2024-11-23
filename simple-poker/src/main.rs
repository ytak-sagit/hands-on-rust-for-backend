use rand::seq::SliceRandom;

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
    // Vec の用意
    let mut deck = Vec::<Card>::new();
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    // トランプの山札を作成
    for suit in suits {
        for rank in 1..=13 {
            // 構造体のインスタンスを生成
            // NOTE: フィールド名と変数名が同じ場合、短く書ける
            let card = Card { suit, rank };
            deck.push(card);
        }
    }

    // 山札をシャッフル
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);

    // NOTE: 構造体やコレクションの中身の表示には {:?} or {:#?} を指定する必要がある
    println!("{:?}", deck);
}
