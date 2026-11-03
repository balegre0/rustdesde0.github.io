fn main() {
    let notas = [7, 9, 4];
    let primera = notas.first();
    let decima = notas.get(10);
    println!("{:?} {:?}", primera, decima);
    println!("{}", decima.unwrap_or(&0));
}
