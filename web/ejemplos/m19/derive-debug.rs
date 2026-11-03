// 1. #[derive(...)] pide al compilador que ESCRIBA código por ti
#[derive(Debug, Clone, PartialEq, Default)]
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let a = Punto { x: 1, y: 2 };
    // 2. Clone, PartialEq, Debug y Default no los hemos escrito: los generó derive
    let b = a.clone();
    println!("{:?}", a);
    println!("{:#?}", b);
    println!("¿iguales? {}", a == b);
    println!("por defecto: {:?}", Punto::default());
}
