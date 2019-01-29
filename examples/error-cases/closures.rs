// Closures won't be desugared by default
fn main() {
    let closure = |i: i32| i * i;
}
