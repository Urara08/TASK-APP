use std::io;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::{Read, Write, BufWriter};
use std::io::prelude::*;
mod register;
mod delete;
use register::file_add_register;
use delete::file_for_delete;


fn main(){
let mut file_path:File= File::open("src/data.txt").unwrap();
let mut contents = String::new();
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
    println!
    ("未完了タスクはありません\n処理を選択してください\n(０:新規タスク登録、１:タスクの完了)");}
    else{
        println!
        ("処理を選択してください\n(０:新規タスク登録、１:タスクの完了)");
    }

//ナンバリングしたlinesを表示
    for (index, line) in &mut lines.clone().into_iter().enumerate()  {
    let line = line; // linesから文字列を取り出す
    println!("{}:{}", index + 1, line); // 行番号 (1から開始) と行文字列を出力
    }

//処理番号を標準入力で取得
let mut Service_type = String::new();
io::stdin().read_line(&mut Service_type).unwrap();
Service_type.trim().to_string().parse::<u32>().unwrap();
println!("{}",Service_type);

if Service_type.trim() == "0"{
//タスクの登録へ
        println!("新規タスクを入力してください");
        let mut Register_name = String::new();
        io::stdin().read_line(&mut Register_name).unwrap();
        Register_name.trim().to_string().parse::<String>().unwrap();

//追加足したタスクをdata.txtに追記する関数へ
        file_add_register(Register_name);

}else if

//タスクの削除へ
Service_type.trim() == "1" && lines.len() != 0{
println!("完了したタスク番号を入力してください");
let mut task_number = String::new();
io::stdin().read_line(&mut task_number).unwrap();
let task_index = task_number.trim().parse::<usize>().unwrap() - 1;

// //指定されたタスクを削除する処理へ
let mut f = File::open("src/data.txt").unwrap();
if lines.len() > task_index{
        lines.remove(task_index);
        println!("未完了のタスク{:?}",&lines);

//削除後のタスクをdata.txtに書き込む関数へ
        file_for_delete(&lines);
    }else {println!("エラー: 指定された番号は範囲外です");}

    }else{
//未完了タスクがない場合
        println!("未完了のタスクはありません");
        return;
    }
}
