use bencode::dispatcher::bdecode::decode_bencode;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

const SAMPLE_BENCODE: &[u8] = b"d3:foo3:bar4:listl4:spam4:eggsi42ee";

fn benchmark_recursive(c: &mut Criterion) {
    c.bench_function("decode_bencode_recursive", |b| {
        b.iter(|| {
            let (value, _rest) = decode_bencode(black_box(SAMPLE_BENCODE)).unwrap();
            black_box(value);
        })
    });
}

criterion_group!(benches, benchmark_recursive);
criterion_main!(benches);
