/// Un vector que, por construcción, nunca está vacío.
/// Diseña el struct para que sea IMPOSIBLE representar un NoVacio sin elementos.
/// Pista: guarda el primer elemento aparte del resto.
struct NoVacio<T> {
    // completa los campos
    _quitar: Vec<T>,
}

impl<T> NoVacio<T> {
    fn nuevo(primero: T) -> Self {
        todo!()
    }

    /// Convierte un Vec: solo puede fallar aquí, una vez, si está vacío.
    fn desde_vec(v: Vec<T>) -> Option<Self> {
        todo!()
    }

    fn push(&mut self, x: T) {
        todo!()
    }

    /// No devuelve Option: el tipo garantiza que hay un primer elemento.
    fn primero(&self) -> &T {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }
}

impl<T: PartialOrd> NoVacio<T> {
    /// Tampoco necesita Option: siempre hay un máximo.
    fn maximo(&self) -> &T {
        todo!()
    }
}

fn main() {
    let mut notas = NoVacio::nuevo(7);
    notas.push(9);
    notas.push(4);
    println!("primero {} · máximo {} · {} notas", notas.primero(), notas.maximo(), notas.len());
    println!("{}", NoVacio::desde_vec(Vec::<i32>::new()).is_none());
    let palabras = NoVacio::desde_vec(vec!["luna", "sol"]).unwrap();
    println!("{} {}", palabras.primero(), palabras.maximo());
}
