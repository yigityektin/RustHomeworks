fn concatenate_strings(string1: &str, string2: &str) -> String {
    let mut result = String::new();
    result.push_str(string1);
    result.push_str(string2);
    return result;
}

fn main() {
    let string1 = String::from("Yigit ");
    let string2 = String::from("Yektin");
    let changed_string = concatenate_strings(&string1, &string2);
    println!("{}", changed_string);
}