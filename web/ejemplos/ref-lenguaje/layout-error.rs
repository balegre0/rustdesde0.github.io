// `repr(transparent)` exige exactamente un campo que no sea de tamaño cero.
#[repr(transparent)]
struct Par(u32, u32);

fn main() {}
