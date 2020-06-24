#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut output = vec![0u8; (data.len() + 5) * 8 / 5 ];
    binascii::b32encode(data, &mut output).expect("all inputs should encode successfuly");
});
