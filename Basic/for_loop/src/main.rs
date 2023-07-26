fn main() {
    //For Loop
    for x in 1..20 {
        if x % 2 != 0 {
            continue;
        }
        println!("{}", x)
    }

    //While loop
    let mut x = 0;
    while x < 10 {
        x += 1;
        println!("{}", x)
    }
    println!("Done!");

    // Loop loop
    let mut y = 0;
    loop {
        y += 1;
        println!("y={}", y);

        if y == 15 {
            break;
        }
    }
}
