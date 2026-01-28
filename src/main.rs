use std::io;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::{Read, Write, BufWriter};
use std::io::prelude::*;
mod appear;
mod register;
mod delete;
use appear::appear_for_unfinished_task;


fn main(){
//data.txtのパスを指定
let File_Path:&str = "src/data.txt";
//data.txtを開く
let mut file_path:File= File::open(File_Path).unwrap();
//読み込んだデータをString型に格納
let mut contents = String::new();
//data.txtの内容を１行ずつcontentsに読み込み
file_path.read_to_string(&mut contents).unwrap();
/*--------------------------------------------------------------------------------------------*/
//data.txtの生データからVecの中身をString型で形成
let mut task_lines: Vec<String> = contents//(Vecをシャドーイング：contents⇒task_lines)
.lines()//改行で分割
.filter(|&line| !&line.trim().is_empty()) //Vecの空行を除外
.map(String::from)//Vec<String>に分割
.collect();//Vecに格納
/*--------------------------------------------------------------------------------------------*/
//未完了タスクの表示
if  task_lines.len() == 0 {
    println!
    ("未完了タスクはありません\n処理を選択してください\n(０:新規タスク登録、１:タスクの完了)");
    }
    else{
    println!
    ("処理を選択してください\n(０:新規タスク登録、１:タスクの完了)");
    }
/*--------------------------------------------------------------------------------------------*/
//未完了タスク(ナンバリングしたtask_lines)を表示する関数へ
    appear_for_unfinished_task(&task_lines);
/*--------------------------------------------------------------------------------------------*/
//処理番号を標準入力で取得
let mut Service_type = String::new();//入力された処理番号を格納
io::stdin().read_line(&mut Service_type).unwrap();//標準入力で取得
Service_type.trim().to_string().parse::<u32>().unwrap();//整数に変換
/*--------------------------------------------------------------------------------------------*/
//処理番号で分岐
if Service_type.trim() == "0"{
/*--------------------------------------------------------------------------------------------*/
//０：タスクの登録へ
    register::run(File_Path);}
/*--------------------------------------------------------------------------------------------*/
else{
//１：タスクの削除へ
    delete::run(File_Path, task_lines);}
/*--------------------------------------------------------------------------------------------*/
}
