#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cidr: CIDR,
    exid: EXID,
}
impl RegisterBlock {
    #[doc = "0x00 - Chip ID Register"]
    #[inline(always)]
    pub const fn cidr(&self) -> &CIDR {
        &self.cidr
    }
    #[doc = "0x04 - Chip ID Extension Register"]
    #[inline(always)]
    pub const fn exid(&self) -> &EXID {
        &self.exid
    }
}
#[doc = "CIDR (r) register accessor: Chip ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr`]
module"]
pub type CIDR = crate::Reg<cidr::CIDR_SPEC>;
#[doc = "Chip ID Register"]
pub mod cidr;
#[doc = "EXID (r) register accessor: Chip ID Extension Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exid`]
module"]
pub type EXID = crate::Reg<exid::EXID_SPEC>;
#[doc = "Chip ID Extension Register"]
pub mod exid;
