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

    loop {
        // 山札を作成
        deck.clear();
        for suit in suits {
            for rank in 1..=13 {
                // 構造体のインスタンスを生成
                // NOTE: フィールド名と変数名が同じ場合、短く書ける
                let card = Card { suit, rank };
                deck.push(card);
            }
        }
        assert_eq!(deck.len(), 52);

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

        // ---
        println!("入れ替えたいカードの番号を入力してください(例: 1 2 3)");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let indexes = input
            .split_whitespace()
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        // 与えられた数字の箇所を、山札から取り出したカードに置き換える
        for i in indexes {
            hands[i - 1] = deck.pop().unwrap();
        }

        // 手札を rank 順にソート
        hands.sort_by(|a, b| a.rank.cmp(&b.rank));

        // 手札の表示
        println!("--- Hand ---");
        for (i, card) in hands.iter().enumerate() {
            println!("{}: {:?} {}", i + 1, card.suit, card.rank);
        }

        // ---

        // フラッシュのチェック
        let suit = hands.first().unwrap().suit;
        let is_flash = hands.iter().all(|card| card.suit == suit);

        // ペア数のチェック
        let mut count = 0;
        for i in 0..hands.len() - 1 {
            for j in i + 1..hands.len() {
                if hands[i].rank == hands[j].rank {
                    count += 1;
                }
            }
        }

        // 役判定
        if is_flash {
            println!("フラッシュ！");
        } else if count >= 3 {
            println!("スリーカード！");
        } else if count == 2 {
            println!("ツーペア！");
        } else if count == 1 {
            println!("ワンペア！");
        } else {
            println!("役なし...T^T");
        }

        // ---
        println!("もう一回やる？ [Y/n]");

        let mut retry = String::new();
        std::io::stdin().read_line(&mut retry).unwrap();

        if retry.trim().to_lowercase() == "n" {
            break;
        }
    }
}
