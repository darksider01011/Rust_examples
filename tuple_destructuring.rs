fn main() {
    let x: (i32, u32) = (-50, 100);
    let (a, b) = x;
    println!("{}", a);
    println!("{}", b);

    println!("{}", x.0);
    println!("{}", x.1);
}
