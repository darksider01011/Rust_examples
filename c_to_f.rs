use std::io;

fn main() {
    println!("");
    println!("Converter");
    println!("");

    println!("Enter your option:");
    println!("(1).C to F");
    println!("(2).F to C");

    let mut sel = String::new();
    io::stdin().read_line(&mut sel).expect("Failed to read line");
    let sel:i32 = sel.trim().parse().expect("Input is not integer");

    if sel == 1 {
       println!("");
       println!("C TO F Converter");
       println!("");
       ccon();
       fn ccon(){
          println!("Enter the temp in C :");

          let mut c = String::new();
          io::stdin().read_line(&mut c).expect("Failed to read line");
          let c:f32 = c.trim().parse().expect("Input is not integer");

          let f:f32 = (c * 9.0) / 5.0 + 32.0;
          println!("f: {f}");}}

    else if sel == 2 {
       println!("");
       println!("F TO C Converter");
       println!("");
       fcon();
       fn fcon(){
          println!("Enter the temp in f :");

          let mut f = String::new();
          io::stdin().read_line(&mut f).expect("Failed to read line");
          let f:f32 = f.trim().parse().expect("Input is not integer");

          let c:f32 = (f - 32.0) * 5.0 / 9.0;
          println!("c: {c}");}}

    else {
       println!("");
       println!("Wrong option");}

}
