pub fn take_ownership(some_string: String) {
    println!("Some string {}", some_string);
}

pub fn make_copy(x: i32) {
    println!("Make a copy {}", x);
}

pub fn give_ownership() -> String {
    let s = String::from("Hello");
    s
}
pub fn take_and_gives_back(a_string: String) -> String {
    a_string
}
pub fn calculate_len(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
pub fn calculate_length_with_refernce(s: &String) -> usize {
    s.len()
}
