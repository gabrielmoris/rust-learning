/*
Relational & logical operators
Relational: <, >, <=, >=, ==, !=
*/
fn main() {
    println!("{}", 7 > 4 && 5 == 5);
    println!("{}", 7 > 4 || 5 < 3);
    println!("{}", !true);

    //////////////////////
    let size = 10;
    if size <= 5 {
        println!("small")
    } else {
        println!("big")
    }

    if size < 3 {
        println!("small")
    } else if size < 7 {
        println!("medium")
    } else {
        println!("large")
    }

    ////////////////

    let job_code = "M";
    let job = match job_code {
        "M" => {
            println!("Marketing");
            "Marketing"
        }
        "A" => "Accounting",
        "S" => "Sales",
        _ => "Other",
    };
    println!("Department: {}", job)
}
