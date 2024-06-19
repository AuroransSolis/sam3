#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    per: Per,
    pdr: Pdr,
    psr: Psr,
    _reserved3: [u8; 0x04],
    oer: Oer,
    odr: Odr,
    osr: Osr,
    _reserved6: [u8; 0x04],
    ifer: Ifer,
    ifdr: Ifdr,
    ifsr: Ifsr,
    _reserved9: [u8; 0x04],
    sodr: Sodr,
    codr: Codr,
    odsr: Odsr,
    pdsr: Pdsr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    isr: Isr,
    mder: Mder,
    mddr: Mddr,
    mdsr: Mdsr,
    _reserved20: [u8; 0x04],
    pudr: Pudr,
    puer: Puer,
    pusr: Pusr,
    _reserved23: [u8; 0x04],
    absr: Absr,
    _reserved24: [u8; 0x0c],
    scifsr: Scifsr,
    difsr: Difsr,
    ifdgsr: Ifdgsr,
    scdr: Scdr,
    _reserved28: [u8; 0x10],
    ower: Ower,
    owdr: Owdr,
    owsr: Owsr,
    _reserved31: [u8; 0x04],
    aimer: Aimer,
    aimdr: Aimdr,
    aimmr: Aimmr,
    _reserved34: [u8; 0x04],
    esr: Esr,
    lsr: Lsr,
    elsr: Elsr,
    _reserved37: [u8; 0x04],
    fellsr: Fellsr,
    rehlsr: Rehlsr,
    frlhsr: Frlhsr,
    _reserved40: [u8; 0x04],
    locksr: Locksr,
    wpmr: Wpmr,
    wpsr: Wpsr,
}
impl RegisterBlock {
    #[doc = "0x00 - PIO Enable Register"]
    #[inline(always)]
    pub const fn per(&self) -> &Per {
        &self.per
    }
    #[doc = "0x04 - PIO Disable Register"]
    #[inline(always)]
    pub const fn pdr(&self) -> &Pdr {
        &self.pdr
    }
    #[doc = "0x08 - PIO Status Register"]
    #[inline(always)]
    pub const fn psr(&self) -> &Psr {
        &self.psr
    }
    #[doc = "0x10 - Output Enable Register"]
    #[inline(always)]
    pub const fn oer(&self) -> &Oer {
        &self.oer
    }
    #[doc = "0x14 - Output Disable Register"]
    #[inline(always)]
    pub const fn odr(&self) -> &Odr {
        &self.odr
    }
    #[doc = "0x18 - Output Status Register"]
    #[inline(always)]
    pub const fn osr(&self) -> &Osr {
        &self.osr
    }
    #[doc = "0x20 - Glitch Input Filter Enable Register"]
    #[inline(always)]
    pub const fn ifer(&self) -> &Ifer {
        &self.ifer
    }
    #[doc = "0x24 - Glitch Input Filter Disable Register"]
    #[inline(always)]
    pub const fn ifdr(&self) -> &Ifdr {
        &self.ifdr
    }
    #[doc = "0x28 - Glitch Input Filter Status Register"]
    #[inline(always)]
    pub const fn ifsr(&self) -> &Ifsr {
        &self.ifsr
    }
    #[doc = "0x30 - Set Output Data Register"]
    #[inline(always)]
    pub const fn sodr(&self) -> &Sodr {
        &self.sodr
    }
    #[doc = "0x34 - Clear Output Data Register"]
    #[inline(always)]
    pub const fn codr(&self) -> &Codr {
        &self.codr
    }
    #[doc = "0x38 - Output Data Status Register"]
    #[inline(always)]
    pub const fn odsr(&self) -> &Odsr {
        &self.odsr
    }
    #[doc = "0x3c - Pin Data Status Register"]
    #[inline(always)]
    pub const fn pdsr(&self) -> &Pdsr {
        &self.pdsr
    }
    #[doc = "0x40 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x44 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x48 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x4c - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x50 - Multi-driver Enable Register"]
    #[inline(always)]
    pub const fn mder(&self) -> &Mder {
        &self.mder
    }
    #[doc = "0x54 - Multi-driver Disable Register"]
    #[inline(always)]
    pub const fn mddr(&self) -> &Mddr {
        &self.mddr
    }
    #[doc = "0x58 - Multi-driver Status Register"]
    #[inline(always)]
    pub const fn mdsr(&self) -> &Mdsr {
        &self.mdsr
    }
    #[doc = "0x60 - Pull-up Disable Register"]
    #[inline(always)]
    pub const fn pudr(&self) -> &Pudr {
        &self.pudr
    }
    #[doc = "0x64 - Pull-up Enable Register"]
    #[inline(always)]
    pub const fn puer(&self) -> &Puer {
        &self.puer
    }
    #[doc = "0x68 - Pad Pull-up Status Register"]
    #[inline(always)]
    pub const fn pusr(&self) -> &Pusr {
        &self.pusr
    }
    #[doc = "0x70 - Peripheral AB Select Register"]
    #[inline(always)]
    pub const fn absr(&self) -> &Absr {
        &self.absr
    }
    #[doc = "0x80 - System Clock Glitch Input Filter Select Register"]
    #[inline(always)]
    pub const fn scifsr(&self) -> &Scifsr {
        &self.scifsr
    }
    #[doc = "0x84 - Debouncing Input Filter Select Register"]
    #[inline(always)]
    pub const fn difsr(&self) -> &Difsr {
        &self.difsr
    }
    #[doc = "0x88 - Glitch or Debouncing Input Filter Clock Selection Status Register"]
    #[inline(always)]
    pub const fn ifdgsr(&self) -> &Ifdgsr {
        &self.ifdgsr
    }
    #[doc = "0x8c - Slow Clock Divider Debouncing Register"]
    #[inline(always)]
    pub const fn scdr(&self) -> &Scdr {
        &self.scdr
    }
    #[doc = "0xa0 - Output Write Enable"]
    #[inline(always)]
    pub const fn ower(&self) -> &Ower {
        &self.ower
    }
    #[doc = "0xa4 - Output Write Disable"]
    #[inline(always)]
    pub const fn owdr(&self) -> &Owdr {
        &self.owdr
    }
    #[doc = "0xa8 - Output Write Status Register"]
    #[inline(always)]
    pub const fn owsr(&self) -> &Owsr {
        &self.owsr
    }
    #[doc = "0xb0 - Additional Interrupt Modes Enable Register"]
    #[inline(always)]
    pub const fn aimer(&self) -> &Aimer {
        &self.aimer
    }
    #[doc = "0xb4 - Additional Interrupt Modes Disables Register"]
    #[inline(always)]
    pub const fn aimdr(&self) -> &Aimdr {
        &self.aimdr
    }
    #[doc = "0xb8 - Additional Interrupt Modes Mask Register"]
    #[inline(always)]
    pub const fn aimmr(&self) -> &Aimmr {
        &self.aimmr
    }
    #[doc = "0xc0 - Edge Select Register"]
    #[inline(always)]
    pub const fn esr(&self) -> &Esr {
        &self.esr
    }
    #[doc = "0xc4 - Level Select Register"]
    #[inline(always)]
    pub const fn lsr(&self) -> &Lsr {
        &self.lsr
    }
    #[doc = "0xc8 - Edge/Level Status Register"]
    #[inline(always)]
    pub const fn elsr(&self) -> &Elsr {
        &self.elsr
    }
    #[doc = "0xd0 - Falling Edge/Low Level Select Register"]
    #[inline(always)]
    pub const fn fellsr(&self) -> &Fellsr {
        &self.fellsr
    }
    #[doc = "0xd4 - Rising Edge/ High Level Select Register"]
    #[inline(always)]
    pub const fn rehlsr(&self) -> &Rehlsr {
        &self.rehlsr
    }
    #[doc = "0xd8 - Fall/Rise - Low/High Status Register"]
    #[inline(always)]
    pub const fn frlhsr(&self) -> &Frlhsr {
        &self.frlhsr
    }
    #[doc = "0xe0 - Lock Status"]
    #[inline(always)]
    pub const fn locksr(&self) -> &Locksr {
        &self.locksr
    }
    #[doc = "0xe4 - Write Protect Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &Wpmr {
        &self.wpmr
    }
    #[doc = "0xe8 - Write Protect Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &Wpsr {
        &self.wpsr
    }
}
#[doc = "PER (w) register accessor: PIO Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per`]
module"]
#[doc(alias = "PER")]
pub type Per = crate::Reg<per::PerSpec>;
#[doc = "PIO Enable Register"]
pub mod per;
#[doc = "PDR (w) register accessor: PIO Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdr`]
module"]
#[doc(alias = "PDR")]
pub type Pdr = crate::Reg<pdr::PdrSpec>;
#[doc = "PIO Disable Register"]
pub mod pdr;
#[doc = "PSR (r) register accessor: PIO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr`]
module"]
#[doc(alias = "PSR")]
pub type Psr = crate::Reg<psr::PsrSpec>;
#[doc = "PIO Status Register"]
pub mod psr;
#[doc = "OER (w) register accessor: Output Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oer::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oer`]
module"]
#[doc(alias = "OER")]
pub type Oer = crate::Reg<oer::OerSpec>;
#[doc = "Output Enable Register"]
pub mod oer;
#[doc = "ODR (w) register accessor: Output Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odr`]
module"]
#[doc(alias = "ODR")]
pub type Odr = crate::Reg<odr::OdrSpec>;
#[doc = "Output Disable Register"]
pub mod odr;
#[doc = "OSR (r) register accessor: Output Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`osr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osr`]
module"]
#[doc(alias = "OSR")]
pub type Osr = crate::Reg<osr::OsrSpec>;
#[doc = "Output Status Register"]
pub mod osr;
#[doc = "IFER (w) register accessor: Glitch Input Filter Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifer::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifer`]
module"]
#[doc(alias = "IFER")]
pub type Ifer = crate::Reg<ifer::IferSpec>;
#[doc = "Glitch Input Filter Enable Register"]
pub mod ifer;
#[doc = "IFDR (w) register accessor: Glitch Input Filter Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifdr`]
module"]
#[doc(alias = "IFDR")]
pub type Ifdr = crate::Reg<ifdr::IfdrSpec>;
#[doc = "Glitch Input Filter Disable Register"]
pub mod ifdr;
#[doc = "IFSR (r) register accessor: Glitch Input Filter Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifsr`]
module"]
#[doc(alias = "IFSR")]
pub type Ifsr = crate::Reg<ifsr::IfsrSpec>;
#[doc = "Glitch Input Filter Status Register"]
pub mod ifsr;
#[doc = "SODR (w) register accessor: Set Output Data Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sodr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sodr`]
module"]
#[doc(alias = "SODR")]
pub type Sodr = crate::Reg<sodr::SodrSpec>;
#[doc = "Set Output Data Register"]
pub mod sodr;
#[doc = "CODR (w) register accessor: Clear Output Data Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codr`]
module"]
#[doc(alias = "CODR")]
pub type Codr = crate::Reg<codr::CodrSpec>;
#[doc = "Clear Output Data Register"]
pub mod codr;
#[doc = "ODSR (rw) register accessor: Output Data Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`odsr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odsr`]
module"]
#[doc(alias = "ODSR")]
pub type Odsr = crate::Reg<odsr::OdsrSpec>;
#[doc = "Output Data Status Register"]
pub mod odsr;
#[doc = "PDSR (r) register accessor: Pin Data Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdsr`]
module"]
#[doc(alias = "PDSR")]
pub type Pdsr = crate::Reg<pdsr::PdsrSpec>;
#[doc = "Pin Data Status Register"]
pub mod pdsr;
#[doc = "IER (w) register accessor: Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "MDER (w) register accessor: Multi-driver Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mder::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mder`]
module"]
#[doc(alias = "MDER")]
pub type Mder = crate::Reg<mder::MderSpec>;
#[doc = "Multi-driver Enable Register"]
pub mod mder;
#[doc = "MDDR (w) register accessor: Multi-driver Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mddr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mddr`]
module"]
#[doc(alias = "MDDR")]
pub type Mddr = crate::Reg<mddr::MddrSpec>;
#[doc = "Multi-driver Disable Register"]
pub mod mddr;
#[doc = "MDSR (r) register accessor: Multi-driver Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdsr`]
module"]
#[doc(alias = "MDSR")]
pub type Mdsr = crate::Reg<mdsr::MdsrSpec>;
#[doc = "Multi-driver Status Register"]
pub mod mdsr;
#[doc = "PUDR (w) register accessor: Pull-up Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pudr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pudr`]
module"]
#[doc(alias = "PUDR")]
pub type Pudr = crate::Reg<pudr::PudrSpec>;
#[doc = "Pull-up Disable Register"]
pub mod pudr;
#[doc = "PUER (w) register accessor: Pull-up Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`puer::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@puer`]
module"]
#[doc(alias = "PUER")]
pub type Puer = crate::Reg<puer::PuerSpec>;
#[doc = "Pull-up Enable Register"]
pub mod puer;
#[doc = "PUSR (r) register accessor: Pad Pull-up Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pusr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pusr`]
module"]
#[doc(alias = "PUSR")]
pub type Pusr = crate::Reg<pusr::PusrSpec>;
#[doc = "Pad Pull-up Status Register"]
pub mod pusr;
#[doc = "ABSR (rw) register accessor: Peripheral AB Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`absr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`absr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@absr`]
module"]
#[doc(alias = "ABSR")]
pub type Absr = crate::Reg<absr::AbsrSpec>;
#[doc = "Peripheral AB Select Register"]
pub mod absr;
#[doc = "SCIFSR (w) register accessor: System Clock Glitch Input Filter Select Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scifsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scifsr`]
module"]
#[doc(alias = "SCIFSR")]
pub type Scifsr = crate::Reg<scifsr::ScifsrSpec>;
#[doc = "System Clock Glitch Input Filter Select Register"]
pub mod scifsr;
#[doc = "DIFSR (w) register accessor: Debouncing Input Filter Select Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`difsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@difsr`]
module"]
#[doc(alias = "DIFSR")]
pub type Difsr = crate::Reg<difsr::DifsrSpec>;
#[doc = "Debouncing Input Filter Select Register"]
pub mod difsr;
#[doc = "IFDGSR (r) register accessor: Glitch or Debouncing Input Filter Clock Selection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifdgsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifdgsr`]
module"]
#[doc(alias = "IFDGSR")]
pub type Ifdgsr = crate::Reg<ifdgsr::IfdgsrSpec>;
#[doc = "Glitch or Debouncing Input Filter Clock Selection Status Register"]
pub mod ifdgsr;
#[doc = "SCDR (rw) register accessor: Slow Clock Divider Debouncing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scdr`]
module"]
#[doc(alias = "SCDR")]
pub type Scdr = crate::Reg<scdr::ScdrSpec>;
#[doc = "Slow Clock Divider Debouncing Register"]
pub mod scdr;
#[doc = "OWER (w) register accessor: Output Write Enable\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ower::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ower`]
module"]
#[doc(alias = "OWER")]
pub type Ower = crate::Reg<ower::OwerSpec>;
#[doc = "Output Write Enable"]
pub mod ower;
#[doc = "OWDR (w) register accessor: Output Write Disable\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`owdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owdr`]
module"]
#[doc(alias = "OWDR")]
pub type Owdr = crate::Reg<owdr::OwdrSpec>;
#[doc = "Output Write Disable"]
pub mod owdr;
#[doc = "OWSR (r) register accessor: Output Write Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`owsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owsr`]
module"]
#[doc(alias = "OWSR")]
pub type Owsr = crate::Reg<owsr::OwsrSpec>;
#[doc = "Output Write Status Register"]
pub mod owsr;
#[doc = "AIMER (w) register accessor: Additional Interrupt Modes Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aimer::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aimer`]
module"]
#[doc(alias = "AIMER")]
pub type Aimer = crate::Reg<aimer::AimerSpec>;
#[doc = "Additional Interrupt Modes Enable Register"]
pub mod aimer;
#[doc = "AIMDR (w) register accessor: Additional Interrupt Modes Disables Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aimdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aimdr`]
module"]
#[doc(alias = "AIMDR")]
pub type Aimdr = crate::Reg<aimdr::AimdrSpec>;
#[doc = "Additional Interrupt Modes Disables Register"]
pub mod aimdr;
#[doc = "AIMMR (r) register accessor: Additional Interrupt Modes Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aimmr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aimmr`]
module"]
#[doc(alias = "AIMMR")]
pub type Aimmr = crate::Reg<aimmr::AimmrSpec>;
#[doc = "Additional Interrupt Modes Mask Register"]
pub mod aimmr;
#[doc = "ESR (w) register accessor: Edge Select Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esr`]
module"]
#[doc(alias = "ESR")]
pub type Esr = crate::Reg<esr::EsrSpec>;
#[doc = "Edge Select Register"]
pub mod esr;
#[doc = "LSR (w) register accessor: Level Select Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`]
module"]
#[doc(alias = "LSR")]
pub type Lsr = crate::Reg<lsr::LsrSpec>;
#[doc = "Level Select Register"]
pub mod lsr;
#[doc = "ELSR (r) register accessor: Edge/Level Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`elsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@elsr`]
module"]
#[doc(alias = "ELSR")]
pub type Elsr = crate::Reg<elsr::ElsrSpec>;
#[doc = "Edge/Level Status Register"]
pub mod elsr;
#[doc = "FELLSR (w) register accessor: Falling Edge/Low Level Select Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fellsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fellsr`]
module"]
#[doc(alias = "FELLSR")]
pub type Fellsr = crate::Reg<fellsr::FellsrSpec>;
#[doc = "Falling Edge/Low Level Select Register"]
pub mod fellsr;
#[doc = "REHLSR (w) register accessor: Rising Edge/ High Level Select Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rehlsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rehlsr`]
module"]
#[doc(alias = "REHLSR")]
pub type Rehlsr = crate::Reg<rehlsr::RehlsrSpec>;
#[doc = "Rising Edge/ High Level Select Register"]
pub mod rehlsr;
#[doc = "FRLHSR (r) register accessor: Fall/Rise - Low/High Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`frlhsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frlhsr`]
module"]
#[doc(alias = "FRLHSR")]
pub type Frlhsr = crate::Reg<frlhsr::FrlhsrSpec>;
#[doc = "Fall/Rise - Low/High Status Register"]
pub mod frlhsr;
#[doc = "LOCKSR (r) register accessor: Lock Status\n\nYou can [`read`](crate::Reg::read) this register and get [`locksr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@locksr`]
module"]
#[doc(alias = "LOCKSR")]
pub type Locksr = crate::Reg<locksr::LocksrSpec>;
#[doc = "Lock Status"]
pub mod locksr;
#[doc = "WPMR (rw) register accessor: Write Protect Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`]
module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: Write Protect Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`]
module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "Write Protect Status Register"]
pub mod wpsr;
