use std::fs::OpenOptions;
use std::fs::File;
use std::io::{Read, Write, BufWriter};
use std::io::prelude::*;

//タスク削除用関数
//x=lines
pub fn file_for_delete(x:&Vec<String>){
let mut file = OpenOptions::new()
.create(true)
.write(true)
.truncate(true)
.open("src/data.txt")
.expect("ファイルを開けませんでした");
writeln!(file,"{}",x.join("\n")).unwrap();
println!("タスクの削除が完了しました");
}