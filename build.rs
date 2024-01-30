// use std::path::PathBuf;
// use std::env;

fn main() {
    // let bindings = bindgen::Builder::default()
    //     .header("ranluxpp-portable/ranluxpp.h")
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    //     .generate()
    //     .expect("Unable to generate bindings");

    // bindings
    //     .write_to_file("./src/bindings.rs")
    //     .expect("Couldn't write bindings!");

    cc::Build::new()
        .file("ranluxpp-portable/ranluxpp.c")
        .include("ranluxpp-portable")
        .flag("-g")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-Wpedantic")
        .flag("-O3")
        .warnings(false)
        .warnings_into_errors(false)
        .debug(false)
        .opt_level(3)
        .compiler("gcc")
        .compile("ranluxpp");
}

// gcc -c -o ranluxpp.o ranluxpp.c -g -Wall -Wextra -Wpedantic -O3
// gcc -c -o ranluxpp-test.o ranluxpp-test.c -g -Wall -Wextra -Wpedantic -O3
// gcc ranluxpp.o ranluxpp-test.o -o ranluxpp-test -g -Wall -Wextra -Wpedantic -O3
