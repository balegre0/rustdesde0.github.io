// Un mini-lenguaje: `pasos!(sube 3 baja 1 sube 2)` calcula la altura final.
macro_rules! pasos {
    // 1. Regla interna: no quedan tokens -> devolvemos el acumulador
    (@acc $acc:expr;) => { $acc };
    // 2. Regla interna: «sube N» -> sumamos y seguimos masticando
    (@acc $acc:expr; sube $n:literal $($resto:tt)*) => {
        pasos!(@acc ($acc + $n); $($resto)*)
    };
    // 3. Regla interna: «baja N» -> restamos y seguimos
    (@acc $acc:expr; baja $n:literal $($resto:tt)*) => {
        pasos!(@acc ($acc - $n); $($resto)*)
    };
    // 4. Punto de entrada (va el último: las reglas @acc deben probarse antes)
    ($($todo:tt)*) => { pasos!(@acc 0; $($todo)*) };
}

fn main() {
    println!("{}", pasos!(sube 3 baja 1 sube 2));
    println!("{}", pasos!());
    println!("{}", pasos!(baja 5));
}
