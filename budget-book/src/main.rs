use clap::{Args, Parser, Subcommand};
use csv::Writer;

#[derive(Parser)]
#[clap(version = "1.0")]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// 新しい口座を作る
    New(NewArgs),
    /// 口座に入金する
    Deposit,
    /// 口座から出金する
    Withdraw,
    /// CSV からインポートする
    Import,
    /// レポートを出力する
    Report,
}

#[derive(Args)]
struct NewArgs {
    account_name: String,
}
impl NewArgs {
    /// new サブコマンドの本体処理
    fn run(&self) {
        let file_name = format!("{}.csv", self.account_name);
        let mut writer = Writer::from_path(file_name).unwrap();
        writer
            .write_record(["日付", "用途", "金額"]) // ヘッダーを書き込む
            .unwrap();
        writer.flush().unwrap();
    }
}

fn main() {
    // 構造体 App で定義した形のサブコマンドを受け取ることを期待して parse を行う
    let args = App::parse();
    match args.command {
        Command::New(args) => args.run(),
        Command::Deposit => unimplemented!(),
        Command::Withdraw => unimplemented!(),
        Command::Import => unimplemented!(),
        Command::Report => unimplemented!(),
    }
}
