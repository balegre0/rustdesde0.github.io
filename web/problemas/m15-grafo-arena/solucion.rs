struct Estacion {
    nombre: String,
    vias: Vec<usize>,
}

struct Mapa {
    estaciones: Vec<Estacion>,
}

impl Mapa {
    fn nueva_estacion(&mut self, nombre: &str) -> usize {
        self.estaciones.push(Estacion { nombre: nombre.to_string(), vias: Vec::new() });
        self.estaciones.len() - 1
    }

    fn unir(&mut self, a: usize, b: usize) {
        self.estaciones[a].vias.push(b);
        self.estaciones[b].vias.push(a);
    }

    fn alcanzables(&self, desde: usize) -> Vec<String> {
        let mut visitada = vec![false; self.estaciones.len()];
        let mut pendientes = vec![desde];
        while let Some(i) = pendientes.pop() {
            if visitada[i] {
                continue;
            }
            visitada[i] = true;
            pendientes.extend(&self.estaciones[i].vias);
        }
        let mut nombres: Vec<String> = (0..self.estaciones.len())
            .filter(|&i| visitada[i])
            .map(|i| self.estaciones[i].nombre.clone())
            .collect();
        nombres.sort();
        nombres
    }
}
