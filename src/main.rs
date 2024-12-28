#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

mod gpio;
mod AtmegaUSART;
mod CortexUSART;
mod Atmega_SPI;
mod Cortex_SPI;
#[cfg(feature = "atmega328p_i2c")]
mod ATmega_I2C;
#[cfg(feature = "cortex_i2c")]
mod Cortex_I2C;

//USART SPI ATMEGA
#[cfg(feature = "atmega")]
#[no_mangle]
pub extern "C" fn main() -> ! {
    // Initialisation de l'USART et SPI pour Atmega
    AtmegaUSART::atmega_usart();
    let mut spi = Atmega_SPI::setup_spi();

    loop {
        //test
        let _ = spi.send(0x42); //envoi
        if let Ok(data) = spi.read() {
            //lecture
            let _ = data;
        }
    }
}

//USART SPI CORTEX
#[cfg(feature = "cortex")]
#[no_mangle]
pub extern "C" fn main() -> ! {
    // Initialisation de l'USART et SPI pour Cortex-M
    CortexUSART::cortex_usart();
    let mut spi = Cortex_SPI::setup_spi();

    loop {
        //test
        let _ = spi.send(0x42);
        if let Ok(data) = spi.read() {
            let _ = data;
        }
    }
}


//I2C ATMEGA
#[cfg(feature = "atmega328p_i2c")]
#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut i2c = ATmega_I2C::ATmegaI2C::new();
    let mut buffer = [0u8; 4];

    loop {
        i2c.read(0x08, &mut buffer).unwrap(); //lecture depuis le cortex
        i2c.write(0x08, b"Well received!").unwrap(); // envoi d'une réponse
    }
}

//I2C CORTEX
[cfg(feature = "cortex_i2c")]
#[no_mangle]
pub extern "C" fn main() -> ! {
    let peripherals = unsafe { stm32f1::stm32f103::Peripherals::steal() };
    let mut i2c = Cortex_I2C::CortexI2C::new(peripherals);

    loop {
        i2c.write(0x08, &[0x01, 0x02, 0x03]).unwrap(); //envoi vers atmega
        delay();
    }
}



//GPIO led
#[no_mangle]
pub extern "C" fn main() -> ! {
    // Configure le pin 13 comme sortie
    gpio::GPIO::configure(13, true);

    // Configure le pin 12 comme entrée
    gpio::GPIO::configure(12, false);

    loop {
        // Écrire HIGH sur le pin 13
        gpio::GPIO::write(13, true);

        delay();

        // Écrire LOW sur le pin 13
        gpio::GPIO::write(13, false);

        delay();

        // Lire l'état du pin 12
        let _pin_value = gpio::GPIO::read(12);
    }
}

//délai pour gpio
fn delay() {
    for _ in 0..1_000_000 {
        unsafe { asm!("nop"); } //instruction vide pour créer un délai
    }
}

// Gestion de panique
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


