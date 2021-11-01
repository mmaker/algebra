#[macro_use]
extern crate criterion;

use ark_ff::Zero;
use ark_ff::fields::Field;
use ark_std::{
    ops::{AddAssign, MulAssign},
    UniformRand,
};
use ark_test_curves::bls12_381::Fr;
use criterion::{black_box, Criterion};

fn bench_mul_add(c: &mut Criterion) {
    let rng = &mut ark_std::test_rng();
    let mut a = Fr::rand(rng);
    let multiplicand = Fr::rand(rng);
    let addend = Fr::rand(rng);

    assert_eq!(a * multiplicand + addend, a.mul_add(&multiplicand, &addend));
    let mut group = c.benchmark_group("ark_ff::mul_add");
    group.bench_function("mul+add_assign", |b| {
        b.iter(|| {
            a.mul_assign(multiplicand);
            a.add_assign(addend);
            assert_ne!(a, Fr::zero())
        })
    });
    group.bench_function("raw", |b| {
        b.iter(|| {
            let result = a * multiplicand + addend;
            assert_ne!(result, Fr::zero())
        })
    });
    group.bench_function("mul_add", |b| {
        b.iter(|| {
            a.mul_add_assign(&multiplicand, &addend);
            assert_ne!(a, Fr::zero())
        })
    });
}

criterion_group!(benches, bench_mul_add);
criterion_main!(benches);
