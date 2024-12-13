// de la même manière que pour l'Atmega


/*
[CORRECTION USART] (Don't hesitate to remove this part)
stm32f7xx_hal is an external HAL, and you shouldn't use it. You are supposed to do the hardware abstraction directly.
*/
use cortex_m_rt::entry;
use stm32f7xx_hal::{
    pac,
    prelude::*,
    serial::{config::Config, Serial},
};

#[entry]
pub fn cortex_usart() -> ! {
    //  // initialisation des pins et de l'horloge
    let dp = pac::Peripherals::take().unwrap();
    let clocks = dp.RCC.constrain().cfgr.sysclk(216.mhz()).freeze(); //(système du STM32 à 216 MHz)

    // configuration de l'USART1 avec TX sur PA9 et RX sur PA10
    let gpioa = dp.GPIOA.split();
    let tx_pin = gpioa.pa9.into_alternate();
    let rx_pin = gpioa.pa10.into_alternate();
    let mut serial = Serial::usart1(
        dp.USART1,
        (tx_pin, rx_pin),
        Config::default().baudrate(9600.bps()),
        clocks,
    )
        .unwrap();

    let message = "Hello from Cortex-M7!\n";

    loop {
        // envoie message depuis TX
        serial.write_str(message).unwrap();

        // recois message via RX
        if let Ok(received) = nb::block!(serial.read()) {
            nb::block!(serial.write(received)).unwrap();
        }
    }
}

