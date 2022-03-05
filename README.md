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

### Linux specific
<details>
  <summary>Click to show Linux setup instructions!</summary>

  The following instructions are Ubuntu specific but may help with other distros.
  Make sure you have `libudev-dev` installed so avoid _error: failed to run custom build command for
  `hidapi v1.3.3`_

  ```
  sudo apt install libudev-dev
  ```

  You need the device to attach with read/write permissions without `sudo`. We do this by updating
  your udev rules. Create a new file `/etc/udev/rules.d/microbit.rules` and add the following:

  ```
  SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", ATTR{idProduct}=="0204", MODE="0666"
  ```

  Now reload your udev rules by running:

  ```
  sudo udevadm control --reload-rules
  ```

  Finally, reconnect your Micro:bit so udev can reconnect it with the new device permissions. You'll
  know this worked if you manage to run the commands in the Running section below without a
  permissions error.
</details>

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

### WSL

Several extra steps are required to allow direct USB access within a WSL environment.

Follow the instructions at https://devblogs.microsoft.com/commandline/connecting-usb-devices-to-wsl/ to forward through your micro:bit to WSL via the USB/IP protocol. 

The device should appear in the list as something like:
```
BUSID  DEVICE                                                        STATE
9-1    USB Mass Storage Device, USB Serial Device (COM3), USB In...  Not attached
```

Then setup the udev rules from https://probe.rs/docs/getting-started/probe-setup/. You'll need to restart the udev service first, and run udev commands as sudo: 
```
sudo wget https://probe.rs/files/69-probe-rs.rules -O /etc/udev/rules.d/69-probe-rs.rules
sudo service udev restart
sudo udevadm control --reload
sudo udevadm trigger
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
