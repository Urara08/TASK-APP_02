use std::io;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::{Read, Write, BufWriter};


pub fn run(file_path: &str){
//標準入力からタスク名を取得しStringへ変換する関数へ
println!("新規タスクを入力してください");
let mut new_task_name = String::new();//入力されたタスク名を格納
io::stdin().read_line(&mut new_task_name).unwrap();//標準入力で取得
new_task_name.trim().to_string().parse::<String>().unwrap();//文字列に変換
/*--------------------------------------------------------------------------------------------*/

//追加足したタスクをdata.txtに追記する関数へ
let new_task_name = file_rewrite_for_register(new_task_name);
/*--------------------------------------------------------------------------------------------*/

//追加足したタスクをdata.txtに追記する関数
pub fn file_rewrite_for_register(file_path:String){
let mut file = OpenOptions::new()
.create(true)// ファイルがなければ作成
.append(true)// 追記モードで開く
.open("src/data.txt")// ファイルを開く
.expect("ファイルを開けませんでした");
writeln!(file, "{}", file_path).unwrap();// 登録内容を追記
println!("新規タスクの登録が完了しました");
}
}