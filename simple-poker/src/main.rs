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
    // トランプの山札用の Vec の用意
    let mut deck = Vec::<Card>::new();
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    // 山札を作成
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
    // println!("{:?}", deck);

    // ---

    // 手札用の Vec の用意
    let mut hands = Vec::<Card>::new();

    // 5枚のカードを引く
    for _ in 0..5 {
        hands.push(deck.pop().unwrap());
    }

    // 手札を rank 順にソート
    hands.sort_by(|a, b| a.rank.cmp(&b.rank));

    // 手札の表示
    println!("--- Hand ---");
    for (i, card) in hands.iter().enumerate() {
        println!("{}: {:?} {}", i + 1, card.suit, card.rank);
    }
}
