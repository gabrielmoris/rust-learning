use std::cmp::Ordering;
fn main() {
    let age = 8;
    if age <= 1 && age <= 18 {
        println!("Happy Birthday");
    } else if age == 21 || age == 50 {
        println!("Happy Birthday old person");
    } else if age > 65 {
        println!("Happy Birthday grandma");
    } else {
        println!("Happy no Birthday");
    }

    vote();
    matchfn();
    matchfn2()
}

fn vote() {
    let my_age = 47;
    let can_vote = if my_age >= 18 { true } else { false };
    println!("Can vote : {}", can_vote)
}

fn matchfn() {
    let age2 = 8;
    match age2 {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not an Important Birthday"),
    }
}

fn matchfn2() {
    let my_age2 = 18;
    let voting_age = 18;

    match my_age2.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You can vote Now!"),
    }
}
