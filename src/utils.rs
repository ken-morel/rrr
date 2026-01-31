pub fn read_line<T>(stream: T) -> Option<String>
where
    T: std::io::Read,
{
    let mut str = String::new();
    for rbyte in stream.bytes() {
        if let Ok(byte) = rbyte {
            str.push(byte.into());
            if str.ends_with('\n') {
                str.remove(str.len() - 1);
                return Some(str);
            }
        } else {
            break;
        }
    }
    if !str.is_empty() {
        Some(str)
    } else {
        None
    }
}
