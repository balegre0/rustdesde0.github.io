fn main() {
    let puntos = [(0, 0), (3, 0), (0, -2), (5, 5), (2, 7)];
    for p in puntos {
        let texto = match p {
            (0, 0) => String::from("origen"),
            (x, 0) => format!("en el eje X, x={x}"),
            (0, y) => format!("en el eje Y, y={y}"),
            (x, y) if x == y => format!("en la diagonal ({x})"),
            (x, y) => format!("en ({x}, {y})"),
        };
        println!("{texto}");
    }
}
