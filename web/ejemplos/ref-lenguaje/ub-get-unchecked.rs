fn main() {
    let v = [10, 20, 30];
    let i = std::hint::black_box(7);
    // `get_unchecked` no comprueba el índice: con i fuera de rango es UB.
    let x = unsafe { *v.get_unchecked(i) };
    println!("{x}");
}
