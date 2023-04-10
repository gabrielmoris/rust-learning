fn main() {
    // Basic opperations
    println!("Addition      : {}", 10 + 10);
    println!("Substraction  : {}", 5 - 9);
    println!("Multiplication: {}", 10 * 10);
    println!("Division      : {}", 10 / 2);
    println!("Modulus       : {}", 8 % 3);

    //Math Functions (We have to specify in the number the data type)
    println!("Absolute      : {}", -5i32.abs());
    println!("Power         : {}", 3i32.pow(2));
    println!("Square        : {}", 36f64.sqrt());
    println!("Cube          : {}", 9f64.cbrt());
    println!("Round         : {}", 1.45f64.round());
    println!("Floor         : {}", 1.45f64.floor());
    println!("Ceiling       : {}", 1.45f64.ceil());
    println!("e ^ x         : {}", 2f64.exp());
    println!("ln            : {}", 2f64.ln());
    println!("log10()       : {}", 2f64.log10());
    println!("x to Radians  : {}", 90f64.to_radians());
    println!("x to Degrees  : {}", 3.14f64.to_degrees());
    println!("Max           : {}", 4f64.max(5f64));
    println!("Min           : {}", 4f64.min(5f64));

    //sin,cos, tan,asin,acos,atan,atan2,sinh, cosh,tanh
    println!("Sin           :{}", 3.14f64.sin())
}
