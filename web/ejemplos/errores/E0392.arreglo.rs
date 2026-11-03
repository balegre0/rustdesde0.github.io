use std::marker::PhantomData;

struct Id<T> {
    valor: u64,
    _tipo: PhantomData<T>, // marca de tipo sin datos: ocupa 0 bytes
}

fn main() {
    let id: Id<String> = Id { valor: 7, _tipo: PhantomData };
    println!("{}", id.valor);
}
