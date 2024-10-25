#![no_std]

use core::arch::asm;

pub struct GPIO;

impl GPIO {
    // Configure le pin spécifié comme entrée ou sortie
    pub fn configure(pin: u8, mode: bool) {
        assert!(pin <= 13, "Le numero du pin doit etre entre 0 et 13");

        let port = if pin < 8 { 0 } else { 1 };
        let bit = (pin % 8) as u8; // bit est calculé à l'exécution

        unsafe {
            if mode {
                // Configure comme sortie
                match port {
                    0 => asm!("sbi 0x24, {}", const(bit)), // PORTD
                    1 => asm!("sbi 0x25, {}", const(bit)), // PORTB
                    _ => unreachable!(),
                }
            } else {
                // Configure comme entrée
                match port {
                    0 => asm!("cbi 0x24, {}", const(bit)), // PORTD
                    1 => asm!("cbi 0x25, {}", const(bit)), // PORTB
                    _ => unreachable!(),
                }
            }
        }
    }

    // Écrire sur un pin
    pub fn write(pin: u8, value: bool) {
        assert!(pin <= 13, "Pin number must be between 0 and 13");

        let port = if pin < 8 { 0 } else { 1 };
        let bit = (pin % 8) as u8; // bit est calculé à l'exécution

        unsafe {
            if value {
                match port {
                    0 => asm!("sbi 0x25, {}", const(bit)), // PORTD
                    1 => asm!("sbi 0x24, {}", const(bit)), // PORTB
                    _ => unreachable!(),
                }
            } else {
                match port {
                    0 => asm!("cbi 0x25, {}", const(bit)), // PORTD
                    1 => asm!("cbi 0x24, {}", const(bit)), // PORTB
                    _ => unreachable!(),
                }
            }
        }
    }

    // Lire l'état d'un pin
    pub fn read(pin: u8) -> bool {
        assert!(pin <= 13, "Pin number must be between 0 and 13");

        let port = if pin < 8 { 0 } else { 1 };
        let bit = (pin % 8) as u8; // bit est calculé à l'exécution

        unsafe {
            match port {
                0 => (core::ptr::read_volatile(0x29 as *const u8) & (1 << bit)) != 0, // PIND
                1 => (core::ptr::read_volatile(0x23 as *const u8) & (1 << bit)) != 0, // PINB
                _ => unreachable!(),
            }
        }
    }
}

