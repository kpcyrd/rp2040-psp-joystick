#![no_std]
#![no_main]

use defmt_rtt as _;
use eh0::adc::OneShot;
use eh0::timer::CountDown;
use embedded_graphics::{
    mono_font::{MonoTextStyle, MonoTextStyleBuilder, ascii},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Triangle},
    text::{Baseline, Text},
};
use fugit::ExtU32;
use fugit::RateExtU32;
use panic_halt as _;
use sh1106::{Builder, prelude::*};
use waveshare_rp2040_zero::{
    Pins, XOSC_CRYSTAL_FREQ, entry,
    hal::{
        Sio,
        adc::{Adc, AdcPin},
        clocks::{Clock, init_clocks_and_plls},
        i2c::I2C,
        pac,
        timer::Timer,
        watchdog::Watchdog,
    },
};

pub const ARROW_PADDING: i32 = 4;
pub const ARROW_ON: PrimitiveStyle<BinaryColor> = PrimitiveStyle::with_fill(BinaryColor::On);
pub const ARROW_OFF: PrimitiveStyle<BinaryColor> = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
pub const ARROW_LOW_THRESHOLD: u16 = 1000;
pub const ARROW_HIGH_THRESHOLD: u16 = 3000;

pub const TEXT_STYLE: MonoTextStyle<BinaryColor> = MonoTextStyleBuilder::new()
    .font(&ascii::FONT_4X6)
    .text_color(BinaryColor::On)
    .build();

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    // Configure clocks and timers
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    let mut delay = timer.count_down();

    // Configure gpio
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure display
    let i2c = I2C::i2c0(
        pac.I2C0,
        pins.gp4.into_pull_type().into_function(), // sda
        pins.gp5.into_pull_type().into_function(), // scl
        400.kHz(),
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
    );
    let mut display: GraphicsMode<I2cInterface<_>> = Builder::new()
        .with_rotation(DisplayRotation::Rotate270)
        .connect_i2c(i2c)
        .into();
    display.init().unwrap();

    // Enable adc
    let mut adc = Adc::new(pac.ADC, &mut pac.RESETS);
    let mut adc_pin_x = AdcPin::new(pins.gp28.into_floating_input()).unwrap();
    let mut adc_pin_y = AdcPin::new(pins.gp29.into_floating_input()).unwrap();

    // enter loop
    loop {
        display.clear();

        let pin_x_count: u16 = adc.read(&mut adc_pin_x).unwrap();
        let pin_y_count: u16 = adc.read(&mut adc_pin_y).unwrap();

        // render values
        for (label, num, pos) in [
            ("x:", pin_x_count, Point::new(0, 0)),
            ("y:", pin_y_count, Point::new(0, 8)),
        ] {
            Text::with_baseline(label, pos, TEXT_STYLE, Baseline::Top)
                .draw(&mut display)
                .unwrap();

            let mut buf = itoa::Buffer::new();
            let buf = buf.format(num);
            Text::with_baseline(buf, pos + Point::new(15, 0), TEXT_STYLE, Baseline::Top)
                .draw(&mut display)
                .unwrap();
        }

        // draw triangle (right)
        Triangle::new(
            Point::new(50 + ARROW_PADDING, 64),
            Point::new(38 + ARROW_PADDING, 58),
            Point::new(38 + ARROW_PADDING, 70),
        )
        .into_styled(if pin_x_count < ARROW_LOW_THRESHOLD {
            ARROW_ON
        } else {
            ARROW_OFF
        })
        .draw(&mut display)
        .unwrap();

        // draw triangle (left)
        Triangle::new(
            Point::new(14 - ARROW_PADDING, 64),
            Point::new(26 - ARROW_PADDING, 70),
            Point::new(26 - ARROW_PADDING, 58),
        )
        .into_styled(if pin_x_count > ARROW_HIGH_THRESHOLD {
            ARROW_ON
        } else {
            ARROW_OFF
        })
        .draw(&mut display)
        .unwrap();

        // draw triangle (down)
        Triangle::new(
            Point::new(32, 82 + ARROW_PADDING),
            Point::new(38, 70 + ARROW_PADDING),
            Point::new(26, 70 + ARROW_PADDING),
        )
        .into_styled(if pin_y_count > ARROW_HIGH_THRESHOLD {
            ARROW_ON
        } else {
            ARROW_OFF
        })
        .draw(&mut display)
        .unwrap();

        // draw triangle (up)
        Triangle::new(
            Point::new(32, 46 - ARROW_PADDING),
            Point::new(38, 58 - ARROW_PADDING),
            Point::new(26, 58 - ARROW_PADDING),
        )
        .into_styled(if pin_y_count < ARROW_LOW_THRESHOLD {
            ARROW_ON
        } else {
            ARROW_OFF
        })
        .draw(&mut display)
        .unwrap();

        display.flush().unwrap();

        // sleep for frame rate
        delay.start(50.millis());
        let _ = nb::block!(delay.wait());
    }
}
