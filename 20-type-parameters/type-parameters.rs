/* enum Option<T> {
    None,
    Some(T),
} */

/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

fn main() {
    let mut my_string: String = "hello world".to_string();
    let mut email_string: String = "abc@gmail.com".to_string();
    // let last_char: char = my_string.pop();
    let last_char: Option<char> = my_string.pop();
    let email: Option<String> = Some(email_string);
    let empty_string: Option<String> = None;
    let success: Result<String, String> = Ok("Success".to_string());
    let error: Result<String, String> = Err("Error".to_string());
}