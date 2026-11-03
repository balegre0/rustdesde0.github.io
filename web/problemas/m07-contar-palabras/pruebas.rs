
fn main() {
    let mut fallos = 0;
    let mut comprobar = |nombre: &str, ok: bool, detalle: String| {
        if ok {
            println!("OK: {nombre}");
        } else {
            println!("FALLO: {nombre}: {detalle}");
            fallos += 1;
        }
    };
    let mut m = HashMap::new();
    contar("el gato y el perro", &mut m);
    let ver = |m: &HashMap<String, u32>, k: &str| m.get(k).copied().unwrap_or(0);
    comprobar("el = 2", ver(&m, "el") == 2, format!("obtuve {}", ver(&m, "el")));
    comprobar("gato = 1", ver(&m, "gato") == 1, format!("obtuve {}", ver(&m, "gato")));
    comprobar("4 palabras distintas", m.len() == 4, format!("obtuve {}", m.len()));
    contar("el   fin\nel", &mut m);
    comprobar("acumula: el = 4", ver(&m, "el") == 4, format!("obtuve {}", ver(&m, "el")));
    comprobar("fin = 1", ver(&m, "fin") == 1, format!("obtuve {}", ver(&m, "fin")));
    contar("", &mut m);
    comprobar("texto vacío no cambia nada", m.len() == 5, format!("obtuve {} claves", m.len()));
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
