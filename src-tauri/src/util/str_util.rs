pub fn load_wstring_vec(vec: &[u16]) -> String {
    match vec.iter().position(|&c| c == 0) {
        Some(null_pos) => String::from_utf16_lossy(&vec[..null_pos]),
        None => String::from_utf16_lossy(vec),
    }
}

pub fn dump_wstring_vec(str: &str) -> Vec<u16> {
    str.encode_utf16().chain(std::iter::once(0)).collect()
}
