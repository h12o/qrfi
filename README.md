# qrfi

`qrfi` is a CLI Wi-Fi QR Code Generator.

![Terminal (Dark Mode)](assets/screenshots/dark-mode/terminal.png#gh-dark-mode-only)
![Terminal (Light Mode)](assets/screenshots/light-mode/terminal.png#gh-light-mode-only)

## Why this version?

- **Privacy First:** Generates codes 100% locally. No passwords ever leave your machine.
- **No Runtime Required:** Just a single binary.
- **High Performance:** Instant startup and QR generation.

## Prerequisites

### CLI

- Rust 1.93.0+ (Edition 2024)

### Scanning Device

- Android
- iOS 11+

## Installation

### From crates.io (Recommended)

```shell
cargo install qrfi
```

### From Source

```shell
cargo install --path . --root <your favorite directory, e.g. ~/.local>
```

## Usage

### Basic

```shell
qrfi SSID -p PASSWORD
```

### Piping from Other Commands

```shell
echo SSID | qrfi -p PASSWORD
```

### Save as PNG

```shell
qrfi SSID -p PASSWORD --format png > qr.png
```

### Supported Formats of QR Code

- default: ascii
- possible values: ascii, png, svg

## Options

See the result of `qrfi -h`.

## Development

```shell
git clone https://github.com/h12o/qrfi
cd qrfi
cargo test
```

Tips: You can also use `cargo run --` during development.

## Contributions

Issues and pull requests are welcome.

## License

Licensed under the MIT License. See [LICENSE.md](LICENSE.md) for details.

## Trademarks

QR Code is a registered trademark of DENSO WAVE INCORPORATED in Japan and in other countries.

## Acknowledgements

Inspired by @kamataryo's [kamataryo/qrfi](https://github.com/kamataryo/qrfi), implementation using Node.js.

Referenced from @zxing's Barcode Contents > [Wi-Fi Network config (Android, iOS 11+)](https://github.com/zxing/zxing/wiki/Barcode-Contents#wi-fi-network-config-android-ios-11) in [zxing/zxing](https://github.com/zxing/zxing).
