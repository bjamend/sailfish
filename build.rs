fn main() {

    cc::Build::new()
        .file("src/physics/hydro.c")
        .flag("-Xpreprocessor")
        .flag("-fopenmp")
        .compile("hydro_omp");

    cc::Build::new()
        .file("src/physics/hydro.c")
        .compile("hydro_cpu");

    #[cfg(feature = "cuda")]
    {
        cc::Build::new()
            .file("src/physics/hydro.cu")
            .cuda(true)
            .compile("hydro_gpu");
            println!("cargo:rustc-link-lib=cudart");
            println!("cargo:rustc-link-lib=cuda");
    }

    #[cfg(not(feature = "cuda"))]
    {
        cc::Build::new()
            .file("src/physics/hydro.c")
            .define("GPU_STUBS", None)
            .flag("-Wno-unused-function")
            .compile("hydro_gpu");
    }
}
