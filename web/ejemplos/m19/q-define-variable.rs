macro_rules! define {
    ($nombre:ident) => { let $nombre = 5; };
}

fn main() {
    define!(x);
    println!("{}", x + 1);
}
