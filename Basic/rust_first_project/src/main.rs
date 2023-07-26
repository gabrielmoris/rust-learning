// To start:
// > cargo new <app name>
// It creates all the necessary files
//to put the dependencies inCargo.toml ==> https://crates.io/
fn main() {
    #[warn(unused_assignments)]
    // Basic data types (For the example I dont specify which kind of data types, but it knows automatic)
    let string = "this is a string";
    let float = 4.5;
    let boolean = true;
    let icon = '❤';

    // Integrer data type (i can specify which kind of data I want)
    let integer: i32 = 20 - 30;
    let integer2: u32 = 10;
    let mark: isize = 10;
    let count: usize = 30;

    // Floats
    let float2 = 10.00;
    let precise: f32 = 8.35;
    let more_precise: f64 = 15_000.001;

    println!("this is a string: {}", string);
    println!("this is a float: {}", float);
    println!("this is a boolean: {}", boolean);
    println!("this is an icon: {}", icon);
    println!("this is an i32 integer: {}", integer);
    println!("this is an u32 integer: {}", integer2);
    println!("this is a mark: {}", mark);
    println!("this is a count: {}", count);
    println!("this is a float normal: {}", float2);
    println!("this is a precise float: {}", precise);
    println!("this is a more precise float: {}", more_precise);

    // Variables (I can redeclare variable to mutate them: this is called shadowing variables")
    let mut var1 = 25_000;
    println!("this is a mutable variable before mute: {}", var1);
    let _privatevar = "done";
    var1 = 35_00;
    println!("this is a mutable variable after mute: {}", var1);
    let var1 = 23_0;
    println!("this is a shadowing variable after redeclaring: {}", var1);

    // Constants (I need to specify the data type and I can declare them outside of the function)
    const CONSTANT: i32 = 100;
    const PI: f32 = 3.141632;
    println!("this is an integer i32 CONSTANT: {}", CONSTANT);
    println!("this is an float f32 PI: {}", PI);
}
