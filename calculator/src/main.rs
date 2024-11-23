use std::io::stdin;

struct KeyValuePair<T> {
    key: String,
    value: T,
}

struct Memory {
    slots: Vec<KeyValuePair<f64>>,
}

impl Memory {
    // 関連関数
    // NOTE: 他言語での静的メソッドのようなもの
    fn new() -> Self {
        Self { slots: vec![] }
    }

    // NOTE: &変数名: 不変参照渡し, &mut 変数名: 可変参照渡し
    // NOTE: *変数名: 参照外し（値への参照から値そのものを取り出す）
    fn add_and_print(&mut self, token: &str, previous_result: f64) {
        let slot_name = &token[3..token.len() - 1];

        // すべてのメモリを探索する
        for slot in self.slots.iter_mut() {
            if slot.key == slot_name {
                // メモリが見つかったので、値を更新・表示して終了
                slot.value += previous_result;
                print_output(slot.value);
                return;
            }
        }

        // メモリが見つからなかったので、最後の要素に追加する
        self.slots.push(KeyValuePair {
            key: slot_name.to_string(),
            value: previous_result,
        });
        print_output(previous_result);
    }

    // NOTE: [T]: 配列のスライス（配列の一部分または全体の覗き窓）
    // NOTE: str: 文字列のスライス
    // NOTE: 参照の借用（borrow）により、値へアクセスするための参照を一時的に借りることができる
    fn eval_token(&self, token: &str) -> f64 {
        if let Some(slot_name) = token.strip_prefix("mem") {
            // すべてのメモリを探索する
            for slot in self.slots.iter() {
                if slot.key == slot_name {
                    // メモリが見つかったので、値を返して終了
                    return slot.value;
                }
            }

            // メモリが見つからなかったので、初期値を返す
            0.0
        } else {
            token.parse::<f64>().unwrap()
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
            memory.add_and_print(tokens[0], previous_result);
            continue;
        } else if is_memory && tokens[0].ends_with("-") {
            memory.add_and_print(tokens[0], -previous_result);
            continue;
        }

        // 式の計算
        let left = memory.eval_token(tokens[0]);
        let operator = tokens[1];
        let right = memory.eval_token(tokens[2]);
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

fn eval_expression(left: f64, operator: &str, right: f64) -> f64 {
    match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => unreachable!(),
    }
}
