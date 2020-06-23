/// Here is the documentation comment
fn main() {
    print!("without new line {} \n", 100);
    println!("Positional, {1}, {1}, {0}, {0}", "hello", 100);
    println!("Named positions {hello}, {world}", world="world", hello="Hello");
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("Hello, world");
    println!("I'm a Rustacean!");

    /* Error */
    eprint!("Error without new line\n");
    eprintln!("Error with new line");
}