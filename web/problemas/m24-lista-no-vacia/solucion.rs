struct NoVacio<T> {
    cabeza: T,
    cola: Vec<T>,
}

impl<T> NoVacio<T> {
    fn desde(mut v: Vec<T>) -> Option<NoVacio<T>> {
        if v.is_empty() {
            return None;
        }
        let cabeza = v.remove(0);
        Some(NoVacio { cabeza, cola: v })
    }

    fn len(&self) -> usize {
        1 + self.cola.len()
    }
}

impl<T: PartialOrd> NoVacio<T> {
    fn minimo(&self) -> &T {
        let mut m = &self.cabeza;
        for x in &self.cola {
            if x < m {
                m = x;
            }
        }
        m
    }
}

fn media(lista: &NoVacio<f64>) -> f64 {
    let suma: f64 = lista.cabeza + lista.cola.iter().sum::<f64>();
    suma / lista.len() as f64
}
