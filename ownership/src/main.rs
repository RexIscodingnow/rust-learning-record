/**
 * Rust Ownership 所有權系統 練習
 */

mod reference;
use crate::reference::get_str_length;
use crate::reference::get_string_length;
use crate::reference::take_ownership_1;
use crate::reference::makes_copy;
use crate::reference::gives_ownership;
use crate::reference::takes_and_gives;


fn main() {
    // str_vs_string();
    // ownership();
    // refernce_example();
    mut_reference();
}


fn str_vs_string() {
    /*
        例子練習: 字串字面值 vs String 類型
     */
    
    // 所有權轉移 例子 1
    // string literals (字串字面值) --> 寫死的
    let str_1 = "hello world";
    println!("字串字面值 (str_1): {}", str_1);

    let str_2 = str_1;
    println!("賦值後 (to str_2)");
    println!("str_1: {}", str_1);
    println!("str_2: {}", str_2);

    println!("-----------------------------------");

    // String 類型 --> 可變動大小
    let string_1 = String::from("0123465879");
    println!("String 類型 (string_1): {}", string_1);
    println!("string_1 length: {}", string_1.len());

    let string_2 = string_1;
    // println!("string_1: {}", string_1);   // 編譯錯誤
    println!("string_2: {}", string_2);
    
}


fn ownership() {
    /*
        做進入點
     */

    println!("---------------------------");
    println!("OwnerShip and function");
    println!("Example 1 (No Return Value)");
    let string_immut = String::from("hello");
    
    // string_immut 的所有權轉移，至 take_ownership_1()，變數 string_immut 就失效
    take_ownership_1(string_immut);
    // println!("{}", string_immut);   // string_immut 失效了，就不能使用，因此報錯
    
    let x = 5;
    makes_copy(x);
    println!("In main: {}", x);
    
    println!("---------------------------");
    println!("Example 2 (Return Value)");

    let s1 = gives_ownership();
    println!("In main: {}", s1);

    // println!("In main: {}", s1);
    let s2 = takes_and_gives(s1);   // s1 的 scope 已經過去了，失效了
    println!("In main: {}", s2);

}


fn refernce_example() {
    /*
        (不可) 參考
     */
    let s1 = String::from("hello");

    // 作為參考 (Refernce)，把 s1 值給函式
    // 但其 s1 的所有權，並沒有轉移到函式
    let s1_len = get_str_length(&s1);
    println!("s1: {}, length: {}", s1, s1_len);

}


fn mut_reference() {
    /*
        可變參考
     */
    
    let mut s = String::from("hello");
    // println!("before");
    // println!("s: {}, len: {}", s, s.len());
    // let len = get_string_length(&mut s);
    // println!("after");
    // println!("s: {}, len: {}", s, s.len());
    // println!("s: {}, len: {}", s, len);
    
    // 非同時創建可變參考
    // {
    //     let s1 = &mut s;
    //     // println!("s1: {}, len: {}", s1, s1.len());
    // }
    
    let r1 = &s;
    let r2 = &s;
    // let s2 = &mut s;   // 由於 s 已經 r1, r2 指派為不可變參考
    
    println!("immutable reference => r1: {}, r2: {}", r1, r2);
    // println!("{}", get_str_length(r1));


    // 迷途參考
    // let s = dangling();

}


// fn dangling() -> &String {
//     let s = String::from("hello");
//     &s
// }


