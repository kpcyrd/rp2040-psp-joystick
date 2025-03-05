# rp2040-psp-joystick

```
rustup target add thumbv6m-none-eabi
cargo build --release
elf2uf2-rs -d target/thumbv6m-none-eabi/release/rp2040-psp-joystick
```

## Bill of materials

- rp2040
- sh1106
- Dual Axis Mini XY Thumb Joystick Sensor (PSP-like)
- some wire

## Pins

There are 4 pins that need to be connected:

- `GPIO4` - display data (sda)
- `GPIO5` - display clock (scl)
- `GPIO28` - joystick adc (x)
- `GPIO29` - joystick adc (y)
- `3V3` - VDD (power, 3.3V or 5V are both fine)
- `GND` - GND (ground)

![](https://www.waveshare.com/img/devkit/RP2040-Zero/RP2040-Zero-details-7.jpg)

[Archive](https://web.archive.org/web/20241228234716if_/https://www.waveshare.com/img/devkit/RP2040-Zero/RP2040-Zero-details-7.jpg)
