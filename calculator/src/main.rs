use std::{
    collections::{hash_map::Entry, HashMap},
    io::stdin,
};

struct Memory {
    slots: HashMap<String, f64>,
}

impl Memory {
    // 関連関数
    // NOTE: 他言語での静的メソッドのようなもの
    fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }

    // メソッド
    // NOTE: [T]: 配列のスライス（配列の一部分または全体の覗き窓）
    // NOTE: str: 文字列のスライス
    // NOTE: 参照の借用（borrow）により、値へアクセスするための参照を一時的に借りることができる
    fn get(&self, slot_name: &str) -> f64 {
        // self.slots.get(slot_name) の戻り値は Option<&f64>
        // Option の中身が参照のままでは値を返せない
        // そのため、copied() メソッドで Option<f64> 型へ変換する
        // また、メモリが見つからなかった場合の値として 0.0 を使う
        self.slots.get(slot_name).copied().unwrap_or(0.0)
    }

    // NOTE: &変数名: 不変参照渡し, &mut 変数名: 可変参照渡し
    // NOTE: *変数名: 参照外し（値への参照から値そのものを取り出す）
    fn add(&mut self, slot_name: &str, previous_result: f64) -> f64 {
        let slot_name = slot_name.to_string();
        match self.slots.entry(slot_name) {
            Entry::Occupied(mut entry) => {
                // メモリが見つかったので、値を更新する
                *entry.get_mut() += previous_result;
                *entry.get()
            }
            Entry::Vacant(entry) => {
                // メモリが見つからなかったので、値を追加する
                entry.insert(previous_result);
                previous_result
            }
        }
    }
}

// NOTE: 列挙子には値を添付できる
#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    MemoryRef(String),
    MemoryPlus(String),
    MemoryMinus(String),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen, // 開き括弧
    RParen, // 閉じ括弧
}

// NOTE: enum も実装できる
impl Token {
    fn parse(value: &str) -> Self {
        match value {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Asterisk,
            "/" => Self::Slash,
            "(" => Self::LParen,
            ")" => Self::RParen,
            // NOTE: match 式は値の一致だけでなく、追加の条件式も書ける
            _ if value.starts_with("mem") => {
                let mut memory_name = value[3..].to_string();
                if value.ends_with("+") {
                    memory_name.pop(); // 末尾の1文字を削除
                    Self::MemoryPlus(memory_name)
                } else if value.ends_with("-") {
                    memory_name.pop(); // 末尾の1文字を削除
                    Self::MemoryMinus(memory_name)
                } else {
                    Self::MemoryRef(memory_name)
                }
            }
            _ => Self::Number(value.parse().unwrap()),
        }
    }

    fn split(text: &str) -> Vec<Self> {
        text.split_whitespace().map(Self::parse).collect()
    }
}

fn main() {
    // 任意の名称で保持できる可変長メモリを用意
    let mut memory = Memory::new();
    let mut previous_result: f64 = 0.0;

    for line in stdin().lines() {
        // 一行読み取って空行なら終了
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        // トークン列に分割
        let tokens = Token::split(&line);

        // 式の評価
        match &tokens[0] {
            Token::MemoryPlus(memory_name) => {
                // メモリへの加算
                let memorized = memory.add(memory_name, previous_result);
                print_output(memorized);
            }
            Token::MemoryMinus(memory_name) => {
                // メモリへの減算
                let memorized = memory.add(memory_name, -previous_result);
                print_output(memorized);
            }
            _ => {
                // 式の値の計算
                let current_result = eval_expression(&tokens, &memory);

                // 直前の計算結果として一時的に保存
                previous_result = current_result;

                // 計算結果の表示
                print_output(current_result);
            }
        }
    }
}

fn print_output(value: f64) {
    println!("  => {}", value);
}

fn eval_expression(tokens: &[Token], memory: &Memory) -> f64 {
    let (result, index) = eval_additive_expression(0, tokens, memory);
    // 正しく計算できていたら、index は式の末尾を指しているはず
    assert_eq!(tokens.len(), index);
    result
}

fn eval_additive_expression(index: usize, tokens: &[Token], memory: &Memory) -> (f64, usize) {
    let mut index = index;
    let mut result;
    (result, index) = eval_mutliplicative_expression(index, tokens, memory);
    while index < tokens.len() {
        match &tokens[index] {
            Token::Plus => {
                let (value, next) = eval_mutliplicative_expression(index + 1, tokens, memory);
                result += value;
                index = next;
            }
            Token::Minus => {
                let (value, next) = eval_mutliplicative_expression(index + 1, tokens, memory);
                result -= value;
                index = next;
            }
            _ => break,
        }
    }
    (result, index)
}

fn eval_mutliplicative_expression(index: usize, tokens: &[Token], memory: &Memory) -> (f64, usize) {
    let mut index = index;
    let mut result;
    (result, index) = eval_primary_expression(index, tokens, memory);
    while index < tokens.len() {
        match &tokens[index] {
            Token::Asterisk => {
                let (value, next) = eval_primary_expression(index + 1, tokens, memory);
                result *= value;
                index = next;
            }
            Token::Slash => {
                let (value, next) = eval_primary_expression(index + 1, tokens, memory);
                result /= value;
                index = next;
            }
            _ => break,
        }
    }
    (result, index)
}

fn eval_primary_expression(index: usize, tokens: &[Token], memory: &Memory) -> (f64, usize) {
    let first_token = &tokens[index];
    match first_token {
        Token::LParen => {
            // 開き括弧で始まっているので、括弧の次のトークンから式を計算する
            let (result, next) = eval_additive_expression(index + 1, tokens, memory);
            // tokens[index] は閉じ括弧になっているはず
            assert_eq!(Token::RParen, tokens[next]);
            // 閉じ括弧の分だけ1トークン進めた位置を返す
            (result, next + 1)
        }
        Token::Number(value) => {
            // 数値を表しているので、その値と次の位置を返す
            (*value, index + 1)
        }
        Token::MemoryRef(memory_name) => {
            // メモリを表しているので、メモリの値と次の位置を返す
            (memory.get(memory_name), index + 1)
        }
        _ => unreachable!(), // 入力が正しいならここには来ない
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn メモリに任意の名称で数値を保存できる() {
        // Arrange
        let mut sut = Memory::new();

        // Act
        let actual = sut.add("Hoge", 123.0);

        // Assert
        assert_eq!(actual, 123.0);
        assert_eq!(sut.get("Hoge"), 123.0);
    }

    #[test]
    fn メモリに保存済の値に加算した数値を上書き保存できる() {
        // Arrange
        let mut sut = Memory::new();
        sut.add("Fuga", 111.1);

        // Act
        let actual = sut.add("Fuga", 123.45);

        // Assert
        assert_eq!(actual, 234.55);
        assert_eq!(sut.get("Fuga"), 234.55);
    }

    #[test]
    fn メモリに保存済の値から減算した数値を上書き保存できる() {
        // Arrange
        let mut sut = Memory::new();
        sut.add("Piyo", 123.45);

        // Act
        let actual = sut.add("Piyo", -123.45);

        // Assert
        assert_eq!(actual, 0.0);
        assert_eq!(sut.get("Piyo"), 0.0);
    }

    #[test]
    fn トークン列の分割ができる_数値のみ() {
        // 加算
        assert_eq!(
            Token::split("1 + 2"),
            vec![Token::Number(1.0), Token::Plus, Token::Number(2.0)]
        );
        // 減算
        assert_eq!(
            Token::split("1.5 - 2.3"),
            vec![Token::Number(1.5), Token::Minus, Token::Number(2.3)]
        );
        // 乗算
        assert_eq!(
            Token::split("0.1 * 9.0"),
            vec![Token::Number(0.1), Token::Asterisk, Token::Number(9.0)]
        );
        // 除算
        assert_eq!(
            Token::split("6.7 / 4.89"),
            vec![Token::Number(6.7), Token::Slash, Token::Number(4.89)]
        );
    }

    #[test]
    fn トークン列の分割ができる_メモリへの加減算() {
        // メモリへの加算
        assert_eq!(
            Token::split("memABC+"),
            vec![Token::MemoryPlus("ABC".to_string())]
        );
        // メモリへの減算
        assert_eq!(
            Token::split("memxyz-"),
            vec![Token::MemoryMinus("xyz".to_string())]
        );
    }

    #[test]
    fn トークン列の分割ができる_メモリの参照() {
        assert_eq!(
            Token::split("mem_ijk + mem+OPQ"),
            vec![
                Token::MemoryRef("_ijk".to_string()),
                Token::Plus,
                Token::MemoryRef("+OPQ".to_string()),
            ]
        );
    }

    #[test]
    fn トークン列の分割ができる_括弧入り() {
        assert_eq!(
            Token::split("( 1 + memTEST ) / 10"),
            vec![
                Token::LParen,
                Token::Number(1.0),
                Token::Plus,
                Token::MemoryRef("TEST".to_string()),
                Token::RParen,
                Token::Slash,
                Token::Number(10.0),
            ]
        );
    }

    #[test]
    fn 指定した演算子に基づいて二項の計算が正しく行われる() {
        let _dummy = Memory::new();
        // 加算
        assert_eq!(
            eval_expression(
                &[Token::Number(1.0), Token::Plus, Token::Number(2.0)],
                &_dummy
            ),
            3.0
        );
        // 減算
        assert_eq!(
            eval_expression(
                &[Token::Number(1.0), Token::Minus, Token::Number(2.0)],
                &_dummy
            ),
            -1.0
        );
        // 乗算
        assert_eq!(
            eval_expression(
                &[Token::Number(1.0), Token::Asterisk, Token::Number(2.0)],
                &_dummy
            ),
            2.0
        );
        // 除算
        assert_eq!(
            eval_expression(
                &[Token::Number(1.0), Token::Slash, Token::Number(2.0)],
                &_dummy
            ),
            0.5
        );
    }

    #[test]
    fn 指定したトークン列で表される式の計算が正しく行われる() {
        let _dummy = Memory::new();
        // 複雑な計算 (1 * 2 * 3 - 4 * 5 + 6 * 7 + 8 * 9)
        assert_eq!(
            eval_expression(
                &[
                    Token::Number(1.0),
                    Token::Asterisk,
                    Token::Number(2.0),
                    Token::Asterisk,
                    Token::Number(3.0),
                    Token::Minus,
                    Token::Number(4.0),
                    Token::Asterisk,
                    Token::Number(5.0),
                    Token::Plus,
                    Token::Number(6.0),
                    Token::Asterisk,
                    Token::Number(7.0),
                    Token::Plus,
                    Token::Number(8.0),
                    Token::Asterisk,
                    Token::Number(9.0),
                ],
                &_dummy
            ),
            100.0
        );
        // 括弧入り [1 + 2 + 3 + 4 + ( 5 + 6 + 7 - 8) * 9]
        assert_eq!(
            eval_expression(
                &[
                    Token::Number(1.0),
                    Token::Plus,
                    Token::Number(2.0),
                    Token::Plus,
                    Token::Number(3.0),
                    Token::Plus,
                    Token::Number(4.0),
                    Token::Plus,
                    Token::LParen,
                    Token::Number(5.0),
                    Token::Plus,
                    Token::Number(6.0),
                    Token::Plus,
                    Token::Number(7.0),
                    Token::Minus,
                    Token::Number(8.0),
                    Token::RParen,
                    Token::Asterisk,
                    Token::Number(9.0),
                ],
                &_dummy
            ),
            100.0
        );
        // メモリ参照 (memA(4.7) + 3 * memB(1))
        let mut memory = Memory::new();
        memory.add("A", 4.7);
        memory.add("B", 1.0);
        assert_eq!(
            eval_expression(
                &[
                    Token::MemoryRef("A".to_string()),
                    Token::Plus,
                    Token::Number(3.0),
                    Token::Asterisk,
                    Token::MemoryRef("B".to_string()),
                ],
                &memory
            ),
            7.7
        );
    }
}
