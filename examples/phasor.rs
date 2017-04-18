extern crate generator;
use generator::*;
fn main() {
    let step = 0.01_f64;
    let range = (0.0_f64, 1.0_f64);
    let domain = (0.0_f64, 0.5_f64);
    let mut phasor = Phasor::new(step, domain, range);
    phasor.sync_to(0.0_f64);

    for _ in 0..100 {
        if let Some(value) = phasor.next() {
            println!("{:?}", value);
        }
    }
}
