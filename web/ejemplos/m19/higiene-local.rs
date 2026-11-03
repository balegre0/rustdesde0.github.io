macro_rules! mas_uno {
    () => { y + 1 };
}

fn main() {
    let y = 10;
    println!("{}", mas_uno!());
}
