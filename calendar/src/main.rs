use chrono::NaiveDateTime;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Schedule {
    /// 予定のID
    id: u64,
    /// 勉強会の名前
    subject: String,
    /// 開始時刻
    start: NaiveDateTime,
    /// 終了時刻
    end: NaiveDateTime,
}
impl Schedule {
    fn intersects(&self, other: &Schedule) -> bool {
        self.start < other.end && other.start < self.end
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Calendar {
    /// 勉強会の予定一覧
    schedules: Vec<Schedule>,
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 予定の一覧表示
    List,
    /// 予定の追加
    Add {
        /// 勉強会の名前
        subject: String,
        /// 開始時刻
        start: NaiveDateTime,
        /// 終了時刻
        end: NaiveDateTime,
    },
}

fn main() {
    let options = Cli::parse();
    match options.command {
        Commands::List => show_list(),
        Commands::Add {
            subject,
            start,
            end,
        } => add_schedule(subject, start, end),
    }
}

fn show_list() {
    // 予定の読み込み
    let calendar: Calendar = {
        let file = File::open("schedule.json").unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    };

    // 予定の表示
    println!("ID\tSTART\tEND\tSUBJECT");
    for schedule in calendar.schedules {
        println!(
            "{}\t{}\t{}\t{}",
            schedule.id, schedule.start, schedule.end, schedule.subject
        );
    }
}

fn add_schedule(subject: String, start: NaiveDateTime, end: NaiveDateTime) {
    // 予定の読み込み
    let mut calendar: Calendar = {
        let file = File::open("schedule.json").unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    };

    // 予定の作成
    let id = calendar.schedules.len() as u64;
    let new_schedule = Schedule {
        id,
        subject,
        start,
        end,
    };

    // 予定の重複判定
    for schedule in &calendar.schedules {
        if schedule.intersects(&new_schedule) {
            println!("エラー：予定が重複しています");
            return;
        }
    }

    // 予定の追加
    calendar.schedules.push(new_schedule);

    // 予定の保存
    let file = File::create("schedule.json").unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &calendar).unwrap();
    println!("予定を追加しました。");
}
