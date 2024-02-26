#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    mr: Mr,
    cdr: Cdr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    isr: Isr,
    _reserved7: [u8; 0xc8],
    wpmr: Wpmr,
    wpsr: Wpsr,
    _reserved9: [u8; 0x1c],
    tpr: Tpr,
    tcr: Tcr,
    _reserved11: [u8; 0x08],
    tnpr: Tnpr,
    tncr: Tncr,
    ptcr: Ptcr,
    ptsr: Ptsr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &Mr {
        &self.mr
    }
    #[doc = "0x08 - Conversion Data Register"]
    #[inline(always)]
    pub const fn cdr(&self) -> &Cdr {
        &self.cdr
    }
    #[doc = "0x0c - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x10 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x14 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x18 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
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
#[doc = "MR (rw) register accessor: Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`]
module"]
#[doc(alias = "MR")]
pub type Mr = crate::Reg<mr::MrSpec>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "CDR (w) register accessor: Conversion Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr`]
module"]
#[doc(alias = "CDR")]
pub type Cdr = crate::Reg<cdr::CdrSpec>;
#[doc = "Conversion Data Register"]
pub mod cdr;
#[doc = "IER (w) register accessor: Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt Status Register"]
pub mod isr;
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
