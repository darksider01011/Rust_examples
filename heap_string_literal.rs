fn main() {
   // store string literal on heap date type
   let mut var = String::from("hello");
   
   var.push_str(", world");
   println!("{var}")

}
