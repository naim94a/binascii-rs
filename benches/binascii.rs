use criterion::{black_box, criterion_group, criterion_main, Criterion, Fun};
use rand::RngCore;

fn make_buffer(sz: usize) -> Vec<u8> {
    let mut rng = rand::rngs::mock::StepRng::new(0xdeadbeef, 0x700100f0039);
    let mut v = vec![0u8; sz];
    rng.fill_bytes(&mut v);
    v
}

fn bench(c: &mut Criterion) {
    let base16_encode = vec![
        Fun::new("iter", |b, input: &&[u8]| {
            b.iter(|| {
                let mut output = [0u8; 4096];
                binascii::iters::bin2hex(input, &mut output).unwrap();
            });
        }),
        Fun::new("plain", |b, input: &&[u8]| {
            b.iter(|| {
                let mut output = [0u8; 4096];
                binascii::bin2hex(input, &mut output).unwrap();
            });
        }),
    ];

    let mut input = black_box(Box::leak(make_buffer(2048).into_boxed_slice()));
    binascii::bin2hex(&vec![0xabu8; 1024], &mut input).unwrap();
    let input = &*input;
    
    c.bench_functions("base16-encode", base16_encode, &*input);
    

    let base32_encode = vec![
        Fun::new("plain", |b, input: &&[u8]| {
            b.iter(|| {
                let mut output = [0u8; 4096];
                binascii::b32encode(&input, &mut output).unwrap();
            })
        }),
        Fun::new("iter", |b, input: &&[u8]| {
            b.iter(|| {
                let mut output = [0u8; 4096];
                binascii::iters::b32encode(&input, &mut output).unwrap();
            })
        }),
    ];

    c.bench_functions("base32-encode", base32_encode, &*input);
}

criterion_group!(benches, bench);
criterion_main!(benches);
