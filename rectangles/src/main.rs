use std::io;
use std::io::Write;

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rect {
        width: input_number("Please enter the width: "),
        height: input_number("Please enter the height: "),
    };

    println!("The area of the rectangle is: {}", rect1.area());
}

fn input_number(prompt: &str) -> u32 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}

// fn area(rect: &Rect) -> u32 {
//     rect.width * rect.height
// }
