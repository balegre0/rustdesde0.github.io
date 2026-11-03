use std::ffi::CString;

fn a_bytes_c(s: &str) -> Result<Vec<u8>, usize> {
    match CString::new(s) {
        Ok(c) => Ok(c.as_bytes_with_nul().to_vec()),
        Err(e) => Err(e.nul_position()),
    }
}
