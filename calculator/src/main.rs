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
            memory += previous_result;
            print_output(memory);
            continue;
        } else if tokens[0] == "mem-" {
            memory -= previous_result;
            print_output(memory);
            continue;
        }

        // 式の計算
        let left = if tokens[0] == "mem" {
            memory
        } else {
            tokens[0].parse::<f64>().unwrap()
        };
        let operator = tokens[1];
        let right = if tokens[2] == "mem" {
            memory
        } else {
            tokens[2].parse::<f64>().unwrap()
        };
        let current_result = match operator {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            _ => unreachable!(),
        };

        // 直前の計算結果として一時的に保存
        previous_result = current_result;

        // 計算結果の表示
        print_output(current_result);
    }
}

fn print_output(value: f64) {
    println!("  => {}", value);
}
