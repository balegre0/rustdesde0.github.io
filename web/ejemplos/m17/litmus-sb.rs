// Prueba de tornasol (litmus test) «store buffering»:
//   Hilo 1: x = 1; r1 = y;        Hilo 2: y = 1; r2 = x;
// Enumeramos TODAS las intercalaciones como si la memoria fuera
// una sola y cada paso ocurriera de uno en uno (consistencia secuencial).

fn generar(q: [usize; 2], act: &mut Vec<usize>, todas: &mut Vec<Vec<usize>>) {
    if q == [0, 0] {
        todas.push(act.clone());
        return;
    }
    for h in 0..2 {
        if q[h] > 0 {
            let mut q2 = q;
            q2[h] -= 1;
            act.push(h);
            generar(q2, act, todas);
            act.pop();
        }
    }
}

fn main() {
    let mut todas = Vec::new();
    generar([2, 2], &mut Vec::new(), &mut todas);

    let mut vistos = Vec::new();
    for orden in &todas {
        let (mut x, mut y, mut r1, mut r2) = (0, 0, 0, 0);
        let mut paso = [0, 0];
        let mut texto = Vec::new();
        for &h in orden {
            match (h, paso[h]) {
                (0, 0) => { x = 1; texto.push("x=1"); }
                (0, _) => { r1 = y; texto.push("r1=y"); }
                (1, 0) => { y = 1; texto.push("y=1"); }
                (_, _) => { r2 = x; texto.push("r2=x"); }
            }
            paso[h] += 1;
        }
        println!("{:<24} -> (r1, r2) = ({r1}, {r2})", texto.join("; "));
        if !vistos.contains(&(r1, r2)) {
            vistos.push((r1, r2));
        }
    }
    vistos.sort();
    println!("resultados posibles: {vistos:?}");
    println!("¿aparece (0, 0)? {}", vistos.contains(&(0, 0)));
}
