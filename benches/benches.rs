
#![feature(test)]

extern crate test;
extern crate generator;

use generator::*;
use test::Bencher;

const TEST_STEP_F64: f64 = 0.001_f64;
const TEST_RANGE_F64: (f64, f64) = (1.00_f64, 0.0_f64);
const TEST_DOMAIN_F64: (f64, f64) = (0.5_f64, 0.0_f64);
const TEST_STEP_F32: f32 = 0.001_f32;
const TEST_RANGE_F32: (f32, f32) = (1.00_f32, 0.0_f32);
const TEST_DOMAIN_F32: (f32, f32) = (0.5_f32, 0.0_f32);

#[bench]
fn bench_phasor_f64(b: &mut Bencher) {
    let mut p = test::black_box(Phasor::new(TEST_STEP_F64, TEST_DOMAIN_F64, TEST_RANGE_F64));
    b.iter(move || p.next().unwrap())
}
#[bench]
fn bench_phasor_f32(b: &mut Bencher) {
    let mut p = test::black_box(Phasor::new(TEST_STEP_F32, TEST_DOMAIN_F32, TEST_RANGE_F32));
    b.iter(move || p.next().unwrap())
}
