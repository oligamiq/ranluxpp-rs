use ranluxpp_rs::Ranluxpp;

fn main() {
  let mut rand = Ranluxpp::new(1);
  let mut x = [0u64; 9];
  rand.rand(&mut x);
  println!("{:?}", x);
}
