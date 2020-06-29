use std::fmt;

struct TupleMatrix ((f32, f32), (f32, f32));

fn transpose(t: TupleMatrix) -> TupleMatrix {
    let TupleMatrix((first, second), (third, forth)) = t;

    TupleMatrix((first, third), (second, forth))
}

impl fmt::Display for TupleMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})\n({}, {})", (self.0).0, (self.0).1, (self.1).0, (self.1).1)
    }
}

fn main() {
    let mat = TupleMatrix((1.1, 1.2), (2.1, 2.2));

    println!("Matrix:\n{}", mat);

    println!("Transpose:\n{}", transpose(mat));
}