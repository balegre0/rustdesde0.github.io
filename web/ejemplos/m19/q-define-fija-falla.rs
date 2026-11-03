macro_rules! define_fija {
    () => { let fija = 7; };
}

fn main() {
    define_fija!();
    println!("{}", fija);
}
