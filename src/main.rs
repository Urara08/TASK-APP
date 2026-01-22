use std::io;//標準入力
use std::fs::OpenOptions;//ファイル追記
use std::fs::File;//ファイル操作用
use std::io::{Read, Write, BufWriter};//ファイル入出力
use std::io::prelude::*;//ファイル入出力


mod register;//新規タスク登録用モジュール 宣言
mod delete;//タスク削除用モジュール 宣言
use register::file_add_register;//新規タスク登録用モジュールの関数をインポート 

fn main(){

let mut file_path:File= File::open("src/data.txt").unwrap();

let mut contents = String::new();

//ファイルの内容を1行ずつ読み込む
file_path.read_to_string(&mut contents).unwrap();

let mut lines: Vec<String> = Vec::new();

//改行でVec<String>に分割
//生データからVecの中身を形成
// (Vecをシャドーイング：contents⇒lines)
let mut lines: Vec<String> = contents
.lines()
.filter(|&line| !&line.trim().is_empty()) // ← Vecの空行を除外
.map(String::from)
.collect();


//未完了タスクの表示
if lines.len() == 0 {
    println!("未完了タスクはありません\n処理を選択してください\n(０:新規タスク登録、１:タスクの完了)");}
    else{
        println!("処理を選択してください\n(０:新規タスク登録、１:タスクの完了)");
    }

//ナンバリングしたlinesを表示
    for (index, line) in &mut lines.clone().into_iter().enumerate()  {
    let line = line; // linesから文字列を取り出す
    println!("{}:{}", index + 1, line); // 行番号 (1から開始) と行文字列を出力
    }

//処理番号を標準入力で取得
//入力を受け取る箱を作る
let mut Service_type = String::new();
io::stdin().read_line(&mut Service_type).unwrap();

Service_type.trim().to_string().parse::<u32>().unwrap();
println!("{}",Service_type);

if Service_type.trim() == "0"{
println!("新規タスクを入力してください");
//タスクの登録へ

let mut Register_name = String::new();
io::stdin().read_line(&mut Register_name).unwrap();
Register_name.trim().to_string().parse::<String>().unwrap();


//Register_nameをdata.txtに追記する処理へ
file_add_register(file_path,Register_name);


}else if


    Service_type.trim() == "1" && lines.len() != 0{
        println!("完了したタスク番号を入力してください");
        let mut task_number = String::new();
        io::stdin().read_line(&mut task_number).unwrap();


        let task_index = task_number.trim().parse::<usize>().unwrap() - 1;
        //println!("{}",task_index);

        //指定されたタスクを削除する処理へ

        let mut f = File::open("src/data.txt").unwrap();
        if lines.len() > task_index{
            lines.remove(task_index);
            println!("残りのタスク{:?}",&lines);//Vecが正しい状態かデバック用に確認
            //ファイルを一旦空にする
            let mut file_path = BufWriter::new(File::create("src/data.txt").unwrap());

            let mut file_path = OpenOptions::new()
            .create(true)
            .append(true)
            .open("src/data.txt")
            .expect("ファイルを開けませんでした");

            writeln!(file_path, "{}", &lines.join("\n")).unwrap();
            println!("タスクの削除が完了しました");

        }else {println!("エラー: 指定された番号は範囲外です");}


    }else{
//未完了タスクがない場合
            println!("未完了のタスクはありません");
            return;
        }
}
