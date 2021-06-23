#![no_std]
#![no_main]

extern crate nrf52833_hal as hal;

use core::fmt::Write;
use cortex_m::asm;
use cortex_m_rt::entry;
use hal::{
    gpio::{Output, PushPull},
    prelude::{InputPin, OutputPin},
};
use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    let p = hal::pac::Peripherals::take().unwrap();

    let p0 = hal::gpio::p0::Parts::new(p.P0);
    let p1 = hal::gpio::p1::Parts::new(p.P1);

    let cdc_pins = hal::uarte::Pins {
        txd: p0
            .p0_06
            .into_push_pull_output(hal::gpio::Level::High)
            .degrade(),
        rxd: p0.p0_08.into_floating_input().degrade(),
        cts: None,
        rts: None,
    };

    let mut uarte = hal::uarte::Uarte::new(
        p.UARTE0,
        cdc_pins,
        hal::uarte::Parity::EXCLUDED,
        hal::uarte::Baudrate::BAUD115200,
    );

    write!(uarte, "Hello, World!\r\n").unwrap();

    let heart = [
        [0, 1, 0, 1, 0],
        [1, 0, 1, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 0, 1, 0],
        [0, 0, 1, 0, 0],
    ];

    let mut row_leds = [
        p0.p0_21
            .into_push_pull_output(hal::gpio::Level::Low)
            .degrade(),
        p0.p0_22
            .into_push_pull_output(hal::gpio::Level::Low)
            .degrade(),
        p0.p0_15
            .into_push_pull_output(hal::gpio::Level::Low)
            .degrade(),
        p0.p0_24
            .into_push_pull_output(hal::gpio::Level::Low)
            .degrade(),
        p0.p0_19
            .into_push_pull_output(hal::gpio::Level::Low)
            .degrade(),
    ];

    let mut col_leds = [
        p0.p0_28
            .into_push_pull_output(hal::gpio::Level::High)
            .degrade(),
        p0.p0_11
            .into_push_pull_output(hal::gpio::Level::High)
            .degrade(),
        p0.p0_31
            .into_push_pull_output(hal::gpio::Level::High)
            .degrade(),
        p1.p1_05
            .into_push_pull_output(hal::gpio::Level::High)
            .degrade(),
        p0.p0_30
            .into_push_pull_output(hal::gpio::Level::High)
            .degrade(),
    ];

    let button_a = p0.p0_14.into_floating_input();

    rtt_init_print!();
    loop {
        if let Ok(true) = button_a.is_low() {
            for row in 0..5 {
                for col in 0..5 {
                    if heart[row][col] == 1 {
                        let _ = row_leds[row].set_high();
                        let _ = col_leds[col].set_low();
                        let _ = row_leds[row].set_low();
                        let _ = col_leds[col].set_high();
                    }
                }
            }
        }

        // rprintln!("Wait for event.");
        // asm::wfe();
    }
}
