pub fn read_line<T>(stream: T) -> Option<Vec<u8>>
where
    T: std::io::Read,
{
    //PERF: Something like this
    // let bytes: Vec<u8> = stream.bytes().flatten().take_while(|i| *i != 10).collect();
    let mut bytes = Vec::new();
    let mut empty = true;
    for b in stream.bytes() {
        empty = false;
        let b = b.ok()?;
        if b == 10 {
            break;
        } else {
            bytes.push(b);
        }
    }

    if !empty { Some(bytes) } else { None }
}
