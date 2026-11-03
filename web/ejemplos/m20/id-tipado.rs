// Identificadores que no se pueden mezclar: Id<Usuario> ≠ Id<Pedido>.
use std::marker::PhantomData;

struct Usuario;
struct Pedido;

struct Id<T> {
    numero: u32,
    _tipo: PhantomData<T>,
}

impl<T> Id<T> {
    fn new(numero: u32) -> Self {
        Id { numero, _tipo: PhantomData }
    }
}

fn nombre_de(id: &Id<Usuario>) -> String {
    format!("usuario #{}", id.numero)
}

fn main() {
    let u: Id<Usuario> = Id::new(7);
    let p: Id<Pedido> = Id::new(7);
    println!("{}", nombre_de(&u));
    // nombre_de(&p); // no debe compilar
    println!("pedido {} pesa {} bytes", p.numero, size_of::<Id<Pedido>>());
}
