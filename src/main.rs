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
//data.txtを開く
let mut file_path:File= File::open("src/data.txt").unwrap();
//読み込んだデータをString型に格納
let mut contents = String::new();
//data.txtの内容を１行ずつcontentsに読み込み
file_path.read_to_string(&mut contents).unwrap();
//読み込んだデータをVec<String>型に格納
let mut lines: Vec<String> = Vec::new();

//data.txtの生データからVecの中身を形成
let mut lines: Vec<String> = contents//(Vecをシャドーイング：contents⇒lines)
.lines()//改行で分割
.filter(|&line| !&line.trim().is_empty()) //Vecの空行を除外
.map(String::from)//Vec<String>に分割
.collect();//Vecに格納

//未完了タスクの表示
if lines.len() == 0 {
    println!
    ("未完了タスクはありません\n処理を選択してください\n(０:新規タスク登録、１:タスクの完了)");}
    else{
    println!
    ("処理を選択してください\n(０:新規タスク登録、１:タスクの完了)");
    }

//ナンバリングしたlinesを表示
    for (index, line) in &mut lines.clone().into_iter().enumerate()  {//インデックスと各要素のペアをタプルにして取得
    let line = line; //linesから文字列を取り出す
    println!("{}:{}", index + 1, line); //行番号 (1から開始) と行文字列を出力
    }

//処理番号を標準入力で取得
let mut Service_type = String::new();//入力された処理番号を格納
io::stdin().read_line(&mut Service_type).unwrap();//標準入力で取得
Service_type.trim().to_string().parse::<u32>().unwrap();//整数に変換

//処理番号で分岐
if Service_type.trim() == "0"{

//０：タスクの登録へ
    println!("新規タスクを入力してください");
    let mut Register_name = String::new();//入力されたタスク名を格納
    io::stdin().read_line(&mut Register_name).unwrap();//標準入力で取得
    Register_name.trim().to_string().parse::<String>().unwrap();//文字列に変換

//追加足したタスクをdata.txtに追記する関数へ
    file_add_register(Register_name);

}else if
//１：タスクの削除へ
    Service_type.trim() == "1" && lines.len() != 0{//残っているタスクがある場合
    println!("完了したタスク番号を入力してください");
    let mut task_number = String::new();//入力されたタスク番号を格納
    io::stdin().read_line(&mut task_number).unwrap();//標準入力で取得
    let task_index = task_number.trim().parse::<usize>().unwrap() - 1;//Vecは0始まりのため-1

//指定されたタスク番号のタスクを削除する処理へ
let mut f = File::open("src/data.txt").unwrap();//ファイルを開く
if  lines.len() > task_index{//指定されたタスク番号がタスク数を超えていない場合
    lines.remove(task_index);//Vecから指定されたタスク番号のタスクを削除
    println!("未完了のタスク{:?}",&lines);

//削除後のタスク一覧をdata.txtに書き込む関数へ
    file_for_delete(&lines);

}else{
//指定されたタスク番号が無い場合
    println!("エラー: 指定された番号は範囲外です");}

}else{
//未完了タスクがない場合
    println!("未完了のタスクはありません");
    return;
    }
}
