fn main() {
    // 1. Sin `move`, la closure solo toma prestado `nombre`.
    let nombre = String::from("Ada");
    let saludar = || println!("hola, {nombre}");
    saludar();
    println!("{nombre} sigue disponible");

    // 2. Con `move`, la closure se queda con la propiedad de lo que usa.
    let datos = vec![1, 2, 3];
    let total = move || datos.iter().sum::<i32>();
    println!("suma = {}", total());
    // `datos` ya pertenece a la closure: aquí no se podría usar.

    // 3. Devolver una closure que sobrevive a la función: necesita `move`.
    fn sumador(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x + n
    }
    let mas5 = sumador(5);
    println!("{}", mas5(10));
}
