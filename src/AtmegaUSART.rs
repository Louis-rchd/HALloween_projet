//on cherche à envoyer un message sur la broche TX et lire les données reçues sur la broche RX.

use embedded_hal::serial;

#[no_mangle]
pub fn atmega_usart() -> ! {

    // Initialisation des périphériques
    let dp = arduino_hal::Peripherals::take().unwrap();
    let mut pins = arduino_hal::pins!(dp);

    // Configuration de l'USART avec TX sur D1 et RX sur D0
    let mut serial = arduino_hal::usart::Usart0::new(
        dp.USART0,
        pins.d0.into_input(&mut pins.ddr), // RX
        pins.d1.into_output(&mut pins.ddr), // TX
        9600, // Baudrate
    );

    let message = b"Hello from ATmega328p!\n";

    loop {
        // Envoi d'un tableau d'octets depuis TX
        serial.write_bytes(message).unwrap();

        // Lecture du message reçu sur RX
        if let Ok(received) = serial.read() {
            // Retour en écho du message reçu
            serial.write(received).unwrap();
        }
    }
}
