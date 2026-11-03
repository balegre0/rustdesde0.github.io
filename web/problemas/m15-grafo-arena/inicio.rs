struct Estacion {
    nombre: String,
    vias: Vec<usize>,
}

struct Mapa {
    estaciones: Vec<Estacion>,
}

impl Mapa {
    fn nueva_estacion(&mut self, nombre: &str) -> usize {
        let _ = nombre;
        todo!()
    }

    fn unir(&mut self, a: usize, b: usize) {
        let _ = (a, b);
        todo!()
    }

    fn alcanzables(&self, desde: usize) -> Vec<String> {
        let _ = desde;
        todo!()
    }
}
