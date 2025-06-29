fn main() {
    let mut counter:i32 = 0;
    
    let result = loop {
        counter += 1;

        if counter == 10 { 
            break counter * 2;
        }
    };

    // output: 20
    println!("counter: {result}");
}
