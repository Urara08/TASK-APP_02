use std::io;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::{Read, Write, BufWriter};
use crate::form::read_for_input;


pub fn run(file_path: &str)-> Result<(), Box<dyn std::error::Error>>{
//標準入力からタスク名を取得しStringへ変換する関数へ
println!("新規タスクを入力してください");
/*--------------------------------------------------------------------------------------------*/
//標準入力を取得する関数へ
let new_task_name:String =  read_for_input()?;//文字列に変換
/*--------------------------------------------------------------------------------------------*/
//追加足したタスクをdata.txtに追記する関数へ
let new_task_name = file_rewrite_for_register(new_task_name);
     Ok(())
}
/*--------------------------------------------------------------------------------------------*/
//追加足したタスクをdata.txtに追記する関数
pub fn file_rewrite_for_register(file_path:String)-> Result<(), Box<dyn std::error::Error>>{
let mut file = OpenOptions::new()
.create(true)// ファイルがなければ作成
.append(true)// 追記モードで開く
.open("src/data.txt")?;// ファイルを開く
writeln!(file, "{}", file_path)?;// 登録内容を追記
println!("新規タスクの登録が完了しました");
Ok(())
}