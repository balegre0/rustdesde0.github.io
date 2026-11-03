// Devuelve un préstamo exclusivo del primer elemento y otro del resto.
// Usa split_at_mut (sin unsafe). Si el slice está vacío, devuelve None.
fn primero_y_resto(s: &mut [i32]) -> Option<(&mut i32, &mut [i32])> {
    if s.is_empty() {
        return None;
    }
    let (a, b) = s.split_at_mut(1);
    Some((&mut a[0], b))
}

fn main() {
    let mut v = [1, 2, 3];
    if let Some((p, resto)) = primero_y_resto(&mut v) {
        for x in resto.iter_mut() {
            *x += *p;
        }
        *p = 0;
    }
    println!("{v:?}");
    let mut vacio: [i32; 0] = [];
    println!("{}", primero_y_resto(&mut vacio).is_none());
}
