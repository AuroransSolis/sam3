#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_cr: [u8; 0x04],
    _reserved_1_mr: [u8; 0x04],
    _reserved_2_ier: [u8; 0x04],
    _reserved_3_idr: [u8; 0x04],
    _reserved_4_imr: [u8; 0x04],
    _reserved_5_csr: [u8; 0x04],
    rhr: Rhr,
    thr: Thr,
    brgr: Brgr,
    rtor: Rtor,
    ttgr: Ttgr,
    _reserved11: [u8; 0x14],
    fidi: Fidi,
    ner: Ner,
    _reserved13: [u8; 0x04],
    if_: If,
    man: Man,
    _reserved15: [u8; 0x90],
    wpmr: Wpmr,
    wpsr: Wpsr,
    _reserved17: [u8; 0x14],
    rpr: Rpr,
    rcr: Rcr,
    tpr: Tpr,
    tcr: Tcr,
    rnpr: Rnpr,
    rncr: Rncr,
    tnpr: Tnpr,
    tncr: Tncr,
    ptcr: Ptcr,
    ptsr: Ptsr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn spi_mode_cr_spi_mode(&self) -> &SpiModeCrSpiMode {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub const fn spi_mode_mr_spi_mode(&self) -> &SpiModeMrSpiMode {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &Mr {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn spi_mode_ier_spi_mode(&self) -> &SpiModeIerSpiMode {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn spi_mode_idr_spi_mode(&self) -> &SpiModeIdrSpiMode {
        unsafe { &*(self as *const Self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        unsafe { &*(self as *const Self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn spi_mode_imr_spi_mode(&self) -> &SpiModeImrSpiMode {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub const fn spi_mode_csr_spi_mode(&self) -> &SpiModeCsrSpiMode {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x18 - Receiver Holding Register"]
    #[inline(always)]
    pub const fn rhr(&self) -> &Rhr {
        &self.rhr
    }
    #[doc = "0x1c - Transmitter Holding Register"]
    #[inline(always)]
    pub const fn thr(&self) -> &Thr {
        &self.thr
    }
    #[doc = "0x20 - Baud Rate Generator Register"]
    #[inline(always)]
    pub const fn brgr(&self) -> &Brgr {
        &self.brgr
    }
    #[doc = "0x24 - Receiver Time-out Register"]
    #[inline(always)]
    pub const fn rtor(&self) -> &Rtor {
        &self.rtor
    }
    #[doc = "0x28 - Transmitter Timeguard Register"]
    #[inline(always)]
    pub const fn ttgr(&self) -> &Ttgr {
        &self.ttgr
    }
    #[doc = "0x40 - FI DI Ratio Register"]
    #[inline(always)]
    pub const fn fidi(&self) -> &Fidi {
        &self.fidi
    }
    #[doc = "0x44 - Number of Errors Register"]
    #[inline(always)]
    pub const fn ner(&self) -> &Ner {
        &self.ner
    }
    #[doc = "0x4c - IrDA Filter Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x50 - Manchester Encoder Decoder Register"]
    #[inline(always)]
    pub const fn man(&self) -> &Man {
        &self.man
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
    #[doc = "0x100 - Receive Pointer Register"]
    #[inline(always)]
    pub const fn rpr(&self) -> &Rpr {
        &self.rpr
    }
    #[doc = "0x104 - Receive Counter Register"]
    #[inline(always)]
    pub const fn rcr(&self) -> &Rcr {
        &self.rcr
    }
    #[doc = "0x108 - Transmit Pointer Register"]
    #[inline(always)]
    pub const fn tpr(&self) -> &Tpr {
        &self.tpr
    }
    #[doc = "0x10c - Transmit Counter Register"]
    #[inline(always)]
    pub const fn tcr(&self) -> &Tcr {
        &self.tcr
    }
    #[doc = "0x110 - Receive Next Pointer Register"]
    #[inline(always)]
    pub const fn rnpr(&self) -> &Rnpr {
        &self.rnpr
    }
    #[doc = "0x114 - Receive Next Counter Register"]
    #[inline(always)]
    pub const fn rncr(&self) -> &Rncr {
        &self.rncr
    }
    #[doc = "0x118 - Transmit Next Pointer Register"]
    #[inline(always)]
    pub const fn tnpr(&self) -> &Tnpr {
        &self.tnpr
    }
    #[doc = "0x11c - Transmit Next Counter Register"]
    #[inline(always)]
    pub const fn tncr(&self) -> &Tncr {
        &self.tncr
    }
    #[doc = "0x120 - Transfer Control Register"]
    #[inline(always)]
    pub const fn ptcr(&self) -> &Ptcr {
        &self.ptcr
    }
    #[doc = "0x124 - Transfer Status Register"]
    #[inline(always)]
    pub const fn ptsr(&self) -> &Ptsr {
        &self.ptsr
    }
}
#[doc = "CR (w) register accessor: Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "SPI_MODE_CR_SPI_MODE (w) register accessor: Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mode_cr_spi_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mode_cr_spi_mode`]
module"]
#[doc(alias = "SPI_MODE_CR_SPI_MODE")]
pub type SpiModeCrSpiMode = crate::Reg<spi_mode_cr_spi_mode::SpiModeCrSpiModeSpec>;
#[doc = "Control Register"]
pub mod spi_mode_cr_spi_mode;
#[doc = "MR (rw) register accessor: Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`]
module"]
#[doc(alias = "MR")]
pub type Mr = crate::Reg<mr::MrSpec>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "SPI_MODE_MR_SPI_MODE (rw) register accessor: Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mode_mr_spi_mode::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mode_mr_spi_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mode_mr_spi_mode`]
module"]
#[doc(alias = "SPI_MODE_MR_SPI_MODE")]
pub type SpiModeMrSpiMode = crate::Reg<spi_mode_mr_spi_mode::SpiModeMrSpiModeSpec>;
#[doc = "Mode Register"]
pub mod spi_mode_mr_spi_mode;
#[doc = "IER (w) register accessor: Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "SPI_MODE_IER_SPI_MODE (w) register accessor: Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mode_ier_spi_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mode_ier_spi_mode`]
module"]
#[doc(alias = "SPI_MODE_IER_SPI_MODE")]
pub type SpiModeIerSpiMode = crate::Reg<spi_mode_ier_spi_mode::SpiModeIerSpiModeSpec>;
#[doc = "Interrupt Enable Register"]
pub mod spi_mode_ier_spi_mode;
#[doc = "IDR (w) register accessor: Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "SPI_MODE_IDR_SPI_MODE (w) register accessor: Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mode_idr_spi_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mode_idr_spi_mode`]
module"]
#[doc(alias = "SPI_MODE_IDR_SPI_MODE")]
pub type SpiModeIdrSpiMode = crate::Reg<spi_mode_idr_spi_mode::SpiModeIdrSpiModeSpec>;
#[doc = "Interrupt Disable Register"]
pub mod spi_mode_idr_spi_mode;
#[doc = "IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "SPI_MODE_IMR_SPI_MODE (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mode_imr_spi_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mode_imr_spi_mode`]
module"]
#[doc(alias = "SPI_MODE_IMR_SPI_MODE")]
pub type SpiModeImrSpiMode = crate::Reg<spi_mode_imr_spi_mode::SpiModeImrSpiModeSpec>;
#[doc = "Interrupt Mask Register"]
pub mod spi_mode_imr_spi_mode;
#[doc = "CSR (r) register accessor: Channel Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "Channel Status Register"]
pub mod csr;
#[doc = "SPI_MODE_CSR_SPI_MODE (r) register accessor: Channel Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mode_csr_spi_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mode_csr_spi_mode`]
module"]
#[doc(alias = "SPI_MODE_CSR_SPI_MODE")]
pub type SpiModeCsrSpiMode = crate::Reg<spi_mode_csr_spi_mode::SpiModeCsrSpiModeSpec>;
#[doc = "Channel Status Register"]
pub mod spi_mode_csr_spi_mode;
#[doc = "RHR (r) register accessor: Receiver Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rhr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rhr`]
module"]
#[doc(alias = "RHR")]
pub type Rhr = crate::Reg<rhr::RhrSpec>;
#[doc = "Receiver Holding Register"]
pub mod rhr;
#[doc = "THR (w) register accessor: Transmitter Holding Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thr`]
module"]
#[doc(alias = "THR")]
pub type Thr = crate::Reg<thr::ThrSpec>;
#[doc = "Transmitter Holding Register"]
pub mod thr;
#[doc = "BRGR (rw) register accessor: Baud Rate Generator Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brgr`]
module"]
#[doc(alias = "BRGR")]
pub type Brgr = crate::Reg<brgr::BrgrSpec>;
#[doc = "Baud Rate Generator Register"]
pub mod brgr;
#[doc = "RTOR (rw) register accessor: Receiver Time-out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtor`]
module"]
#[doc(alias = "RTOR")]
pub type Rtor = crate::Reg<rtor::RtorSpec>;
#[doc = "Receiver Time-out Register"]
pub mod rtor;
#[doc = "TTGR (rw) register accessor: Transmitter Timeguard Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttgr`]
module"]
#[doc(alias = "TTGR")]
pub type Ttgr = crate::Reg<ttgr::TtgrSpec>;
#[doc = "Transmitter Timeguard Register"]
pub mod ttgr;
#[doc = "FIDI (rw) register accessor: FI DI Ratio Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fidi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fidi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fidi`]
module"]
#[doc(alias = "FIDI")]
pub type Fidi = crate::Reg<fidi::FidiSpec>;
#[doc = "FI DI Ratio Register"]
pub mod fidi;
#[doc = "NER (r) register accessor: Number of Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ner::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ner`]
module"]
#[doc(alias = "NER")]
pub type Ner = crate::Reg<ner::NerSpec>;
#[doc = "Number of Errors Register"]
pub mod ner;
#[doc = "IF (rw) register accessor: IrDA Filter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`]
module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IfSpec>;
#[doc = "IrDA Filter Register"]
pub mod if_;
#[doc = "MAN (rw) register accessor: Manchester Encoder Decoder Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`man::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`man::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@man`]
module"]
#[doc(alias = "MAN")]
pub type Man = crate::Reg<man::ManSpec>;
#[doc = "Manchester Encoder Decoder Register"]
pub mod man;
#[doc = "WPMR (rw) register accessor: Write Protect Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`]
module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: Write Protect Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`]
module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "Write Protect Status Register"]
pub mod wpsr;
#[doc = "RPR (rw) register accessor: Receive Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpr`]
module"]
#[doc(alias = "RPR")]
pub type Rpr = crate::Reg<rpr::RprSpec>;
#[doc = "Receive Pointer Register"]
pub mod rpr;
#[doc = "RCR (rw) register accessor: Receive Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr`]
module"]
#[doc(alias = "RCR")]
pub type Rcr = crate::Reg<rcr::RcrSpec>;
#[doc = "Receive Counter Register"]
pub mod rcr;
#[doc = "TPR (rw) register accessor: Transmit Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpr`]
module"]
#[doc(alias = "TPR")]
pub type Tpr = crate::Reg<tpr::TprSpec>;
#[doc = "Transmit Pointer Register"]
pub mod tpr;
#[doc = "TCR (rw) register accessor: Transmit Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr`]
module"]
#[doc(alias = "TCR")]
pub type Tcr = crate::Reg<tcr::TcrSpec>;
#[doc = "Transmit Counter Register"]
pub mod tcr;
#[doc = "RNPR (rw) register accessor: Receive Next Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rnpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnpr`]
module"]
#[doc(alias = "RNPR")]
pub type Rnpr = crate::Reg<rnpr::RnprSpec>;
#[doc = "Receive Next Pointer Register"]
pub mod rnpr;
#[doc = "RNCR (rw) register accessor: Receive Next Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rncr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rncr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rncr`]
module"]
#[doc(alias = "RNCR")]
pub type Rncr = crate::Reg<rncr::RncrSpec>;
#[doc = "Receive Next Counter Register"]
pub mod rncr;
#[doc = "TNPR (rw) register accessor: Transmit Next Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tnpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tnpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tnpr`]
module"]
#[doc(alias = "TNPR")]
pub type Tnpr = crate::Reg<tnpr::TnprSpec>;
#[doc = "Transmit Next Pointer Register"]
pub mod tnpr;
#[doc = "TNCR (rw) register accessor: Transmit Next Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tncr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tncr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tncr`]
module"]
#[doc(alias = "TNCR")]
pub type Tncr = crate::Reg<tncr::TncrSpec>;
#[doc = "Transmit Next Counter Register"]
pub mod tncr;
#[doc = "PTCR (w) register accessor: Transfer Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptcr`]
module"]
#[doc(alias = "PTCR")]
pub type Ptcr = crate::Reg<ptcr::PtcrSpec>;
#[doc = "Transfer Control Register"]
pub mod ptcr;
#[doc = "PTSR (r) register accessor: Transfer Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptsr`]
module"]
#[doc(alias = "PTSR")]
pub type Ptsr = crate::Reg<ptsr::PtsrSpec>;
#[doc = "Transfer Status Register"]
pub mod ptsr;
