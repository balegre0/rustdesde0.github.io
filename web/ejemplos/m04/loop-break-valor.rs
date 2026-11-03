fn main() {
    // 1. `loop` repite para siempre... hasta que algo hace `break`
    let mut potencia = 1;
    let mut vueltas = 0;
    let resultado = loop {
        potencia *= 2;
        vueltas += 1;
        // 2. `break valor` sale del bucle y entrega `valor` como resultado del `loop`
        if potencia > 1000 {
            break potencia;
        }
    };
    println!("primera potencia de 2 mayor que 1000: {resultado}");
    println!("hicieron falta {vueltas} vueltas");
}
