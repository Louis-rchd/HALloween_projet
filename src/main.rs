#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

mod gpio;
mod AtmegaUSART;
mod CortexUSART;
mod Atmega_SPI;
mod Cortex_SPI;

//USART SPI
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

// Fonction de délai (utile uniquement si vous utilisez GPIO)
fn delay() {
    for _ in 0..1_000_000 {
        unsafe { asm!("nop"); } // Instruction vide pour créer un délai
    }
}

// Gestion de panique
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


