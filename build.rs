use std::env;

fn main() {
	println!("cargo:rerun-if-changed=ffi");

	let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
	let (dir, lib) = match target_arch.as_str() {
		"x86_64" => ("ffi/x64", "nvdaControllerClient64"),
		"x86" => ("ffi/x86", "nvdaControllerClient32"),
		"aarch64" => ("ffi/arm64", "nvdaControllerClient32"),
		_ => panic!("the target architecture {target_arch} is not supported"),
	};

	let p = env::current_dir().unwrap();
	println!("cargo:rustc-link-search=native={}", p.join(dir).display());
	println!("cargo:rustc-link-lib=dylib={lib}")
}
