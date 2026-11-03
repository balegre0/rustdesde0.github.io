#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
fn sumar_rapido(a: &[f32], b: &[f32], c: &mut [f32]) {
    // Aquí el compilador puede usar instrucciones AVX2.
    for i in 0..c.len() {
        c[i] = a[i] + b[i];
    }
}

fn main() {
    let a = [1.0f32; 8];
    let b = [2.0f32; 8];
    let mut c = [0.0f32; 8];
    #[cfg(target_arch = "x86_64")]
    if is_x86_feature_detected!("avx2") {
        // SAFETY: acabamos de comprobar que la CPU tiene AVX2.
        unsafe { sumar_rapido(&a, &b, &mut c) };
    }
    println!("{:?}", c);
}
