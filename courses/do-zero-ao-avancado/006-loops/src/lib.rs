pub fn for_loop() {
    let vector = vec![1, 2, 3, 4, 5];
    for i in vector {
        println!("{}", i);
    }
}

pub fn while_loop() {
    let mut i = 0;
    while i < 10 {
        println!("{}", i);
        i += 1;
    }
}

pub fn loop_break() {
    let mut i = 0;
    loop {
        if i == 10 {
            break;
        }
        println!("{}", i);
        i += 1;
    }
}
