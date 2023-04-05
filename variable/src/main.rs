// 預導入 (preiude)
use std::io::stdin;
// use std::cmp::Ordering;
// use rand::Rng;

// const MAX_POINTS: u32 = 100_000;   // 常數

mod if_statement;
mod loop_statement;   // 導入 .rs 檔案

fn main() {
    // println!("Hello, world!");

    // 常數(常量)，不可變 (預設不可變動)
    // let constVar = 4152;
    // println!("{}", constVar);


    // 在 let 後面加上 mut 就可以變動
    // let mut var = 0;
    // println!("{}", var);

    // 隱藏 (shadowing)
    // let x = 5;
    // println!("{}", x);

    // let x = x + 5;
    // println!("{}", x);
    
    // let x = x + 3;
    // println!("{}", x);

    // let space = "     ";
    // println!("{}", space);
    // let space =  space.len();
    // println!("{}", space);

    // 浮點數
    // let x = 32.45;
    // let x: f32 = 6.45;
    
    // 基本運算
    // let mut num = 54;
    // let mut temp = num;
    // println!("原先數值 num = {}", num);
    // temp = num;
    // num *= 123;
    // println!("{} x 123 = {}", temp, num);
    // temp = num;
    // num /= 88;
    // println!("{} / 123 = {}", temp, num);
    // temp = num;
    // num -= 456;
    // println!("{} - 123 = {}", temp, num);
    // temp = num;
    // num += 1000;
    // println!("{} + 123 = {}", temp, num);

    // 布林值
    // let t: bool = true;
    // let f = false;


    // 字元
    // let x = 'X';
    // let y = 'Z';
    // let emoji = '😂';

    // println!("{}", x);
    // println!("{}", y);
    // println!("{}", emoji);

    // Tuple
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("x = {}, y = {}, z = {}", x, y, z);
    // println!("x = {}, y = {}, z = {}", tup.0, tup.1, tup.2);

    // 數組
    // let a = [1, 2, 3, 4, 5];

    // let one = a[0];
    // let two = a[1];
    // println!("{}, {}", one, two);
    
    // println!("{}", a[4]);   // 超過索引值範圍，不會抱錯

    // if statement
    // let is_bigger = if_statement::general(0, 5, 8);
    // println!("{}", is_bigger);

    // if_statement::score_status();

    // 迴圈
    /* 1. loop */
    // loop_statement::loop_example();

    /* 2. while */
    // loop_statement::while_loop();

    /* 3. for */
    // loop_statement::for_loop();

    
    /* 4. String */
    // let mut str = String::from("hello");
    // str.push_str(", world");
    // println!("{}", str);

    // let s1 = String::from(" hello");
    // let s2 = s1;
    // println!("{}", s2);

    // let str1 = "hello";
    // let str2 = str1;
    // println!("str1: {}, str2: {}", str1, str2);

    // println!("{}", add(12, 56));
    // let age: u8 = 12;
    // println!("Is adult {}", is_adult(age));
    
    // 函式指標
    // let fn_ptr = function_ptr;
    // println!("Function Pointer: {}", fn_ptr(50));
    // diverges();
    
}

// fn add(x: i32, y: i32) -> i32 {
//     x + y
// }

// fn is_adult(age: u8) -> bool {
//     if age >= 18 {
//         return true;
//     }
//     else {
//         return false;
//     }
// }

// fn diverges() -> ! {
//     panic!("這函式不回傳值");
// }

// fn function_ptr(number: i32) -> i32 {
//     number * 2
// }

