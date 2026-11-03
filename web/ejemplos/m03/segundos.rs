fn main() {
    let total = 3725;
    let horas = total / 3600;
    let minutos = total % 3600 / 60;
    let segundos = total % 60;
    println!("{total} s = {horas} h {minutos} min {segundos} s");
}
