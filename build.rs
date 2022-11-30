extern crate cc;
// use std::env;


fn main() {
    cc::Build::new()
        .cuda(true)
        .flag("-cudart=shared")
        .file("cpv_gpu.cu")
        .compile("cpv.a");

    println!("cargo:rustc-link-search=native=C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v11.8/lib");
    // println!("cargo:rustc-link-search=C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v11.8/lib");
    // println!("cargo:rustc-env=LD_LIBRARY_PATH=C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v11.8/lib");
    println!("cargo:rustc-link-lib=dylib=cudart");
}