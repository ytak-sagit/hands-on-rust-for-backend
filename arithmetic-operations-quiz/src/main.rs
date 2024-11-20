use rand::Rng;

fn main() {
    let mut num_of_correct = 0; // 正解数を数える変数を追加

    while num_of_correct < 3 {
        // 正解数が3問以下の間は繰り返し

        // quiz_mode をランダムに決め、加算クイズか減算クイズを決める
        let quiz_mode = rand::thread_rng().gen_range(0..=1);
        let is_addition = match quiz_mode {
            // NOTE: match は式なので値が返却される
            0 => true,
            1 => false,
            _ => unreachable!(),
        };

        let op1 = rand::thread_rng().gen_range(0..100);
        let op2 = rand::thread_rng().gen_range(0..100);

        println!(
            "{} {} {} = ??",
            op1,
            if is_addition { "+" } else { "-" }, // NOTE: 三項演算子がないので if-else で書く
            op2
        );
        println!("?? の値を入力してください:");

        // ユーザーからの回答を保持する変数
        let mut ans_input = String::new();

        // 標準入力から1行取得し、ans_input に代入する
        std::io::stdin().read_line(&mut ans_input).unwrap();

        // ans_input から trim() で改行を除去し、parse() で符号あり整数（i32）型に変換する
        // NOTE: 同名の変数を定義できる（シャドーイング）
        let ans_input = ans_input.trim().parse::<i32>().unwrap();

        if dbg!(ans_input == if is_addition { op1 + op2 } else { op1 - op2 }) {
            println!("正解!");
            num_of_correct += 1; // 正解したら正解数を1増やす
        } else {
            println!("不正解!");
        }
    }

    println!("クリア!");
}
