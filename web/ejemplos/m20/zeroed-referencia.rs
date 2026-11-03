// mem::zeroed() da «todo ceros». Para un entero vale; para una referencia, NO.
fn main() {
    let n: u32 = unsafe { std::mem::zeroed() }; // bien: 0 es un u32 válido
    println!("{n}");
    let r: &i32 = unsafe { std::mem::zeroed() }; // ¡MAL! referencia nula
    println!("{r}");
}
