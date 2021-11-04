#[macro_use]
extern crate criterion;

use ark_ff::fields::Field;
use ark_std::{
    ops::{AddAssign, MulAssign},
    UniformRand,
};
use ark_test_curves::bls12_381::Fr as F;
use criterion::Criterion;

fn bench_mul_add(c: &mut Criterion) {
    let rng = &mut ark_std::test_rng();

    let mut group = c.benchmark_group("ark_ff::mul_add");
    group.bench_function("mul+add_assign", |b| {
        let mut a = F::rand(rng);
        let multiplicand = F::rand(rng);
        let addend = F::rand(rng);
        b.iter(|| {
            a.mul_assign(&multiplicand);
            a.add_assign(&addend);
        })
    });
    group.bench_function("raw", |b| {
        let mut a = F::rand(rng);
        let multiplicand = F::rand(rng);
        let addend = F::rand(rng);
        b.iter(|| {
            a = a * &multiplicand + &addend;
        })
    });
    group.bench_function("slow_raw", |b| {
        let mut a = F::rand(rng);
        let multiplicand = F::rand(rng);
        let addend = F::rand(rng);
        b.iter(|| {
            a = a * &multiplicand;
            a = a + &addend;
        })
    });
    group.bench_function("mul_add", |b| {
        let mut a = F::rand(rng);
        let multiplicand = F::rand(rng);
        let addend = F::rand(rng);
        b.iter(|| {
            a.mul_add_assign(&multiplicand, &addend);
        })
    });
}

criterion_group!(benches, bench_mul_add);
criterion_main!(benches);
