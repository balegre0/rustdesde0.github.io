// Completa `primer_par` usando `let else`: si `buscar_par` da None, devuelve
// el texto "sin pares"; si da Some(n), devuelve "primer par: n".

fn buscar_par(v: &[i32]) -> Option<i32> {
    for &x in v {
        if x % 2 == 0 {
            return Some(x);
        }
    }
    None
}

fn primer_par(v: &[i32]) -> String {
    todo!()
}

fn main() {
    println!("{}", primer_par(&[3, 7, 10, 4]));
    println!("{}", primer_par(&[1, 5]));
}
