fn main() {
    // 1. Macros que «miran» el código fuente
    println!("archivo terminado en .rs: {}", file!().ends_with(".rs"));
    println!("estoy en la línea {}", line!());
    println!("{}", stringify!(x * (y + 1)));

    // 2. concat! pega literales EN COMPILACIÓN
    const SALUDO: &str = concat!("Hola", ", ", "mundo", '!', 2024);
    println!("{}", SALUDO);

    // 3. matches! responde sí/no a «¿encaja con este patrón?»
    let c = 'ñ';
    println!("¿es letra minúscula española? {}", matches!(c, 'a'..='z' | 'ñ' | 'á' | 'é' | 'í' | 'ó' | 'ú'));

    // 4. cfg! pregunta por la configuración de compilación
    println!("¿compilado para wasm? {}", cfg!(target_family = "wasm"));
    println!("¿con debug_assertions? {}", cfg!(debug_assertions));
}
