fn plus_or_substract(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }

    x + 5
}

fn main() {
    let x = plus_or_substract(5);

    println!("The value of x is: {}", x);
}
