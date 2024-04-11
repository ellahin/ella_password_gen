mod password_type;
mod service_accounts;
mod user_accounts;
use password_type::PasswordOption;
use service_accounts::gen_service_pass;
use std::io::stdin;
use user_accounts::gen_user_pass;

fn main() {
    let mut input_string = String::new();

    let options_structs: Vec<PasswordOption> = [
        PasswordOption {
            option_char: 's',
            discription_string: "Service Account".to_owned(),
            password_function: gen_service_pass,
        },
        PasswordOption {
            option_char: 'u',
            discription_string: "User account".to_owned(),
            password_function: gen_user_pass,
        },
    ]
    .to_vec();

    let mut working_option: Option<PasswordOption> = None;

    while working_option.is_none() {
        println!("{}", "Select from the below options:");
        for _p in &options_structs {
            println!("{}: {}", _p.option_char, _p.discription_string)
        }

        stdin()
            .read_line(&mut input_string)
            .ok()
            .expect("Failed to read line");
        let input_vec: Vec<char> = input_string.trim().chars().collect();
        input_string = String::new();

        for _p in &options_structs {
            if input_vec[0] == _p.option_char {
                working_option = Some(_p.clone());
                break;
            }
        }
    }

    println!("{}", "How many passwords do you want: ");
    stdin()
        .read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");

    let pass_amount: i32 = input_string.trim().parse().unwrap();

    for _n in 1..=pass_amount {
        println!("{}", (working_option.clone().unwrap().password_function)())
    }
}
