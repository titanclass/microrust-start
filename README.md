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
rustup update
rustup target add thumbv7em-none-eabihf
```

For running:

```
cargo install probe-run
```

For debugging:

```
cargo install --force --git https://github.com/probe-rs/probe-rs probe-rs-debugger

```

...and then visit https://github.com/probe-rs/vscode#vs-code-probe-rs-debugger for instructions on 
how to install the VSCode plugin. Here's an example though given 0.3.4 of the plugin (you may
wish to check the site to determine the latest version):

```
wget https://github.com/probe-rs/vscode/releases/download/v0.3.4/probe-rs-debugger-0.3.4.vsix -O /tmp/probe-rs-debugger-0.3.4.vsix
code --install-extension /tmp/probe-rs-debugger-0.3.4.vsix
```

You should also install the `CodeLLDB` plugin from the Visual Studio Code market place so that you are
able to debug unit tests.

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
