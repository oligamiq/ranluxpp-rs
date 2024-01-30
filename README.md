# make
https://github.com/jirka-h/ranluxpp-portable binding
# what this
ranlux++ pRNG

# use
```rust
let rand = Ranluxpp::new();
let mut x = [0u64; 9];
rand.rand(&mut x);
println!("{:?}", x);
```

# ref
https://github.com/jirka-h/ranluxpp-portable
