fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
    // let x = None;
    match Some(123) {
        None => println!("Nothing"),
        Some(x) => println!("Yes! {x}")
    }
}
