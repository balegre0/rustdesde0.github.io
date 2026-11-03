fn con_refs(a: &mut i32, b: &mut i32) -> i32 {
    *a = 1;
    *b = 2;
    *a
}

fn main() {
    let mut x = 0;
    // Dos préstamos exclusivos del mismo valor a la vez: justo lo que `noalias` prohíbe.
    println!("{}", con_refs(&mut x, &mut x));
}
