// clippy1.rs
// The Clippy tool is a collection of lints to analyze your code
// so you can catch common mistakes and improve your Rust code.
//
// Execute `rustlings hint clippy1` for hints :)

fn main() {
    let x = 1.2331f64;
    let y = 1.2332f64;
    let error = 0.0001f64;
    if (x - y).abs() < error {
        println!("Success!");
    }
}
