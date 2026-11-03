use std::fmt::Display;

// 1. Un genérico con un bound en línea: T debe saber mostrarse con {}.
fn anunciar<T: Display>(cosa: T) {
    println!("Atención: {cosa}");
}

// 2. Con varios bounds, `where` lo deja más legible.
fn resumen<T, U>(a: T, b: U) -> String
where
    T: Display + Clone,
    U: Display,
{
    let copia = a.clone();
    format!("{a} y {b} (copia: {copia})")
}

// 3. Un struct genérico: un par de cosas del mismo tipo.
struct Par<T> {
    a: T,
    b: T,
}

// 4. Métodos solo para los Par<T> cuyo T se puede comparar y mostrar.
impl<T: PartialOrd + Display> Par<T> {
    fn el_mayor(&self) -> &T {
        if self.a >= self.b { &self.a } else { &self.b }
    }
}

fn main() {
    anunciar(42);
    anunciar("hola");
    println!("{}", resumen(3.5, 'ñ'));

    let p = Par { a: 10, b: 7 };
    let q = Par { a: "pera", b: "uva" };
    println!("{} {}", p.el_mayor(), q.el_mayor());
}
