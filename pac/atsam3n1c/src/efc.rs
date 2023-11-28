#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    fmr: FMR,
    fcr: FCR,
    fsr: FSR,
    frr: FRR,
}
impl RegisterBlock {
    #[doc = "0x00 - EEFC Flash Mode Register"]
    #[inline(always)]
    pub const fn fmr(&self) -> &FMR {
        &self.fmr
    }
    #[doc = "0x04 - EEFC Flash Command Register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    #[doc = "0x08 - EEFC Flash Status Register"]
    #[inline(always)]
    pub const fn fsr(&self) -> &FSR {
        &self.fsr
    }
    #[doc = "0x0c - EEFC Flash Result Register"]
    #[inline(always)]
    pub const fn frr(&self) -> &FRR {
        &self.frr
    }
}
#[doc = "FMR (rw) register accessor: EEFC Flash Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmr`]
module"]
pub type FMR = crate::Reg<fmr::FMR_SPEC>;
#[doc = "EEFC Flash Mode Register"]
pub mod fmr;
#[doc = "FCR (w) register accessor: EEFC Flash Command Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "EEFC Flash Command Register"]
pub mod fcr;
#[doc = "FSR (r) register accessor: EEFC Flash Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsr`]
module"]
pub type FSR = crate::Reg<fsr::FSR_SPEC>;
#[doc = "EEFC Flash Status Register"]
pub mod fsr;
#[doc = "FRR (r) register accessor: EEFC Flash Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frr`]
module"]
pub type FRR = crate::Reg<frr::FRR_SPEC>;
#[doc = "EEFC Flash Result Register"]
pub mod frr;
