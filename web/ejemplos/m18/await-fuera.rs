async fn tarea() -> u8 { 1 }

fn main() {
    let x = tarea().await;
    println!("{x}");
}
