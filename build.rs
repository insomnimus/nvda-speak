use std::env;

fn main() {
	if !cfg!(windows) {
		panic!("this program is for windows");
	}
	println!("cargo:rerun-if-changed=ffi");

	let (dir, lib) = if cfg!(target_arch = "x86_64") {
		("ffi/x64", "nvdaControllerClient64")
	} else if cfg!(target_arch = "x86") {
		("ffi/x86", "nvdaControllerClient32")
	} else if cfg!(target_arch = "aarch64") {
		("ffi/arm64", "nvdaControllerClient32")
	} else {
		panic!("the target architecture is not supported");
	};

	let p = env::current_dir().unwrap();
	println!("cargo:rustc-link-search=native={}", p.join(dir).display());
	println!("cargo:rustc-link-lib=dylib={lib}")
}
