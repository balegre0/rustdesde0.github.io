fn main() {
    let total = 2 + 2;
    assert!(total > 3);
    assert!(total % 2 == 0, "el total debería ser par y es {total}");
    assert_eq!(total, 4);
    assert_ne!(total, 5, "no puede ser cinco");
    debug_assert!(total < 100); // solo se comprueba en debug
    println!("todas las comprobaciones pasaron");
    assert_eq!(total, 5, "suma mal hecha");
}
