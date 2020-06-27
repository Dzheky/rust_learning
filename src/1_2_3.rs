use std::fmt::{self, Formatter, Display};

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({0}, {1}, {2}) 0x{0:X}{1:X}{2:X}", self.red, self.green, self.blue)
    }
}

fn main() {
    let color = Color {
        red: 255,
        green: 200,
        blue: 128,
    };

    println!("{}", color);
}