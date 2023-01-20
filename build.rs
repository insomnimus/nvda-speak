use std::env;

fn main() {
	let p = env::current_dir().unwrap();
	println!("cargo:rustc-link-search=native={}", p.join("ffi/x64").display());
}
