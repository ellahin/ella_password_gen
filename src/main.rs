mod service_accounts;
mod user_accounts;
use std::io::stdin;
use service_accounts::gen_service_pass;
use user_accounts::gen_user_pass;


fn main() {

    let mut input_string = String::new();
    let mut input_vec: Vec<char>= Vec::new();

    let options: Vec<char> = ['s', 'u'].to_vec();
    let options_description: Vec<String> = ["Service account".to_owned(), "User account".to_owned()].to_vec();
    let options_functions: Vec<&dyn Fn() -> String> = vec![&gen_service_pass, &gen_user_pass];
    
    let mut options_correct: bool = false;

    while !options_correct {
        println!("{}", "Select from the below options:");
        for (pos, _c) in options.iter().enumerate(){
            println!("{}: {}", options[pos], options_description[pos])
        }

        stdin().read_line(&mut input_string).ok().expect("Failed to read line");
        input_vec = input_string.trim().chars().collect();
        input_string = String::new();
        options_correct = options.contains(&input_vec[0]);
    }

    println!("{}", "How many passwords do you want: ");
    stdin().read_line(&mut input_string).ok().expect("Failed to read line");

    let pass_amount: i32 = input_string.trim().parse().unwrap();

    let function_index = options.iter().position(|&r| r == input_vec[0]).unwrap();

    for _n in 1..=pass_amount {
        println!("{}", options_functions[function_index]())
    }

}