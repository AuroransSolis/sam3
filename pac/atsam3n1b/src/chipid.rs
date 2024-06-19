#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cidr: Cidr,
    exid: Exid,
}
impl RegisterBlock {
    #[doc = "0x00 - Chip ID Register"]
    #[inline(always)]
    pub const fn cidr(&self) -> &Cidr {
        &self.cidr
    }
    #[doc = "0x04 - Chip ID Extension Register"]
    #[inline(always)]
    pub const fn exid(&self) -> &Exid {
        &self.exid
    }
}
#[doc = "CIDR (r) register accessor: Chip ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr`]
module"]
#[doc(alias = "CIDR")]
pub type Cidr = crate::Reg<cidr::CidrSpec>;
#[doc = "Chip ID Register"]
pub mod cidr;
#[doc = "EXID (r) register accessor: Chip ID Extension Register\n\nYou can [`read`](crate::Reg::read) this register and get [`exid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exid`]
module"]
#[doc(alias = "EXID")]
pub type Exid = crate::Reg<exid::ExidSpec>;
#[doc = "Chip ID Extension Register"]
pub mod exid;
