use std::io::stdin;

fn main() {
    let mut memory: f64 = 0.0;
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
        if tokens[0] == "mem+" {
            add_and_print_memory(&mut memory, previous_result);
            continue;
        } else if tokens[0] == "mem-" {
            add_and_print_memory(&mut memory, -previous_result);
            continue;
        }

        // 式の計算
        let left = eval_token(tokens[0], memory);
        let operator = tokens[1];
        let right = eval_token(tokens[2], memory);
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

fn eval_token(token: &str, memory: f64) -> f64 {
    if token == "mem" {
        memory
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

// NOTE: &変数名: 不変参照渡し, &mut 変数名: 可変参照渡し
// NOTE: *変数名: 参照外し（値への参照から値そのものを取り出す）
fn add_and_print_memory(memory: &mut f64, previous_result: f64) {
    *memory += previous_result;
    print_output(*memory);
}
