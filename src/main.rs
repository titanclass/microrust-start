#![no_std]
#![no_main]

extern crate nrf52833_hal as hal;

use core::fmt::Write;
use cortex_m::asm;
use cortex_m_rt::entry;
use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    let p = hal::pac::Peripherals::take().unwrap();

    let p0 = hal::gpio::p0::Parts::new(p.P0);
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

    rtt_init_print!();
    loop {
        rprintln!("Sleeping!");
        asm::wfe();
    }
}
