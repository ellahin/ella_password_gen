mod service_accounts;
use std::io::stdin;


fn main() {

    let mut input_string = String::new();

    println!("{}", "How many passwords do you want: ");
    stdin().read_line(&mut input_string).ok().expect("Failed to read line");

    let pass_amount: i32 = input_string.trim().parse().unwrap();

    for _n in 1..=pass_amount {
        println!("{}", service_accounts::get_strong_pass())
    }

}