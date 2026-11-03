// Un sistema de consultas (queries) de juguete con memoización y grafo de dependencias,
// la idea central del compilador: «calcula bajo demanda y no repitas trabajo».
use std::collections::HashMap;

struct Bd {
    fuentes: HashMap<&'static str, String>,       // entradas: el «código fuente»
    cache: HashMap<&'static str, (usize, u64)>,   // resultado + versión de la fuente usada
    version: HashMap<&'static str, u64>,
    ejecuciones: u32,
}

impl Bd {
    fn fijar_fuente(&mut self, nombre: &'static str, texto: &str) {
        self.fuentes.insert(nombre, texto.to_string());
        *self.version.entry(nombre).or_insert(0) += 1;
    }

    // 1. Query «número de líneas de un archivo»: depende de una sola entrada
    fn lineas(&mut self, nombre: &'static str) -> usize {
        let v = self.version[nombre];
        if let Some(&(res, v_cache)) = self.cache.get(nombre) {
            if v_cache == v {
                return res; // 2. VERDE: la entrada no cambió; reutilizamos
            }
        }
        // 3. ROJO: hay que recalcular
        self.ejecuciones += 1;
        let res = self.fuentes[nombre].lines().count();
        self.cache.insert(nombre, (res, v));
        res
    }

    // 4. Query derivada: suma de líneas de todo el «crate»; pide otras queries
    fn total(&mut self) -> usize {
        let nombres: Vec<_> = self.fuentes.keys().copied().collect();
        nombres.into_iter().map(|n| self.lineas(n)).sum()
    }
}

fn main() {
    let mut bd = Bd { fuentes: HashMap::new(), cache: HashMap::new(), version: HashMap::new(), ejecuciones: 0 };
    bd.fijar_fuente("main.rs", "fn main() {\n    hola();\n}");
    bd.fijar_fuente("lib.rs", "pub fn hola() {}");

    println!("total = {}  (ejecuciones: {})", bd.total(), bd.ejecuciones);
    println!("total = {}  (ejecuciones: {})", bd.total(), bd.ejecuciones);
    // 5. Editamos un solo archivo: solo se recalcula su query
    bd.fijar_fuente("lib.rs", "pub fn hola() {\n}\n");
    println!("total = {}  (ejecuciones: {})", bd.total(), bd.ejecuciones);
}
