//! # Functional description of the DACC
//!
//! Mostly pages 1358-1359 of the datasheet verbatim, though I have taken some liberties by way of
//! rephrasing some things so they make a bit more sense to me.
//!
//!
//! ## Digital-to-Analog Conversion
//!
//! The DACC uses the master clock (MCK) divided by two to perform coversions. This clock is named
//! DACC Clock. Once a conversion starts, the DACC takes 25 clock periods to provide the analog
//! result on the selected analog output.
//!
//!
//! ## Conversion Results
//!
//! When a conversion is completed, the resulting analog value is available at the selected DACC
//! channel output and the EOC bit in the DACC Interrupt Status Register is set. Reading the
//! `DACC_ISR` clears the EOC bit.
//!
//!
//! ## Conversion triggers
//!
//! In free running mode, conversion starts as soon as at least one channel is enabled and data is
//! written in the DACC Conversion Data Register. Then 25 DACC Clock periods later, the converted
//! data is available at the corresponding analog output as stated above.
//!
//! In external trigger mode, the conversion waits for a rising edge on the selected trigger to
//! begin.
//!
//! **WARNING:** Disabling thee external trigger mode automatically sets the DACC in free running
//! mode.
//!
//!
//! ## Conversion FIFO
//!
//! A 4 half-word FIFO is used to handle the data to be converted.
//!
//! As long as the `TXRDY` flag in the DACC Interrupt Status Register is active, the DAC Controller
//! is ready to accept conversion requests by writing data into the DACC Conversion Data Register.
//! Data which cannot be converted immediately are stored in the DACC FIFO.
//!
//! When the FIFO is full or the DACC is not ready to accept conversion requests, the `TXRDY` flag
//! is inactive.
//!
//! The `WORD` field of the DACC Mode Register allows the user to switch between half-word and word
//! transfer for writing into the FIFO.
//!
//! In half-word transfer mode, only the 16 LSB of `DACC_CDR` data are taken into account,
//! `DACC_CDR[15:0]` is stored in the FIFO.
//!
//! The `DACC_CDR[11:0]` field is used as data and the `DACC_CDR[15:12]` bits are used for channel
//! selection if the `TAG` field is set in the `DACC_MR` register.
//!
//! In word transfer mode, each time the `DACC_CDR` register is written to, 2 data items are stored
//! in the FIFO. The first data item sampled for conversion is `DACC_CDR[15:0]` and the second is
//! `DACC_CDR[31:16]`.
//!
//! Fields `DACC_CDR[15:12]` and `DACC_CDR[31:28]` are used for channel selection if the `TAG` field
//! is set in the `DACC_MR` register.
//!
//! **WARNING:** Writing in the `DACC_CDR` register while the `TXRDY` flag is inactive will corrupt
//! FIFO data.
//!
//!
//! ## Channel Selection
//! There are two means by which to select the channel to perform data conversion.
//!     - The default method is to use the `USER_SEL` field of the DACC Mode Register. Data requests
//!         will merely be converted to the channel selected with the `USER_SEL` field.
//!     - A more flexible option is to select the channel for the data to be converted into is to
//!         use the tag mode, setting the `TAG` field of the DACC Mode REgister to `1`. In this mode
//!         the two bits `DACC_CDR[13:12]`, which are otherwise unused, are employed to select the
//!         output channel in the same way as the `USER_SEL` field. Finally, if the `WORD` field is
//!         set, the 2 bits `DACC_CDR[13:12]` are used for channel selection for the first datum and
//!         `DACC_CDR[29:28]` are used for the channel selection of the second.
//!
//!
//! ## Sleep Mode
//!
//! The DACC Sleep Mode maximizes power saving by automatically deactivating the DACC when it is not
//! being used for conversions.
//!
//! When a start conversion request occurs, the DACC is automatically activated. As the analog cell
//! requires a start-up time, the logic waits during this time and starts the conversion on the
//! selected channel. When all conversion requests are complete, the DACC is deactivated until the
//! next request for conversion.
//!
//! A fast wake-up mode is available in the DACC Mode Register as a compromise between power saving
//! strategy and responsiveness. Setting the `FASTW` bit to `1` enables the fast wake-up mode. In
//! fast wake-up mode the DACC is not fully deactivated while no conversion is requested, thereby
//! providing less power saving but faster wake-up (4 times faster).
//!
//!
//! ## DACC Timings
//!
//! The DACC startup time must be defined by the user in the `STARTUP` field of the DACC Mode
//! Register.
//!
//! This startup time differs depending on the use of the fast wake-up mode along with sleep mode,
//! in this case the user must set the `STARTUP` time according to the fast wake up and not the
//! standard startup time.
//!
//! A max speed mode is available by setting the `MAXS` bit to `1` in the `DACC_MR` register. Using
//! this mode, the DAC Controller no longer waits to sample the end of cycle signal coming from the
//! DACC block to stasrt the next conversion and uses an internal counter instead. This mode gains 2
//! DACC Clock periods between each consecutive conversion.
//!
//! **WARNING:** Using this mode, the `EOC` interrupt of the `DACC_IER` register should not be used
//! as it is 2 DACC Clock periods late.
//!
//! After 20μs the analog voltage resulting from the converted data will start decreasing, therefore
//! it is necessary to refresh the channel on a regular basis to prevent this voltage loss. This is
//! the purpose of the `REFRESH` field in the DACC Mode Register, where the user will define the
//! period for the analog channels to be refreshed.
//!
//! **WARNING:** A `REFRESH` period set to `0` will disable the refresh function of the DACC channels.
//!
//!
//! ## Write Protection Registers
//!
//! In order to provide security to the DACC, a write protection system has been implemented.
//!
//! The write protection mode prevents the writing of certain registers. While this mode is enabled
//! and one of the protected registers is written, an error is generated in the DACC Write Protect
//! Status Register and the register write request is cancelled. When a write protection error
//! occurs, the `WPROTERR` flag is set and the address of the corresponding cancelled register write
//! is available in the `WPROTADDR` field of the DACC Write Protect Status Register.
//!
//! Due to the nature of the write protection feature, enabling and disabling the write protection
//! mode requires the use of a security code. Thus when enabling or disabling the write protection
//! mode the `WPKEY` field of the DACC Write Protect Mode Register must be filled with the "DAC"
//! ASCII code (corresponding to `0x444143`), otherwise the register write is cancelled.
//!
//! The protected registers are:
//!     - Mode Register (`MR`)
//!     - Channel Enable Register (`CHER`)
//!     - Channel Disable Register (`CHDR`)
//!     - Analog Current Register (`ACR`)

use super::cdr_data::CdrData;
use crate::{
    pac::{dacc::mr::*, DACC, PMC},
    peripheral_id::PeripheralId,
    pmc::*,
    write_protect::{wp_impl, WriteProtect},
};

pub const DACC_PID: u32 = PeripheralId::DACC as u32;

/// Digital to analog converter controller
pub struct Dacc {
    dacc: DACC,
}

wp_impl! {
    ///   - Mode register (`DACC_MR`)
    ///   - Channel enable register (`DACC_CHER`)
    ///   - Channel disable register (`DACC_CHDR`)
    ///   - Analog current register (`DACC_ACR`)
    Dacc => dacc(wproterr, wprotaddr<u8>): b"DAC",
}

impl Dacc {
    /// Make a new DACC converter controller. None of the initialisation necessary to make use of
    /// the DACC is performed.
    pub fn new(dacc: DACC) -> Self {
        Self { dacc }
    }

    /// Enable the clock for the DACC peripheral.
    pub fn enable_dacc_clock(&self, pmc: &mut PMC) {
        unsafe { enable_peripheral_clk(pmc, DACC_PID) };
    }

    /// Disable the clock for the DACC peripheral.
    pub fn disable_dacc_clock(&self, pmc: &mut PMC) {
        unsafe { disable_peripheral_clk(pmc, DACC_PID) };
    }

    /// Perform a software reset the DACC peripheral.
    pub fn reset(&mut self) {
        unsafe {
            self.dacc
                .cr
                .write_with_zero(|cr_reg| cr_reg.swrst().set_bit())
        };
    }

    /// Attempt to set the conversion trigger. Fails if write protection is enabled.
    ///
    /// Briefly, each trigger is as follows:
    ///
    ///   - `TRGSEL_A::EXTERN`: Conversions triggered by external signal
    ///   - `TRGSEL_A::TIOOTCC0`: **T**imer **I**/**O** **o**utput of the **t**imer **c**ounter
    ///       **c**hannel **0**
    ///   - `TRGSEL_A::TIOOTCC1`: **T**imer **I**/**O** **o**utput of the **t**imer **c**ounter
    ///       **c**hannel **1**
    ///   - `TRGSEL_A::TIOOTCC2`: **T**imer **I**/**O** **o**utput of the **t**imer **c**ounter
    ///       **c**hannel **2**
    ///   - `TRGSEL_A::PWM0`: **PWM** event line **0**
    ///   - `TRGSEL_A::PWM1`: **PWM** event line **1**
    pub fn set_trigger(&mut self, trigger: TRGSEL_A_A) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.set_trigger_unchecked(trigger) };
            Ok(())
        }
    }

    /// Attempt to set the external trigger without checking the write protection register.
    ///
    /// # Safety
    ///
    /// Operation failure is silent apart from setting the `WPROTERR` flag in the `WPSR` register.
    ///
    /// May overwrite data in the `WPROTADDR` field of the `WPSR` register if the operation fails.
    #[rustfmt::skip]
    pub unsafe fn set_trigger_unchecked(&mut self, trigger: TRGSEL_A_A) {
        self.dacc.mr.write(|mode_reg| {
            mode_reg
                .trgen().variant(TRGEN_A::En)
                .trgsel().variant(trigger)
        });
    }

    /// Attempt to disable external triggering. Fails if write protection is enabled.
    pub fn disable_trigger(&mut self) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.disable_trigger_unchecked() };
            Ok(())
        }
    }

    /// Attempt to disable external triggering without checking the write protection register.
    ///
    /// # Safety
    ///
    /// Operation failure is silent apart from setting the `WPROTERR` flag in the `WPSR` register.
    ///
    /// May overwrite data in the `WPROTADDR` field of the `WPSR` register if the operation fails.
    pub unsafe fn disable_trigger_unchecked(&mut self) {
        self.dacc
            .mr
            .write(|mode_reg| mode_reg.trgen().variant(TRGEN_A::Dis));
    }

    /// Attempt to set the transfer mode. Fails if write protection is enabled.
    ///
    /// The behaviour of these is described on page 1358 of the datasheet, found
    /// [here](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11057-32-bit-Cortex-M3-Microcontroller-SAM3X-SAM3A_Datasheet.pdf).
    pub fn set_transfer_mode(&mut self, mode: WORD_A) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.set_transfer_mode_unchecked(mode) };
            Ok(())
        }
    }

    /// Attempt to set the transfer mode without checking the write protection register.
    ///
    /// # Safety
    ///
    /// Operation failure is silent apart from setting the `WPROTERR` flag in the `WPSR` register.
    ///
    /// May overwrite data in the `WPROTADDR` field of the `WPSR` register if the operation fails.
    pub unsafe fn set_transfer_mode_unchecked(&mut self, mode: WORD_A) {
        self.dacc.mr.write(|mode_reg| mode_reg.word().variant(mode));
    }

    /// Get the current transfer mode.
    pub fn get_transfer_mode(&self) -> WORD_A {
        self.dacc.mr.read().word().variant()
    }

    /// Enable interrupts flags. A brief description of each interrupt is as follows:
    ///
    ///   - `txrdy`: Transmit ready
    ///   - `eoc`: End of conversion
    ///   - `endtx`: End of transmit buffer
    ///   - `txbufe`: Transmit buffer empty
    pub fn enable_interrupts(&mut self, enable: DaccInterrupts) {
        unsafe {
            self.dacc.ier.write_with_zero(|int_en_reg| {
                if enable.txrdy {
                    int_en_reg.txrdy().set_bit();
                }
                if enable.eoc {
                    int_en_reg.eoc().set_bit();
                }
                if enable.endtx {
                    int_en_reg.endtx().set_bit();
                }
                if enable.txbufe {
                    int_en_reg.txbufe().set_bit();
                }
                int_en_reg
            });
        }
    }

    /// Disable interrupt flags.
    pub fn disable_interrupts(&mut self, disable: DaccInterrupts) {
        unsafe {
            self.dacc.idr.write_with_zero(|int_dis_reg| {
                if disable.txrdy {
                    int_dis_reg.txrdy().set_bit();
                }
                if disable.eoc {
                    int_dis_reg.eoc().set_bit();
                }
                if disable.endtx {
                    int_dis_reg.endtx().set_bit();
                }
                if disable.txbufe {
                    int_dis_reg.txbufe().set_bit();
                }
                int_dis_reg
            });
        }
    }

    /// Get current interrupt mask.
    pub fn get_interrupts_mask(&self) -> DaccInterrupts {
        let imr = self.dacc.imr.read();
        DaccInterrupts {
            txrdy: imr.txrdy().bit(),
            eoc: imr.eoc().bit(),
            endtx: imr.endtx().bit(),
            txbufe: imr.txbufe().bit(),
        }
    }

    /// Get current interrupt status.
    pub fn get_interrupts_status(&self) -> DaccInterrupts {
        let isr = self.dacc.isr.read();
        DaccInterrupts {
            txrdy: isr.txrdy().bit(),
            eoc: isr.eoc().bit(),
            endtx: isr.endtx().bit(),
            txbufe: isr.txbufe().bit(),
        }
    }

    /// Read the `TXRDY` (transmit ready) interrupt flag.
    ///
    ///   - `false`: DACC is not ready to accept new conversion requests.
    ///   - `true`: DACC is ready to accept new conversion requests.
    pub fn transmit_ready(&self) -> bool {
        self.dacc.isr.read().txrdy().bit()
    }

    /// Read the `EOC` (end of conversion) interrupt flag.
    ///
    ///   - `false`: No conversion has been performed since the last
    ///       [`DACC_ISR`](crate::pac::dacc::isr) read.
    ///   - `true`: At least one conversion has been performed since the last
    ///       [`DACC_ISR`](crate::pac::dacc::isr) read.
    pub fn end_of_conversion(&self) -> bool {
        self.dacc.isr.read().eoc().bit()
    }

    /// Read the `ENDTX` (end of DMA) interrupt flag.
    ///
    ///   - `false`: The transmit counter register has not reached 0 since the last write in
    ///       [`DACC_TCR`](crate::pac::dacc::tcr) or [`DACC_TNCR`](crate::pac::dacc::tncr).
    ///   - `true`: The transmit counter register has reached 0 since the last write in
    ///       [`DACC_TCR`](crate::pac::dacc::tcr) or [`DACC_TNCR`](crate::pac::dacc::tncr).
    pub fn end_of_transmit_buffer(&self) -> bool {
        self.dacc.isr.read().endtx().bit()
    }

    /// Read the `TXBUFE` (transmit buffer empty) interrupt flag.
    ///
    ///   - `false`: The transmit counter register has not reached 0 since the last write in
    ///       [`DACC_TCR`](crate::pac::dacc::tcr) or [`DACC_TNCR`](crate::pac::dacc::tncr).
    ///   - `true`: The transmit counter register has reached 0 since the last write in
    ///       [`DACC_TCR`](crate::pac::dacc::tcr) or [`DACC_TNCR`](crate::pac::dacc::tncr).
    pub fn transmit_buffer_empty(&self) -> bool {
        self.dacc.isr.read().txbufe().bit()
    }

    /// Write data to be converted into the convert data register. The write will succeed only if
    /// the `TXRDY` flag is set.
    pub fn write_conversion_data<D: CdrData>(&mut self, data: D) -> DaccResult {
        if self.transmit_ready() {
            unsafe { self.write_conversion_data_unchecked(data) };
            Ok(())
        } else {
            Err(DaccError::NotTransmitReady)
        }
    }

    /// Write data to be converted into the convert data register without checking whether the
    /// `TXRDY` flag is set.
    ///
    /// # Safety
    ///
    /// If the `TXRDY` flag is not set, writing to the conversion data register may corrupt the
    /// DACC FIFO.
    pub unsafe fn write_conversion_data_unchecked<D: CdrData>(&mut self, data: D) {
        self.dacc.cdr.write(|cdr| cdr.bits(data.bits()));
    }

    /// Attempt to set startup time. Fails if write protection is enabled.
    ///
    /// Actual startup time values can be in table 45-41 (page 1411) of the datasheet, found
    /// [here](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11057-32-bit-Cortex-M3-Microcontroller-SAM3X-SAM3A_Datasheet.pdf).
    pub fn set_startup(&mut self, startup: STARTUP_A) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.set_startup_unchecked(startup) };
            Ok(())
        }
    }

    /// Attempt to set startup time without checking the write protection register.
    ///
    /// # Safety
    ///
    /// Operation failure is silent apart from setting the `WPROTERR` flag in the `WPSR` register.
    ///
    /// May overwrite data in the `WPROTADDR` field of the `WPSR` register if the operation fails.
    pub unsafe fn set_startup_unchecked(&mut self, startup: STARTUP_A) {
        self.dacc
            .mr
            .write(|mode_reg| mode_reg.startup().variant(startup));
    }

    /// Attempt to set the refresh period length. Fails if write protection is enabled.
    ///
    /// The actual refresh period length is calculated as follows:
    ///
    /// `Refresh period = (1024 * refresh) / DACC Clock`
    pub fn set_refresh(&mut self, refresh: u8) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.set_refresh_unchecked(refresh) };
            Ok(())
        }
    }

    /// Attempt to set the refresh period length without checking the writeprotect register.
    ///
    /// # Safety
    ///
    /// Operation failure is silent apart from setting the `WPROTERR` flag in the `WPSR` register.
    ///
    /// May overwrite data in the `WPROTADDR` field of the `WPSR` register if the operation fails.
    pub unsafe fn set_refresh_unchecked(&mut self, refresh: u8) {
        self.dacc
            .mr
            .write(|mode_reg| mode_reg.refresh().bits(refresh));
    }

    /// Attempt to select the output channel to write CDR values to. Fails if write protection is
    /// enabled.
    pub fn select_channel(&mut self, channel: USER_SEL_A) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.select_channel_unchecked(channel) };
            Ok(())
        }
    }

    /// Attempt to select the output channel without checking the write protection register.
    ///
    /// # Safety
    ///
    /// Operation failure is silent apart from setting the `WPROTERR` flag in the `WPSR` register.
    ///
    /// May overwrite data in the `WPROTADDR` field of the `WPSR` register if the operation fails.
    pub unsafe fn select_channel_unchecked(&mut self, channel: USER_SEL_A) {
        self.dacc
            .mr
            .write(|mode_reg| mode_reg.user_sel().variant(channel));
    }

    /// Attempt to enable use of tag bits in CDR halfwords. Fails if write protection is enabled.
    ///
    /// Tag bits are the two bits that follow the 12 bit value in each halfword. Specifically, the
    /// breakdown of each halfword is:
    ///
    ///   - `halfword[11:0]`: 12 bit value used for DAC output
    ///   - `halfword[13:12]`: tag bits
    ///   - `halfword[15:14]`: unused
    ///
    /// This is the same for the upper halfword in the CDR if the DACC is set to `WORD` mode, just
    /// the bit ranges are `[27:16]`, `[29:28]`, and `[31:30]` respectively instead. The tag bits
    /// are effectively a `USER_SEL` value for each halfword in the CDR.
    pub fn enable_tag_bits(&mut self) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.enable_tag_bits_unchecked() };
            Ok(())
        }
    }

    /// Attempt to enable tag bits without checking the write protection register.
    ///
    /// # Safety
    ///
    /// Operation failure is silent apart from setting the `WPROTERR` flag in the `WPSR` register.
    ///
    /// May overwrite data in the `WPROTADDR` field of the `WPSR` register if the operation fails.
    pub unsafe fn enable_tag_bits_unchecked(&mut self) {
        self.dacc.mr.write(|mode_reg| mode_reg.tag().en());
    }

    /// Attempt to disable use of tag bits in CDR halfwords. Fails if write protection is enabled.
    pub fn disable_tag_bits(&mut self) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.disable_tag_bits_unchecked() };
            Ok(())
        }
    }

    /// Attempt to disable tag bits without checking the write protection register.
    ///
    /// # Safety
    ///
    /// Operation failure is silent apart from setting the `WPROTERR` flag in the `WPSR` register.
    ///
    /// May overwrite data in the `WPROTADDR` field of the `WPSR` register if the operation fails.
    pub unsafe fn disable_tag_bits_unchecked(&mut self) {
        self.dacc.mr.write(|mode_reg| mode_reg.tag().dis());
    }

    /// Attempt to enable sleep mode. Fails if write protection is enabled.
    ///
    /// In sleep mode, the DAC core and reference voltage circuitry are turned off between
    /// conversions.
    pub fn enable_sleep_mode(&mut self) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.enable_sleep_mode_unchecked() };
            Ok(())
        }
    }

    /// Attempt to enable sleep mode without checking the write protection register.
    ///
    /// # Safety
    ///
    /// Operation failure is silent apart from setting the `WPROTERR` flag in the `WPSR` register.
    ///
    /// May overwrite data in the `WPROTADDR` field of the `WPSR` register if the operation fails.
    pub unsafe fn enable_sleep_mode_unchecked(&mut self) {
        self.dacc.mr.write(|mode_reg| mode_reg.sleep().set_bit());
    }

    /// Attempt to disable sleep mode. Fails if write protection is enabled.
    ///
    /// When not in sleep mode, the DAC core and reference voltage circuitry are kept on between
    /// conversions.
    pub fn disable_sleep_mode(&mut self) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.disable_sleep_mode_unchecked() };
            Ok(())
        }
    }

    /// Attempt to disable sleep mode without checking the write protection register.
    ///
    /// # Safety
    ///
    /// Operation failure is silent apart from setting the `WPROTERR` flag in the `WPSR` register.
    ///
    /// May overwrite data in the `WPROTADDR` field of the `WPSR` register if the operation fails.
    pub unsafe fn disable_sleep_mode_unchecked(&mut self) {
        self.dacc.mr.write(|mode_reg| mode_reg.sleep().clear_bit());
    }

    /// Attempt to enable fast wake-up mode. Fails if write protection is enabled.
    ///
    /// In fast-wake-up mode, the reference voltage is kept on between conversions, but the DAC core
    /// is kept off.
    pub fn enable_fast_wakeup(&mut self) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.enable_fast_wakeup_unchecked() };
            Ok(())
        }
    }

    /// Attempt to enable fast wake-up mode without checking the write protection register.
    ///
    /// # Safety
    ///
    /// Operation failure is silent apart from setting the `WPROTERR` flag in the `WPSR` register.
    ///
    /// May overwrite data in the `WPROTADDR` field of the `WPSR` register if the operation fails.
    pub unsafe fn enable_fast_wakeup_unchecked(&mut self) {
        self.dacc.mr.write(|mode_reg| mode_reg.fastwkup().set_bit());
    }

    /// Attempt to disable fast wake-up mode. Fails if write protection is enabled.
    ///
    /// When not in fast wake-up mode, the sleep mode is defined only by the selected sleep mode.
    pub fn disable_fast_wakeup(&mut self) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.disable_fast_wakeup_unchecked() };
            Ok(())
        }
    }

    /// Attempt to disable fast wake-up mode without checking the write protection register.
    ///
    /// # Safety
    ///
    /// Operation failure is silent apart from setting the `WPROTERR` flag in the `WPSR` register.
    ///
    /// May overwrite data in the `WPROTADDR` field of the `WPSR` register if the operation fails.
    pub unsafe fn disable_fast_wakeup_unchecked(&mut self) {
        self.dacc
            .mr
            .write(|mode_reg| mode_reg.fastwkup().clear_bit());
    }

    /// Attempt to enable max speed mode. Fails if write protection is enabled.
    ///
    /// In max speed mode, the DACC no longer waits to sample the end of the cycle signal from the
    /// DACC block to start the next conversion and uses an internal counter instead. This mode
    /// gains 2 DACC clock periods between each consecutive conversion.
    ///
    /// # Warning
    ///
    /// In this mode, the EOC interrupt of the `DACC_IER` register should not be used, as it is 2
    /// DACC clock periods late.
    pub fn enable_max_speed(&mut self) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.enable_max_speed_unchecked() };
            Ok(())
        }
    }

    /// Attempt to enable max speed mode without checking the write protection register.
    ///
    /// # Safety
    ///
    /// Operation failure is silent apart from setting the `WPROTERR` flag in the `WPSR` register.
    ///
    /// May overwrite data in the `WPROTADDR` field of the `WPSR` register if the operation fails.
    pub unsafe fn enable_max_speed_unchecked(&mut self) {
        self.dacc.mr.write(|mode_reg| mode_reg.maxs().set_bit());
    }

    /// Attempt to disable max speed mode. Fails if write protection is enabled.
    pub fn disable_max_speed(&mut self) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.disable_max_speed_unchecked() };
            Ok(())
        }
    }

    /// Attempt to disable max speed mode without checking the write protection register.
    ///
    /// # Safety
    ///
    /// Operation failure is silent apart from setting the `WPROTERR` flag in the `WPSR` register.
    ///
    /// May overwrite data in the `WPROTADDR` field of the `WPSR` register if the operation fails.
    pub unsafe fn disable_max_speed_unchecked(&mut self) {
        self.dacc.mr.write(|mode_reg| mode_reg.maxs().clear_bit());
    }

    /// Attempt to enable DAC output channels. Fails if write protection is enabled.
    pub fn enable_channels(&mut self, channels: DacChannels) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.enable_channels_unchecked(channels) };
            Ok(())
        }
    }

    /// Attempt to enable DAC output channels without checking the write protection register.
    ///
    /// # Safety
    ///
    /// Operation failure is silent apart from setting the `WPROTERR` flag in the `WPSR` register.
    ///
    /// May overwrite data in the `WPROTADDR` field of the `WPSR` register if the operation fails.
    pub unsafe fn enable_channels_unchecked(&mut self, channels: DacChannels) {
        self.dacc.cher.write_with_zero(|channel_en_reg| {
            if channels.channel_0 {
                channel_en_reg.ch0().set_bit();
            }
            if channels.channel_1 {
                channel_en_reg.ch1().set_bit();
            }
            channel_en_reg
        });
    }

    /// Attempt to disable DAC output channels. Fails if write protection is enabled.
    pub fn disable_channels(&mut self, channels: DacChannels) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.disable_channels_unchecked(channels) };
            Ok(())
        }
    }

    /// Attempt to disable DAC output channels without checking the write protection register.
    ///
    /// # Safety
    ///
    /// Operation failure is silent apart from setting the `WPROTERR` flag in the `WPSR` register.
    ///
    /// May overwrite data in the `WPROTADDR` field of the `WPSR` register if the operation fails.
    pub unsafe fn disable_channels_unchecked(&mut self, channels: DacChannels) {
        self.dacc.chdr.write_with_zero(|channel_en_reg| {
            if channels.channel_0 {
                channel_en_reg.ch0().clear_bit();
            }
            if channels.channel_1 {
                channel_en_reg.ch1().clear_bit();
            }
            channel_en_reg
        });
    }

    /// Attempt to apply a configuration to the DAC output channels. Fails if write protection is
    /// enabled.
    pub fn configure_channels(&mut self, channels: DacChannels) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.configure_channels_unchecked(channels) };
            Ok(())
        }
    }

    /// Attempt to apply a configuration to the DAC output channels without checking the write
    /// protection register.
    ///
    /// # Safety
    ///
    /// Operation failure is silent apart from setting the `WPROTERR` flag in the `WPSR` register.
    ///
    /// May overwrite data in the `WPROTADDR` field of the `WPSR` register if the operation fails.
    #[rustfmt::skip]
    pub unsafe fn configure_channels_unchecked(&mut self, channels: DacChannels) {
        self.dacc.chdr.write_with_zero(|chdr| {
            chdr
                .ch0().bit(channels.channel_0)
                .ch1().bit(channels.channel_1)
        });
    }

    /// Get the current status of the DAC channels.
    pub fn channels_status(&self) -> DacChannels {
        let chsr = self.dacc.chsr.read();
        DacChannels {
            channel_0: chsr.ch0().bit(),
            channel_1: chsr.ch1().bit(),
        }
    }

    /// Attempt to apply the default bias current control config. Fails if write protection is
    /// enabled.
    pub fn apply_default_bias_current_config(&mut self) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.apply_default_bias_current_config_unchecked() };
            Ok(())
        }
    }

    /// Attempt to apply the default bias current control config without checking the write
    /// protection register.
    ///
    /// # Safety
    ///
    /// Operation failure is silent apart from setting the `WPROTERR` flag in the `WPSR` register.
    ///
    /// May overwrite data in the `WPROTADDR` field of the `WPSR` register if the operation fails.
    #[rustfmt::skip]
    pub unsafe fn apply_default_bias_current_config_unchecked(&mut self) {
        self.dacc.acr.write(|acr| {
            acr
                .ibctldaccore().bits(0b01)
                .ibctlch0().bits(0b10)
                .ibctlch1().bits(0b10)
        });
    }
}

/// Enum representing the various ways operations with the DACC can fail.
#[derive(Clone, Copy, Debug)]
pub enum DaccError {
    WriteProtected,
    NotTransmitReady,
}

pub type DaccResult = Result<(), DaccError>;

/// Wrapper struct for DACC interrupts.
///
/// A brief description of each interrupt is as follows:
///
///   - `txrdy`: Transmit ready
///   - `eoc`: End of conversion
///   - `endtx`: End of transmit buffer
///   - `txbufe`: Transmit buffer empty
#[derive(Clone, Copy, Debug)]
pub struct DaccInterrupts {
    pub txrdy: bool,
    pub eoc: bool,
    pub endtx: bool,
    pub txbufe: bool,
}

/// Wrapper struct for the status of the DAC output channels.
#[derive(Clone, Copy, Debug)]
pub struct DacChannels {
    pub channel_0: bool,
    pub channel_1: bool,
}

#[cfg(feature = "unproven")]
use crate::CurrentBias;

#[cfg(feature = "unproven")]
impl Dacc {
    /// Attempt to set the current bias on the DAC core, which allows you to choose performance or
    /// power consumption. Fails if write protection is enabled.
    pub fn set_daccore_current_bias(&mut self, config: CurrentBias) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.set_daccore_current_bias_unchecked(config) };
            Ok(())
        }
    }

    /// Attempt to set the current bias on the DAC core without checking the write protection
    /// register.
    ///
    /// # Safety
    ///
    /// Operation failure is silent apart from setting the `WPROTERR` flag in the `WPSR` register.
    ///
    /// May overwrite data in the `WPROTADDR` field of the `WPSR` register if the operation fails.
    pub unsafe fn set_daccore_current_bias_unchecked(&mut self, config: CurrentBias) {
        self.dacc
            .acr
            .write(|acr| acr.ibctldaccore().bits(config as u8));
    }

    /// Get the current bias configuration for the DAC core.
    pub fn get_daccore_current_bias(&self) -> CurrentBias {
        unsafe { core::mem::transmute(self.dacc.acr.read().ibctldaccore().bits()) }
    }

    /// Attempt to set the current bias on channel 0, which changes the slew rate of the analog
    /// output. Fails if write protection is enabled.
    pub fn set_ch0_current_bias(&mut self, config: CurrentBias) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.set_ch0_current_bias_unchecked(config) };
            Ok(())
        }
    }

    /// Attempt to set the current bias on channel 0 without checking the write protection register.
    ///
    /// # Safety
    ///
    /// Operation failure is silent apart from setting the `WPROTERR` flag in the `WPSR` register.
    ///
    /// May overwrite data in the `WPROTADDR` field of the `WPSR` register if the operation fails.
    pub unsafe fn set_ch0_current_bias_unchecked(&mut self, config: CurrentBias) {
        self.dacc.acr.write(|acr| acr.ibctlch0().bits(config as u8));
    }

    /// Get the current bias configuration for channel 0.
    pub fn get_ch0_current_bias(&self) -> CurrentBias {
        unsafe { core::mem::transmute(self.dacc.acr.read().ibctlch0().bits()) }
    }

    /// Attempt to set the current bias on channel 1, which changes the slew rate of the analog
    /// output. Fails if write protection is enabled.
    pub fn set_ch1_current_bias(&mut self, config: CurrentBias) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.set_ch1_current_bias_unchecked(config) };
            Ok(())
        }
    }

    /// Attempt to set the current bias on channel 1 without checking the write protection register.
    ///
    /// # Safety
    ///
    /// Operation failure is silent apart from setting the `WPROTERR` flag in the `WPSR` register.
    ///
    /// May overwrite data in the `WPROTADDR` field of the `WPSR` register if the operation fails.
    pub unsafe fn set_ch1_current_bias_unchecked(&mut self, config: CurrentBias) {
        self.dacc.acr.write(|acr| acr.ibctlch1().bits(config as u8));
    }

    /// Get the current bias configuration for channel 1.
    pub fn get_ch1_current_bias(&self) -> CurrentBias {
        unsafe { core::mem::transmute(self.dacc.acr.read().ibctlch1().bits()) }
    }

    /// Attempt to apply an analog current configuration to the DAC core and both output channels.
    /// Fails if write protection is enabled.
    pub fn set_current_config(&mut self, config: DaccCurrentConfig) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.set_current_config_unchecked(config) };
            Ok(())
        }
    }

    /// Attempt to apply an analog current configuration to the DAC core and both output channels
    /// without checking the write protection register.
    ///
    /// # Safety
    ///
    /// Operation failure is silent apart from setting the `WPROTERR` flag in the `WPSR` register.
    ///
    /// May overwrite data in the `WPROTADDR` field of the `WPSR` register if the operation fails.
    #[rustfmt::skip]
    pub unsafe fn set_current_config_unchecked(&mut self, config: DaccCurrentConfig) {
        self.dacc.acr.write(|acr| {
            acr
                .ibctldaccore().bits(config.daccore as u8)
                .ibctlch0().bits(config.ch0 as u8)
                .ibctlch1().bits(config.ch1 as u8)
        })
    }

    /// Get the current bias configuration for the DACC.
    pub fn get_current_config(&self) -> DaccCurrentConfig {
        DaccCurrentConfig::new(
            self.get_daccore_current_bias(),
            self.get_ch0_current_bias(),
            self.get_ch1_current_bias(),
        )
    }
}

#[cfg(feature = "unproven")]
/// Struct containing the current bias configs for the DAC core and both output channels.
#[derive(Clone, Copy, Debug)]
pub struct DaccCurrentConfig {
    pub daccore: CurrentBias,
    pub ch0: CurrentBias,
    pub ch1: CurrentBias,
}

#[cfg(feature = "unproven")]
impl DaccCurrentConfig {
    pub fn new(daccore: CurrentBias, ch0: CurrentBias, ch1: CurrentBias) -> Self {
        Self { daccore, ch0, ch1 }
    }
}
