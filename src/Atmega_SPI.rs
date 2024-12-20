//SPI configuration pour Atmega328p 
use embedded_hal::spi::{FullDuplex, Mode, Phase, Polarity};
use avr_hal_generic::spi::{Spi, Config};
use avr_hal_generic::port::{PB5, PB6, PB7};

//initialisation du périphérique SPI, 
pub fn setup_spi() -> impl FullDuplex<u8> {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let mut pins = arduino_hal::pins!(dp);

    //configuration de MOSI sur D11, MISO sur D12 et SCK sur D13
    let spi = Spi::new(
        pins.d13.into_output(&mut pins.ddr), // SCK
        pins.d12.into_input(&mut pins.ddr),   // MISO
        pins.d11.into_output(&mut pins.ddr),  // MOSI
        Config::default()
            .baudrate(1_000_000.bps())  // Baudrate = 1 Mbps
            .mode(Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            }),
    );
    spi
}
