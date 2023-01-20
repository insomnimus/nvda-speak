#[cfg(not(windows))]
compile_error!("this program is for windows");

use std::{
	env,
	io::{
		self,
		Error,
		Read,
		Result,
	},
	process,
};

extern "C" {
	fn nvdaController_speakText(s: *const i16) -> i32;
	fn nvdaController_testIfRunning() -> i32;
}

fn run() -> Result<()> {
	unsafe {
		if nvdaController_testIfRunning() != 0 {
			eprintln!("error: could not communicate with NVDA");
			process::exit(2);
		}
	}

	let mut s = Vec::<i16>::new();
	let args = env::args().skip(1).collect::<Vec<_>>();

	if args.is_empty() {
		let mut buf = Vec::new();
		io::stdin().read_to_end(&mut buf)?;
		s.reserve_exact(buf.len() + 1);
		s.extend(buf.into_iter().map(|n| n as i16));
	} else {
		let size = args.iter().map(|s| s.len()).sum::<usize>() + args.len();
		s.reserve_exact(size);
		for (i, arg) in args.iter().enumerate() {
			if i > 0 {
				s.push(b'\n' as i16);
			}
			s.extend(arg.bytes().map(|n| n as i16));
		}
	}

	s.push(0);

	unsafe {
		let res = nvdaController_speakText(s.as_ptr());
		if res == 0 {
			Ok(())
		} else {
			Err(Error::from_raw_os_error(res))
		}
	}
}

fn main() {
	if let Err(e) = run() {
		eprintln!("error: {e}");
		process::exit(1);
	}
}
