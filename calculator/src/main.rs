use std::io::stdin;

fn main() {
    for line in stdin().lines() {
        // 一行読み取って空行なら終了
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        // 入力値を空白で分割
        let tokens = line.split_whitespace().collect::<Vec<&str>>();

        // 式の計算
        let left = tokens[0].parse::<f64>().unwrap();
        let operator = tokens[1];
        let right = tokens[2].parse::<f64>().unwrap();
        let result = match operator {
            "+" => add(left, right),
            "-" => subtract(left, right),
            "*" => multiply(left, right),
            "/" => divide(left, right),
            _ => unreachable!(),
        };

        // 計算結果の表示
        print_value(result);
    }
}

fn print_value(value: f64) {
    println!("  => {}", value);
}

fn add(left: f64, right: f64) -> f64 {
    left + right
}

fn subtract(left: f64, right: f64) -> f64 {
    left - right
}

fn multiply(left: f64, right: f64) -> f64 {
    left * right
}

fn divide(left: f64, right: f64) -> f64 {
    left / right
}
