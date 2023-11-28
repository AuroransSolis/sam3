#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_cr: [u8; 0x04],
    _reserved_1_mr: [u8; 0x04],
    _reserved_2_ier: [u8; 0x04],
    _reserved_3_idr: [u8; 0x04],
    _reserved_4_imr: [u8; 0x04],
    _reserved_5_csr: [u8; 0x04],
    rhr: RHR,
    thr: THR,
    brgr: BRGR,
    rtor: RTOR,
    ttgr: TTGR,
    _reserved11: [u8; 0x14],
    fidi: FIDI,
    ner: NER,
    _reserved13: [u8; 0x04],
    if_: IF,
    man: MAN,
    linmr: LINMR,
    linir: LINIR,
    linbrr: LINBRR,
    _reserved18: [u8; 0x84],
    wpmr: WPMR,
    wpsr: WPSR,
    _reserved20: [u8; 0x14],
    rpr: RPR,
    rcr: RCR,
    tpr: TPR,
    tcr: TCR,
    rnpr: RNPR,
    rncr: RNCR,
    tnpr: TNPR,
    tncr: TNCR,
    ptcr: PTCR,
    ptsr: PTSR,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn spi_mode_cr_spi_mode(&self) -> &SPI_MODE_CR_SPI_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub const fn spi_mode_mr_spi_mode(&self) -> &SPI_MODE_MR_SPI_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &MR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn lin_mode_ier_lin_mode(&self) -> &LIN_MODE_IER_LIN_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn spi_mode_ier_spi_mode(&self) -> &SPI_MODE_IER_SPI_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn lin_mode_idr_lin_mode(&self) -> &LIN_MODE_IDR_LIN_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn spi_mode_idr_spi_mode(&self) -> &SPI_MODE_IDR_SPI_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn lin_mode_imr_lin_mode(&self) -> &LIN_MODE_IMR_LIN_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn spi_mode_imr_spi_mode(&self) -> &SPI_MODE_IMR_SPI_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub const fn lin_mode_csr_lin_mode(&self) -> &LIN_MODE_CSR_LIN_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub const fn spi_mode_csr_spi_mode(&self) -> &SPI_MODE_CSR_SPI_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x18 - Receive Holding Register"]
    #[inline(always)]
    pub const fn rhr(&self) -> &RHR {
        &self.rhr
    }
    #[doc = "0x1c - Transmit Holding Register"]
    #[inline(always)]
    pub const fn thr(&self) -> &THR {
        &self.thr
    }
    #[doc = "0x20 - Baud Rate Generator Register"]
    #[inline(always)]
    pub const fn brgr(&self) -> &BRGR {
        &self.brgr
    }
    #[doc = "0x24 - Receiver Time-out Register"]
    #[inline(always)]
    pub const fn rtor(&self) -> &RTOR {
        &self.rtor
    }
    #[doc = "0x28 - Transmitter Timeguard Register"]
    #[inline(always)]
    pub const fn ttgr(&self) -> &TTGR {
        &self.ttgr
    }
    #[doc = "0x40 - FI DI Ratio Register"]
    #[inline(always)]
    pub const fn fidi(&self) -> &FIDI {
        &self.fidi
    }
    #[doc = "0x44 - Number of Errors Register"]
    #[inline(always)]
    pub const fn ner(&self) -> &NER {
        &self.ner
    }
    #[doc = "0x4c - IrDA Filter Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &IF {
        &self.if_
    }
    #[doc = "0x50 - Manchester Configuration Register"]
    #[inline(always)]
    pub const fn man(&self) -> &MAN {
        &self.man
    }
    #[doc = "0x54 - LIN Mode Register"]
    #[inline(always)]
    pub const fn linmr(&self) -> &LINMR {
        &self.linmr
    }
    #[doc = "0x58 - LIN Identifier Register"]
    #[inline(always)]
    pub const fn linir(&self) -> &LINIR {
        &self.linir
    }
    #[doc = "0x5c - LIN Baud Rate Register"]
    #[inline(always)]
    pub const fn linbrr(&self) -> &LINBRR {
        &self.linbrr
    }
    #[doc = "0xe4 - Write Protection Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &WPMR {
        &self.wpmr
    }
    #[doc = "0xe8 - Write Protection Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &WPSR {
        &self.wpsr
    }
    #[doc = "0x100 - Receive Pointer Register"]
    #[inline(always)]
    pub const fn rpr(&self) -> &RPR {
        &self.rpr
    }
    #[doc = "0x104 - Receive Counter Register"]
    #[inline(always)]
    pub const fn rcr(&self) -> &RCR {
        &self.rcr
    }
    #[doc = "0x108 - Transmit Pointer Register"]
    #[inline(always)]
    pub const fn tpr(&self) -> &TPR {
        &self.tpr
    }
    #[doc = "0x10c - Transmit Counter Register"]
    #[inline(always)]
    pub const fn tcr(&self) -> &TCR {
        &self.tcr
    }
    #[doc = "0x110 - Receive Next Pointer Register"]
    #[inline(always)]
    pub const fn rnpr(&self) -> &RNPR {
        &self.rnpr
    }
    #[doc = "0x114 - Receive Next Counter Register"]
    #[inline(always)]
    pub const fn rncr(&self) -> &RNCR {
        &self.rncr
    }
    #[doc = "0x118 - Transmit Next Pointer Register"]
    #[inline(always)]
    pub const fn tnpr(&self) -> &TNPR {
        &self.tnpr
    }
    #[doc = "0x11c - Transmit Next Counter Register"]
    #[inline(always)]
    pub const fn tncr(&self) -> &TNCR {
        &self.tncr
    }
    #[doc = "0x120 - Transfer Control Register"]
    #[inline(always)]
    pub const fn ptcr(&self) -> &PTCR {
        &self.ptcr
    }
    #[doc = "0x124 - Transfer Status Register"]
    #[inline(always)]
    pub const fn ptsr(&self) -> &PTSR {
        &self.ptsr
    }
}
#[doc = "CR (w) register accessor: Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "SPI_MODE_CR_SPI_MODE (w) register accessor: Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mode_cr_spi_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mode_cr_spi_mode`]
module"]
pub type SPI_MODE_CR_SPI_MODE = crate::Reg<spi_mode_cr_spi_mode::SPI_MODE_CR_SPI_MODE_SPEC>;
#[doc = "Control Register"]
pub mod spi_mode_cr_spi_mode;
#[doc = "MR (rw) register accessor: Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`]
module"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "SPI_MODE_MR_SPI_MODE (rw) register accessor: Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mode_mr_spi_mode::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mode_mr_spi_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mode_mr_spi_mode`]
module"]
pub type SPI_MODE_MR_SPI_MODE = crate::Reg<spi_mode_mr_spi_mode::SPI_MODE_MR_SPI_MODE_SPEC>;
#[doc = "Mode Register"]
pub mod spi_mode_mr_spi_mode;
#[doc = "IER (w) register accessor: Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "SPI_MODE_IER_SPI_MODE (w) register accessor: Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mode_ier_spi_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mode_ier_spi_mode`]
module"]
pub type SPI_MODE_IER_SPI_MODE = crate::Reg<spi_mode_ier_spi_mode::SPI_MODE_IER_SPI_MODE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod spi_mode_ier_spi_mode;
#[doc = "LIN_MODE_IER_LIN_MODE (w) register accessor: Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lin_mode_ier_lin_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lin_mode_ier_lin_mode`]
module"]
pub type LIN_MODE_IER_LIN_MODE = crate::Reg<lin_mode_ier_lin_mode::LIN_MODE_IER_LIN_MODE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod lin_mode_ier_lin_mode;
#[doc = "IDR (w) register accessor: Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "SPI_MODE_IDR_SPI_MODE (w) register accessor: Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mode_idr_spi_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mode_idr_spi_mode`]
module"]
pub type SPI_MODE_IDR_SPI_MODE = crate::Reg<spi_mode_idr_spi_mode::SPI_MODE_IDR_SPI_MODE_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod spi_mode_idr_spi_mode;
#[doc = "LIN_MODE_IDR_LIN_MODE (w) register accessor: Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lin_mode_idr_lin_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lin_mode_idr_lin_mode`]
module"]
pub type LIN_MODE_IDR_LIN_MODE = crate::Reg<lin_mode_idr_lin_mode::LIN_MODE_IDR_LIN_MODE_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod lin_mode_idr_lin_mode;
#[doc = "IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "SPI_MODE_IMR_SPI_MODE (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mode_imr_spi_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mode_imr_spi_mode`]
module"]
pub type SPI_MODE_IMR_SPI_MODE = crate::Reg<spi_mode_imr_spi_mode::SPI_MODE_IMR_SPI_MODE_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod spi_mode_imr_spi_mode;
#[doc = "LIN_MODE_IMR_LIN_MODE (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lin_mode_imr_lin_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lin_mode_imr_lin_mode`]
module"]
pub type LIN_MODE_IMR_LIN_MODE = crate::Reg<lin_mode_imr_lin_mode::LIN_MODE_IMR_LIN_MODE_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod lin_mode_imr_lin_mode;
#[doc = "CSR (r) register accessor: Channel Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Channel Status Register"]
pub mod csr;
#[doc = "SPI_MODE_CSR_SPI_MODE (r) register accessor: Channel Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mode_csr_spi_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mode_csr_spi_mode`]
module"]
pub type SPI_MODE_CSR_SPI_MODE = crate::Reg<spi_mode_csr_spi_mode::SPI_MODE_CSR_SPI_MODE_SPEC>;
#[doc = "Channel Status Register"]
pub mod spi_mode_csr_spi_mode;
#[doc = "LIN_MODE_CSR_LIN_MODE (r) register accessor: Channel Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lin_mode_csr_lin_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lin_mode_csr_lin_mode`]
module"]
pub type LIN_MODE_CSR_LIN_MODE = crate::Reg<lin_mode_csr_lin_mode::LIN_MODE_CSR_LIN_MODE_SPEC>;
#[doc = "Channel Status Register"]
pub mod lin_mode_csr_lin_mode;
#[doc = "RHR (r) register accessor: Receive Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rhr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rhr`]
module"]
pub type RHR = crate::Reg<rhr::RHR_SPEC>;
#[doc = "Receive Holding Register"]
pub mod rhr;
#[doc = "THR (w) register accessor: Transmit Holding Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thr`]
module"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "BRGR (rw) register accessor: Baud Rate Generator Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brgr`]
module"]
pub type BRGR = crate::Reg<brgr::BRGR_SPEC>;
#[doc = "Baud Rate Generator Register"]
pub mod brgr;
#[doc = "RTOR (rw) register accessor: Receiver Time-out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtor`]
module"]
pub type RTOR = crate::Reg<rtor::RTOR_SPEC>;
#[doc = "Receiver Time-out Register"]
pub mod rtor;
#[doc = "TTGR (rw) register accessor: Transmitter Timeguard Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttgr`]
module"]
pub type TTGR = crate::Reg<ttgr::TTGR_SPEC>;
#[doc = "Transmitter Timeguard Register"]
pub mod ttgr;
#[doc = "FIDI (rw) register accessor: FI DI Ratio Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fidi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fidi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fidi`]
module"]
pub type FIDI = crate::Reg<fidi::FIDI_SPEC>;
#[doc = "FI DI Ratio Register"]
pub mod fidi;
#[doc = "NER (r) register accessor: Number of Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ner::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ner`]
module"]
pub type NER = crate::Reg<ner::NER_SPEC>;
#[doc = "Number of Errors Register"]
pub mod ner;
#[doc = "IF (rw) register accessor: IrDA Filter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`]
module"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "IrDA Filter Register"]
pub mod if_;
#[doc = "MAN (rw) register accessor: Manchester Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`man::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`man::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@man`]
module"]
pub type MAN = crate::Reg<man::MAN_SPEC>;
#[doc = "Manchester Configuration Register"]
pub mod man;
#[doc = "LINMR (rw) register accessor: LIN Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linmr`]
module"]
pub type LINMR = crate::Reg<linmr::LINMR_SPEC>;
#[doc = "LIN Mode Register"]
pub mod linmr;
#[doc = "LINIR (rw) register accessor: LIN Identifier Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linir`]
module"]
pub type LINIR = crate::Reg<linir::LINIR_SPEC>;
#[doc = "LIN Identifier Register"]
pub mod linir;
#[doc = "LINBRR (r) register accessor: LIN Baud Rate Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linbrr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linbrr`]
module"]
pub type LINBRR = crate::Reg<linbrr::LINBRR_SPEC>;
#[doc = "LIN Baud Rate Register"]
pub mod linbrr;
#[doc = "WPMR (rw) register accessor: Write Protection Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`]
module"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: Write Protection Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`]
module"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
#[doc = "RPR (rw) register accessor: Receive Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpr`]
module"]
pub type RPR = crate::Reg<rpr::RPR_SPEC>;
#[doc = "Receive Pointer Register"]
pub mod rpr;
#[doc = "RCR (rw) register accessor: Receive Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr`]
module"]
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
#[doc = "Receive Counter Register"]
pub mod rcr;
#[doc = "TPR (rw) register accessor: Transmit Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpr`]
module"]
pub type TPR = crate::Reg<tpr::TPR_SPEC>;
#[doc = "Transmit Pointer Register"]
pub mod tpr;
#[doc = "TCR (rw) register accessor: Transmit Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr`]
module"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Transmit Counter Register"]
pub mod tcr;
#[doc = "RNPR (rw) register accessor: Receive Next Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rnpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnpr`]
module"]
pub type RNPR = crate::Reg<rnpr::RNPR_SPEC>;
#[doc = "Receive Next Pointer Register"]
pub mod rnpr;
#[doc = "RNCR (rw) register accessor: Receive Next Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rncr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rncr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rncr`]
module"]
pub type RNCR = crate::Reg<rncr::RNCR_SPEC>;
#[doc = "Receive Next Counter Register"]
pub mod rncr;
#[doc = "TNPR (rw) register accessor: Transmit Next Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tnpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tnpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tnpr`]
module"]
pub type TNPR = crate::Reg<tnpr::TNPR_SPEC>;
#[doc = "Transmit Next Pointer Register"]
pub mod tnpr;
#[doc = "TNCR (rw) register accessor: Transmit Next Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tncr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tncr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tncr`]
module"]
pub type TNCR = crate::Reg<tncr::TNCR_SPEC>;
#[doc = "Transmit Next Counter Register"]
pub mod tncr;
#[doc = "PTCR (w) register accessor: Transfer Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptcr`]
module"]
pub type PTCR = crate::Reg<ptcr::PTCR_SPEC>;
#[doc = "Transfer Control Register"]
pub mod ptcr;
#[doc = "PTSR (r) register accessor: Transfer Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptsr`]
module"]
pub type PTSR = crate::Reg<ptsr::PTSR_SPEC>;
#[doc = "Transfer Status Register"]
pub mod ptsr;
