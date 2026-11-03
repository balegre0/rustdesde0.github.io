// Simulamos DOS hilos que hacen `x += 1` sobre la misma variable.
// Para la CPU, `x += 1` son tres pasos: leer, sumar y escribir.
// Probamos TODOS los órdenes posibles en que se pueden mezclar.

#[derive(Clone, Copy)]
enum Paso {
    Leer,
    Sumar,
    Escribir,
}

const PASOS: [Paso; 3] = [Paso::Leer, Paso::Sumar, Paso::Escribir];

// Ejecuta una intercalación: `orden[i]` dice qué hilo (0 o 1) da el paso i.
fn ejecutar(orden: &[usize]) -> i32 {
    let mut x = 0; // la memoria compartida
    let mut tmp = [0, 0]; // el «registro» privado de cada hilo
    let mut siguiente = [0, 0]; // qué paso le toca a cada hilo
    for &h in orden {
        match PASOS[siguiente[h]] {
            Paso::Leer => tmp[h] = x,
            Paso::Sumar => tmp[h] += 1,
            Paso::Escribir => x = tmp[h],
        }
        siguiente[h] += 1;
    }
    x
}

// Genera todas las intercalaciones de 3 pasos del hilo 0 y 3 del hilo 1.
fn generar(quedan: [usize; 2], actual: &mut Vec<usize>, todas: &mut Vec<Vec<usize>>) {
    if quedan == [0, 0] {
        todas.push(actual.clone());
        return;
    }
    for h in 0..2 {
        if quedan[h] > 0 {
            let mut q = quedan;
            q[h] -= 1;
            actual.push(h);
            generar(q, actual, todas);
            actual.pop();
        }
    }
}

fn main() {
    let mut todas = Vec::new();
    generar([3, 3], &mut Vec::new(), &mut todas);

    let mut con_2 = 0;
    let mut con_1 = 0;
    let mut ejemplo_malo = None;
    for orden in &todas {
        match ejecutar(orden) {
            2 => con_2 += 1,
            1 => {
                con_1 += 1;
                ejemplo_malo.get_or_insert(orden.clone());
            }
            otro => println!("¿{otro}?"),
        }
    }

    println!("intercalaciones posibles: {}", todas.len());
    println!("x = 2 (correcto): {con_2}");
    println!("x = 1 (se pierde una suma): {con_1}");
    let malo: Vec<String> = ejemplo_malo.unwrap().iter().map(|h| format!("H{}", h + 1)).collect();
    println!("ejemplo que pierde: {}", malo.join(" "));
}
