// T puede ser cualquier tipo que se pueda comparar (PartialOrd) y copiar (Copy).
fn mayor<T: PartialOrd + Copy>(lista: &[T]) -> T {
    let mut el_mayor = lista[0];
    for &x in lista {
        if x > el_mayor {
            el_mayor = x;
        }
    }
    el_mayor
}

fn main() {
    println!("{}", mayor(&[3, 9, 2]));
    println!("{}", mayor(&[1.5, -2.0]));
    println!("{}", mayor(&['r', 'u', 's', 't']));
}
