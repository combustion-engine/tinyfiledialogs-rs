/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate gcc;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    let mut cfg = gcc::Config::new();
    cfg.file("libtinyfiledialogs/tinyfiledialogs.c");
    cfg.flag("-v");
    cfg.compile("libtinyfiledialogs.a");

    if target.contains("windows") {
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=comdlg32");
        println!("cargo:rustc-link-lib=ole32");
    }
}
