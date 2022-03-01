# Cargo doen't pass target features to build.rs

See `[target.'cfg(target_arch="x86_64")']` in `.cargo/config.toml`
If we specify target features under `[build]` it works right

````
$ cat test.sh 
#!/bin/bash -e

rustup show | head -n1
rm -rf target
cargo build
cat ./target/debug/build/*/output

$ ./test.sh 
Default host: x86_64-unknown-linux-gnu
   Compiling somelib v0.1.0 (/home/edvard/cargo-build-rs-rustflags/somelib)
   Compiling cargo-build-rs-rustflags v0.1.0 (/home/edvard/cargo-build-rs-rustflags)
    Finished dev [unoptimized + debuginfo] target(s) in 0.60s
fxsr,sse,sse2
fxsr,sse,sse2

```
