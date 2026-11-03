fn main() {
    let x = 5;
    let x = x * 2;
    {
        let x = x + 100;
        println!("dentro: {x}");
    }
    println!("fuera: {x}");
}
