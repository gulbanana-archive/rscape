use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let target = env::var("TARGET").unwrap();
    let debug = env::var("DEBUG").unwrap() == "true";
    
    // libpath for linking
    let lib_dir = Path::new("lib").join(&target);
    println!("cargo:rustc-link-search={}", lib_dir.to_str().unwrap());
    
    // dlls for running
    let output_dir = Path::new("target").join(if debug {"debug"} else {"release"});
    for dll in ["SDL2.dll", "SDL2_ttf.dll", "libfreetype-6.dll", "zlib1.dll"].iter() {
        fs::copy(lib_dir.join(dll), output_dir.join(dll)).unwrap();
    }
}