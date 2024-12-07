use clap::Parser;
use std::{
    fs::{create_dir_all, read_dir},
    path::PathBuf,
    sync::{mpsc::channel, Arc, Mutex},
    thread,
};

#[derive(Parser)]
struct Args {
    /// サムネイル化する元画像が入っているフォルダ
    input: PathBuf,
    /// サムネイルにした画像を保存するフォルダ
    output: PathBuf,
}

fn main() {
    let args = Args::parse();

    // 出力先フォルダの作成
    create_dir_all(&args.output).unwrap();

    let mut handles = vec![];
    let mut channels = vec![];
    let (sernder_count, reciever_count) = channel::<usize>();

    // 受信側＝サムネイル作成処理側 の立ち上げ
    for _ in 0..4 {
        let (sc, rc) = channel::<PathBuf>();
        channels.push(sc);

        let sender_count = sernder_count.clone();
        let output = args.output.clone();

        handles.push(thread::spawn(move || {
            while let Ok(path) = rc.recv() {
                // 画像ファイルの読み込み
                let img = image::open(&path);
                if let Ok(img) = img {
                    // サムネイル化
                    let thumbnail = img.thumbnail(64, 64);

                    // サムネイルをファイルとして保存
                    let output_path = output.join(path.file_name().unwrap());
                    thumbnail.save(output_path).unwrap();

                    // カウントアップして受信側へカウント値を送信
                    sender_count.send(1).unwrap();
                }
            }
        }));
    }

    // 送信側は画像ファイルのパスを送信
    for (index, item) in read_dir(&args.input).unwrap().enumerate() {
        let item = item.unwrap();
        let input_path = item.path();
        if input_path.is_dir() {
            // フォルダは処理しない
            continue;
        }
        channels[index % channels.len()].send(input_path).unwrap();
    }

    // 処理の完了通知
    for channel in channels {
        drop(channel);
    }
    drop(sernder_count);

    // 各スレッドの終了を待機
    for handle in handles {
        handle.join().unwrap();
    }

    // 最後に受信側のカウント値を合計して表示
    println!("Processed {} images", reciever_count.iter().sum::<usize>());
}

/// スレッド + Mutex を使用する version
/// NOTE: 比較のためにコードを残してある
fn _ver_thread_and_mutex() {
    let args = Args::parse();

    // 出力先フォルダの作成
    create_dir_all(&args.output).unwrap();

    // 指定したフォルダ内の画像ファイルパスを抽出
    let mut all_paths = vec![];
    for item in read_dir(&args.input).unwrap() {
        let item = item.unwrap();
        let input_path = item.path();
        if input_path.is_dir() {
            // フォルダは処理しない
            continue;
        }
        all_paths.push(input_path);
    }

    // スレッドごとに画像ファイルをサムネイル化
    let processed_count = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for chunk in all_paths.chunks((all_paths.len() + 3) / 4) {
        let chunk = chunk.to_vec();
        let processed_count = processed_count.clone();
        let output = args.output.clone();

        handles.push(thread::spawn(move || {
            let mut local_count = 0;
            for path in chunk {
                // 画像ファイルの読み込み
                let img = image::open(&path);
                if let Ok(img) = img {
                    // サムネイル化
                    let thumbnail = img.thumbnail(64, 64);

                    // サムネイルをファイルとして保存
                    let output_path = output.join(path.file_name().unwrap());
                    thumbnail.save(output_path).unwrap();

                    local_count += 1;
                }
            }

            // 最後にまとめてカウント値を加算
            // NOTE: 同期回数をなるべく少なくし、処理の効率化を図るため
            let mut writer = processed_count.lock().unwrap();
            *writer += local_count;
        }));
    }

    // 各スレッドの終了を待機
    for handle in handles {
        handle.join().unwrap();
    }

    // 最後に各スレッドのカウント値を合計して表示
    println!(
        "Processed {} images",
        processed_count.as_ref().lock().unwrap()
    );
}
