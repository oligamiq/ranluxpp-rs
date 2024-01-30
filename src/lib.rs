use std::fmt::Debug;

use bindings::{ranluxpp_init, ranluxpp_t};

pub mod bindings;

#[derive(Clone, Copy, Debug)]
pub struct Ranluxpp {
    r: ranluxpp_t,
}

impl Ranluxpp {
    /// let rand = Ranluxpp::new(1);
    pub fn new(seed: u64) -> Self {
        let mut r: ranluxpp_t = ranluxpp_t {
            x: [0u64; 9],
            A: [0u64; 9],
        };
        unsafe {
            ranluxpp_init(&mut r, seed, 2048);
        }
        Self { r }
    }

    /// let mut rand = Ranluxpp::new(1);
    /// let mut x = [0u64; 9];
    /// rand.rand(&mut x);
    /// println!("{:?}", x);
    pub fn rand(&mut self, x: &mut [u64; 9]) {
        x.copy_from_slice(&self.r.x);
        unsafe {
            bindings::ranluxpp_nextstate(&mut self.r);
        };
    }
}
