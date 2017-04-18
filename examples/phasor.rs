extern crate generator;
use generator::*;
fn main() {
    let step = 0.1_f64;
    let range = (1.0_f64, 0.0_f64);
    let domain = (0.5_f64, 0.0_f64);
    let mut phasor = Phasor::new(step, domain, range);

    for _ in 0..20 {
        if let Some(value) = phasor.next() {
            println!("{:?}", value);
        }
    }
}
