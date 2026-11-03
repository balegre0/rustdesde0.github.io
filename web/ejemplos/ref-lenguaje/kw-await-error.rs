async fn dato() -> u32 { 1 }

fn main() {
    // `.await` solo se puede usar dentro de código async.
    let x = dato().await;
    println!("{x}");
}
