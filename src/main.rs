// 預導入 (preiude)
use std::io;

fn main() {
    println!("Hello, world!");
    // let mut foo = 1;  // 加上 mut 可變的
    // let bar = foo;    // 不可變的

    
    /*
    取得用戶輸入
    io::stdin().read_line(&mut 變數)

        io::Result 
        返回值: Ok + 結果值
        Err + 失敗原因
        expect() : 若返回值為 Err，則中斷當前運作，且把傳入的字串顯示出來
        反之，Ok 則是提取 附加的值，作為結果返回給使用者
    */
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("讀取失敗");

    println!("猜的數字是: {}", guess);
}
