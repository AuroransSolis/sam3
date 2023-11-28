#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    gpbr0: GPBR0,
    gpbr1: GPBR1,
    gpbr2: GPBR2,
    gpbr3: GPBR3,
    gpbr4: GPBR4,
    gpbr5: GPBR5,
    gpbr6: GPBR6,
    gpbr7: GPBR7,
}
impl RegisterBlock {
    #[doc = "0x00 - General Purpose Backup Register 0"]
    #[inline(always)]
    pub const fn gpbr0(&self) -> &GPBR0 {
        &self.gpbr0
    }
    #[doc = "0x04 - General Purpose Backup Register 1"]
    #[inline(always)]
    pub const fn gpbr1(&self) -> &GPBR1 {
        &self.gpbr1
    }
    #[doc = "0x08 - General Purpose Backup Register 2"]
    #[inline(always)]
    pub const fn gpbr2(&self) -> &GPBR2 {
        &self.gpbr2
    }
    #[doc = "0x0c - General Purpose Backup Register 3"]
    #[inline(always)]
    pub const fn gpbr3(&self) -> &GPBR3 {
        &self.gpbr3
    }
    #[doc = "0x10 - General Purpose Backup Register 4"]
    #[inline(always)]
    pub const fn gpbr4(&self) -> &GPBR4 {
        &self.gpbr4
    }
    #[doc = "0x14 - General Purpose Backup Register 5"]
    #[inline(always)]
    pub const fn gpbr5(&self) -> &GPBR5 {
        &self.gpbr5
    }
    #[doc = "0x18 - General Purpose Backup Register 6"]
    #[inline(always)]
    pub const fn gpbr6(&self) -> &GPBR6 {
        &self.gpbr6
    }
    #[doc = "0x1c - General Purpose Backup Register 7"]
    #[inline(always)]
    pub const fn gpbr7(&self) -> &GPBR7 {
        &self.gpbr7
    }
}
#[doc = "GPBR0 (rw) register accessor: General Purpose Backup Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpbr0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpbr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpbr0`]
module"]
pub type GPBR0 = crate::Reg<gpbr0::GPBR0_SPEC>;
#[doc = "General Purpose Backup Register 0"]
pub mod gpbr0;
#[doc = "GPBR1 (rw) register accessor: General Purpose Backup Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpbr1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpbr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpbr1`]
module"]
pub type GPBR1 = crate::Reg<gpbr1::GPBR1_SPEC>;
#[doc = "General Purpose Backup Register 1"]
pub mod gpbr1;
#[doc = "GPBR2 (rw) register accessor: General Purpose Backup Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpbr2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpbr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpbr2`]
module"]
pub type GPBR2 = crate::Reg<gpbr2::GPBR2_SPEC>;
#[doc = "General Purpose Backup Register 2"]
pub mod gpbr2;
#[doc = "GPBR3 (rw) register accessor: General Purpose Backup Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpbr3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpbr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpbr3`]
module"]
pub type GPBR3 = crate::Reg<gpbr3::GPBR3_SPEC>;
#[doc = "General Purpose Backup Register 3"]
pub mod gpbr3;
#[doc = "GPBR4 (rw) register accessor: General Purpose Backup Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpbr4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpbr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpbr4`]
module"]
pub type GPBR4 = crate::Reg<gpbr4::GPBR4_SPEC>;
#[doc = "General Purpose Backup Register 4"]
pub mod gpbr4;
#[doc = "GPBR5 (rw) register accessor: General Purpose Backup Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpbr5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpbr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpbr5`]
module"]
pub type GPBR5 = crate::Reg<gpbr5::GPBR5_SPEC>;
#[doc = "General Purpose Backup Register 5"]
pub mod gpbr5;
#[doc = "GPBR6 (rw) register accessor: General Purpose Backup Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpbr6::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpbr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpbr6`]
module"]
pub type GPBR6 = crate::Reg<gpbr6::GPBR6_SPEC>;
#[doc = "General Purpose Backup Register 6"]
pub mod gpbr6;
#[doc = "GPBR7 (rw) register accessor: General Purpose Backup Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpbr7::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpbr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpbr7`]
module"]
pub type GPBR7 = crate::Reg<gpbr7::GPBR7_SPEC>;
#[doc = "General Purpose Backup Register 7"]
pub mod gpbr7;
