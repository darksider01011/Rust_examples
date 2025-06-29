fn main() {
    let a: i32 = func_one();
    let b: char = func_two();

    println!("return value from func_one is: {a}");
    println!("return value from func_two is: {b}");

}

fn func_one() -> i32{
    let x = 5;
    x + 10
}

fn func_two() -> char{
    let a = 'H';
    return a
}

