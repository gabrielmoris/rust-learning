fn main() {
    let var = "Hello World! ";
    let var2 = "Jim,John,Jake";

    println!("{}", var);
    println!("{}", var.replace("World!", "Jim!")); //replaces
    println!("{}", var.len()); //returns length
    println!("{}", var.trim()); //trims white space
    println!("{}", var.to_owned() + var2); //"concat"
    println!("{}", var2.to_owned() + ",James");

    for n in var.split_whitespace() {
        //prints each word separated by space
        println!("{}", n);
    }
    for n in var2.split(",") {
        //prints each word wirh separator ",", if I use "" prints each letter
        println!("{}", n);
    }
}
