use std::fs::OpenOptions;
use std::fs::File;
use std::io::{Read, Write, BufWriter};
use std::io::prelude::*;

//タスク削除用関数
//x=lines
pub fn file_for_delete(x:&Vec<String>){
let mut file = OpenOptions::new()
.create(true)// ファイルがなければ作成
.write(true)// 書き込みモードで開く
.truncate(true)// 既存の内容を削除(一度クリア)
.open("src/data.txt")// ファイルを開く
.expect("ファイルを開けませんでした");
writeln!(file,"{}",x.join("\n")).unwrap();// 削除後のタスク一覧を書き込み
println!("タスクの削除が完了しました");
}