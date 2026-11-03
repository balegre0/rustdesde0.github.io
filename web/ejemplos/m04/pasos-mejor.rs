fn main() {
    let notas = [6, 9, 4];
    let mut mejor = notas[0];
    for nota in notas {
        if nota > mejor {
            mejor = nota;
        }
    }
    println!("La mejor nota es {mejor}");
}
