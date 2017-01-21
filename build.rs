extern crate gcc;

use std::env;

fn main() {
    gcc::compile_library("libtinyfiledialogs.a", &["libtinyfiledialogs/tinyfiledialogs.c"]);

    if env::var("TARGET").unwrap().contains("windows") {
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=comdlg32");
        println!("cargo:rustc-link-lib=ole32");
    }
}
