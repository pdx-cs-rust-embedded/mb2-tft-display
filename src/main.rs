#![no_main]
#![no_std]
// Referenced https://github.com/yuri91/ili9341-rs for SPI setup

use core::default;

use cortex_m::{asm::delay, delay::{self, Delay}, prelude::_embedded_hal_spi_FullDuplex};
use cortex_m_rt::entry;
use display_interface_spi::SPIInterface;
use embedded_hal::{delay::DelayNs, digital::OutputPin, spi::{SpiBus, SpiDevice}};
use embedded_hal_bus::spi::{ExclusiveDevice};
use gc9a01::Gc9a01;
use microbit::{
    Peripherals, board::{self, Board, Edge}, display, gpio::{self, EDGE02}, hal::{
        Spim, gpio::{Level, p0::{self, P0_02, Parts}}, spi::Spi, spim::{self, Frequency}, time::MegaHertz, timer::Timer
    }, pac::{
        SPIM0, SPIM1, spim0::{
            self,
            orc::{ORC_R, ORC_SPEC},
        }
    }
};
use mipidsi::models::GC9A01;
use nrf52833_pac::SPI0;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use embedded_graphics::{self, prelude::Point};

enum State {
    LedOn,
    LedOff,
}

#[entry]
fn init() -> ! {
    rtt_init_print!();
    let peripherals = microbit::pac::Peripherals::take().unwrap();

    // let board = microbit::Board::take().unwrap();
    let timer = Timer::new(board.TIMER0);

    // Put port 0 pins into gpio list
    let port0 = Parts::new(peripherals.P0);

    let sck = board.pins.p0_17.into_push_pull_output(Level::Low).degrade();
    let mosi = board.pins.p0_13.into_push_pull_output(Level::Low).degrade();

    let dc = board.edge.e16.into_push_pull_output(Level::Low);
    let cs = board.edge.e12.into_push_pull_output(Level::Low);
    let mut rst = board.edge.e09.into_push_pull_output(Level::Low);

    let spi_bus = Spim::new(
        microbit::hal::pac::SPIM0,
        microbit::hal::spim::Pins {
            sck: Some(sck),
            mosi: Some(mosi),
            miso: None,
        },
        Frequency::M8,
        spim::MODE_0,
        0xFF
    );

    let spi = display_interface_spi::SPIInterface::new(
        ExclusiveDevice::new_no_delay(
            spi_bus,
            cs,
        ).unwrap(),
        dc,
    );

    let mut display = Gc9a01::new(
        spi,
        gc9a01::prelude::DisplayResolution240x240,
        gc9a01::prelude::DisplayRotation::Rotate180,
    );
    display.reset(&mut rst, &mut Timer::new(board.TIMER1));

    // display.init(&mut timer).unwrap();
    match display.bounded_draw(&[0xFF; 240*240], 240, (0, 0), (0, 0)) {
        Ok(_) => {}
        Err(e) => {}
    }

    loop {}
}
