// Un `dyn Forma` construido a mano: puntero a datos + puntero a una tabla de funciones.
struct Circulo(f64);
struct Cuadrado(f64);

// 1. La «vtable»: lo mismo que genera el compilador (tamaño, alineamiento y un puntero por método)
struct VTable {
    tamano: usize,
    alineamiento: usize,
    area: fn(*const ()) -> f64,
    nombre: fn(*const ()) -> &'static str,
}

fn area_circulo(p: *const ()) -> f64 { let c = unsafe { &*(p as *const Circulo) }; 3.0 * c.0 * c.0 }
fn nombre_circulo(_: *const ()) -> &'static str { "círculo" }
fn area_cuadrado(p: *const ()) -> f64 { let c = unsafe { &*(p as *const Cuadrado) }; c.0 * c.0 }
fn nombre_cuadrado(_: *const ()) -> &'static str { "cuadrado" }

static VT_CIRCULO: VTable = VTable { tamano: std::mem::size_of::<Circulo>(), alineamiento: std::mem::align_of::<Circulo>(), area: area_circulo, nombre: nombre_circulo };
static VT_CUADRADO: VTable = VTable { tamano: std::mem::size_of::<Cuadrado>(), alineamiento: std::mem::align_of::<Cuadrado>(), area: area_cuadrado, nombre: nombre_cuadrado };

// 2. El «puntero gordo»: dos palabras
struct DynForma { datos: *const (), vtable: &'static VTable }

fn main() {
    let c = Circulo(1.0);
    let q = Cuadrado(2.0);
    let formas = [
        DynForma { datos: &c as *const Circulo as *const (), vtable: &VT_CIRCULO },
        DynForma { datos: &q as *const Cuadrado as *const (), vtable: &VT_CUADRADO },
    ];
    // 3. Llamada dinámica: leer el puntero a función de la tabla y saltar a él
    for f in &formas {
        println!("{:<9} área {:>4}  ({} bytes, alineamiento {})",
            (f.vtable.nombre)(f.datos), (f.vtable.area)(f.datos), f.vtable.tamano, f.vtable.alineamiento);
    }
    // 4. Comparación con el de verdad: un &dyn mide dos punteros
    println!("size_of::<&dyn Fn()>() = {}", std::mem::size_of::<&dyn Fn()>());
}
