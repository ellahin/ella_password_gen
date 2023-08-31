#[derive(Clone)]
pub struct PasswordOption {
    pub option_char: char,
    pub discription_string: String,
    pub password_function: fn() -> String,
}