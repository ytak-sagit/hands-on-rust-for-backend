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
    /// 予定の削除
    Delete {
        /// 予定のID
        id: u64,
    },
}

fn main() {
    match read_calendar() {
        Ok(calendar) => {
            let options = Cli::parse();
            match options.command {
                Commands::List => show_list(calendar),
                Commands::Add {
                    subject,
                    start,
                    end,
                } => add_schedule(calendar, subject, start, end),
                Commands::Delete { id } => delete_schedule(calendar, id),
            }
        }
        Err(error) => println!("カレンダーの読み込みに失敗しました：{:?}", error),
    }
}

fn read_calendar() -> Result<Calendar, std::io::Error> {
    // NOTE: Result 型の後ろに ? を付けることで、Err が返る場合はそのまま返すことができる
    let file = File::open("schedule.json")?;
    let reader = BufReader::new(file);
    let calendar = serde_json::from_reader(reader).unwrap();
    Ok(calendar)
}

fn show_list(calendar: Calendar) {
    // 予定の表示
    println!("ID\tSTART\tEND\tSUBJECT");
    for schedule in calendar.schedules {
        println!(
            "{}\t{}\t{}\t{}",
            schedule.id, schedule.start, schedule.end, schedule.subject
        );
    }
}

fn add_schedule(mut calendar: Calendar, subject: String, start: NaiveDateTime, end: NaiveDateTime) {
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

fn delete_schedule(mut calendar: Calendar, id: u64) {
    // 予定の削除
    if let Some(index) = calendar
        .schedules
        .iter()
        .position(|schedule| schedule.id == id)
    {
        calendar.schedules.remove(index);
    } else {
        println!("エラー：IDが不正です");
        return;
    }

    // 予定の保存
    let file = File::create("schedule.json").unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &calendar).unwrap();
    println!("予定を削除しました。");
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use rstest::rstest;

    fn naive_date_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, minute, second)
            .unwrap()
    }

    #[rstest]
    #[case(18, 15, 18, 45, false)]
    #[case(18, 15, 19, 15, true)]
    #[case(18, 30, 20, 15, true)]
    #[case(19, 15, 19, 45, true)]
    #[case(19, 45, 20, 45, true)]
    #[case(20, 15, 20, 45, false)]
    fn test_schedule_intersects(
        #[case] h0: u32,
        #[case] m0: u32,
        #[case] h1: u32,
        #[case] m1: u32,
        #[case] should_intersect: bool,
    ) {
        let schedule = Schedule {
            id: 0,
            subject: "既存予定".to_string(),
            start: naive_date_time(2024, 1, 1, h0, m0, 0),
            end: naive_date_time(2024, 1, 1, h1, m1, 0),
        };
        let new_schedule = Schedule {
            id: 999,
            subject: "新規予定".to_string(),
            start: naive_date_time(2024, 1, 1, 19, 0, 0),
            end: naive_date_time(2024, 1, 1, 20, 0, 0),
        };
        assert_eq!(should_intersect, schedule.intersects(&new_schedule));
    }
}
