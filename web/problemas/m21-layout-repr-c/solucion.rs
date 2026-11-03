fn redondear(x: usize, a: usize) -> usize {
    (x + a - 1) / a * a
}

pub fn layout_c(campos: &[(usize, usize)]) -> (Vec<usize>, usize, usize) {
    let mut desplazamientos = Vec::new();
    let mut pos = 0;
    let mut alin = 1;
    for &(tam, a) in campos {
        pos = redondear(pos, a);
        desplazamientos.push(pos);
        pos += tam;
        alin = alin.max(a);
    }
    (desplazamientos, redondear(pos, alin), alin)
}
