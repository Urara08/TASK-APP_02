//タスク表示用関数

//data.txtの生データからVecの中身をString型で形成
//x=task_lines
pub fn model_for_task_lines(contents:String)-> Vec<String>{
    contents
    .lines()//改行で分割
    .filter(|&line| !&line.trim().is_empty()) //Vecの空行を除外
    .map(String::from)//Vec<String>に分割
    .collect()//Vecに格納

}

//インデックスとペアにして改行表示
//x=task_lines
pub fn appear_for_unfinished_task(x:&Vec<String>){

    let mut print_list = String::new();
    for (index, line) in x.iter().enumerate()  {//インデックスと各要素のペアをタプルにして取得
    let line = line; //x:task_linesから文字列を取り出す
    print_list.push_str(&format!("{}:{}\n", index + 1, line)); //行番号 (1から開始) と行文字列を出力
    }
    println!("未完了タスク一覧\n{}",print_list);//改行コードを維持して表示
}