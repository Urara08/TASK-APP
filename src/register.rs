use std::io;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::{Read, Write, BufWriter};

//登録内容をdata.txtに追記する関数
//x=file_path
pub fn file_add_register(x:String){
let mut file = OpenOptions::new()
.create(true)// ファイルがなければ作成
.append(true)// 追記モードで開く
.open("src/data.txt")// ファイルを開く
.expect("ファイルを開けませんでした");
writeln!(file, "{}", x).unwrap();// 登録内容を追記
println!("新規タスクの登録が完了しました");
}