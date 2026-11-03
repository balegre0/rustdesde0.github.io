use std::cell::RefCell;

fn main() {
    let puntos = RefCell::new(vec![10, 20]);
    let total: i32 = puntos.borrow().iter().sum(); // el préstamo muere al acabar la línea
    puntos.borrow_mut().push(total);
    println!("{:?}", puntos.borrow());
}
