<h4 align="right"><strong>English</strong> | <a href="./README_CN.md">简体中文</a></h4>

>[!WARNING]
> 🚧Under developmen and not ready for production.

>[!NOTE]
> [Unifi Protect](https://ui.com/us/en/physical-security/door-access) is too expensive, so I'm building my own.

# OXIDOOR 🔒

An intelligent door lock system built with Rust and ESP32-S3, supporting multiple connectivity options and power supply methods.

## Features

- 🔐 **Smart Door Lock Control**: Electromagnetic lock control with GPIO-based relay switching
- 🎮 **Multi-Input Support**: Dual button input for manual operation
- 🔗 **Connectivity Ready**: Designed for NFC, Bluetooth, Wi-Fi, and Ethernet integration
- 🔌 **Power Options**: Supports PoE (Power over Ethernet) power supply
- 🦀 **Rust-Powered**: Built with Rust for safety and performance on ESP32-S3
- ⚡️ **Async Runtime**: Embassy-based asynchronous task handling

## Hardware Requirements

- ESP32-S3 microcontroller
- Electromagnetic door lock (controlled via GPIO5)
- Two input buttons (GPIO6, GPIO7)
- Power supply (standard or PoE)
- PN532/PN5180 NFC reader
- 3.3V relay

## Current Implementation

The current version provides basic door lock functionality:

- **Door Lock Control**: Electromagnetic lock connected to GPIO5
- **Button Inputs**: Two buttons (GPIO6, GPIO7) with internal pull-up resistors
- **Automatic Locking**: 3-second unlock duration when buttons are pressed
- **Async Operation**: Non-blocking operation using Embassy executor

## Building and Flashing

### Prerequisites

```bash
# Install Rust ESP toolchain
cargo install espup
espup install

# Source the environment
source ~/export-esp.sh
```

### Build and Flash

```bash
# Build the project
cargo build

# Flash to ESP32-S3
cargo run
```

## Pin Configuration

| Function  | GPIO Pin | Description                                          |
| --------- | -------- | ---------------------------------------------------- |
| Door Lock | GPIO5    | Relay control output (HIGH = locked, LOW = unlocked) |
| Button 1  | GPIO6    | Input with pull-up (LOW when pressed)                |
| Button 2  | GPIO7    | Input with pull-up (LOW when pressed)                |

## Project Structure

```
src/
├── main.rs          # Main application entry point
├── task.rs          # Async door control task implementation
```

## Future Roadmap

- 📱 NFC card authentication
- 📡 Bluetooth Low Energy support
- 📱 Wi-Fi connectivity and remote control
- 🌐 Ethernet connectivity
- 📊 Access logging and monitoring
- 🔒 Enhanced security features
- 📱 Mobile app integration

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

## Author

- [AprilNEA](https://github.com/aprilnea)

Repository: [https://github.com/aprilnea/oxidoor](https://github.com/aprilnea/oxidoor)

---

*OXIDOOR - Securing your space with Rust reliability*