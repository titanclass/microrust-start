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

For running:

```
cargo install probe-run
```

For debugging:

```
cargo install probe-rs-debugger
```

...and then visit https://github.com/probe-rs/vscode#vs-code-probe-rs-debugger for instructions on 
how to install the VSCode plugin.

For more information on sizing:

```
cargo install size
```

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