// Configuration SPI pour STM32F746 
use embedded_hal::spi::{FullDuplex, Mode, Polarity, Phase};
use stm32f7::stm32f746::{SPI1, GPIOA};

//configuration et initialisation du SPI
pub fn setup_spi() -> impl FullDuplex<u8> {
    let dp = stm32f7::stm32f746::Peripherals::take().unwrap();
    // Activation des horloges pour SPI et GPIOA
    dp.RCC.ahb1enr.modify(|_, w| w.gpioaen().set_bit());
    dp.RCC.apb2enr.modify(|_, w| w.spi1en().set_bit());

    //configuration des broches GPIOA pour SPI
  //en alternatif
    let gpioa = &dp.GPIOA;
    gpioa.moder.modify(|_, w| {
        w.moder5().bits(0b10); // PA5 (SCK) 
        w.moder7().bits(0b10); // PA7 (MOSI) 
        w.moder6().bits(0b10); // PA6 (MISO)
        w
    });
//en push-pull
    gpioa.otyper.modify(|_, w| {
        w.ot5().clear_bit(); // PA5 (SCK) 
        w.ot7().clear_bit(); // PA7 (MOSI) 
        w.ot6().clear_bit(); // PA6 (MISO)
        w
    });

    gpioa.afrl.modify(|_, w| {
        w.afrl5().bits(0b0100); // PA5 (SCK) assigné à l'AF4
        w.afrl7().bits(0b0100); // PA7 (MOSI) assigné à l'AF4
        w.afrl6().bits(0b0100); // PA6 (MISO) assigné à l'AF4
        w
    });

    //configuration des registres SPI
    let spi = &dp.SPI1;
    //configure le registre "CR1" pour définir les paramètres SPI
    spi.cr1.write(|w| {
        w.spe().clear_bit(); 
        w.mstr().set_bit();  
        w.ssi().set_bit();  
        w.lsbfirst().clear_bit(); 
        w.br().bits(0b011);  
        w.cpol().clear_bit(); 
        w.cpha().clear_bit(); 
        w.spe().set_bit();   
        w
    });
    spi 
}
