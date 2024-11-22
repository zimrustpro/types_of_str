fn prints_str(my_str: &str) {
    println!("{}", my_str);
}

fn works() -> &'static str {
    "I live forever"
}

fn main() {
    let my_string = String::from("I am a string");
    prints_str(&my_string);
    works();
}
