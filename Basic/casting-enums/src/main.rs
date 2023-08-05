#![allow(unused)]
fn main() {
    // casting
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    println!("u32 sum {}", int3_u32);

    // enums
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }

    let today: Day = Day::Monday;
    match today {
        Day::Monday => println!("I hate Mondays"),
        Day::Tuesday => println!("Still too far from weekend"),
        Day::Wednesday => println!("Far, but better"),
        Day::Thursday => println!("Closer!"),
        Day::Friday => println!("Yeah! last day"),
        Day::Saturday => println!("I am bored"),
        Day::Sunday => println!("Prepare to work!"),
    }

    println!(" Is today the  weekend? {}", today.is_weekend())
}
