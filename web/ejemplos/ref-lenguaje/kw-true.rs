fn main() {
    let listo = true;
    println!("{}", listo as i32);
    println!("{}", !true);
    // `true || ...` no evalúa la parte derecha (cortocircuito).
    println!("{}", true || { println!("no se evalúa"); false });
    // `bool` implementa `Ord`: false < true.
    println!("{}", false < true);
    // `then_some`: Some si es true, None si es false.
    println!("{:?} {:?}", true.then_some(1), false.then_some(1));
}
