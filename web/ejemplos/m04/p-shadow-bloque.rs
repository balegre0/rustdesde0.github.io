fn main() {
    let x = 1;
    {
        let x = x + 10;
        println!("dentro: {x}");
    }
    println!("fuera: {x}");
}
