use rand::prelude::*;
use chbslib::get_entropy;
use std::io::stdin;

fn gen_pass(len: i32, sybols: &String) -> String {

    let char_vec: Vec<char> = sybols.chars().collect();

    drop(sybols);

    let mut generated_password:String = String::new();

    for _n in 0..=len {

        generated_password.push(char_vec[thread_rng().gen_range(1..=(char_vec.len() - 1))])

    }
    
    return generated_password
}

fn get_strong_pass() -> String {

    let mut current_password: String = String::from("");

    let mut entropy: i16 = 0;

    let numbers: &str = "0123456789";
    let lower_caps: &str = "abcdefghijklmnopqrstuvwxyz";
    let upper_caps: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let sybols: &str = "!@#$%^&*()";

    let mut all_symbols: String = "".to_owned();

    all_symbols.push_str(numbers);
    all_symbols.push_str(lower_caps);
    all_symbols.push_str(upper_caps);
    all_symbols.push_str(sybols);
    
    // checks that it has required_symbols_count of each type of sybols
    let mut required_symbols: bool = false;

    // how many of each sybol there should bu
    let required_symbols_count: i32 = 3;

    while (entropy < 3) & !required_symbols{
        current_password = gen_pass(thread_rng().gen_range(32..=64), &all_symbols);

        entropy = get_entropy(&current_password);

        required_symbols = check_requried_symbols(&current_password, &[numbers, lower_caps, upper_caps, sybols], required_symbols_count.clone());

    }

    return current_password

}

fn check_requried_symbols (password: &str, list_str_sybols: &[&str], sybol_count: i32) -> bool {

    let list_leng:usize = list_str_sybols.len().try_into().unwrap();
    let mut valid_list: Vec<bool> = Vec::new();
    let working_pasword_vec: Vec<char> = password.chars().collect();
    let working_count: usize = sybol_count as usize;


    for _n in 0..(list_leng - 1){

        let mut working_valid_list: Vec<bool> = Vec::new();

        let work_chars: Vec<char> = list_str_sybols[_n].chars().collect();

        for _c in 0..(working_pasword_vec.len() - 1){

            if work_chars.contains(&working_pasword_vec[_c]){
                working_valid_list.push(true)
            }

        }

        valid_list.push(working_valid_list.len() >= working_count)

    }

    return !(valid_list.contains(&false));
}

fn main() {

    let mut input_string = String::new();

    println!("{}", "How many passwords do you want: ");
    stdin().read_line(&mut input_string).ok().expect("Failed to read line");

    let pass_amount: i32 = input_string.trim().parse().unwrap();

    for _n in 1..=pass_amount {
        println!("{}", get_strong_pass())
    }

}