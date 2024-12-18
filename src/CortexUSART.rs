// de la même manière que pour l'Atmega

use cortex_m_rt::entry;
use embedded_hal::serial; 
use nb::block;

#[entry]
pub fn cortex_usart() -> ! {
    // Initialisation des périphériques et de l'horloge
    let dp = pac::Peripherals::take().unwrap();
    let clocks = dp.RCC.constrain().cfgr.sysclk(216.mhz()).freeze(); // Système du STM32 à 216 MHz

    // Configuration de l'USART1 avec TX sur PA9 et RX sur PA10
    let gpioa = dp.GPIOA.split();
    let tx_pin = gpioa.pa9.into_alternate();
    let rx_pin = gpioa.pa10.into_alternate();
    
    // Remplacer Serial avec une abstraction générique
    let mut serial = stm32f7xx_hal::serial::Serial::usart1(
        dp.USART1,
        (tx_pin, rx_pin),
        Config::default().baudrate(9600.bps()),
        clocks,
    )
    .unwrap();

    let message = "Hello from Cortex-M7!\n";

    loop {
        // Envoi du message via TX
        nb::block!(serial.write_str(message)).unwrap();

        // Lecture du message reçu via RX
        if let Ok(received) = block!(serial.read()) {
            block!(serial.write(received)).unwrap();
        }
    }
}
