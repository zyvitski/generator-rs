#[cfg(test)]

extern crate generator;
use generator::*;

const TEST_STEP_F64: f64 = 0.001_f64;
const TEST_RANGE_F64: (f64, f64) = (0.0_f64, 1.0_f64);
const TEST_DOMAIN_F64: (f64, f64) = (0.0_f64, 0.5_f64);
const TEST_STEP_F32: f32 = 0.001_f32;
const TEST_RANGE_F32: (f32, f32) = (0.0_f32, 1.0_f32);
const TEST_DOMAIN_F32: (f32, f32) = (0.0_f32, 0.5_f32);

fn in_bounds_f64(value: f64, bounds: &(f64, f64)) -> bool {
    let (lower, upper) = *bounds;
    value < upper || value > lower
}
fn in_bounds_f32(value: f32, bounds: &(f32, f32)) -> bool {
    let (lower, upper) = *bounds;
    value < upper || value > lower
}

#[test]
fn test_phasor_f64_in_range() {
    let mut p = Phasor::new(TEST_STEP_F64, TEST_DOMAIN_F64, TEST_RANGE_F64);
    for _ in 0..100 {
        if let Some(value) = p.next() {
            assert!(in_bounds_f64(value, p.range()))
        } else {
            unreachable!()
        }
    }
}
#[test]
fn test_phasor_f32_in_range() {
    let mut p = Phasor::new(TEST_STEP_F32, TEST_DOMAIN_F32, TEST_RANGE_F32);
    for _ in 0..100 {
        if let Some(value) = p.next() {
            assert!(in_bounds_f32(value, p.range()))
        } else {
            unreachable!()
        }
    }
}
#[test]
fn test_phasor_f64_is_normal() {
    let mut p = Phasor::new(TEST_STEP_F64, TEST_DOMAIN_F64, TEST_RANGE_F64);
    for _ in 0..100 {
        if let Some(value) = p.next() {
            assert!(value.is_normal() || value == 0.0_f64);
        } else {
            unreachable!()
        }
    }
}
#[test]
fn test_phasor_f32_is_normal() {
    let mut p = Phasor::new(TEST_STEP_F32, TEST_DOMAIN_F32, TEST_RANGE_F32);
    for _ in 0..100 {
        if let Some(value) = p.next() {
            assert!(value.is_normal() || value == 0.0_f32);
        } else {
            unreachable!()
        }
    }
}

#[test]
fn test_wrap_f64() {
    let bounds = (0.0_f64, 1.0_f64);
    let value = 1.5_f64;
    assert_eq!(value.wrap(&bounds), 0.5_f64);
    let value = -0.5_f64;
    assert_eq!(value.wrap(&bounds), 0.5_f64);
}
#[test]
fn test_wrap_f32() {
    let bounds = (0.0_f32, 1.0_f32);
    let value = 1.5_f32;
    assert_eq!(value.wrap(&bounds), 0.5_f32);
    let value = -0.5_f32;
    assert_eq!(value.wrap(&bounds), 0.5_f32);
}
#[test]
fn test_clamp_f64() {
    let bounds = (0.0_f64, 1.0_f64);
    let value = 1.5_f64.clamp(&bounds);
    assert_eq!(value, 1.0_f64);
    assert!(in_bounds_f64(value, &bounds));
    let value = (-0.5_f64).clamp(&bounds);
    assert_eq!(value, 0.0_f64);
    assert!(in_bounds_f64(value, &bounds));
}
#[test]
fn test_clamp_f32() {
    let bounds = (0.0_f32, 1.0_f32);
    let value = 1.5_f32.clamp(&bounds);
    assert_eq!(value, 1.0_f32);
    assert!(in_bounds_f32(value, &bounds));
    let value = (-0.5_f32).clamp(&bounds);
    assert_eq!(value, 0.0_f32);
    assert!(in_bounds_f32(value, &bounds));
}
