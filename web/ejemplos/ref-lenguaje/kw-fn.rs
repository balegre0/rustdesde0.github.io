// 1. Una función con parámetros tipados y tipo de retorno.
fn sumar(a: i32, b: i32) -> i32 {
    a + b // la última expresión, sin `;`, es el valor devuelto
}

// 2. Sin `-> Tipo`, la función devuelve `()`.
fn saludar(nombre: &str) {
    println!("hola, {nombre}");
}

// 3. Genérica: funciona con cualquier tipo que se pueda mostrar.
fn mostrar_dos<T: std::fmt::Display>(x: T, y: T) {
    println!("{x} y {y}");
}

// 4. `fn` como tipo: puntero a función.
fn aplicar(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn doble(x: i32) -> i32 { x * 2 }

fn main() {
    println!("{}", sumar(2, 3));
    saludar("Ana");
    mostrar_dos(1.5, 2.5);
    println!("{}", aplicar(doble, 21));
    // 5. Una función anidada: solo visible dentro de `main`.
    fn interna() -> &'static str { "interna" }
    println!("{}", interna());
}
