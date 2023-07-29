fn main() {
    println!("ARRAYS");
    // arrays have static sizes
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("1st : {}", arr[0]);
    println!("Length : {}", arr.len());
    // LOOP
    println!("LOOP");
    let mut loop_idx = 0;
    loop {
        if arr[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr[loop_idx] == 9 {
            break;
        }
        println!("VAL : {}", arr[loop_idx]);
        loop_idx += 1;
    }

    loop_idx = 0;

    // WHILE LOOP
    println!("WHILE LOOP");
    while loop_idx < arr.len() {
        println!("ARR : {}", arr[loop_idx]);
        loop_idx += 1;
    }

    // FOR LOOP
    println!("FOR LOOP");

    for val in arr.iter() {
        println!("FORLOOP : {}", val);
    }

    //TUPLES

    let tuple: (u8, String, f64) = (47, "Gabriel".to_string(), 50_000.0);
    print!(" NAME : {}", tuple.1);
    let (v1, v2, v3) = tuple;
    print!(" AGE : {}", v1);
    print!(" Name : {}", v2);
    print!(" Salary : {}", v3)
}
