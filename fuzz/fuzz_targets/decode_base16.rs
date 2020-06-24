#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut output = vec![0u8; (data.len() + 1) / 2];
    if let Err(e) = binascii::hex2bin(data, &mut output)  {
        if e == binascii::ConvertError::InvalidOutputLength {
            // no real logic wouldn't be tested...
            panic!("Bad output length");
        }
    }
});
