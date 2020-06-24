#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut output = vec![0u8; (data.len() + 3) * 4 / 3];
    binascii::b64encode(data, &mut output).expect("all inputs should encode successfuly");
});
