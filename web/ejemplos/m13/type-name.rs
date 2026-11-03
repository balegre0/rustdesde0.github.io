use std::any::type_name;

fn que_soy<T>(_x: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    println!("{}", que_soy(7));
    println!("{}", que_soy(7u8));
    println!("{}", que_soy("siete"));
}
