use std::fmt;

fn main() {
    #[derive(Debug)]
    struct Complex {
        real: f64,
        imaginary: f64,
    };

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imaginary)
        }
    }

    let complex_number = Complex { real: 3.2, imaginary: 7.2 };

    println!("Display: {}", complex_number);
    println!("Debug: {:?}", complex_number);
}