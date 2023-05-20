/**
 * 所有權 & reference 練習
 */


/* Example 1 (OwnerShip) */
pub fn makes_copy(number: i32) {
    println!("In func: {}", number);
}

pub fn take_ownership_1(string: String) {
    // 接收被 Move 進來的所有權
    // 接收之變數: string
    println!("In func: {}", string);

}  // string 的 scope 結束，被釋放

/* Example 2 (OwnerShip) */
pub fn gives_ownership() -> String {
    let gen_str = String::from("hello");
    gen_str   // gen_str 的所有權，以返回值的方式，轉移所有權
}

pub fn takes_and_gives(string: String) -> String {
    println!("In func: {}", string);   // 一樣接收轉移過來的
    string   // 把接收過來的所有權，又轉移出去
}


/* Example 1 (Borrowing) */
pub fn get_str_length(str: &String) -> usize {
    // 把參考作為函式參數，這個行為稱作為借用 (Borrowing)
    // str.push_str(" world");

    str.len()   // str 並未擁有外部傳入參數的   所有權
}  // 離開作用域後，從外部傳入參數，不會被釋放掉


/* Example 2 (mutable Reference) */
pub fn get_string_length(str: &mut String) -> usize {
    str.push_str(" world");
    
    str.len()
}

