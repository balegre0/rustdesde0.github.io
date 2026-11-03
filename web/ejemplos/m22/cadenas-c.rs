use std::ffi::{CStr, CString, OsString};

fn main() {
    // 1. Un literal c"..." es un &CStr: bytes terminados en un 0
    let saludo: &CStr = c"hola";
    println!("{:?} -> bytes con el 0 final: {:?}", saludo, saludo.to_bytes_with_nul());

    // 2. CString es la versión con propiedad (como String para &str)
    let propia = CString::new("año").expect("sin bytes 0 dentro");
    println!("CString de \"año\": {} bytes + el 0", propia.as_bytes().len());

    // 3. Un 0 en medio no se puede representar: se cortaría ahí
    match CString::new("ho\0la") {
        Ok(_) => println!("aceptado"),
        Err(e) => println!("rechazado: {e}"),
    }

    // 4. De vuelta a texto de Rust: puede no ser UTF-8, así que devuelve Result
    println!("to_str: {:?}", propia.as_c_str().to_str());

    // 5. OsString: el texto tal como lo entiende el sistema operativo
    let os = OsString::from("archivo.txt");
    println!("OsString: {:?} -> {:?}", os, os.to_str());
}
