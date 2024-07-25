use super::bitstream::Stream;

fn read_string(stream: &mut (impl Stream + ?Sized), bytes: Option<usize>) -> String {
    if bytes.is_some() && bytes.unwrap() == 0 {
        return String::from("");
    }

    let mut i = 0;
    let mut chars: Vec<u8> = vec![];
    let mut append = true;
    let fixed_length = bytes.unwrap_or(0) != 0;

    let real_length = bytes.unwrap_or_else(|| stream.byte_length() * 8 - stream.get_index());

    while i < real_length {
        let c = stream.read_uint8();

        // Stop appending chars once we hit null char
        if c == 0 {
            append = false;
            if !fixed_length {
                break;
            }
        }

        if append {
            chars.push(c);
        }

        i += 1;
    }

    String::from_utf8(chars).unwrap()
}

fn string_to_byte_array(string: &str) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];

    for char in string.chars() {
        let unicode = char as u128;
        match unicode {
            0..=0x7F => res.push(unicode as u8),
            0x80..=0x7FF => {
                res.push(((unicode >> 6) | 0xC0) as u8);
                res.push(((unicode & 0x3F) | 0x80) as u8);
            }
            0x800..=0x7FFFF => {
                res.push(((unicode >> 12) | 0xE0) as u8);
                res.push((((unicode >> 6) & 0x3F) | 0x80) as u8);
                res.push(((unicode & 0x3F) | 0x80) as u8);
            }
            _ => {
                res.push(((unicode >> 18) | 0xF0) as u8);
                res.push((((unicode >> 12) & 0x3F) | 0x80) as u8);
                res.push((((unicode >> 6) & 0x3F) | 0x80) as u8);
                res.push(((unicode & 0x3F) | 0x80) as u8);
            }
        }
    }

    res.clone()
}

pub fn write_utf8_string(stream: &mut (impl Stream + ?Sized), string: &str, bytes: Option<usize>) {
    let byte_array = string_to_byte_array(string);
    let byte_len = byte_array.len();
    let length = bytes.unwrap_or(byte_len + 1); // +1 for null char

    for i in 0..length {
        stream.write_uint8(if i < byte_len { byte_array[i] } else { 0 });
    }
}

pub fn read_utf8_string(stream: &mut (impl Stream + ?Sized), bytes: Option<usize>) -> String {
    read_string(stream, bytes)
}

pub fn write_ascii_string(stream: &mut (impl Stream + ?Sized), string: &str, bytes: Option<usize>) {
    assert!(string.is_ascii(), "String must be ASCII-only");

    let str_len = string.len();
    let length = bytes.unwrap_or(str_len + 1); // +1 for null char
    let chars = string.chars().collect::<Vec<_>>();

    for i in 0..length {
        stream.write_uint8(if i < str_len {
            (*chars.get(i).unwrap()) as u8
        } else {
            0
        });
    }
}

pub fn read_ascii_string(stream: &mut (impl Stream + ?Sized), bytes: Option<usize>) -> String {
    let str = read_string(stream, bytes);
    assert!(str.is_ascii(), "String read from stream is not ASCII-only");
    str
}