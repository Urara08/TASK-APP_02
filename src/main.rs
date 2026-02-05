use std::io;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::{Read, Write, BufWriter};
use std::io::prelude::*;
mod appear;
mod register;
mod delete;
mod validata;
mod form;
use crate::appear::model_for_task_lines;
use appear::appear_for_unfinished_task;
use form::read_for_input;


//main()にエラーを集約
fn main() -> Result<(), Box<dyn std::error::Error>> {
    run()?;
    Ok(())
}

//main()にエラーを返す
fn run() -> Result<(), Box<dyn std::error::Error>> {
//data.txtのパスを指定
let File_Path:&str = "src/data.txt";
//data.txtを開く
let mut file_path:File= File::open(File_Path)?;
//読み込んだデータをString型に格納
let mut contents = String::new();
//data.txtの内容を１行ずつcontentsに読み込み
file_path.read_to_string(&mut contents)?;
/*--------------------------------------------------------------------------------------------*/
//data.txtの生データからVecの中身をString型で形成
let mut task_lines: Vec<String> = model_for_task_lines(contents);
/*--------------------------------------------------------------------------------------------*/
//未完了タスクの有無について表示
if  task_lines.len() == 0 {
    println!
    ("未完了タスクはありません\n処理を選択してください\n(０:新規タスク登録、１:タスクの完了)");
    }
    else{
    println!
    ("処理を選択してください\n(０:新規タスク登録、１:タスクの完了)");
/*--------------------------------------------------------------------------------------------*/
//未完了タスク(ナンバリングしたtask_lines)を表示する関数へ
    appear_for_unfinished_task(&task_lines);
    }
/*--------------------------------------------------------------------------------------------*/
//処理番号を標準入力で取得
    let service_type: u32 = read_for_input()?;
/*--------------------------------------------------------------------------------------------*/
// 入力のバリデーション
    validata::inputvalidator::validate_service_type(service_type);
/*--------------------------------------------------------------------------------------------*/
//処理番号で分岐
if service_type == 0{
/*--------------------------------------------------------------------------------------------*/
//０：タスクの登録へ
    register::run(File_Path);}
/*--------------------------------------------------------------------------------------------*/
else{
//１：タスクの削除へ
    delete::run(File_Path, task_lines);}
/*--------------------------------------------------------------------------------------------*/
    Ok(())
}