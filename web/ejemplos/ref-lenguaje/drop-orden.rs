struct Ruidoso(&'static str);

impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("  destruyo {}", self.0);
    }
}

struct Par { _primero: Ruidoso, _segundo: Ruidoso }

fn crear(n: &'static str) -> Ruidoso { Ruidoso(n) }

fn main() {
    println!("1. variables: en orden inverso de declaración");
    {
        let _a = Ruidoso("a");
        let _b = Ruidoso("b");
    }
    println!("2. campos: en orden de declaración");
    {
        let _p = Par { _primero: Ruidoso("primero"), _segundo: Ruidoso("segundo") };
    }
    println!("3. `let _ =` no enlaza: el temporal se destruye en el acto");
    {
        let _ = crear("temporal");
        println!("  (fin del bloque)");
    }
    println!("4. un valor movido no se destruye dos veces");
    {
        let x = Ruidoso("x");
        let y = x;
        println!("  movido a y");
        drop(y);
        println!("  después de drop(y)");
    }
    println!("5. elementos de un array: en orden");
    {
        let _v = [Ruidoso("v0"), Ruidoso("v1")];
    }
    println!("6. mem::forget: nunca se destruye (fuga segura)");
    std::mem::forget(Ruidoso("olvidado"));
    println!("fin");
}
