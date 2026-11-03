fn poner(reg: u32, bit: u32) -> u32 {
    reg | (1 << bit)
}
fn quitar(reg: u32, bit: u32) -> u32 {
    reg & !(1 << bit)
}
fn alternar(reg: u32, bit: u32) -> u32 {
    reg ^ (1 << bit)
}
fn leer(reg: u32, bit: u32) -> bool {
    reg & (1 << bit) != 0
}
fn campo(reg: u32, desde: u32, ancho: u32) -> u32 {
    (reg >> desde) & ((1 << ancho) - 1)
}
