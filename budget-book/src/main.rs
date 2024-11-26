use chrono::NaiveDate;
use clap::{Args, Parser, Subcommand};
use csv::{Reader, Writer, WriterBuilder};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::OpenOptions};

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
    Deposit(DepositArgs),
    /// 口座から出金する
    Withdraw(WithdrawArgs),
    /// CSV からインポートする
    Import(ImportArgs),
    /// レポートを出力する
    Report(ReportArgs),
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

#[derive(Args)]
struct DepositArgs {
    account_name: String,
    date: NaiveDate,
    usage: String,
    amount: u32,
}
impl DepositArgs {
    /// deposit サブコマンドの本体処理
    fn run(&self) {
        let file_name = format!("{}.csv", self.account_name);
        let open_option = OpenOptions::new().append(true).open(file_name).unwrap();
        let mut writer = Writer::from_writer(open_option);
        writer
            .write_record([
                self.date.format("%Y-%m-%d").to_string(),
                self.usage.to_string(),
                self.amount.to_string(),
            ])
            .unwrap();
        writer.flush().unwrap();
    }
}

#[derive(Args)]
struct WithdrawArgs {
    account_name: String,
    date: NaiveDate,
    usage: String,
    amount: u32,
}
impl WithdrawArgs {
    /// withdraw サブコマンドの本体処理
    fn run(&self) {
        let file_name = format!("{}.csv", self.account_name);
        let open_option = OpenOptions::new().append(true).open(file_name).unwrap();
        let mut writer = Writer::from_writer(open_option);
        writer
            .write_record([
                self.date.format("%Y-%m-%d").to_string(),
                self.usage.to_string(),
                format!("-{}", self.amount), // deposit との差分はココだけ
            ])
            .unwrap();
        writer.flush().unwrap();
    }
}

#[derive(Args)]
struct ImportArgs {
    src_file_name: String,
    dst_account_name: String,
}
impl ImportArgs {
    /// import サブコマンドの本体処理
    fn run(&self) {
        let dst_file_name = format!("{}.csv", self.dst_account_name);
        let open_option = OpenOptions::new().append(true).open(dst_file_name).unwrap();
        let mut writer = WriterBuilder::new()
            .has_headers(false) // 1行目のヘッダーをスキップ
            .from_writer(open_option);

        let mut reader = Reader::from_path(&self.src_file_name).unwrap();
        for result in reader.deserialize() {
            // Reader は先頭行をヘッダーとして扱うので、ループは2行目以降について実行される

            // CSV の各行が Record 型として読み取れることを想定
            let record: Record = result.unwrap();
            writer.serialize(record).unwrap();
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Record {
    日付: NaiveDate,
    用途: String,
    金額: i32,
}

#[derive(Args)]
struct ReportArgs {
    files: Vec<String>,
}
impl ReportArgs {
    /// report サブコマンドの本体処理
    fn run(&self) {
        let mut map = HashMap::new();
        for file in self.files.iter() {
            let mut reader = Reader::from_path(file).unwrap();
            for result in reader.records() {
                let record = result.unwrap();
                let date = record[0].parse::<NaiveDate>().unwrap();
                let amount = record[2].parse::<i32>().unwrap();
                let sum = map.entry(date.format("%Y-%m").to_string()).or_insert(0);
                *sum += amount;
            }
        }
        println!("{:?}", map);
    }
}

fn main() {
    // 構造体 App で定義した形のサブコマンドを受け取ることを期待して parse を行う
    let args = App::parse();
    match args.command {
        Command::New(args) => args.run(),
        Command::Deposit(args) => args.run(),
        Command::Withdraw(args) => args.run(),
        Command::Import(args) => args.run(),
        Command::Report(args) => args.run(),
    }
}
