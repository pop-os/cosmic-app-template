# COSMIC Watch

A modern clock application for the COSMIC desktop environment, featuring world clock, alarms, stopwatch, and timer functionality with system notifications.

## Features

- 🌍 **World Clock** - Display current time and date
- ⏰ **Alarms** - Set multiple alarms with custom labels
- ⏱️ **Stopwatch** - Precise timing with start/stop/reset
- ⏲️ **Timer** - Countdown timer with notifications
- 🔔 **Notifications** - System notifications with audio feedback
- 🌐 **Internationalization** - Multi-language support


## Prerequisites
[text](../../../../..)

### System Dependencies

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install libasound2-dev pkg-config build-essential

# Fedora/RHEL
sudo dnf install alsa-lib-devel pkgconfig gcc

# Arch Linux
sudo pacman -S alsa-lib pkgconf base-devel
```

### Audio Support (Optional)

```bash
# For better audio feedback
sudo apt install beep sox pulseaudio-utils

# Or for PipeWire users
sudo apt install pipewire-pulse
```

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/Moon-Mind/cosmic-watch.git
cd cosmic-watch

# Install Rust dependencies
cargo add chrono --features serde
cargo add futures-util
cargo add i18n-embed-fl
cargo add notify-rust
cargo add open
cargo add rodio --features wav,vorbis --no-default-features
cargo add rust-embed
cargo add tokio --features full

# Build the application
cargo build --release

# Run the application
cargo run
```

### Development Setup

```bash
# For development with hot reload
cargo install cargo-watch
cargo watch -x run
```

## Configuration

### Cargo.toml Setup

```toml
[package]
name = "cosmic-watch"
version = "0.1.0"
edition = "2021"
license = "MPL-2.0"
description = "A modern clock application for COSMIC desktop"
repository = "https://github.com/Moon-Mind/cosmic-watch.git"

[build-dependencies]
vergen = { version = "8", features = ["git", "gitcl"] }

[dependencies]
chrono = { version = "0.4.41", features = ["serde"] }
futures-util = "0.3.31"
i18n-embed-fl = "0.9.2"
notify-rust = "4.0"
open = "5.3.0"
rodio = { version = "0.17", default-features = false, features = ["wav", "vorbis"] }
rust-embed = "8.5.0"
tokio = { version = "1.41.0", features = ["full"] }

[dependencies.i18n-embed]
version = "0.15"
features = ["fluent-system", "desktop-requester"]

[dependencies.libcosmic]
git = "https://github.com/pop-os/libcosmic.git"
features = [
    "a11y",
    "dbus-config",
    "multi-window",
    "single-instance",
    "tokio",
    "winit",
    "wayland",
    "wgpu",
]
```

## Development Commands

### Building and Running

```bash
# Clean build
cargo clean
cargo build

# Run in development mode
cargo run

# Release build
cargo build --release

# Run tests
cargo test

# Check code without building
cargo check
```

### Localization Setup

```bash
# Create localization files
mkdir -p i18n/en
cat > i18n/en/cosmic_watch.ftl << 'EOF'
app-title = COSMIC Watch
world-clock = World Clock
alarms = Alarms
stopwatch = Stopwatch
timer = Timer
start = Start
stop = Stop
reset = Reset
add-alarm = Add Alarm
edit-alarm = Edit Alarm
delete-alarm = Delete Alarm
save-alarm = Save Alarm
cancel = Cancel
no-alarms = No alarms set
alarm-label = Alarm Label
new-alarm = New Alarm
hour = Hour
minute = Minute
enter-label = Enter alarm label
about = About
view = View
git-description = Git commit {$hash} on {$date}
EOF
```

### Troubleshooting Commands

```bash
# Fix ALSA compilation errors
sudo apt install libasound2-dev pkg-config

# Remove conflicting dependencies
cargo remove rodio  # if audio issues persist

# Check for missing dependencies
ldd target/release/cosmic-watch

# Test audio system
paplay /usr/share/sounds/alsa/Front_Left.wav
speaker-test -t sine -f 1000 -l 1

# Check notification system
notify-send "Test" "Notification test"
```

### Code Quality Commands

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Fix clippy warnings
cargo clippy --fix

# Security audit
cargo audit

# Update dependencies
cargo update
```

### Debugging Commands

```bash
# Run with debug output
RUST_LOG=debug cargo run

# Run with backtrace
RUST_BACKTRACE=1 cargo run

# Profile build time
cargo build --timings

# Check binary size
ls -lh target/release/cosmic-watch
```

### Audio Testing Commands

```bash
# Test system sounds
paplay /usr/share/sounds/freedesktop/stereo/alarm-clock-elapsed.oga
aplay /usr/share/sounds/alsa/Front_Left.wav

# Test beep command
beep -f 800 -l 1000

# Generate tone with sox
sox -n -t wav - synth 0.5 sine 800 | paplay

# List available sounds
find /usr/share/sounds -name "*.wav" -o -name "*.oga"
```

### File Structure Commands

```bash
# Create project structure
mkdir -p src resources/icons/hicolor/scalable/apps i18n/en

# Set up source files
touch src/main.rs src/app.rs src/config.rs src/notifications.rs

# Create desktop entry
cat > cosmic-watch.desktop << 'EOF'
[Desktop Entry]
Name=COSMIC Watch
Comment=Modern clock application for COSMIC desktop
Exec=cosmic-watch
Icon=cosmic-watch
Type=Application
Categories=Utility;Clock;
EOF
```

### Git Commands Used

```bash
# Initialize repository
git init
git add .
git commit -m "Initial commit"

# Add remote
git remote add origin https://github.com/Moon-Mind/cosmic-watch.git
git push -u origin main

# Development workflow
git add .
git commit -m "Add notifications with audio support"
git push
```

## Usage

1. **Launch the application**:
   ```bash
   cargo run
   ```

2. **Navigate between features**:
   - Use the navigation bar to switch between World Clock, Alarms, Stopwatch, and Timer
   - Each feature has its own dedicated interface

3. **Set alarms**:
   - Go to Alarms tab
   - Click "Add Alarm"
   - Set hour, minute, and label
   - Save the alarm

4. **Use stopwatch**:
   - Go to Stopwatch tab
   - Click Start/Stop to control timing
   - Click Reset to clear

5. **Use timer**:
   - Go to Timer tab
   - Set desired duration
   - Click Start to begin countdown

## Notifications

The application sends system notifications for:
- **Alarms**: When an alarm time is reached
- **Timer**: When the countdown finishes
- **Stopwatch**: When stopped (shows final time)

Audio feedback is provided through:
- System beep commands
- PulseAudio/PipeWire integration
- Fallback terminal bell

## Troubleshooting

### Audio Issues

```bash
# Check audio system
pulseaudio --check -v
pipewire --version

# Test notification sounds
paplay /usr/share/sounds/freedesktop/stereo/complete.oga
```

### Compilation Issues

```bash
# Missing ALSA development files
sudo apt install libasound2-dev

# Missing pkg-config
sudo apt install pkg-config

# Clean and rebuild
cargo clean && cargo build
```

### Runtime Issues

```bash
# Check dependencies
ldd target/release/cosmic-watch

# Run with debug information
RUST_LOG=debug cargo run
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

This project is licensed under the MPL-2.0 License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [libcosmic](https://github.com/pop-os/libcosmic)
- Icons from the COSMIC icon theme
- Audio support via [rodio](https://github.com/RustAudio/rodio)
- Notifications via [notify-rust](https://github.com/hoodie/notify-rust)