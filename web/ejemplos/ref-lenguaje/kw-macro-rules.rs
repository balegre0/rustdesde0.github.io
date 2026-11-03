// 1. `macro_rules!` define una macro por ejemplo: reglas «patrón => plantilla».
macro_rules! cuadrado {
    ($x:expr) => {
        $x * $x
    };
}

// 2. Repeticiones: `$( ... ),*` acepta cero o más elementos separados por comas.
macro_rules! suma {
    () => { 0 };
    ($($x:expr),+ $(,)?) => { 0 $(+ $x)+ };
}

fn main() {
    println!("{}", cuadrado!(7));
    println!("{}", cuadrado!(1 + 2)); // $x es una expresión completa: (1+2)*(1+2)
    println!("{}", suma!());
    println!("{}", suma!(1, 2, 3,));
    // 3. Fuera de `macro_rules!`, la palabra es un identificador normal.
    let macro_rules = "solo un nombre";
    println!("{macro_rules}");
}
