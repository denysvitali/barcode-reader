# Barcode Scanner
This simple Rust application takes care of the communication between my
Raspberry Pi 2 B+ and my Datalogic Magellan 1100i scanner - connected via USB (w/ USB COM emulator mode).

This simple app will be used to see what's the fridge content - because I'm too lazy to check it every time.

## Requirements
- [Datalogic Magellan 1100i Scanner](http://www.datalogic.com/eng/products/transportation-logistics-retail/presentation-scanners/magellan-1100i-pd-157.html) (or similar)
- Raspberry Pi (any) running any Linux Distro (I suggest you to use [Arch Linux ARM](http://archlinuxarm.org/))
- Rust (`pacman -S rust`)
- (A fridge?)

## Run
```
git clone https://github.com/denysvitali/barcode-reader
cd barcode-reader
cargo run
```

## Troubleshooting

### Permission denied while opening `/dev/ttyWhatever`
Add yourself to the `uucp` / `serial` / `$(stat -c "%G" /dev/ttyWhatever)` group. Reboot or logout, log back in and try again.  
Fast but *very* ugly solution: run as root.