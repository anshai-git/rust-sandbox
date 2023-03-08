fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String
}
