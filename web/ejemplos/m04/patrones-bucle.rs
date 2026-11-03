fn main() {
    let notas = [7, 4, 9, 5, 10, 3, 8];

    // 1. Contar: cuántas cumplen algo.
    let mut aprobadas = 0;
    for n in notas {
        if n >= 5 {
            aprobadas += 1;
        }
    }

    // 2. Acumular: sumar (o multiplicar, o juntar texto).
    let mut suma = 0;
    for n in notas {
        suma += n;
    }

    // 3. El mejor hasta ahora: empezar con el primero y comparar.
    let mut maxima = notas[0];
    let mut posicion = 0;
    for (i, n) in notas.iter().enumerate() {
        if *n > maxima {
            maxima = *n;
            posicion = i;
        }
    }

    // 4. Buscar y parar: loop con break devolviendo el valor.
    let mut i = 0;
    let primer_suspenso = loop {
        if i == notas.len() {
            break None;
        }
        if notas[i] < 5 {
            break Some(i);
        }
        i += 1;
    };

    println!("aprobadas: {aprobadas} de {}", notas.len());
    println!("media: {:.2}", suma as f64 / notas.len() as f64);
    println!("máxima: {maxima} (posición {posicion})");
    println!("primer suspenso en la posición: {:?}", primer_suspenso);
}
