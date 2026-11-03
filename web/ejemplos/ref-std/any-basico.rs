use std::any::{type_name, type_name_of_val, Any, TypeId};

fn describir(x: &dyn Any) -> String {
    if let Some(n) = x.downcast_ref::<i32>() {
        format!("un i32: {n}")
    } else if let Some(s) = x.downcast_ref::<String>() {
        format!("un String: {s}")
    } else {
        String::from("otra cosa")
    }
}

fn main() {
    println!("{}", describir(&5));
    println!("{}", describir(&String::from("hola")));
    println!("{}", describir(&1.5));
    println!("{}", TypeId::of::<u8>() == TypeId::of::<u8>());
    println!("{}", type_name::<Vec<Option<char>>>());
    println!("{}", type_name_of_val(&2.0f32));
    let cosas: Vec<Box<dyn Any>> = vec![Box::new(1), Box::new('x')];
    println!("{}", cosas.iter().filter(|c| c.is::<i32>()).count());
}
