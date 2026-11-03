fn main() {
    let mut nivel = 1;
    let r = &nivel;
    println!("{r}"); // último uso del préstamo
    nivel = 2;
    println!("{nivel}");
}
