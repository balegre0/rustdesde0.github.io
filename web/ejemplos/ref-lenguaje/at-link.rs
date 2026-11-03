// Llamar a una función de Windows (kernel32.dll) con otro nombre en Rust.
#[link(name = "kernel32")]
unsafe extern "system" {
    #[link_name = "GetTickCount64"]
    safe fn milisegundos_desde_arranque() -> u64;
}

fn main() {
    println!("el sistema lleva {} ms encendido", milisegundos_desde_arranque());
}
