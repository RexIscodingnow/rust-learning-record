// 預導入 (preiude)
// use std::io::stdin;
// use std::cmp::Ordering;
// use rand::Rng;

fn main() {
    println!("Hello, world!");
    // let mut foo = 1;  // 加上 mut 可變的
    // let bar = foo;    // 不可變的

    // loop 關鍵字，無窮迴圈
    // loop {
    //     println!("猜數字");
    //     let secret_num = rand::thread_rng().gen_range(1..101);
    //     // println!("生成數字: {}", secret_num);
    
    //     /*
    //     取得用戶輸入
    //     io::stdin().read_line(&mut 變數)
    
    //         io::Result 
    //         返回值: 
    //                 Ok + 結果值
    //                 Err + 失敗原因
    //                 expect() : 若返回值為 Err，則中斷當前運作，且把傳入的字串顯示出來
    //                 反之，Ok 則是提取 附加的值，作為結果返回給使用者
    //     */
    //     let mut guess = String::new();
    //     stdin().read_line(&mut guess).expect("讀取失敗");
    
    //     // 隱藏機制 (shadow)  字串轉換型別為 無號整數 (32 位元)
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
        
    //     println!("猜的數字是: {}", guess);
    
    //     match guess.cmp(&secret_num) {
    //         Ordering::Less => println!("太小"),
    //         Ordering::Greater => println!("太大"),
    //         Ordering::Equal => {
    //             println!("猜對了");
    //             break;
    //         },
    //     }
    // }

	
	// if statement
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
    // let mut msg = String::new();
    // stdin().read_line(&mut msg).expect("輸入有誤");

    // let result = if msg.len() > 2 { msg.len() } else { 0 };    // 在 let 語句使用
    // println!("{}", result);

    // 迴圈
    // let mut x = 0;
    /* 1. loop */
    // loop {
    //     if x == 100 {
    //         break x;
    //     }
    //     x += 1;
    //     println!("{}", x);
    // }
    // let result = loop {
    //     if x == 100 {
    //         break x;
    //     }
    //     x += 1;
    //     println!("{}", x);
    // };

    // println!("loop 結束, result = {}", result);

    /* 2. while */
    // while x < 10 {
    //     println!("{}", x);
    //     x += 1;
    // }
    // println!("while loop done");

    /* 3. for */
    // for i in 0..10 {
    //     println!("{}", i);
    // }

    // for (i, j) in (5..10).enumerate() {
    //     println!("i = {}, j = {}", i, j);
    // }

}
