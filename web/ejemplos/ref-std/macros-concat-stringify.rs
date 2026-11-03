const VERSION: &str = concat!("v", 1, ".", 99, "-", true);

macro_rules! mostrar {
    ($e:expr) => {
        println!("{} = {}", stringify!($e), $e)
    };
}

fn main() {
    println!("{VERSION}");
    mostrar!(2 + 3 * 4);
    mostrar!("hola".len());
    println!("{}", stringify!(fn  main  ( ) { }));
}
