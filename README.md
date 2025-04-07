# `tinysys-quickstart`
> A template for building applications for the `tinysys` project

## Dependencies

To build embedded programs using this template you'll need:
- The `cargo generate` subcommand. [Installation
  instructions](https://github.com/ashleygwilliams/cargo-generate#installation).
- `rust-std` components (pre-compiled `core` crate) for `tinysys`'s `rv32im_zicsr_zifencei_zfinx` target. Run:
```sh
$ rustup target add riscv32im-unknown-none-elf
```

## Using this template

1. Instantiate the template
```sh
$ cargo generate --git https://github.com/Chris--B/tinysys-quickstart
🤷   Project Name: my-cool-demo
🔧   Destination: ~/my-cool-demo ...
🔧   project-name: my-cool-demo ...
🔧   Generating template ...
🔧   Initializing a fresh Git repository
✨   Done! New project created ~/my-cool-demo
```

2. Build your project
```sh
$ cd my-cool-demo
$ cargo build
```

3. Run your project to copy it to the emulator sd card
  - [`.cargo/config.toml`](.cargo/config.toml) controls the runner
  - It defaults to the helper script [`copy_to_sdcard.sh`](copy_to_sdcard.sh)
```sh
$ cargo run
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.02s
     Running `~/my-cool-demo/./copy_to_sdcard.sh target/riscv32im-unknown-none-elf/debug/my-cool-demo`
target/riscv32im-unknown-none-elf/debug/my-cool-demo -> ../tinysys/software/emulator/sdcard/my-cool-demo
+ cd ../tinysys/software/emulator/sdcard/
./my-cool-demo -> ./my-cool-demo.elf
```

## Known Issues

### `tinysys-rs` Versioning
The `tinysys-rs` crate dependency is a github link, and a working commit is pinned in the `Cargo.lock`. Running `cargo update` can move this forward and this quickstart guide may break.

### "`unstable feature specified`"

We pass `target-feature`s to Rust that the compiler doesn't yet recognize. The warnings look something like:
```sh
warning: unstable feature specified for `-Ctarget-feature`: `zicsr`
  |
  = note: this feature is not stably supported; its behavior can change in the future

warning: unstable feature specified for `-Ctarget-feature`: `zifencei`
  |
  = note: this feature is not stably supported; its behavior can change in the future

warning: unstable feature specified for `-Ctarget-feature`: `zfinx`
  |
  = note: this feature is not stably supported; its behavior can change in the future
```
We don't know how to disable these warnings, but we do depend on current behavior of the features.
