// Only valid for a few chips - put this behind feature gates when generalizing.
/// ID numbers for peripherals
#[repr(u32)]
pub enum PeripheralId {
    /// Supply controller
    SUPC = 0,
    /// Reset controller
    RSTC = 1,
    /// Real time clock
    RTC = 2,
    /// Real time timer
    RTT = 3,
    /// Watchdog timer
    WDT = 4,
    /// Power management controller
    PMC = 5,
    /// Enhanced flash controller 0
    EFC0 = 6,
    /// Enhanced flash controller 1
    EFC1 = 7,
    /// Universal asynchronous receiver transceiver
    UART = 8,
    /// Static memory controller
    SMC = 9,
    /// Synchronous dynamic RAM controller
    SDRAMC = 10,
    /// Parallel I/O controller A
    PIOA = 11,
    /// Parallel I/O controller B
    PIOB = 12,
    #[cfg(feature = "pioc")]
    /// Parallel I/O controller C
    PIOC = 13,
    #[cfg(feature = "piod")]
    /// Parallel I/O controller D
    PIOD = 14,
    #[cfg(feature = "pioe")]
    /// Parallel I/O controller E
    PIOE = 15,
    #[cfg(feature = "piof")]
    /// Parallel I/O controller F
    PIOF = 16,
    /// USART 0
    USART0 = 17,
    /// USART 1
    USART1 = 18,
    /// USART 2
    USART2 = 19,
    /// USART 3
    USART3 = 20,
    /// Multimedia card interface
    HSMCI = 21,
    /// Two-wire interface 0
    TWI0 = 22,
    /// Two-wire interface 1
    TWI1 = 23,
    /// Serial peripheral interface 0
    SPI0 = 24,
    /// Serial peripheral interface 1
    SPI1 = 25,
    /// Synchronous serial controller
    SSC = 26,
    /// Timer counter 0
    TC0 = 27,
    /// Timer counter 1
    TC1 = 28,
    /// Timer counter 2
    TC2 = 29,
    /// Timer counter 3
    TC3 = 30,
    /// Timer counter 4
    TC4 = 31,
    /// Timer counter 5
    TC5 = 32,
    /// Timer counter 6
    TC6 = 33,
    /// Timer counter 7
    TC7 = 34,
    /// Timer counter 8
    TC8 = 35,
    /// Pulse width modulation controller
    PWM = 36,
    /// Analog to digital controller
    ADC = 37,
    /// Digital to analog controller
    DACC = 38,
    /// Direct memory access controller
    DMAC = 39,
    /// USB OTG high speed
    UOTGHS = 40,
    /// True random number generator
    TRNG = 41,
    /// Ethernet MAC
    EMAC = 42,
    /// Controller area network controller 0
    CAN0 = 43,
    /// Controller area network controller 1
    CAN1 = 44,
}
