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
    abcdsr0: Abcdsr0,
    abcdsr1: Abcdsr1,
    _reserved25: [u8; 0x08],
    ifscdr: Ifscdr,
    ifscer: Ifscer,
    ifscsr: Ifscsr,
    scdr: Scdr,
    ppddr: Ppddr,
    ppder: Ppder,
    ppdsr: Ppdsr,
    _reserved32: [u8; 0x04],
    ower: Ower,
    owdr: Owdr,
    owsr: Owsr,
    _reserved35: [u8; 0x04],
    aimer: Aimer,
    aimdr: Aimdr,
    aimmr: Aimmr,
    _reserved38: [u8; 0x04],
    esr: Esr,
    lsr: Lsr,
    elsr: Elsr,
    _reserved41: [u8; 0x04],
    fellsr: Fellsr,
    rehlsr: Rehlsr,
    frlhsr: Frlhsr,
    _reserved44: [u8; 0x04],
    locksr: Locksr,
    wpmr: Wpmr,
    wpsr: Wpsr,
    _reserved47: [u8; 0x14],
    schmitt: Schmitt,
    _reserved48: [u8; 0x4c],
    pcmr: Pcmr,
    pcier: Pcier,
    pcidr: Pcidr,
    pcimr: Pcimr,
    pcisr: Pcisr,
    pcrhr: Pcrhr,
    rpr: Rpr,
    rcr: Rcr,
    _reserved56: [u8; 0x08],
    rnpr: Rnpr,
    rncr: Rncr,
    _reserved58: [u8; 0x08],
    ptcr: Ptcr,
    ptsr: Ptsr,
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
    #[doc = "0x70 - Peripheral Select Register 0"]
    #[inline(always)]
    pub const fn abcdsr0(&self) -> &Abcdsr0 {
        &self.abcdsr0
    }
    #[doc = "0x74 - Peripheral Select Register 1"]
    #[inline(always)]
    pub const fn abcdsr1(&self) -> &Abcdsr1 {
        &self.abcdsr1
    }
    #[doc = "0x80 - Input Filter Slow Clock Disable Register"]
    #[inline(always)]
    pub const fn ifscdr(&self) -> &Ifscdr {
        &self.ifscdr
    }
    #[doc = "0x84 - Input Filter Slow Clock Enable Register"]
    #[inline(always)]
    pub const fn ifscer(&self) -> &Ifscer {
        &self.ifscer
    }
    #[doc = "0x88 - Input Filter Slow Clock Status Register"]
    #[inline(always)]
    pub const fn ifscsr(&self) -> &Ifscsr {
        &self.ifscsr
    }
    #[doc = "0x8c - Slow Clock Divider Debouncing Register"]
    #[inline(always)]
    pub const fn scdr(&self) -> &Scdr {
        &self.scdr
    }
    #[doc = "0x90 - Pad Pull-down Disable Register"]
    #[inline(always)]
    pub const fn ppddr(&self) -> &Ppddr {
        &self.ppddr
    }
    #[doc = "0x94 - Pad Pull-down Enable Register"]
    #[inline(always)]
    pub const fn ppder(&self) -> &Ppder {
        &self.ppder
    }
    #[doc = "0x98 - Pad Pull-down Status Register"]
    #[inline(always)]
    pub const fn ppdsr(&self) -> &Ppdsr {
        &self.ppdsr
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
    #[doc = "0x100 - Schmitt Trigger Register"]
    #[inline(always)]
    pub const fn schmitt(&self) -> &Schmitt {
        &self.schmitt
    }
    #[doc = "0x150 - Parallel Capture Mode Register"]
    #[inline(always)]
    pub const fn pcmr(&self) -> &Pcmr {
        &self.pcmr
    }
    #[doc = "0x154 - Parallel Capture Interrupt Enable Register"]
    #[inline(always)]
    pub const fn pcier(&self) -> &Pcier {
        &self.pcier
    }
    #[doc = "0x158 - Parallel Capture Interrupt Disable Register"]
    #[inline(always)]
    pub const fn pcidr(&self) -> &Pcidr {
        &self.pcidr
    }
    #[doc = "0x15c - Parallel Capture Interrupt Mask Register"]
    #[inline(always)]
    pub const fn pcimr(&self) -> &Pcimr {
        &self.pcimr
    }
    #[doc = "0x160 - Parallel Capture Interrupt Status Register"]
    #[inline(always)]
    pub const fn pcisr(&self) -> &Pcisr {
        &self.pcisr
    }
    #[doc = "0x164 - Parallel Capture Reception Holding Register"]
    #[inline(always)]
    pub const fn pcrhr(&self) -> &Pcrhr {
        &self.pcrhr
    }
    #[doc = "0x168 - Receive Pointer Register"]
    #[inline(always)]
    pub const fn rpr(&self) -> &Rpr {
        &self.rpr
    }
    #[doc = "0x16c - Receive Counter Register"]
    #[inline(always)]
    pub const fn rcr(&self) -> &Rcr {
        &self.rcr
    }
    #[doc = "0x178 - Receive Next Pointer Register"]
    #[inline(always)]
    pub const fn rnpr(&self) -> &Rnpr {
        &self.rnpr
    }
    #[doc = "0x17c - Receive Next Counter Register"]
    #[inline(always)]
    pub const fn rncr(&self) -> &Rncr {
        &self.rncr
    }
    #[doc = "0x188 - Transfer Control Register"]
    #[inline(always)]
    pub const fn ptcr(&self) -> &Ptcr {
        &self.ptcr
    }
    #[doc = "0x18c - Transfer Status Register"]
    #[inline(always)]
    pub const fn ptsr(&self) -> &Ptsr {
        &self.ptsr
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
#[doc = "ABCDSR0 (rw) register accessor: Peripheral Select Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`abcdsr0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abcdsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@abcdsr0`]
module"]
#[doc(alias = "ABCDSR0")]
pub type Abcdsr0 = crate::Reg<abcdsr0::Abcdsr0Spec>;
#[doc = "Peripheral Select Register 0"]
pub mod abcdsr0;
#[doc = "ABCDSR1 (rw) register accessor: Peripheral Select Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`abcdsr1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abcdsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@abcdsr1`]
module"]
#[doc(alias = "ABCDSR1")]
pub type Abcdsr1 = crate::Reg<abcdsr1::Abcdsr1Spec>;
#[doc = "Peripheral Select Register 1"]
pub mod abcdsr1;
#[doc = "IFSCDR (w) register accessor: Input Filter Slow Clock Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifscdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifscdr`]
module"]
#[doc(alias = "IFSCDR")]
pub type Ifscdr = crate::Reg<ifscdr::IfscdrSpec>;
#[doc = "Input Filter Slow Clock Disable Register"]
pub mod ifscdr;
#[doc = "IFSCER (w) register accessor: Input Filter Slow Clock Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifscer::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifscer`]
module"]
#[doc(alias = "IFSCER")]
pub type Ifscer = crate::Reg<ifscer::IfscerSpec>;
#[doc = "Input Filter Slow Clock Enable Register"]
pub mod ifscer;
#[doc = "IFSCSR (r) register accessor: Input Filter Slow Clock Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifscsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifscsr`]
module"]
#[doc(alias = "IFSCSR")]
pub type Ifscsr = crate::Reg<ifscsr::IfscsrSpec>;
#[doc = "Input Filter Slow Clock Status Register"]
pub mod ifscsr;
#[doc = "SCDR (rw) register accessor: Slow Clock Divider Debouncing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scdr`]
module"]
#[doc(alias = "SCDR")]
pub type Scdr = crate::Reg<scdr::ScdrSpec>;
#[doc = "Slow Clock Divider Debouncing Register"]
pub mod scdr;
#[doc = "PPDDR (w) register accessor: Pad Pull-down Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppddr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppddr`]
module"]
#[doc(alias = "PPDDR")]
pub type Ppddr = crate::Reg<ppddr::PpddrSpec>;
#[doc = "Pad Pull-down Disable Register"]
pub mod ppddr;
#[doc = "PPDER (w) register accessor: Pad Pull-down Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppder::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppder`]
module"]
#[doc(alias = "PPDER")]
pub type Ppder = crate::Reg<ppder::PpderSpec>;
#[doc = "Pad Pull-down Enable Register"]
pub mod ppder;
#[doc = "PPDSR (r) register accessor: Pad Pull-down Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ppdsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppdsr`]
module"]
#[doc(alias = "PPDSR")]
pub type Ppdsr = crate::Reg<ppdsr::PpdsrSpec>;
#[doc = "Pad Pull-down Status Register"]
pub mod ppdsr;
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
#[doc = "SCHMITT (rw) register accessor: Schmitt Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`schmitt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`schmitt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@schmitt`]
module"]
#[doc(alias = "SCHMITT")]
pub type Schmitt = crate::Reg<schmitt::SchmittSpec>;
#[doc = "Schmitt Trigger Register"]
pub mod schmitt;
#[doc = "PCMR (rw) register accessor: Parallel Capture Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcmr`]
module"]
#[doc(alias = "PCMR")]
pub type Pcmr = crate::Reg<pcmr::PcmrSpec>;
#[doc = "Parallel Capture Mode Register"]
pub mod pcmr;
#[doc = "PCIER (w) register accessor: Parallel Capture Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcier`]
module"]
#[doc(alias = "PCIER")]
pub type Pcier = crate::Reg<pcier::PcierSpec>;
#[doc = "Parallel Capture Interrupt Enable Register"]
pub mod pcier;
#[doc = "PCIDR (w) register accessor: Parallel Capture Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcidr`]
module"]
#[doc(alias = "PCIDR")]
pub type Pcidr = crate::Reg<pcidr::PcidrSpec>;
#[doc = "Parallel Capture Interrupt Disable Register"]
pub mod pcidr;
#[doc = "PCIMR (r) register accessor: Parallel Capture Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcimr`]
module"]
#[doc(alias = "PCIMR")]
pub type Pcimr = crate::Reg<pcimr::PcimrSpec>;
#[doc = "Parallel Capture Interrupt Mask Register"]
pub mod pcimr;
#[doc = "PCISR (r) register accessor: Parallel Capture Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcisr`]
module"]
#[doc(alias = "PCISR")]
pub type Pcisr = crate::Reg<pcisr::PcisrSpec>;
#[doc = "Parallel Capture Interrupt Status Register"]
pub mod pcisr;
#[doc = "PCRHR (r) register accessor: Parallel Capture Reception Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrhr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrhr`]
module"]
#[doc(alias = "PCRHR")]
pub type Pcrhr = crate::Reg<pcrhr::PcrhrSpec>;
#[doc = "Parallel Capture Reception Holding Register"]
pub mod pcrhr;
#[doc = "RPR (rw) register accessor: Receive Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpr`]
module"]
#[doc(alias = "RPR")]
pub type Rpr = crate::Reg<rpr::RprSpec>;
#[doc = "Receive Pointer Register"]
pub mod rpr;
#[doc = "RCR (rw) register accessor: Receive Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr`]
module"]
#[doc(alias = "RCR")]
pub type Rcr = crate::Reg<rcr::RcrSpec>;
#[doc = "Receive Counter Register"]
pub mod rcr;
#[doc = "RNPR (rw) register accessor: Receive Next Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rnpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnpr`]
module"]
#[doc(alias = "RNPR")]
pub type Rnpr = crate::Reg<rnpr::RnprSpec>;
#[doc = "Receive Next Pointer Register"]
pub mod rnpr;
#[doc = "RNCR (rw) register accessor: Receive Next Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rncr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rncr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rncr`]
module"]
#[doc(alias = "RNCR")]
pub type Rncr = crate::Reg<rncr::RncrSpec>;
#[doc = "Receive Next Counter Register"]
pub mod rncr;
#[doc = "PTCR (w) register accessor: Transfer Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptcr`]
module"]
#[doc(alias = "PTCR")]
pub type Ptcr = crate::Reg<ptcr::PtcrSpec>;
#[doc = "Transfer Control Register"]
pub mod ptcr;
#[doc = "PTSR (r) register accessor: Transfer Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptsr`]
module"]
#[doc(alias = "PTSR")]
pub type Ptsr = crate::Reg<ptsr::PtsrSpec>;
#[doc = "Transfer Status Register"]
pub mod ptsr;
