/// Un vector que, por construcción, nunca está vacío.
struct NoVacio<T> {
    primero: T,
    resto: Vec<T>,
}

impl<T> NoVacio<T> {
    fn nuevo(primero: T) -> Self {
        NoVacio { primero, resto: Vec::new() }
    }

    /// Convierte un Vec: solo puede fallar aquí, una vez, si está vacío.
    fn desde_vec(mut v: Vec<T>) -> Option<Self> {
        if v.is_empty() {
            return None;
        }
        let primero = v.remove(0);
        Some(NoVacio { primero, resto: v })
    }

    fn push(&mut self, x: T) {
        self.resto.push(x);
    }

    /// No devuelve Option: el tipo garantiza que hay un primer elemento.
    fn primero(&self) -> &T {
        &self.primero
    }

    fn len(&self) -> usize {
        1 + self.resto.len()
    }
}

impl<T: PartialOrd> NoVacio<T> {
    /// Tampoco necesita Option: siempre hay un máximo.
    fn maximo(&self) -> &T {
        let mut m = &self.primero;
        for x in &self.resto {
            if x > m {
                m = x;
            }
        }
        m
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
