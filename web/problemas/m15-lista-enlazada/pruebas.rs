
fn prueba_lista(v: &[i32]) -> Lista {
    let mut l = Lista::Vacia;
    for &x in v.iter().rev() {
        l = Lista::Nodo(x, Box::new(l));
    }
    l
}

fn main() {
    let casos: [(&[i32], usize, Option<i32>); 5] = [
        (&[1, 5, 3], 3, Some(5)),
        (&[], 0, None),
        (&[-4, -9], 2, Some(-4)),
        (&[7], 1, Some(7)),
        (&[2, 2, 8, 1, 8], 5, Some(8)),
    ];
    let mut fallos = 0;
    for (v, largo, max) in casos {
        let l = prueba_lista(v);
        let (r1, r2) = (longitud(&l), maximo(&l));
        if r1 == largo && r2 == max {
            println!("OK: {v:?} → longitud = {r1}, maximo = {r2:?}");
        } else {
            println!("FALLO: {v:?} → longitud = {r1}, maximo = {r2:?}; se esperaba {largo} y {max:?}");
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
