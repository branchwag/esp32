# ESP32 experiments with Rust

I recently was given this ESP32 as an amazing present and decided to do some experiments with it.

[Amazon link](https://www.amazon.com/dp/B0C947BHK5 "Arduino Nano ESP32 on Amazon")

## Setup (Arch Linux)

The board is an Arduino Nano ESP32 (ESP32-S3-MINI-1). It cannot be flashed with `espflash` directly because it has no exposed BOOT/GPIO0 button — uploads go through its DFU bootloader instead.

### One-time toolchain install

```sh
sudo pacman -S rustup pkgconf openssl dfu-util
rustup default stable
cargo install espup espflash esp-generate --locked
espup install
echo '. $HOME/export-esp.sh' >> ~/.bashrc
source ~/.bashrc
```

If pacman warns that `rust` and `rustup` conflict, remove `rust` — `rustup` will manage it from now on (and is required for the Xtensa toolchain).

### One-time udev rule (so dfu-util doesn't need sudo)

Grants access to Arduino (`2341`) and Espressif (`303a`) USB devices for members of the `uucp` group, which is Arch's conventional group for serial/USB device access. Confirm you're in `uucp` with `groups`; if not, add yourself with `sudo gpasswd -a $USER uucp` and log out/in.

```sh
sudo tee /etc/udev/rules.d/99-esp32.rules > /dev/null <<'EOF'
SUBSYSTEM=="usb", ATTRS{idVendor}=="2341", MODE="0660", GROUP="uucp"
SUBSYSTEM=="usb", ATTRS{idVendor}=="303a", MODE="0660", GROUP="uucp"
SUBSYSTEM=="tty", ATTRS{idVendor}=="2341", MODE="0660", GROUP="uucp"
SUBSYSTEM=="tty", ATTRS{idVendor}=="303a", MODE="0660", GROUP="uucp"
EOF
sudo udevadm control --reload-rules && sudo udevadm trigger
```

Then unplug and replug the board.

### Build and flash

```sh
cargo build
espflash save-image --chip esp32s3 \
    target/xtensa-esp32s3-none-elf/debug/my-first-project \
    target/app.bin
```

Double-tap the white reset button on the board (two presses within ~half a second) to enter DFU mode. Confirm with `dfu-util --list` — you should see `[2341:0070] ... name="Arduino DFU"`.

Then flash:

```sh
dfu-util --device 2341:0070 --alt 0 --download target/app.bin --reset
```

The blue element of the onboard RGB LED will blink on/off every half second.

### Persistence across power cycles

The Arduino bootloader uses ESP-IDF's OTA (Over-The-Air update) scheme, which boots a freshly-flashed app in a "pending verification" state and rolls it back on the next power cycle unless the app explicitly marks itself valid. To survive power cycles, our `main.rs` calls `OtaUpdater::set_current_ota_state(OtaImageState::Valid)` early in boot. Without this call the board would revert to DFU mode (rainbow LED cycle) on every power-up and require re-flashing.

