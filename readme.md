# nvda-speak
This tool uses the [NVDA](https://github.com/nvaccess/nvda) screen reader to speak text.

> Note: The library used to interface with NVDA is provided from [here](https://github.com/nvaccess/nvda/tree/master/extras/controllerClient). For convenience the latest master release as of 2023-01-20 is duplicated in this repository, under the "ffi" folder.

## Usage
Pipe text to it or pass it as an argument.

```powershell
echo "Hello, world!" | nvda-speak
nvda-speak "Hello, world!"
```

## Installation
1. Clone the repository: `git clone https://github.com/insomnimus/nvda-speak`
2. Change to the directory: `cd nvda-speak`
3. Run cargo: `cargo build --release`
4. move `target/release/nvda-speak.exe` and the dll corresponding to your architecture in `ffi/` into somewhere in `$PATH`.
	- 64 bit: `ffi/x64/nvdaControllerClient64.dll`
	- 32 bit: `ffi/x86/nvdaControllerClient32.dll`
	- arm64: `ffi/arm64/nvdaControllerClient32.dll` (yes, it's called nvdaControllerClient32.dll on arm64 too)
