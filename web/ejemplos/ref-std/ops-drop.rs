struct Guardia {
    nombre: &'static str,
}

impl Drop for Guardia {
    fn drop(&mut self) {
        println!("  drop: {}", self.nombre);
    }
}

fn main() {
    let _a = Guardia { nombre: "a" };
    {
        let _b = Guardia { nombre: "b" };
        println!("fin del bloque");
    }
    let _c = Guardia { nombre: "c" };
    let d = Guardia { nombre: "d" };
    drop(d);
    let v = vec![Guardia { nombre: "v0" }, Guardia { nombre: "v1" }];
    println!("fin de main ({} en el vector)", v.len());
} // se destruyen en orden inverso a su creación: v (y sus elementos en orden), c, a
