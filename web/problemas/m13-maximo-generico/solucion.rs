fn maximo<T: PartialOrd>(xs: &[T]) -> Option<&T> {
    let mut mejor = xs.first()?;
    for x in xs {
        if x > mejor {
            mejor = x;
        }
    }
    Some(mejor)
}
