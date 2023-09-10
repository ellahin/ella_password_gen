use rand::prelude::*;

pub fn gen_user_pass() -> String {
    let numbers: Vec<char> = "0123456789".chars().collect();
    let lower_caps: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let upper_caps: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let sybols: Vec<char> = "!@#$%^&*".chars().collect();

    let mut working_password: Vec<char> = Vec::new();

    // setting first char to a random upper case

    working_password.push(upper_caps[thread_rng().gen_range(0..=(upper_caps.len() - 1))]);

    // pushing a length of 3-5 lower case

    for _x in 0..thread_rng().gen_range(3..=4) {
        working_password.push(lower_caps[thread_rng().gen_range(0..=(lower_caps.len() - 1))]);
    }

    // pushing a length of 3-5 numbers

    for _x in 0..thread_rng().gen_range(3..=4) {
        working_password.push(numbers[thread_rng().gen_range(0..=(numbers.len() - 1))]);
    }

    // pushing 1 symbols

    working_password.push(sybols[thread_rng().gen_range(0..=(sybols.len() - 1))]);

    return working_password.iter().collect();
}
