use chrono::NaiveDateTime;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader};

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
}

fn main() {
    let options = Cli::parse();
    match options.command {
        Commands::List => show_list(),
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
