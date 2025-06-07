# AzLang 

A Python-3 (CPython >= 3.13.0) Interpreter written in Rust :snake: :scream:
:metal:.

## Usage

AzLang requires Rust latest stable version (e.g 1.67.1 at February 7th 2023). If you don't
currently have Rust installed on your system you can do so by following the instructions at [rustup.rs](https://rustup.rs/).

To check the version of Rust you're currently running, use `rustc --version`. If you wish to update,
`rustup update stable` will update your Rust installation to the most recent stable release.

To build AzLang locally, first, clone the source code:

```bash
git clone https://github.com/AzLang/AzLang
```

AzLang uses symlinks to manage python libraries in `Lib/`. If on windows, running the following helps:
```bash
git config core.symlinks true
```

Then you can change into the AzLang directory and run the demo (Note: `--release` is
needed to prevent stack overflow on Windows):

```bash
$ cd AzLang
$ cargo run --release demo_closures.py
Hello, AzLang!
```

Or use the interactive shell:

```bash
$ cargo run --release
Welcome to rustpython
>>>>> 2+2
4
```

NOTE: For windows users, please set `RUSTPYTHONPATH` environment variable as `Lib` path in project directory.
(e.g. When AzLang directory is `C:\AzLang`, set `RUSTPYTHONPATH` as `C:\AzLang\Lib`)

You can also install and run AzLang with the following:

```bash
$ cargo install --git https://github.com/AzLang/AzLang rustpython
$ rustpython
Welcome to the magnificent Rust Python interpreter
>>>>>
```

If you'd like to make https requests, you can enable the `ssl` feature, which
also lets you install the `pip` package manager. Note that on Windows, you may
need to install OpenSSL, or you can enable the `ssl-vendor` feature instead,
which compiles OpenSSL for you but requires a C compiler, perl, and `make`.
OpenSSL version 3 is expected and tested in CI. Older versions may not work.

Once you've installed rustpython with SSL support, you can install pip by
running:

```bash
cargo install --git https://github.com/AzLang/AzLang --features ssl
rustpython --install-pip
```

You can also install AzLang through the `conda` package manager, though
this isn't officially supported and may be out of date:

```bash
conda install rustpython -c conda-forge
rustpython
```

### WASI

You can compile AzLang to a standalone WebAssembly WASI module so it can run anywhere.

Build

```bash
cargo build --target wasm32-wasip1 --no-default-features --features freeze-stdlib,stdlib --release
```

Run by wasmer

```bash
wasmer run --dir `pwd` -- target/wasm32-wasip1/release/rustpython.wasm `pwd`/extra_tests/snippets/stdlib_random.py
```

Run by wapm

```bash
$ wapm install rustpython
$ wapm run rustpython
>>>>> 2+2
4
```

#### Building the WASI file

You can build the WebAssembly WASI file with:

```bash
cargo build --release --target wasm32-wasip1 --features="freeze-stdlib"
```

> Note: we use the `freeze-stdlib` to include the standard library inside the binary. You also have to run once `rustup target add wasm32-wasip1`.

### JIT (Just in time) compiler

AzLang has a **very** experimental JIT compiler that compile python functions into native code.

#### Building

By default the JIT compiler isn't enabled, it's enabled with the `jit` cargo feature.

```bash
cargo run --features jit
```

This requires autoconf, automake, libtool, and clang to be installed.

#### Using

To compile a function, call `__jit__()` on it.

```python
def foo():
    a = 5
    return 10 + a

foo.__jit__()  # this will compile foo to native code and subsequent calls will execute that native code
assert foo() == 15
```

## Embedding AzLang into your Rust Applications

Interested in exposing Python scripting in an application written in Rust,
perhaps to allow quickly tweaking logic where Rust's compile times would be inhibitive?
Then `examples/hello_embed.rs` and `examples/mini_repl.rs` may be of some assistance.

## Disclaimer

AzLang is in development, and while the interpreter certainly can be used
in interesting use cases like running Python in WASM and embedding into a Rust
project, do note that AzLang is not totally production-ready.

Contribution is more than welcome! See our contribution section for more
information on this.

## Goals

- Full Python-3 environment entirely in Rust (not CPython bindings)
- A clean implementation without compatibility hacks

You can also generate documentation locally by running:

```shell
cargo doc # Including documentation for all dependencies
cargo doc --no-deps --all # Excluding all dependencies
```

Documentation HTML files can then be found in the `target/doc` directory or you can append `--open` to the previous commands to
have the documentation open automatically on your default browser.

For a high level overview of the components, see the [architecture](architecture/architecture.md) document.

## Contributing

Contributions are more than welcome, and in many cases we are happy to guide
contributors through PRs or on Discord. Please refer to the
[development guide](DEVELOPMENT.md) as well for tips on developments.

With that in mind, please note this project is maintained by volunteers, some of
the best ways to get started are below:

Most tasks are listed in the
[issue tracker](https://github.com/AzLang/AzLang/issues).

Another approach is to checkout the source code: builtin functions and object
methods are often the simplest and easiest way to contribute.

You can also simply run `uv run python -I whats_left.py` to assist in finding any unimplemented
method.

## Compiling to WebAssembly

[See this doc](wasm/README.md)

## Credit

- Sabuhi Sariyev
- Tunjay Akbarli

## Links

These are some useful links to related projects:

- https://github.com/ProgVal/pythonvm-rust
- https://github.com/shinglyu/AzLang
- https://github.com/windelbouwman/rspython

## License

This project is licensed under the MIT license. Please see the
[LICENSE](LICENSE) file for more details.

The [project logo](logo.png) is licensed under the CC-BY-4.0
license. Please see the [LICENSE-logo](LICENSE-logo) file
for more details.
