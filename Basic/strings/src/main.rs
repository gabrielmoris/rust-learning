fn main() {
    let mut st1 = String::new();
    st1.push('¡');
    st1.push_str("Hello World!");
    for word in st1.split_whitespace() {
        println!("{}", word);
    }
    let st2 = st1.replace("¡", "Another ");
    println!("{}", st2);

    let st3 = String::from("x r t y h w e f f e r s");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }
    let st4: &str = "random string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);
    let byte_arr1 = st5.as_bytes();
    for byte in byte_arr1 {
        println!("Byte: {} ", byte);
    }
    let st6 = &st5[0..6];
    println!("String length : {} ", st6.len());
    st5.clear();
    let st6 = String::from("Just some ");
    let st7 = String::from("strings");
    let st8 = st6 + &st7;
    for char in st8.bytes() {
        println!("{}", char);
    }
}
