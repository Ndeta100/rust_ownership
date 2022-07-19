mod ownership_and_functions;
fn main() {
    println!("Hello, world!");
    let s = "hello";
    {
        let s = "hello";
    }
    println!("{}", s);
    let mut sx = String::from("hello");
    sx.push_str(" world!");
    println!("{}", sx);
    let x = 5;
    let y = x;
    let tup: (u32, bool) = (34, true);
    let tup_clone = tup;
    let (tup_clone_x, tup_clone_y) = tup_clone;
    println!("{}::{}", tup_clone_x, tup_clone_y);
    let s2 = String::from("Ndeta");
    let (len, s) = ownership_and_functions::calculate_len(s2);
    println!("{}, {}", len, s);
    let s4 = String::from("Ndeta");
    ownership_and_functions::calculate_length_with_refernce(&s4);
}
