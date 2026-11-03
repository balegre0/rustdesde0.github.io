use std::pin::Pin;

fn main() {
    // 1. Box::pin pone el valor en el heap y promete no moverlo nunca de ahí.
    let fijado: Pin<Box<i32>> = Box::pin(41);
    // 2. Se puede leer como siempre (Pin implementa Deref).
    println!("valor = {}", *fijado + 1);
    // 3. i32 es Unpin: moverlo no rompe nada, así que se puede «desfijar».
    let caja: Box<i32> = Pin::into_inner(fijado);
    println!("de vuelta en una Box: {caja}");
    // 4. Para tipos Unpin, Pin::new funciona sobre cualquier &mut.
    let mut x = 5;
    let mut p = Pin::new(&mut x);
    *p += 1;
    println!("x = {x}");
}
