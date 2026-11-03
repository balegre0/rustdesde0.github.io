fn media(v: &[i32]) -> i32 {
    // Comprobación cara o «solo para desarrollo»: desaparece en release.
    debug_assert!(!v.is_empty(), "media de una lista vacía");
    debug_assert_eq!(v.len() % 1, 0);
    v.iter().sum::<i32>() / v.len().max(1) as i32
}

fn main() {
    println!("{}", media(&[4, 6, 8]));
    println!("comprobaciones activas: {}", cfg!(debug_assertions));
    println!("{}", media(&[]));
}
