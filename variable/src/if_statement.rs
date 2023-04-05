/**
 * if statement examples
 */

use std::io::stdin;


pub fn general(data_1: i32, data_2: i32, target: i32) -> bool {
    if data_1 + data_2 > target {
        true
    } else {
        false
    }
}

pub fn score_status() {
    // let mut score = String::new();
    // stdin().read_line(&mut score).expect("");
    // let score: i32 = match score.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => todo!(),
    // };

    // if score == 100 {
    //     println!("學霸");
    // }
    // else if score >= 60 {
    //     println!("及格");
    // }
    // else {
    //     println!("不及格");
    // }
    let mut msg = String::new();
    stdin().read_line(&mut msg).expect("輸入有誤");

    // let result = if msg.len() > 2 { msg.len() } else { 0 };    // 在 let 語句使用
    // println!("{}", msg.len());
    // println!("{}", result);
}





