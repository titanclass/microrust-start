#![no_std]
#![no_main]

extern crate nrf52833_hal as hal;

use core::{cell::RefCell, fmt::Write};
use cortex_m::{asm, interrupt::Mutex};
use cortex_m_rt::entry;
use hal::{pac::interrupt, prelude::*};
#[cfg(debug_assertions)]
use panic_probe as _;
#[cfg(not(debug_assertions))]
use panic_reset as _;
use rtt_target::{rprintln, rtt_init_print};

static GPIOTE: Mutex<RefCell<Option<hal::gpiote::Gpiote>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn GPIOTE() {
    cortex_m::interrupt::free(|cs| {
        let gpiote = GPIOTE.borrow(cs).borrow();
        if let Some(gpiote) = gpiote.as_ref() {
            gpiote.reset_events();
        }
    });
}

#[entry]
fn main() -> ! {
    let mut cp = hal::pac::CorePeripherals::take().unwrap();
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

    unsafe {
        hal::pac::NVIC::unmask(hal::pac::Interrupt::GPIOTE);
        cp.NVIC.set_priority(hal::pac::Interrupt::GPIOTE, 32 << 5);
    }

    let button_a = p0.p0_14.into_floating_input().degrade();
    let gpiote = hal::gpiote::Gpiote::new(p.GPIOTE);
    gpiote
        .channel0()
        .input_pin(&button_a)
        .hi_to_lo()
        .enable_interrupt();
    cortex_m::interrupt::free(|cs| {
        GPIOTE.borrow(cs).replace(Some(gpiote));
    });

    rtt_init_print!();

    let graphic = [
        [1, 1, 1, 0, 0],
        [0, 1, 0, 1, 1],
        [0, 1, 1, 0, 0],
        [0, 1, 1, 0, 0],
        [0, 0, 0, 1, 1],
    ];

    loop {
        let button_a_pressed = app::is_result_ok_and_true(button_a.is_low());
        if button_a_pressed {
            for (row, row_led) in row_leds.iter_mut().enumerate() {
                for (col, col_led) in col_leds.iter_mut().enumerate() {
                    if graphic[row][col] == 1 {
                        let _ = row_led.set_high();
                        let _ = col_led.set_low();
                        let _ = row_led.set_low();
                        let _ = col_led.set_high();
                    }
                }
            }
        } else {
            rprintln!("Wait for event.");
            asm::wfe();
        }
    }
}
