fn main() {
    // 1. Un entero se copia: los dos siguen siendo válidos
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    // 2. Tuplas y arrays hechos de tipos Copy también se copian
    let par = (3, true);
    let otro = par;
    let fila = [1, 2, 3];
    let otra_fila = fila;
    println!("{par:?} {otro:?} {fila:?} {otra_fila:?}");

    // 3. Un &str (una vista prestada) también es Copy
    let texto: &str = "hola";
    let t2 = texto;
    println!("{texto} {t2}");
}
