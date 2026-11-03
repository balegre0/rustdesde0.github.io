// Un tipo es un conjunto de valores. Contamos cuántos tiene cada uno.
fn main() {
    let bools = [false, true];
    println!("|bool| = {}", bools.len());

    // Producto: una tupla tiene un valor de cada componente.
    let mut pares = Vec::new();
    for a in bools {
        for b in bools {
            pares.push((a, b));
        }
    }
    println!("|(bool, bool)| = {}  {:?}", pares.len(), pares);

    // Suma: un Option<bool> es «un bool» o «nada».
    let mut opciones: Vec<Option<bool>> = vec![None];
    for b in bools {
        opciones.push(Some(b));
    }
    println!("|Option<bool>| = {}  {:?}", opciones.len(), opciones);

    // El tipo unidad tiene un único valor.
    let unidad = [()];
    println!("|()| = {}", unidad.len());

    // Bytes que ocupa cada uno: lo que hace falta para distinguir todos sus valores (o un poco más).
    println!("bytes: bool={} (bool,bool)={} Option<bool>={} ()={}",
        size_of::<bool>(), size_of::<(bool, bool)>(), size_of::<Option<bool>>(), size_of::<()>());
}
