fn main() {
    let s1 = String::from("hello");
//    s1 would be invalid
//    let s2 = s1;
    // we clone s1 instead. It's more expensive
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let string = String::from("Hello world, my name is Anton");
    let part = first_word(&string);
    println!("{}", part);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
