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
    fn get(&self, slot_name: &str) -> f64 {
        // self.slots.get(slot_name) の戻り値は Option<&f64>
        // Option の中身が参照のままでは値を返せない
        // そのため、copied() メソッドで Option<f64> 型へ変換する
        // また、メモリが見つからなかった場合の値として 0.0 を使う
        self.slots.get(slot_name).copied().unwrap_or(0.0)
    }

    // NOTE: &変数名: 不変参照渡し, &mut 変数名: 可変参照渡し
    // NOTE: *変数名: 参照外し（値への参照から値そのものを取り出す）
    fn add(&mut self, token: &str, previous_result: f64) -> f64 {
        let slot_name = token[3..token.len() - 1].to_string();

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

        // 入力値を空白で分割
        let tokens = line.split_whitespace().collect::<Vec<&str>>();

        // メモリへの書き込み処理かどうか判定
        let is_memory = tokens[0].starts_with("mem");
        if is_memory && tokens[0].ends_with("+") {
            let memorized = memory.add(tokens[0], previous_result);
            print_output(memorized);
            continue;
        } else if is_memory && tokens[0].ends_with("-") {
            let memorized = memory.add(tokens[0], -previous_result);
            print_output(memorized);
            continue;
        }

        // 式の計算
        let left = eval_token(tokens[0], &memory);
        let operator = tokens[1];
        let right = eval_token(tokens[2], &memory);
        let current_result = eval_expression(left, operator, right);

        // 直前の計算結果として一時的に保存
        previous_result = current_result;

        // 計算結果の表示
        print_output(current_result);
    }
}

fn print_output(value: f64) {
    println!("  => {}", value);
}

// NOTE: [T]: 配列のスライス（配列の一部分または全体の覗き窓）
// NOTE: str: 文字列のスライス
// NOTE: 参照の借用（borrow）により、値へアクセスするための参照を一時的に借りることができる
fn eval_token(token: &str, memory: &Memory) -> f64 {
    if let Some(slot_name) = token.strip_prefix("mem") {
        memory.get(slot_name)
    } else {
        token.parse::<f64>().unwrap()
    }
}

fn eval_expression(left: f64, operator: &str, right: f64) -> f64 {
    match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => unreachable!(),
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
        let actual = sut.add("memHoge+", 123.0);

        // Assert
        assert_eq!(actual, 123.0);
        assert_eq!(sut.get("Hoge"), 123.0);
    }

    #[test]
    fn メモリに保存済の値に加算した数値を上書き保存できる() {
        // Arrange
        let mut sut = Memory::new();
        sut.add("memFuga+", 111.1);

        // Act
        let actual = sut.add("memFuga+", 123.45);

        // Assert
        assert_eq!(actual, 234.55);
        assert_eq!(sut.get("Fuga"), 234.55);
    }

    #[test]
    fn メモリに保存済の値から減算した数値を上書き保存できる() {
        // Arrange
        let mut sut = Memory::new();
        sut.add("memPiyo+", 123.45);

        // Act
        let actual = sut.add("memPiyo-", -123.45);

        // Assert
        assert_eq!(actual, 0.0);
        assert_eq!(sut.get("Piyo"), 0.0);
    }

    #[test]
    fn 指定した演算子に基づいて二項の計算が正しく行われる() {
        // 加算
        assert_eq!(eval_expression(1.0, "+", 2.0), 3.0);
        // 減算
        assert_eq!(eval_expression(1.0, "-", 2.0), -1.0);
        // 乗算
        assert_eq!(eval_expression(1.0, "*", 2.0), 2.0);
        // 除算
        assert_eq!(eval_expression(1.0, "/", 2.0), 0.5);
    }
}
