use super::ConvertError;

const BASE16_DICTIONARY: &[u8; 16] = b"0123456789abcdef";
const BASE32_DICTIONARY: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

#[cfg(feature = "encode")]
pub fn bin2hex<'a>(input: &[u8], output: &'a mut [u8]) -> Result<&'a mut [u8], ConvertError> {
    let out_len = input.len() * 2;
    if out_len > output.len() {
        return Err(ConvertError::InvalidOutputLength);
    }

    let encoded = input
        .iter().map(|&input| {
            let n1 = (input >> 4) as usize;
            let n2 = (input & 0x0f) as usize;

            [ BASE16_DICTIONARY[n1], BASE16_DICTIONARY[n2] ]
        });

    output
        .chunks_exact_mut(2)
        .zip(encoded)
        .for_each(|(o, i)| {
            o.copy_from_slice(&i)
        });
    Ok(&mut output[..out_len])
}

// calculate the number of bytes required for encoding data_len bytes
#[inline]
fn b32_calc_encoded(data_len: usize) -> usize {
    let req_len = data_len * 8 / 5;
    let remainder = req_len % 8;
    if remainder == 0 {
        req_len
    } else {
        req_len + (8 - remainder)
    }
}

#[cfg(feature = "encode")]
pub fn b32encode<'a>(input: &[u8], output: &'a mut [u8]) -> Result<&'a mut [u8], ConvertError> {
    let req_len = b32_calc_encoded(input.len());
    // the output length must be able to contain the encoded buffer + padding
    if req_len > output.len() {
        return Err(ConvertError::InvalidOutputLength);
    }

    // encoding all chunks of 5 bytes where possible
    input
        .chunks_exact(5)
        .zip(output.chunks_exact_mut(8))
        .for_each(|(input, output)| {
            // Bits in the base32 encoding:
            // AAAAABBB BBCCCCCD DDDDEEEE EFFFFFGG GGGHHHHH 

            let a = (input[0] >> 3) & 0b11111;
            let b = ((input[0] & 0b111) << 2) | ((input[1] >> 6) & 0b11);
            let c = (input[1] >> 1) & 0b11111;
            let d = ((input[1] & 0b1) << 4) | ((input[2] >> 4) & 0b1111);
            let e = ((input[2] & 0b1111) << 1) | ((input[3] >> 7) & 0b1);
            let f = (input[3] >> 2) & 0b11111;
            let g = ((input[3] & 0b11) << 3) | ((input[4] >> 5) & 0b1111);
            let h = input[4] & 0b11111;

            output[0] = BASE32_DICTIONARY[a as usize];
            output[1] = BASE32_DICTIONARY[b as usize];
            output[2] = BASE32_DICTIONARY[c as usize];
            output[3] = BASE32_DICTIONARY[d as usize];
            output[4] = BASE32_DICTIONARY[e as usize];
            output[5] = BASE32_DICTIONARY[f as usize];
            output[6] = BASE32_DICTIONARY[g as usize];
            output[7] = BASE32_DICTIONARY[h as usize];
        });

    // If the input isn't a multiple of 5, encode the last chunk;
    // this is the same like the iterator, except it contains many bound checks
    if input.len() % 5 != 0 {
        let a;
        let b;
        let c;
        let d;
        let e;
        let f;
        let g;
        let pad_bytes;

        // Get the block index, so we can determine our input & output offsets
        let blocks = input.len() / 5;
        let block_len = input.len() % 5;
        let input = &input[blocks * 5..];
        let output = &mut output[blocks * 8..req_len];
        
        // input len >= 1; otherwise we wouldn't reach this block (input.len() % 5 != 0)
        a = (input[0] >> 3) & 0b11111;
        output[0] = BASE32_DICTIONARY[a as usize];
        
        if block_len >= 2 {
            b = ((input[0] & 0b111) << 2) | ((input[1] >> 6) & 0b11);
            c = (input[1] >> 1) & 0b11111;
            output[1] = BASE32_DICTIONARY[b as usize];
            output[2] = BASE32_DICTIONARY[c as usize];
            
            if block_len >= 3 {
                d = ((input[1] & 0b1) << 4) | ((input[2] >> 4) & 0b1111);
                output[3] = BASE32_DICTIONARY[d as usize];
                
                if block_len >= 4 {
                    e = ((input[2] & 0b1111) << 1) | ((input[3] >> 7) & 0b1);
                    f = (input[3] >> 2) & 0b11111;
                    g = (input[3] & 0b11) << 3;
                    output[4] = BASE32_DICTIONARY[e as usize];
                    output[5] = BASE32_DICTIONARY[f as usize];
                    output[6] = BASE32_DICTIONARY[g as usize];
                    pad_bytes = 1;
                } else {
                    // block_len == 3
                    pad_bytes = 3;
                    
                    e = (input[2] & 0b1111) << 1;
                    output[4] = BASE32_DICTIONARY[e as usize];
                }
            }
            else {
                // block_len == 2
                pad_bytes = 4;

                d = (input[1] & 0b1) << 4;
                output[3] = BASE32_DICTIONARY[d as usize];
            }
        } else {
            // block_len == 1
            pad_bytes = 6;
            
            b = (input[0] & 0b111) << 2;
            output[1] = BASE32_DICTIONARY[b as usize];
        }

        // fill remaining untouched bytes with padding
        (&mut output[8-pad_bytes..]).iter_mut().for_each(|v| *v = b'=');
    }

    Ok(&mut output[..req_len])
}
