fn main() {
    let mut v = [1u8, 2, 3, 4, 5];
    let p = v.as_mut_ptr();
    // `copy_nonoverlapping` exige que origen y destino NO se solapen.
    // Aquí se solapan: es UB. En modo debug, std lo comprueba y entra en pánico.
    unsafe { std::ptr::copy_nonoverlapping(p, p.add(1), 3) };
    println!("{v:?}");
}
