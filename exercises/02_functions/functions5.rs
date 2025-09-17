// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    num * num
}

fn main() {
    let answer = square(3);
    // let answer = square(1_000_000_000); will throw runtime error
    println!("The square of 3 is {answer}");
}
