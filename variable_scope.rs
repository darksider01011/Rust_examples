fn main() {
    {
        let var = "hello";
        println!("{var}");
    }
    // error[E0425]: cannot find value `var` in this scope
    println!("{var}");
}
