use cmake::Config;
use dunce::canonicalize;
use std::env;
use std::path::{Path, PathBuf};

// pub enum LinkType {
//     Dynamic,
//     Static,
// }

// impl LinkType {
//     fn detect() -> LinkType {
//         if cfg!(feature = "static") {
//             LinkType::Static
//         } else {
//             LinkType::Dynamic
//         }
//     }

//     fn link_lib(&self) -> &'static str {
//         match self {
//             LinkType::Dynamic => "dylib=",
//             LinkType::Static => "static=",
//         }
//     }

//     fn target_name(&self) -> &'static str {
//         match self {
//             LinkType::Dynamic => "aeron_archive_client",
//             LinkType::Static => "aeron_archive_client_wrapper",
//         }
//     }
// }

pub fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=aeron_cpp_wrapper_client.h");

    let aeron_path = canonicalize(Path::new("./aeron")).unwrap();
    let header_path = aeron_path.join("aeron-client/src/main/cpp_wrapper");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // let link_type = LinkType::detect();
    // println!(
    //     "cargo:rustc-link-lib={}{}",
    //     link_type.link_lib(),
    //     link_type.target_name()
    // );

    // if let LinkType::Static = link_type {
    //     // On Windows, there are some extra libraries needed for static link
    //     // that aren't included by Aeron.
    //     if cfg!(target_os = "windows") {
    //         println!("cargo:rustc-link-lib=shell32");
    //         println!("cargo:rustc-link-lib=iphlpapi");
    //     }
    // }

    let cmake_output = Config::new(&aeron_path)
        // .build_target(link_type.target_name())
        .build();

    // Trying to figure out the final path is a bit weird;
    // For Linux/OSX, it's just build/lib
    // For Windows, the .lib file is in build/lib/{profile}, but the DLL
    // is shipped in build/binaries/{profile}
    let base_lib_dir = cmake_output.join("build");
    println!(
        "cargo:rustc-link-search=native={}",
        base_lib_dir.join("lib").display()
    );
    // Because the `cmake_output` path is different for debug/release, we're not worried
    // about accidentally linking the Debug library when this is a release build or vice-versa
    println!(
        "cargo:rustc-link-search=native={}",
        base_lib_dir.join("lib/Debug").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        base_lib_dir.join("binaries/Debug").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        base_lib_dir.join("lib/Release").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        base_lib_dir.join("binaries/Release").display()
    );


    // println!("cargo:rustc-link-lib=static=aeron_cpp_wrapper_client_static");
    println!("cargo:rustc-link-lib=stdc++");

    println!("cargo:include={}", header_path.display());
    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", header_path.display()))
        .clang_arg(format!(
            "-I{}",
            aeron_path.join("aeron-client/src/main/c").display()
        ))
        .header("aeron_cpp_wrapper_client.h")
        .enable_cxx_namespaces()
        // enable C++
        // .clang_args(&["-x", "c++", "--std=c++14", "-fkeep-inline-functions"])
        // .opaque_type("std::.*")
        // .generate_inline_functions(true)
        .clang_args(&["-x", "c++", "--std=c++14"])
        .opaque_type("std::.*")
        .allowlist_function("aeron::.*")
        .allowlist_function("aeron.*")
        .allowlist_type("aeron::.*")
        .allowlist_type("aeron.*")
        .allowlist_var("aeron*")
        .allowlist_var("AERON_.*")
        // .allowlist_function("aeron_.*")
        // .allowlist_type("aeron_.*")
        // .allowlist_var("AERON_.*")
        // .allowlist_function("aeron_.*")
        // .allowlist_type("aeron_.*")
        // .allowlist_var("AERON_.*")
        .constified_enum_module("aeron_.*_enum")
        .derive_debug(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate aeron bindings");

    bindings
        .write_to_file(out_path.join("aeron_cpp_wrapper_client.rs"))
        .expect("Couldn't write bindings!");
}
