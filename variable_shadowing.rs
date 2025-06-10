fn main() {
    //shadowed by second x
    let x = 5;
    let x = x * 2;

    {
    let x = x / 2;
    }
    
    //output: 10
    println!("the value of the x: {x}")
}
