use std::io;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::{Read, Write, BufWriter};

//登録内容をdata.txtに追記する関数
//x=file_path, y=Register_name
pub fn file_add_register(x:String){
let mut file = OpenOptions::new()
.create(true)
.append(true)
.open("src/data.txt")
.expect("ファイルを開けませんでした");
writeln!(file, "{}", x).unwrap();
println!("新規タスクの登録が完了しました");
}