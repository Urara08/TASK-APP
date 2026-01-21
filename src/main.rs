
use std::fs::File;//ファイル操作用
use std::io::prelude::*;//ファイルの入出力用
use std::io;//標準入力用

fn main(){

let mut f:File= File::open("src/data.rs").unwrap();

let mut contents = String::new();

f.read_to_string(&mut contents).unwrap();



if contents.len() == 0 {
    println!("未完了タスクはありません\n処理を選択してください\n(０:新規タスク登録、１:タスクの完了)");}
    else{
        println!("処理を選択してください\n(０:新規タスク登録、１:タスクの完了)");
    }

//ナンバリングしたcontentsを表示(Rust Vec 自動でナンバリング?)
//
    for (index, line) in &mut contents.lines().enumerate() {
        let line = line; // contentsから文字列を取り出す
        println!("{}: {}", index + 1, line); // 行番号 (1から開始) と行文字列を出力
    }

let mut Service_type = String::new();
io::stdin().read_line(&mut Service_type).unwrap();

Service_type.trim().to_string().parse::<u32>().unwrap();
println!("{}",Service_type);

if Service_type.trim() == "0"{
    println!("新規タスクを入力してください");}else if 

    Service_type.trim() == "1" && contents.len() != 0{
        println!("完了したタスク番号を入力してください");
        let mut task_number = String::new();
        io::stdin().read_line(&mut task_number).unwrap();
        
        task_number.trim().to_string().parse::<u32>().unwrap();
        println!("{}",task_number);}else{
            println!("未完了のタスクはありません");
            return;
        }
}


