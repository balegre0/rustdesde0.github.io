fn guardar<T: 'static>(x: T) -> T { x }

fn main() {
    let local = String::from("hola");
    // `&local` es un préstamo de una variable local: no cumple `'static`.
    let r = guardar(&local);
    println!("{r}");
}
