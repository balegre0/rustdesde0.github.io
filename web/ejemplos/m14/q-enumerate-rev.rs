fn main() {
    for (i, c) in ['a', 'b', 'c'].iter().enumerate().rev() {
        print!("{i}{c} ");
    }
    println!();
}
