#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    mr: Mr,
    timr: Timr,
    calr: Calr,
    timalr: Timalr,
    calalr: Calalr,
    sr: Sr,
    sccr: Sccr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    ver: Ver,
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
    #[doc = "0x08 - Time Register"]
    #[inline(always)]
    pub const fn timr(&self) -> &Timr {
        &self.timr
    }
    #[doc = "0x0c - Calendar Register"]
    #[inline(always)]
    pub const fn calr(&self) -> &Calr {
        &self.calr
    }
    #[doc = "0x10 - Time Alarm Register"]
    #[inline(always)]
    pub const fn timalr(&self) -> &Timalr {
        &self.timalr
    }
    #[doc = "0x14 - Calendar Alarm Register"]
    #[inline(always)]
    pub const fn calalr(&self) -> &Calalr {
        &self.calalr
    }
    #[doc = "0x18 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x1c - Status Clear Command Register"]
    #[inline(always)]
    pub const fn sccr(&self) -> &Sccr {
        &self.sccr
    }
    #[doc = "0x20 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x24 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x28 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x2c - Valid Entry Register"]
    #[inline(always)]
    pub const fn ver(&self) -> &Ver {
        &self.ver
    }
}
#[doc = "CR (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
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
#[doc = "TIMR (rw) register accessor: Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timr`]
module"]
#[doc(alias = "TIMR")]
pub type Timr = crate::Reg<timr::TimrSpec>;
#[doc = "Time Register"]
pub mod timr;
#[doc = "CALR (rw) register accessor: Calendar Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calr`]
module"]
#[doc(alias = "CALR")]
pub type Calr = crate::Reg<calr::CalrSpec>;
#[doc = "Calendar Register"]
pub mod calr;
#[doc = "TIMALR (rw) register accessor: Time Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timalr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timalr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timalr`]
module"]
#[doc(alias = "TIMALR")]
pub type Timalr = crate::Reg<timalr::TimalrSpec>;
#[doc = "Time Alarm Register"]
pub mod timalr;
#[doc = "CALALR (rw) register accessor: Calendar Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calalr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calalr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calalr`]
module"]
#[doc(alias = "CALALR")]
pub type Calalr = crate::Reg<calalr::CalalrSpec>;
#[doc = "Calendar Alarm Register"]
pub mod calalr;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "SCCR (w) register accessor: Status Clear Command Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sccr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccr`]
module"]
#[doc(alias = "SCCR")]
pub type Sccr = crate::Reg<sccr::SccrSpec>;
#[doc = "Status Clear Command Register"]
pub mod sccr;
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
#[doc = "VER (r) register accessor: Valid Entry Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ver::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ver`]
module"]
#[doc(alias = "VER")]
pub type Ver = crate::Reg<ver::VerSpec>;
#[doc = "Valid Entry Register"]
pub mod ver;
