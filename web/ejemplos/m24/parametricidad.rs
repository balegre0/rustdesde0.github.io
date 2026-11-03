// Solo hay UNA función total con esta firma: no sabe nada de T, así que solo puede devolver x.
fn identidad<T>(x: T) -> T {
    x
}

// Con esta firma, solo puede elegir uno de los dos (o el mismo siempre): no puede inventar un T.
fn elegir<T>(a: T, b: T, primero: bool) -> T {
    if primero { a } else { b }
}

// Esta no puede mirar los elementos: solo reordenarlos, duplicarlos o quitarlos.
// Por eso, para cualquier f, «aplicar f y luego invertir» = «invertir y luego aplicar f».
fn invertir<T>(v: Vec<T>) -> Vec<T> {
    v.into_iter().rev().collect()
}

fn main() {
    println!("{} {}", identidad(5), identidad("cinco"));
    println!("{}", elegir('a', 'b', false));
    let v = vec![1, 2, 3];
    let f = |x: i32| x * 10;
    let a: Vec<i32> = invertir(v.clone()).into_iter().map(f).collect();
    let b: Vec<i32> = invertir(v.into_iter().map(f).collect());
    println!("{a:?} == {b:?}: {}", a == b);
}
