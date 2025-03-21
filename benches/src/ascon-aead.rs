use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use ascon_aead::aead::{AeadInPlaceDetached, KeyInit};
use ascon_aead::AsconAead128;

const KB: usize = 1024;

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
type Benchmarker = Criterion;
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
type Benchmarker = Criterion<criterion_cycles_per_byte::CyclesPerByte>;

fn bench<A: AeadInPlaceDetached + KeyInit>(name: &str, c: &mut Benchmarker) {
    let mut group = c.benchmark_group(name);
    let nonce = black_box(Default::default());
    let cipher = black_box(A::new(&Default::default()));

    let mut buf = vec![0u8; 16 * KB];
    for size in [KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB] {
        let buf = &mut buf[..size];
        let tag = cipher.encrypt_in_place_detached(&nonce, b"", buf).unwrap();

        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("encrypt-128", size), |b| {
            b.iter(|| cipher.encrypt_in_place_detached(&nonce, b"", buf))
        });
        group.bench_function(BenchmarkId::new("decrypt-128", size), |b| {
            b.iter(|| cipher.decrypt_in_place_detached(&nonce, b"", buf, &tag))
        });
    }

    group.finish();
}

fn bench_ascon128(c: &mut Benchmarker) {
    bench::<AsconAead128>("Ascon-AEAD128", c);
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench_ascon128,
);

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(criterion_cycles_per_byte::CyclesPerByte);
    targets = bench_ascon128,
);

criterion_main!(benches);
