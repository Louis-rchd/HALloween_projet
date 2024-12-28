#![no_std]
#![no_main]

use embedded_hal::blocking::i2c::{Read, Write};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {} 
}
//mode esclave poru l'atmega
pub struct ATmegaI2C;

impl ATmegaI2C {
    pub fn new() -> Self {
        let twar = 0x08 << 1; //adresse esclave sur 0x08 (décalage de 1 bit à gauche)
        
        unsafe {
            core::ptr::write_volatile(0xB8 as *mut u8, twar); //écriture dans le registre TWAR
            core::ptr::write_volatile(0xBC as *mut u8, 0x85); //activation ACK et le TWI (TWCR)
        }
        
        Self
    }
}

//permission de recevoir des données
impl Read for ATmegaI2C {
    type Error = (); 

    fn read(&mut self, _address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        for byte in buffer.iter_mut() {
            unsafe {
                //attend de la fin de la transmission
                while (core::ptr::read_volatile(0xBC as *mut u8) & 0x80) == 0 {}
                //lis les données reçues
                *byte = core::ptr::read_volatile(0xBB as *mut u8); // TWDR
                //confirmation de la réception
                core::ptr::write_volatile(0xBC as *mut u8, 0x85); // TWCR
            }
        }
        Ok(())
    }
}

//permission d'envoyer des données
impl Write for ATmegaI2C {
    type Error = ();

    fn write(&mut self, _address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        for &byte in bytes {
            unsafe {
                core::ptr::write_volatile(0xBB as *mut u8, byte); 
                //transmission
                core::ptr::write_volatile(0xBC as *mut u8, 0x85);
                while (core::ptr::read_volatile(0xBC as *mut u8) & 0x80) == 0 {}
            }
        }
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut i2c = ATmegaI2C::new(); //initialisation de l'I2C Esclave
    let mut buffer = [0u8; 4]; //mémoire tampon pour stocker les données reçues

    loop {
        i2c.read(0x08, &mut buffer).unwrap(); //lecture des données envoyées par le maître
        i2c.write(0x08, b"Well received !").unwrap(); //réponse avec confirmation
    }
}
