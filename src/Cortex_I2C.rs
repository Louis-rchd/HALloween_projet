#![no_std]
#![no_main]

use embedded_hal::blocking::i2c::{Write, Read};
use core::panic::PanicInfo;
use cortex_m::asm; 

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

//contrôleur I2C
pub struct CortexI2C {
    i2c1: stm32f1::stm32f103::I2C1, //pointeur vers I2C1
}

impl CortexI2C {
    //intialisation de l'I2C en mode maître
    pub fn new(peripherals: stm32f1::stm32f103::Peripherals) -> Self {
        let i2c1 = peripherals.I2C1;

        //activation de l'horloge I2C1
        peripherals.RCC.apb1enr.modify(|_, w| w.i2c1en().set_bit());

        //configuration pour 100 kHz
        i2c1.cr2.write(|w| w.freq().bits(36)); //fréquence horloge : 36 MHz
        i2c1.ccr.write(|w| w.ccr().bits(180)); //pour 100 kHz
        i2c1.trise.write(|w| w.trise().bits(37));

        //activation I2C
        i2c1.cr1.write(|w| w.pe().set_bit());

        CortexI2C { i2c1 }
    }
}


impl Write for CortexI2C {
    type Error = ();

    //envoi de données à un esclave
    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        //début transmission
        self.i2c1.cr1.modify(|_, w| w.start().set_bit());
        while self.i2c1.sr1.read().sb().bit_is_clear() {}

        //envoyer l'adresse
        self.i2c1.dr.write(|w| w.dr().bits(address << 1));
        while self.i2c1.sr1.read().addr().bit_is_clear() {}
        let _ = self.i2c1.sr2.read(); // Réinitialise ADDR

        //envoyer les données
        for &byte in bytes {
            self.i2c1.dr.write(|w| w.dr().bits(byte));
            while self.i2c1.sr1.read().tx_e().bit_is_clear() {}
        }

        //fin transmission
        self.i2c1.cr1.modify(|_, w| w.stop().set_bit());
        Ok(())
    }
}

impl Read for CortexI2C {
    type Error = ();

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        //début de transmission
        self.i2c1.cr1.modify(|_, w| w.start().set_bit());
        while self.i2c1.sr1.read().sb().bit_is_clear() {}

        //envoi l'adresse en mode lecture
        self.i2c1.dr.write(|w| w.dr().bits((address << 1) | 1));
        while self.i2c1.sr1.read().addr().bit_is_clear() {}
        let _ = self.i2c1.sr2.read();

        //lecture
        for i in 0..buffer.len() {
            while self.i2c1.sr1.read().rx_ne().bit_is_clear() {}
            buffer[i] = self.i2c1.dr.read().dr().bits();
        }
        //envoi d'un NACK après le dernier octet
        self.i2c1.cr1.modify(|_, w| w.ack().clear_bit());
        self.i2c1.cr1.modify(|_, w| w.stop().set_bit());

        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let peripherals = unsafe { stm32f1::stm32f103::Peripherals::steal() };
    let mut i2c = CortexI2C::new(peripherals);

    let mut read_buffer = [0u8; 4]; //mémoire tampon pour les données reçues

    loop {
        //envoi des données à l'esclave
        i2c.write(0x08, &[0x01, 0x02, 0x03]).unwrap();

        //délai avant lecture
        asm::delay(8_000_000);

        //lecture
        i2c.read(0x08, &mut read_buffer).unwrap();
        asm::delay(8_000_000);
    }
}
