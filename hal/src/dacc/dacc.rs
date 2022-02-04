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
//!     - DACC Mode Register
//!     - DACC Channel Enable Register
//!     - DACC Channel Disable Register
//!     - DACC Analog Current Register

use crate::{
    pac::{dacc::mr::*, DACC, PMC},
    pmc::*,
    peripheral_id::PeripheralId,
};
use super::cdr_val::CdrVal;

pub const DACC_PID: u32 = PeripheralId::DACC as u32;
const WPKEY: u32 = 0x444143; // DAC in ASCII

pub struct Dacc {
    dacc: DACC,
}

impl Dacc {
    pub fn new(dacc: DACC) -> Self {
        Self {
            dacc
        }
    }

    pub fn init(&self, pmc: &mut PMC) {
        self.enable_dacc_clock(pmc);
    }

    /// Enable the clock for the DAC peripheral.
    pub fn enable_dacc_clock(&self, pmc: &mut PMC) {
        unsafe { enable_peripheral_clk(pmc, DACC_PID) };
    }

    /// Disable the clock for the DAC peripheral.
    pub fn disable_dacc_clock(&self, pmc: &mut PMC) {
        unsafe { disable_peripheral_clk(pmc, DACC_PID) };
    }

    /// Reset the DAC peripheral.
    pub fn reset(&mut self) {
        unsafe { self.dacc.cr.write_with_zero(|cr_reg| cr_reg.swrst().set_bit()) };
    }

    /// Set the external trigger. Briefly, each trigger is as follows:
    ///     - `TRGSEL_A::EXTERN`: ???
    ///     - `TRGSEL_A::TIOOTCC0`: **T**imer **I**/**O** **o**output of the **t**imer **c**ounter
    ///         **c**hannel **0**
    ///     - `TRGSEL_A::TIOOTCC1`: **T**imer **I**/**O** **o**output of the **t**imer **c**ounter
    ///         **c**hannel **1**
    ///     - `TRGSEL_A::TIOOTCC2`: **T**imer **I**/**O** **o**output of the **t**imer **c**ounter
    ///         **c**hannel **2**
    ///     - `TRGSEL_A::PWM0`: **PWM** event line **0**
    ///     - `TRGSEL_A::PWM1`: **PWM** event line **1**
    pub fn set_trigger(&mut self, trigger: TRGSEL_A) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.set_trigger_unchecked(trigger) };
            Ok(())
        }
    }

    /// Set the external trigger without checking the writeprotect register. May overwrite data in
    /// the WPSR register if the operation fails.
    pub unsafe fn set_trigger_unchecked(&mut self, trigger: TRGSEL_A) {
        self.dacc.mr.write(|mode_reg| {
            mode_reg.trgen().variant(TRGEN_A::EN);
            mode_reg.trgsel().variant(trigger);
            mode_reg
        });
    }

    /// Disable external triggering.
    pub fn disable_trigger(&mut self) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.disable_trigger_unchecked() };
            Ok(())
        }
    }

    /// Disable external triggering without checking the writeprotect register. May overwrite data
    /// in the WPSR register if the operation fails.
    pub unsafe fn disable_trigger_unchecked(&mut self) {
        self.dacc.mr.write(|mode_reg| mode_reg.trgen().variant(TRGEN_A::DIS));
    }

    /// Set the transfer mode. The behaviour of these is described on page 1358 of the datasheet,
    /// found [here](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11057-32-bit-Cortex-M3-Microcontroller-SAM3X-SAM3A_Datasheet.pdf).
    pub fn set_transfer_mode(&mut self, mode: WORD_A) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.set_transfer_mode_unchecked(mode) };
            Ok(())
        }
    }

    /// Set the transfer mode without checking the writeprotect register. May overwrite data in the
    /// WPSR register if the operation fails.
    pub unsafe fn set_transfer_mode_unchecked(&mut self, mode: WORD_A) {
        self.dacc.mr.write(|mode_reg| mode_reg.word().variant(mode));
    }

    pub fn get_transfer_mode(&self) -> WORD_A {
        self.dacc.mr.read().word().variant()
    }

    /// Enable interrupts. A brief description of each interrupt is as follows:
    ///     - `txrdy`: Transmit ready
    ///     - `eoc`: End of conversion
    ///     - `endtx`: End of transmit buffer
    ///     - `txbufe`: Transmit buffer empty
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

    /// Set disable interrupt flags.
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

    pub fn transmit_ready(&self) -> bool {
        self.dacc.isr.read().txrdy().bit()
    }

    pub fn end_of_conversion(&self) -> bool {
        self.dacc.isr.read().eoc().bit()
    }

    pub fn end_of_transmit_buffer(&self) -> bool {
        self.dacc.isr.read().endtx().bit()
    }

    pub fn transmit_buffer_empty(&self) -> bool {
        self.dacc.isr.read().txbufe().bit()
    }

    /// Write data to be converted into the convert data register.
    pub fn write_conversion_data(&mut self, data: CdrVal) -> DaccResult {
        if self.transmit_ready() {
            unsafe { self.write_conversion_data_unchecked(data) };
            Ok(())
        } else {
            Err(DaccError::NotTransmitReady)
        }
    }

    /// Write data to be converted into the convert data register without checking whether the
    /// `TXRDY` flag is active. If it's not, performing the write will corrupt the DACC FIFO.
    pub unsafe fn write_conversion_data_unchecked(&mut self, data: CdrVal) {
        self.dacc.cdr.write(|conversion_data| unsafe { conversion_data.data().bits(data.bits()) });
    }

    /// Enables write protection to the following registers:
    ///     - Mode register (`MR`)
    ///     - Channel enable register (`CHER`)
    ///     - Channel disable register (`CHDR`)
    ///     - Analog current register (`ACR`)
    pub fn enable_writeprotect(&mut self) {
        self.dacc.wpmr.write(|write_protect_mode_reg| {
            unsafe { write_protect_mode_reg.wpkey().bits(WPKEY) };
            write_protect_mode_reg.wpen().set_bit();
            write_protect_mode_reg
        });
    }

    /// Disables write protection to the following registers:
    ///     - Mode register (`MR`)
    ///     - Channel enable register (`CHER`)
    ///     - Channel disable register (`CHDR`)
    ///     - Analog current register (`ACR`)
    pub fn disable_writeprotect(&mut self) {
        self.dacc.wpmr.write(|write_protect_mode_reg| {
            unsafe { write_protect_mode_reg.wpkey().bits(WPKEY) };
            write_protect_mode_reg.wpen().clear_bit();
            write_protect_mode_reg
        });
    }

    /// Check whether write protection is currently enabled.
    pub fn writeprotect_enabled(&self) -> bool {
        self.dacc.wpmr.read().wpen().bit()
    }

    /// Get the address of the register write request that last generated an error, if one exists.
    pub fn writeprotect_status(&self) -> Option<u8> {
        let wpsr = self.dacc.wpsr.read();
        if wpsr.wproterr().bit() {
            Some(wpsr.wprotaddr().bits())
        } else {
            None
        }
    }

    /// Startup time selection. Actual startup time values can be in table 45-41 (page 1411) of the
    /// datasheet, found [here](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11057-32-bit-Cortex-M3-Microcontroller-SAM3X-SAM3A_Datasheet.pdf).
    pub fn set_startup(&mut self, startup: STARTUP_A) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.set_startup_unchecked(startup) };
            Ok(())
        }
    }

    /// Set startup time without checking the writeprotect register. May overwrite data in the WPSR
    /// register if the operation fails.
    pub unsafe fn set_startup_unchecked(&mut self, startup: STARTUP_A) {
        self.dacc.mr.write(|mode_reg| mode_reg.startup().variant(startup));
    }

    /// Set the refresh period length. The actual refresh period length is calculated as follows:
    /// 
    /// Refresh period = (1024 * refresh) / DACC Clock
    pub fn set_refresh(&mut self, refresh: u8) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.set_refresh_unchecked(refresh) };
            Ok(())
        }
    }

    /// Set refresh without checking the writeprotect register. May overwrite data in the WPSR
    /// register if the operation fails.
    pub unsafe fn set_refresh_unchecked(&mut self, refresh: u8) {
        self.dacc.mr.write(|mode_reg| mode_reg.refresh().bits(refresh));
    }

    pub fn select_channel(&mut self, channel: USER_SEL_A) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.select_channel_unchecked(channel) };
            Ok(())
        }
    }

    /// Select the output channel without checking the writeprotect register. May overwrite data in
    /// the WPSR register if the operation fails.
    pub unsafe fn select_channel_unchecked(&mut self, channel: USER_SEL_A) {
        self.dacc.mr.write(|mode_reg| mode_reg.user_sel().variant(channel));
    }

    /// Enable use of tag bits in CDR values. Tag bits are the two most significant bits after each
    /// halfword in the CDR. Specifically, the breakdown of each halfword is:
    ///     - `halfword[11:0]`: 12 bit value used for DAC output
    ///     - `halfword[13:12]`: tag bits
    ///     - `halfword[15:14]`: unused
    /// This is the same for the upper halfword in the CDR if the DACC is set to `WORD` mode, just
    /// the bit ranges are `[27:16]`, `[29:28]`, and `[31:30]` respectively instead. The tag bits
    /// are effectively a `USER_SEL` value for each halfword in the CDR. Please refer to 
    pub fn enable_tag_bits(&mut self) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.enable_tag_bits_unchecked() };
            Ok(())
        }
    }

    /// Enable tag bits without checking the writeprotect register. May overwrite data in the WPSR
    /// register if the operation fails.
    pub unsafe fn enable_tag_bits_unchecked(&mut self) {
        self.dacc.mr.write(|mode_reg| mode_reg.tag().en());
    }

    /// Disable use of tag bits in CDR values. Tag bits are the two most significant bits after each
    /// halfword in the CDR. Specifically, the breakdown of each halfword is:
    ///     - `halfword[11:0]`: 12 bit value used for DAC output
    ///     - `halfword[13:12]`: tag bits
    ///     - `halfword[15:14]`: unused
    /// This is the same for the upper halfword in the CDR if the DACC is set to `WORD` mode, just
    /// the bit ranges are `[27:16]`, `[29:28]`, and `[31:30]` respectively instead. The tag bits
    /// are effectively a `USER_SEL` value for each halfword in the CDR. Please refer to 
    pub fn disable_tag_bits(&mut self) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.disable_tag_bits_unchecked() };
            Ok(())
        }
    }

    /// Disable tag bits without checking the writeprotect register. May overwrite data in the WPSR
    /// register if the operation fails.
    pub unsafe fn disable_tag_bits_unchecked(&mut self) {
        self.dacc.mr.write(|mode_reg| mode_reg.tag().dis());
    }

    /// Enable sleep mode. In sleep mode, the DAC core and reference voltage circuitry are turned
    /// off between conversions.
    pub fn enable_sleep_mode(&mut self) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.enable_sleep_mode_unchecked() };
            Ok(())
        }
    }

    /// Enable sleep mode without checking the writeprotect register. May overwrite data in the WPSR
    /// register if the operation fails.
    pub unsafe fn enable_sleep_mode_unchecked(&mut self) {
        self.dacc.mr.write(|mode_reg| mode_reg.sleep().set_bit());
    }

    /// Disable sleep mode. When not in sleep mode, the DAC core and reference voltage circuitry are
    /// kept on between conversions.
    pub fn disable_sleep_mode(&mut self) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.disable_sleep_mode_unchecked() };
            Ok(())
        }
    }

    /// Disable sleep mode without checking the writeprotect register. May overwrite data in the
    /// WPSR register if the operation fails.
    pub unsafe fn disable_sleep_mode_unchecked(&mut self) {
        self.dacc.mr.write(|mode_reg| mode_reg.sleep().clear_bit());
    }

    /// Enable fast wake-up mode. In fast-wake-up mode, the reference voltage is kept on between
    /// conversions, but the DAC core is kept off.
    pub fn enable_fast_wakeup(&mut self) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.enable_fast_wakeup_unchecked() };
            Ok(())
        }
    }

    /// Enable fast wake-up mode without checking the writeprotect register. May overwrite data in
    /// the WPSR register if the operation fails.
    pub unsafe fn enable_fast_wakeup_unchecked(&mut self) {
        self.dacc.mr.write(|mode_reg| mode_reg.fastwkup().set_bit());
    }

    /// Disable fast wake-up mode. When not in fast wake-up mode, the sleep mode is defined only by
    /// the selected sleep mode.
    pub fn disable_fast_wakeup(&mut self) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.disable_fast_wakeup_unchecked() };
            Ok(())
        }
    }

    /// Disable fast wake-up mode without checking the writeprotect register. May overwrite data in
    /// the WPSR register if the operation fails.
    pub unsafe fn disable_fast_wakeup_unchecked(&mut self) {
        self.dacc.mr.write(|mode_reg| mode_reg.fastwkup().clear_bit());
    }

    /// Enable max speed mode. In max speed mode, the DACC no longer waits to sample the end of the
    /// cycle signal from the DACC block to start the next conversion and uses an internal counter
    /// instead. This mode gains 2 DACC clock periods between each consecutive conversion.
    /// 
    /// **WARNING**: In this mode, the EOC interrupt of the `DACC_IER` register should not be used,
    /// as it is 2 DACC clock periods late.
    pub fn enable_max_speed(&mut self) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.enable_max_speed_unchecked() };
            Ok(())
        }
    }

    /// Enable max speed mode without checking the writeprotect register. May overwrite data in the
    /// WPSR register if the operation fails.
    pub unsafe fn enable_max_speed_unchecked(&mut self) {
        self.dacc.mr.write(|mode_reg| mode_reg.maxs().set_bit());
    }

    /// Disable max speed mode.
    pub fn disable_max_speed(&mut self) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.disable_max_speed_unchecked() };
            Ok(())
        }
    }

    /// Disable max speed mode without checking the writeprotect register. May overwrite data in the
    /// WPSR register if the operation fails.
    pub unsafe fn disable_max_speed_unchecked(&mut self) {
        self.dacc.mr.write(|mode_reg| mode_reg.maxs().clear_bit());
    }

    /// Enable DAC output channels.
    pub fn enable_channels(&mut self, channels: DacChannels) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.enable_channels_unchecked(channels) };
            Ok(())
        }
    }

    /// Enable DAC output channels without checking the writeprotect register. May overwrite data in
    /// the WPSR register if the operation fails.
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

    /// Disable DAC output channels.
    pub fn disable_channels(&mut self, channels: DacChannels) -> DaccResult {
        if self.writeprotect_enabled() {
            Err(DaccError::WriteProtected)
        } else {
            unsafe { self.disable_channels_unchecked(channels) };
            Ok(())
        }
    }

    /// Disable DAC output channels without checking the writeprotect register. May overwrite data in
    /// the WPSR register if the operation fails.
    pub unsafe fn disable_channels_unchecked(&mut self, channels: DacChannels) {
        self.dacc.chdr.write_with_zero(|channel_en_reg| {
            if channels.channel_0 {
                channel_en_reg.ch0().set_bit();
            }
            if channels.channel_1 {
                channel_en_reg.ch1().set_bit();
            }
            channel_en_reg
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
}

/// Enum representing the various ways operations with the DACC can fail.
pub enum DaccError {
    WriteProtected,
    NotTransmitReady,
}

pub type DaccResult = Result<(), DaccError>;

/// Wrapper struct for DACC interrupts. A brief description of each interrupt is as follows:
///     - `txrdy`: Transmit ready
///     - `eoc`: End of conversion
///     - `endtx`: End of transmit buffer
///     - `txbufe`: Transmit buffer empty
pub struct DaccInterrupts {
    pub txrdy: bool,
    pub eoc: bool,
    pub endtx: bool,
    pub txbufe: bool,
}

pub struct DacChannels {
    pub channel_0: bool,
    pub channel_1: bool,
}
