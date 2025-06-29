fn main() {
    let a = [10, 20, 30, 40, 50];

    for i in a {
        print!("{i}, ");
    }

    println!("");

    for number in (1..4){
        print!("{number}, ");
    }

    println!("");

    for number in (1..4).rev(){
        print!("{number}, ");
    }

}
