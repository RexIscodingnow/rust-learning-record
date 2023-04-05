pub fn loop_example() {
    // println!("一般的 loop 用法");
    // let mut x = 0;
    // loop {
    //     if x == 100 {
    //         break;
    //     }
    //     x += 1;
    //     println!("{}", x);
    // }
    // println!("有指派數值的 loop 用法");
    
    // let result = loop {
    //     if x == 100 {
    //         break x * 2;
    //     }
    //     x += 1;
    //     println!("{}", x);
    // };

    // println!("loop 結束, result = {}", result);
}


pub fn while_loop() {
    /* while loop */
    // let mut x = 0;
    
    // while x < 10 {
    //     println!("{}", x);
    //     x += 1;
    // }
    // println!("while loop done");

    // let number = [1, 2, 3, 4, 5];
    // let mut index = 0;
    // while index < 5 {
    //     print!("{} ", number[index]);
    //     index += 1;
    // }
}


pub fn for_loop() {
    // for i in 0..10 {
    //     println!("{}", i);
    // }

    // for (i, j) in (5..10).enumerate() {
    //     println!("i = {}, j = {}", i, j);
    // }

    // let nums = [123, 41, 8456, 465];

    // for element in nums.iter() {
    //     println!("{}", element);
    // }

    // for number in (1..4).rev() {
    //     println!("{}", number);
    // }
    // println!("for 迴圈結束");
    
    // 'outer: for i in 0..10 {
    //     'inner: for j in 0..10 {
    //         if i % 2 == 0 { println!("outer loop"); continue 'outer; }
    //         if j % 2 == 0 { println!("inner loop"); continue 'inner; }

    //         println!("i: {}, j: {}", i, j);
    //     }
    // }

    // let lines = "hello\nworld".lines();
    // let lines = lines.enumerate();
    
    // for (line_number, line) in lines {
    //     // println!("{}", line);
    //     println!("lineNumber: {} => {}", line_number, line);
    // }

}