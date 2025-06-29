fn main() {
    let a: i32 = func_one();
    let b: char = func_two();
    let h: i32 = func_three(50);

    println!("return value from func_one is: {a}");
    println!("return value from func_two is: {b}");
    println!("return value from func_three is: {h}");
}

// return expression
fn func_one() -> i32{
    let x = 5;
    x + 5
}

// return with return keyword 
fn func_two() -> char{
    let a = 'H';
    return a
}

// define function parameter with return value 
fn func_three(c : i32) -> i32 {
    let d = c;
    return d
} 

