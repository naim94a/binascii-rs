#![no_std]

pub fn hex2bin<'a>(input: &[u8], output: &'a mut [u8]) -> Result<&'a mut [u8], ()> {
    if input.len() % 2 != 0 {
        return Err(());
    }
    if input.len() / 2 > output.len() {
        return Err(());
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
                return Err(());
            };
            num <<= 4;
            num |= val;
        }
        output[block_num] = num;
    }

    return Ok(&mut output[0..input.len()/2]);
}

pub fn bin2hex<'a>(input: &[u8], output: &'a mut [u8]) -> Result<&'a mut [u8], ()> {
    const DIGITS: &[u8] = &[b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a'
        , b'b', b'c', b'd', b'e', b'f'];

    if output.len() < input.len() * 2 {
        return Err(());
    }

    for idx in 0..input.len() {
        let byte = input[idx] as usize;
        output[idx * 2 + 0] = DIGITS[(byte >> 4) & 0x0f];
        output[idx * 2 + 1] = DIGITS[(byte >> 0) & 0x0f];
    }

    return Ok(&mut output[0..input.len() * 2]);
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