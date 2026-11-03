use std::mem::size_of;

// 1. Recibir &[T] en vez de &Vec<T>: acepta Vec, arrays y trozos.
fn maximo(datos: &[i32]) -> Option<i32> {
    let mut max = *datos.first()?;
    for &d in datos {
        if d > max {
            max = d;
        }
    }
    Some(max)
}

// 2. &mut [T]: cambiar los elementos (pero no añadir ni quitar).
fn duplicar(datos: &mut [i32]) {
    for d in datos.iter_mut() {
        *d *= 2;
    }
}

fn main() {
    let mut v = vec![4, 9, 2, 7, 5];
    let arr = [3, 8, 1];

    // 3. Rangos: [a..b] desde a hasta b (sin incluirlo), [..b], [a..], [..].
    println!("v[1..3] = {:?}", &v[1..3]);
    println!("v[..2] = {:?}, v[3..] = {:?}", &v[..2], &v[3..]);

    println!("máximo de v: {:?}", maximo(&v));
    println!("máximo de arr: {:?}", maximo(&arr));
    println!("máximo de v[2..]: {:?}", maximo(&v[2..]));
    println!("máximo de nada: {:?}", maximo(&[]));

    duplicar(&mut v[..2]);
    println!("tras duplicar los dos primeros: {:?}", v);

    // 4. Métodos de slice: dividir, trocear, ventanas.
    let (izq, der) = v.split_at(2);
    println!("split_at(2): {:?} | {:?}", izq, der);
    for trozo in v.chunks(2) {
        print!("{:?} ", trozo);
    }
    println!();
    for ventana in arr.windows(2) {
        print!("{:?} ", ventana);
    }
    println!();

    // 5. Un slice es un puntero GORDO: dirección + longitud.
    println!("size_of::<&[i32]>() = {} bytes, size_of::<&i32>() = {} bytes", size_of::<&[i32]>(), size_of::<&i32>());
}
