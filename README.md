# microrust-start

A sample application for teaching embedded development using the BBC micro:bit board. The project demonstrates the
following concepts:

* PACs, HALs, BSPs...
* RTT
* Panic handling
* Serial communication
* GPIO and interrupts
* Testing

## Pre-requisities

For building:

```
rustup target add thumbv7em-none-eabihf
```

...and the [GNU Arm toolchain](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads).

For running:

```
cargo install probe-run
```

For debugging:

```
cargo install --git https://github.com/probe-rs/probe-rs probe-rs-debugger
```

...and then visit https://github.com/probe-rs/vscode#vs-code-probe-rs-debugger for instructions on 
how to install the VSCode plugin.

## Running

```
cargo run --target thumbv7em-none-eabihf
```

## Debugging

Launch via the VSCode debugger.

## Testing

```
cargo test
```