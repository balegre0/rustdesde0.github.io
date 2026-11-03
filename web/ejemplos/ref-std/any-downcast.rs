use std::any::{Any, TypeId, type_name};

fn describir(valor: &dyn Any) -> String {
    if let Some(n) = valor.downcast_ref::<i32>() {
        format!("un i32: {n}")
    } else if let Some(s) = valor.downcast_ref::<String>() {
        format!("un String de {} bytes", s.len())
    } else {
        String::from("algo que no conozco")
    }
}

fn main() {
    let cosas: Vec<Box<dyn Any>> = vec![Box::new(5i32), Box::new(String::from("ñu")), Box::new(2.5f64)];
    for c in &cosas {
        println!("{}", describir(c.as_ref()));
    }
    println!("{}", TypeId::of::<i32>() == TypeId::of::<i32>());
    println!("{}", type_name::<Vec<Option<&str>>>());
}
