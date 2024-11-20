use rand::Rng;

fn main() {
    let op1 = rand::thread_rng().gen_range(0..100);
    let op2 = rand::thread_rng().gen_range(0..100);

    println!("{} + {} = ??", op1, op2);
    println!("?? の値を入力してください:");

    // ユーザーからの回答を保持する変数
    let mut ans_input = String::new();

    // 標準入力から1行取得し、ans_input に代入する
    std::io::stdin().read_line(&mut ans_input).unwrap();

    // ans_input から trim() で改行を除去し、parse() で符号あり整数（i32）型に変換する
    // NOTE: 同名の変数を定義できる（シャドーイング）
    let ans_input = ans_input.trim().parse::<i32>().unwrap();

    dbg!(ans_input); // => cargo run した後に入力した値が確認できる

    if dbg!(ans_input == op1 + op2) {
        println!("正解!");
    } else {
        println!("不正解!");
    }

    // ---

    let op1 = rand::thread_rng().gen_range(0..100);
    let op2 = rand::thread_rng().gen_range(0..100);

    println!("{} - {} = ??", op1, op2);
    println!("?? の値を入力してください:");

    let mut ans_input = String::new();

    std::io::stdin().read_line(&mut ans_input).unwrap();

    let ans_input = ans_input.trim().parse::<i32>().unwrap();

    dbg!(ans_input);

    if dbg!(ans_input == op1 - op2) {
        println!("正解!");
    } else {
        println!("不正解!");
    }
}
