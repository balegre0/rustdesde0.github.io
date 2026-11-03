use std::fmt::Display;

fn longitud(s: &str) -> usize { s.len() }
fn suma(v: &[i32]) -> i32 { v.iter().sum() }

trait Animal { fn nombre(&self) -> &str; }
trait Perro: Animal {}
struct Toby;
impl Animal for Toby { fn nombre(&self) -> &str { "Toby" } }
impl Perro for Toby {}

fn main() {
    // 1. Deref: &String -> &str, &Box<i32> -> &i32 (al pasar un argumento).
    let s = String::from("hola");
    println!("{}", longitud(&s));
    // 2. Unsize: &[i32; 3] -> &[i32] y &T -> &dyn Trait.
    let a = [1, 2, 3];
    println!("{}", suma(&a));
    let d: &dyn Display = &42;
    println!("{d}");
    // 3. &mut T -> &T.
    let mut n = 5;
    let r: &i32 = &mut n;
    println!("{r}");
    // 4. Item de función y closure sin capturas -> puntero a función.
    fn doble(x: i32) -> i32 { x * 2 }
    let fs: [fn(i32) -> i32; 2] = [doble, |x| x + 1];
    println!("{} {}", fs[0](10), fs[1](10));
    // 5. Conversión hacia el supertrait (desde 1.86).
    let p: &dyn Perro = &Toby;
    let an: &dyn Animal = p;
    println!("{}", an.nombre());
    // 6. `!` se convierte en cualquier tipo.
    let v: i32 = if n > 0 { 1 } else { panic!("nunca") };
    println!("{v}");
    // 7. Referencia -> puntero crudo.
    let ptr: *const i32 = &n;
    println!("{}", !ptr.is_null());
}
