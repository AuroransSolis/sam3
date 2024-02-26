#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mr: Mr,
    ar: Ar,
    vr: Vr,
    sr: Sr,
}
impl RegisterBlock {
    #[doc = "0x00 - Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &Mr {
        &self.mr
    }
    #[doc = "0x04 - Alarm Register"]
    #[inline(always)]
    pub const fn ar(&self) -> &Ar {
        &self.ar
    }
    #[doc = "0x08 - Value Register"]
    #[inline(always)]
    pub const fn vr(&self) -> &Vr {
        &self.vr
    }
    #[doc = "0x0c - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
}
#[doc = "MR (rw) register accessor: Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`]
module"]
#[doc(alias = "MR")]
pub type Mr = crate::Reg<mr::MrSpec>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "AR (rw) register accessor: Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar`]
module"]
#[doc(alias = "AR")]
pub type Ar = crate::Reg<ar::ArSpec>;
#[doc = "Alarm Register"]
pub mod ar;
#[doc = "VR (r) register accessor: Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vr`]
module"]
#[doc(alias = "VR")]
pub type Vr = crate::Reg<vr::VrSpec>;
#[doc = "Value Register"]
pub mod vr;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register"]
pub mod sr;
