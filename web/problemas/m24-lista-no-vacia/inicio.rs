struct NoVacio<T> {
    cabeza: T,
    cola: Vec<T>,
}

impl<T> NoVacio<T> {
    fn desde(v: Vec<T>) -> Option<NoVacio<T>> {
        todo!()
    }

    fn len(&self) -> usize {
        1 + self.cola.len()
    }
}

impl<T: PartialOrd> NoVacio<T> {
    fn minimo(&self) -> &T {
        todo!()
    }
}

fn media(lista: &NoVacio<f64>) -> f64 {
    todo!()
}
