#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mr: Mr,
    tr: Tr,
    cr: Cr,
    _reserved3: [u8; 0x04],
    lpr: Lpr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    isr: Isr,
    mdr: Mdr,
    cr1: Cr1,
    ocms: Ocms,
}
impl RegisterBlock {
    #[doc = "0x00 - SDRAMC Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &Mr {
        &self.mr
    }
    #[doc = "0x04 - SDRAMC Refresh Timer Register"]
    #[inline(always)]
    pub const fn tr(&self) -> &Tr {
        &self.tr
    }
    #[doc = "0x08 - SDRAMC Configuration Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x10 - SDRAMC Low Power Register"]
    #[inline(always)]
    pub const fn lpr(&self) -> &Lpr {
        &self.lpr
    }
    #[doc = "0x14 - SDRAMC Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x18 - SDRAMC Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x1c - SDRAMC Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x20 - SDRAMC Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x24 - SDRAMC Memory Device Register"]
    #[inline(always)]
    pub const fn mdr(&self) -> &Mdr {
        &self.mdr
    }
    #[doc = "0x28 - SDRAMC Configuration Register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x2c - SDRAMC OCMS Register 1"]
    #[inline(always)]
    pub const fn ocms(&self) -> &Ocms {
        &self.ocms
    }
}
#[doc = "MR (rw) register accessor: SDRAMC Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`]
module"]
#[doc(alias = "MR")]
pub type Mr = crate::Reg<mr::MrSpec>;
#[doc = "SDRAMC Mode Register"]
pub mod mr;
#[doc = "TR (rw) register accessor: SDRAMC Refresh Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr`]
module"]
#[doc(alias = "TR")]
pub type Tr = crate::Reg<tr::TrSpec>;
#[doc = "SDRAMC Refresh Timer Register"]
pub mod tr;
#[doc = "CR (rw) register accessor: SDRAMC Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "SDRAMC Configuration Register"]
pub mod cr;
#[doc = "LPR (rw) register accessor: SDRAMC Low Power Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpr`]
module"]
#[doc(alias = "LPR")]
pub type Lpr = crate::Reg<lpr::LprSpec>;
#[doc = "SDRAMC Low Power Register"]
pub mod lpr;
#[doc = "IER (w) register accessor: SDRAMC Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "SDRAMC Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: SDRAMC Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "SDRAMC Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: SDRAMC Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "SDRAMC Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: SDRAMC Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "SDRAMC Interrupt Status Register"]
pub mod isr;
#[doc = "MDR (rw) register accessor: SDRAMC Memory Device Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdr`]
module"]
#[doc(alias = "MDR")]
pub type Mdr = crate::Reg<mdr::MdrSpec>;
#[doc = "SDRAMC Memory Device Register"]
pub mod mdr;
#[doc = "CR1 (rw) register accessor: SDRAMC Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "SDRAMC Configuration Register 1"]
pub mod cr1;
#[doc = "OCMS (rw) register accessor: SDRAMC OCMS Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocms::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocms::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocms`]
module"]
#[doc(alias = "OCMS")]
pub type Ocms = crate::Reg<ocms::OcmsSpec>;
#[doc = "SDRAMC OCMS Register 1"]
pub mod ocms;
