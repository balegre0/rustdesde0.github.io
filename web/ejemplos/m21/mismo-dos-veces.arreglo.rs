fn con_refs(a: &mut i32, b: &mut i32) -> i32 {
    *a = 1;
    *b = 2;
    *a
}

fn main() {
    let mut x = 0;
    let mut y = 0;
    // Dos valores distintos: dos préstamos exclusivos sin solaparse.
    println!("{}", con_refs(&mut x, &mut y));
    println!("x = {x}, y = {y}");
}
