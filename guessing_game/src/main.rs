fn main() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = "xyz0123";
        result = longest(string1.as_str(), string2);
    }
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
