use std::fs::OpenOptions;
use std::fs::File;
use std::io::{Read, Write, BufWriter};
use std::io::prelude::*;
use std::io;

use crate::form::read_for_input;

pub fn run(file_path: &str, mut task_lines:Vec<String>){
//１：タスクの削除へ
    if task_lines.len() != 0{//残っているタスクがある場合
    println!("完了したタスク番号を入力してください");
/*--------------------------------------------------------------------------------------------*/
//標準入力を取得する関数へ
    let number_for_delete:usize =  read_for_input();
//Vecは0始まりのため-1(number_for_deleteをシャドーイング)
    let number_for_delete:usize =  number_for_delete -1;
/*--------------------------------------------------------------------------------------------*/
//指定されたタスク番号のタスクを削除する処理へ
if  task_lines.len() > number_for_delete{//指定されたタスク番号がタスク数を超えていない場合
    task_lines.remove(number_for_delete);//Vecから指定されたタスク番号のタスクを削除
    println!("未完了のタスク{:?}",&task_lines);
/*--------------------------------------------------------------------------------------------*/
//削除後のタスク一覧をdata.txtに書き込む関数へ
    file_rewrite_for_delete(&task_lines);}
/*--------------------------------------------------------------------------------------------*/
else{
//指定されたタスク番号が無い場合
    println!("エラー: 指定された番号は範囲外です");}
/*--------------------------------------------------------------------------------------------*/
}else{
//未完了タスクがない場合
    println!("未完了のタスクはありません");
    return;
    }
 }
/*--------------------------------------------------------------------------------------------*/
//タスク削除用関数
//x=lines
pub fn file_rewrite_for_delete(x:&Vec<String>)-> Result<(), Box<dyn std::error::Error>>{
let mut file = OpenOptions::new()
.create(true)// ファイルがなければ作成
.write(true)// 書き込みモードで開く
.truncate(true)// 既存の内容を削除(一度クリア)
.open("src/data.txt")?;// ファイルを開く
writeln!(file,"{}",x.join("\n"))?;// 削除後のタスク一覧を書き込み
println!("タスクの削除が完了しました");
Ok(())
}
/*--------------------------------------------------------------------------------------------*/