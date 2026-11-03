fn main() {
    let v = vec![10, 20, 30, 40, 50];
    let trozo = &v[1..4];
    println!("trozo = {trozo:?}");
    println!("empieza en v[1]: {}", std::ptr::eq(&v[1], &trozo[0]));
    println!("longitud: {}", trozo.len());
    // Una slice es un puntero ancho: dirección + longitud.
    println!("bytes de &[i32]: {}", size_of::<&[i32]>());
    println!("bytes de &i32:   {}", size_of::<&i32>());
}
