use bindings::{ranluxpp_init, ranluxpp_t};

pub mod bindings;

#[derive(Clone, Copy, Debug)]
pub struct Ranluxpp {
    r: ranluxpp_t,
}

impl Ranluxpp {
    pub fn new() -> Self {
        let mut r: ranluxpp_t = ranluxpp_t {
            x: [0u64; 9],
            A: [0u64; 9],
        };
        unsafe {
            ranluxpp_init(&mut r, 1, 2048);
        }
        Self { r }
    }

    pub fn rand(&mut self, x: &mut [u64; 9]) {
      x.copy_from_slice(&self.r.x);
      unsafe {
        bindings::ranluxpp_nextstate(&mut self.r);
      };
    }
}
