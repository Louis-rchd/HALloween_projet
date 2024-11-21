//on cherche à envoyer un message sur la broche TX et lire les données reçues sur la broche RX.

use arduino_hal::prelude::*;

#[arduino_hal::entry]
pub fn atmega_usart() -> ! {

    // initialisation des pins
    let dp = arduino_hal::Peripherals::take().unwrap();
    let mut pins = arduino_hal::pins!(dp);

    // configuration de l'USART avec TX sur D1 et RX sur D0
    let mut serial = arduino_hal::usart::Usart0::new(
        dp.USART0,
        pins.d0.into_input(&mut pins.ddr), // RX
        pins.d1.into_output(&mut pins.ddr), // TX
        9600, // baudrate
    );

    let message = b"Message from ATmega328p!\n";

    loop {
        // envoie un tableau d'octets depuis TX
        serial.write_bytes(message).unwrap();

        // lis le message recu sur RX
        if let Ok(received) = serial.read() {
            // retourne en écho du message reçu
            serial.write(received).unwrap();
        }
    }
}

