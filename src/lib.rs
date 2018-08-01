#![no_std]

pub enum ConvertError {
    // if the length of and encoded buffer isn't valid
    InvalidInputLength,

    // if the length of the output is too short
    InvalidOutputLength,

    // if the input contains invalid characters
    InvalidInput,
}

pub fn hex2bin<'a>(input: &[u8], output: &'a mut [u8]) -> Result<&'a mut [u8], ConvertError> {
    if input.len() % 2 != 0 {
        return Err(ConvertError::InvalidInputLength);
    }
    if input.len() / 2 > output.len() {
        return Err(ConvertError::InvalidOutputLength);
    }

    for block_num in 0..(input.len()/2) {
        let mut num = 0u8;
        for digit in &input[block_num*2..(block_num * 2 + 2)] {
            let val = if *digit >= b'a' && *digit <= b'f' {
                *digit - b'a' + 10
            } else if *digit >= b'A' && *digit <= b'F' {
                *digit - b'A' + 10
            } else if *digit >= b'0' && *digit <= b'9' {
                *digit - b'0'
            } else {
                // bad input
                return Err(ConvertError::InvalidInput);
            };
            num <<= 4;
            num |= val;
        }
        output[block_num] = num;
    }

    return Ok(&mut output[0..input.len()/2]);
}

pub fn bin2hex<'a>(input: &[u8], output: &'a mut [u8]) -> Result<&'a mut [u8], ConvertError> {
    const DIGITS: &[u8] = &[b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a'
        , b'b', b'c', b'd', b'e', b'f'];

    if output.len() < input.len() * 2 {
        return Err(ConvertError::InvalidOutputLength);
    }

    for idx in 0..input.len() {
        let byte = input[idx] as usize;
        output[idx * 2 + 0] = DIGITS[(byte >> 4) & 0x0f];
        output[idx * 2 + 1] = DIGITS[(byte >> 0) & 0x0f];
    }

    return Ok(&mut output[0..input.len() * 2]);
}

pub fn b64encode<'a>(input: &[u8], output: &'a mut [u8]) -> Result<&'a mut [u8], ConvertError> {
    const DIGITS: &[u8] = &[b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z',
        b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
        b'+', b'/'];
    assert_eq!(DIGITS.len(), 64);

    let data_len = input.len() * 4 / 3;
    let pad_len = if data_len % 4 != 0 {
        4 - (data_len % 4)
    } else {
        0
    };
    let required_len = data_len + pad_len;

    if required_len > output.len() {
        return Err(ConvertError::InvalidOutputLength);
    }

    for block_idx in 0..(input.len() / 3 + 1) {
        let block_end = if block_idx*3+3 > input.len() { input.len() } else { block_idx*3 + 3 };
        let block = &input[block_idx * 3..block_end];

        if block.len() == 0 {
            break;
        }

        // convert block to a u32
        let mut raw_num = 0u32;
        for i in 0..block.len() {
            raw_num |= (block[i] as u32) << (16 - (i * 8));
        }

        for i in 0..4 {
            let di = (raw_num >> (18 - (6 * i))) & 0b111111;

            output[block_idx * 4 + i] = DIGITS[di as usize];
        }
    }

    for ch in &mut output[data_len + 1..] {
        *ch = b'=';
    }

    return Ok(&mut output[0..required_len]);
}

pub fn b64decode<'a>(input: &[u8], output: &'a mut [u8]) -> Result<&'a mut [u8], ConvertError> {
    if input.len() % 4 != 0 {
        return Err(ConvertError::InvalidInputLength);
    }

    let mut output_length = input.len() / 4 * 3;

    if output_length > output.len() {
        return Err(ConvertError::InvalidOutputLength);
    }

    for block_idx in 0..(input.len() / 3) {
        let block = &input[block_idx * 4..(block_idx * 4 + 4)];

        let mut num = 0u32;

        for i in 0..4 {
            let ch = block[i];
            if ch == b'=' {
                if i < 2 {
                    return Err(ConvertError::InvalidInput);
                }

                output_length = block_idx * 3 + i - 1;
                break;
            }

            let c_val = if ch >= b'A' && ch <= b'Z' {
                ch - b'A'
            } else if ch >= b'a' && ch <= b'z' {
                ch - b'a' + 26
            } else if ch >= b'0' && ch <= b'9' {
                ch - b'0' + 52
            } else if ch == b'+' {
                62
            } else if ch == b'/' {
                63
            } else {
                return Err(ConvertError::InvalidInput);
            };
            num |= (c_val as u32) << (26 - 6 * i);
        }

        for i in 0..3 {
            output[block_idx * 3 + i] = ((num >> (24 - i * 8)) & 0xff) as u8;
        }
    }

    return Ok(&mut output[0..output_length]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex2bin() {
        let mut output_buffer = [0u8; 100];
        let input = "1f2F3d4d".as_bytes();

        // check good case
        assert_eq!(hex2bin(&input, &mut output_buffer).ok().unwrap(), &[0x1f, 0x2f, 0x3d, 0x4d]);

        // check bad input
        assert!(hex2bin("z1".as_bytes(), &mut output_buffer).is_err());

        // check short output buffer
        assert!(hex2bin("a1a2a3a4".as_bytes(), &mut output_buffer[0..2]).is_err());

        // check odd input
        assert!(hex2bin("a".as_bytes(), &mut output_buffer).is_err());
    }

    #[test]
    fn test_bin2hex() {
        let mut buffer = [0u8; 200];

        // normal use
        assert_eq!(bin2hex(&[0x1f, 0xf2], &mut buffer).ok().unwrap(), "1ff2".as_bytes());

        // short buffer
        assert!(bin2hex(&[0x1f, 0xf2], &mut buffer[0..2]).is_err());
    }
}