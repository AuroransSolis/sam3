#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpbr0: Gpbr0,
    gpbr1: Gpbr1,
    gpbr2: Gpbr2,
    gpbr3: Gpbr3,
    gpbr4: Gpbr4,
    gpbr5: Gpbr5,
    gpbr6: Gpbr6,
    gpbr7: Gpbr7,
}
impl RegisterBlock {
    #[doc = "0x00 - General Purpose Backup Register 0"]
    #[inline(always)]
    pub const fn gpbr0(&self) -> &Gpbr0 {
        &self.gpbr0
    }
    #[doc = "0x04 - General Purpose Backup Register 1"]
    #[inline(always)]
    pub const fn gpbr1(&self) -> &Gpbr1 {
        &self.gpbr1
    }
    #[doc = "0x08 - General Purpose Backup Register 2"]
    #[inline(always)]
    pub const fn gpbr2(&self) -> &Gpbr2 {
        &self.gpbr2
    }
    #[doc = "0x0c - General Purpose Backup Register 3"]
    #[inline(always)]
    pub const fn gpbr3(&self) -> &Gpbr3 {
        &self.gpbr3
    }
    #[doc = "0x10 - General Purpose Backup Register 4"]
    #[inline(always)]
    pub const fn gpbr4(&self) -> &Gpbr4 {
        &self.gpbr4
    }
    #[doc = "0x14 - General Purpose Backup Register 5"]
    #[inline(always)]
    pub const fn gpbr5(&self) -> &Gpbr5 {
        &self.gpbr5
    }
    #[doc = "0x18 - General Purpose Backup Register 6"]
    #[inline(always)]
    pub const fn gpbr6(&self) -> &Gpbr6 {
        &self.gpbr6
    }
    #[doc = "0x1c - General Purpose Backup Register 7"]
    #[inline(always)]
    pub const fn gpbr7(&self) -> &Gpbr7 {
        &self.gpbr7
    }
}
#[doc = "GPBR0 (rw) register accessor: General Purpose Backup Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpbr0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpbr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpbr0`]
module"]
#[doc(alias = "GPBR0")]
pub type Gpbr0 = crate::Reg<gpbr0::Gpbr0Spec>;
#[doc = "General Purpose Backup Register 0"]
pub mod gpbr0;
#[doc = "GPBR1 (rw) register accessor: General Purpose Backup Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpbr1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpbr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpbr1`]
module"]
#[doc(alias = "GPBR1")]
pub type Gpbr1 = crate::Reg<gpbr1::Gpbr1Spec>;
#[doc = "General Purpose Backup Register 1"]
pub mod gpbr1;
#[doc = "GPBR2 (rw) register accessor: General Purpose Backup Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpbr2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpbr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpbr2`]
module"]
#[doc(alias = "GPBR2")]
pub type Gpbr2 = crate::Reg<gpbr2::Gpbr2Spec>;
#[doc = "General Purpose Backup Register 2"]
pub mod gpbr2;
#[doc = "GPBR3 (rw) register accessor: General Purpose Backup Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpbr3::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpbr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpbr3`]
module"]
#[doc(alias = "GPBR3")]
pub type Gpbr3 = crate::Reg<gpbr3::Gpbr3Spec>;
#[doc = "General Purpose Backup Register 3"]
pub mod gpbr3;
#[doc = "GPBR4 (rw) register accessor: General Purpose Backup Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`gpbr4::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpbr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpbr4`]
module"]
#[doc(alias = "GPBR4")]
pub type Gpbr4 = crate::Reg<gpbr4::Gpbr4Spec>;
#[doc = "General Purpose Backup Register 4"]
pub mod gpbr4;
#[doc = "GPBR5 (rw) register accessor: General Purpose Backup Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`gpbr5::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpbr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpbr5`]
module"]
#[doc(alias = "GPBR5")]
pub type Gpbr5 = crate::Reg<gpbr5::Gpbr5Spec>;
#[doc = "General Purpose Backup Register 5"]
pub mod gpbr5;
#[doc = "GPBR6 (rw) register accessor: General Purpose Backup Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`gpbr6::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpbr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpbr6`]
module"]
#[doc(alias = "GPBR6")]
pub type Gpbr6 = crate::Reg<gpbr6::Gpbr6Spec>;
#[doc = "General Purpose Backup Register 6"]
pub mod gpbr6;
#[doc = "GPBR7 (rw) register accessor: General Purpose Backup Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`gpbr7::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpbr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpbr7`]
module"]
#[doc(alias = "GPBR7")]
pub type Gpbr7 = crate::Reg<gpbr7::Gpbr7Spec>;
#[doc = "General Purpose Backup Register 7"]
pub mod gpbr7;
