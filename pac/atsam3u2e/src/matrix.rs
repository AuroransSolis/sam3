#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mcfg0: Mcfg0,
    mcfg1: Mcfg1,
    mcfg2: Mcfg2,
    mcfg3: Mcfg3,
    mcfg4: Mcfg4,
    _reserved5: [u8; 0x2c],
    scfg0: Scfg0,
    scfg1: Scfg1,
    scfg2: Scfg2,
    scfg3: Scfg3,
    scfg4: Scfg4,
    scfg5: Scfg5,
    scfg6: Scfg6,
    scfg7: Scfg7,
    scfg8: Scfg8,
    scfg9: Scfg9,
    _reserved15: [u8; 0x18],
    pras0: Pras0,
    _reserved16: [u8; 0x04],
    pras1: Pras1,
    _reserved17: [u8; 0x04],
    pras2: Pras2,
    _reserved18: [u8; 0x04],
    pras3: Pras3,
    _reserved19: [u8; 0x04],
    pras4: Pras4,
    _reserved20: [u8; 0x04],
    pras5: Pras5,
    _reserved21: [u8; 0x04],
    pras6: Pras6,
    _reserved22: [u8; 0x04],
    pras7: Pras7,
    _reserved23: [u8; 0x04],
    pras8: Pras8,
    _reserved24: [u8; 0x04],
    pras9: Pras9,
    _reserved25: [u8; 0x34],
    mrcr: Mrcr,
    _reserved26: [u8; 0xe0],
    wpmr: Wpmr,
    wpsr: Wpsr,
}
impl RegisterBlock {
    #[doc = "0x00 - Master Configuration Register 0"]
    #[inline(always)]
    pub const fn mcfg0(&self) -> &Mcfg0 {
        &self.mcfg0
    }
    #[doc = "0x04 - Master Configuration Register 1"]
    #[inline(always)]
    pub const fn mcfg1(&self) -> &Mcfg1 {
        &self.mcfg1
    }
    #[doc = "0x08 - Master Configuration Register 2"]
    #[inline(always)]
    pub const fn mcfg2(&self) -> &Mcfg2 {
        &self.mcfg2
    }
    #[doc = "0x0c - Master Configuration Register 3"]
    #[inline(always)]
    pub const fn mcfg3(&self) -> &Mcfg3 {
        &self.mcfg3
    }
    #[doc = "0x10 - Master Configuration Register 4"]
    #[inline(always)]
    pub const fn mcfg4(&self) -> &Mcfg4 {
        &self.mcfg4
    }
    #[doc = "0x40 - Slave Configuration Register 0"]
    #[inline(always)]
    pub const fn scfg0(&self) -> &Scfg0 {
        &self.scfg0
    }
    #[doc = "0x44 - Slave Configuration Register 1"]
    #[inline(always)]
    pub const fn scfg1(&self) -> &Scfg1 {
        &self.scfg1
    }
    #[doc = "0x48 - Slave Configuration Register 2"]
    #[inline(always)]
    pub const fn scfg2(&self) -> &Scfg2 {
        &self.scfg2
    }
    #[doc = "0x4c - Slave Configuration Register 3"]
    #[inline(always)]
    pub const fn scfg3(&self) -> &Scfg3 {
        &self.scfg3
    }
    #[doc = "0x50 - Slave Configuration Register 4"]
    #[inline(always)]
    pub const fn scfg4(&self) -> &Scfg4 {
        &self.scfg4
    }
    #[doc = "0x54 - Slave Configuration Register 5"]
    #[inline(always)]
    pub const fn scfg5(&self) -> &Scfg5 {
        &self.scfg5
    }
    #[doc = "0x58 - Slave Configuration Register 6"]
    #[inline(always)]
    pub const fn scfg6(&self) -> &Scfg6 {
        &self.scfg6
    }
    #[doc = "0x5c - Slave Configuration Register 7"]
    #[inline(always)]
    pub const fn scfg7(&self) -> &Scfg7 {
        &self.scfg7
    }
    #[doc = "0x60 - Slave Configuration Register 8"]
    #[inline(always)]
    pub const fn scfg8(&self) -> &Scfg8 {
        &self.scfg8
    }
    #[doc = "0x64 - Slave Configuration Register 9"]
    #[inline(always)]
    pub const fn scfg9(&self) -> &Scfg9 {
        &self.scfg9
    }
    #[doc = "0x80 - Priority Register A for Slave 0"]
    #[inline(always)]
    pub const fn pras0(&self) -> &Pras0 {
        &self.pras0
    }
    #[doc = "0x88 - Priority Register A for Slave 1"]
    #[inline(always)]
    pub const fn pras1(&self) -> &Pras1 {
        &self.pras1
    }
    #[doc = "0x90 - Priority Register A for Slave 2"]
    #[inline(always)]
    pub const fn pras2(&self) -> &Pras2 {
        &self.pras2
    }
    #[doc = "0x98 - Priority Register A for Slave 3"]
    #[inline(always)]
    pub const fn pras3(&self) -> &Pras3 {
        &self.pras3
    }
    #[doc = "0xa0 - Priority Register A for Slave 4"]
    #[inline(always)]
    pub const fn pras4(&self) -> &Pras4 {
        &self.pras4
    }
    #[doc = "0xa8 - Priority Register A for Slave 5"]
    #[inline(always)]
    pub const fn pras5(&self) -> &Pras5 {
        &self.pras5
    }
    #[doc = "0xb0 - Priority Register A for Slave 6"]
    #[inline(always)]
    pub const fn pras6(&self) -> &Pras6 {
        &self.pras6
    }
    #[doc = "0xb8 - Priority Register A for Slave 7"]
    #[inline(always)]
    pub const fn pras7(&self) -> &Pras7 {
        &self.pras7
    }
    #[doc = "0xc0 - Priority Register A for Slave 8"]
    #[inline(always)]
    pub const fn pras8(&self) -> &Pras8 {
        &self.pras8
    }
    #[doc = "0xc8 - Priority Register A for Slave 9"]
    #[inline(always)]
    pub const fn pras9(&self) -> &Pras9 {
        &self.pras9
    }
    #[doc = "0x100 - Master Remap Control Register"]
    #[inline(always)]
    pub const fn mrcr(&self) -> &Mrcr {
        &self.mrcr
    }
    #[doc = "0x1e4 - Write Protect Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &Wpmr {
        &self.wpmr
    }
    #[doc = "0x1e8 - Write Protect Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &Wpsr {
        &self.wpsr
    }
}
#[doc = "MCFG0 (rw) register accessor: Master Configuration Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mcfg0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcfg0`]
module"]
#[doc(alias = "MCFG0")]
pub type Mcfg0 = crate::Reg<mcfg0::Mcfg0Spec>;
#[doc = "Master Configuration Register 0"]
pub mod mcfg0;
#[doc = "MCFG1 (rw) register accessor: Master Configuration Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mcfg1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcfg1`]
module"]
#[doc(alias = "MCFG1")]
pub type Mcfg1 = crate::Reg<mcfg1::Mcfg1Spec>;
#[doc = "Master Configuration Register 1"]
pub mod mcfg1;
#[doc = "MCFG2 (rw) register accessor: Master Configuration Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`mcfg2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcfg2`]
module"]
#[doc(alias = "MCFG2")]
pub type Mcfg2 = crate::Reg<mcfg2::Mcfg2Spec>;
#[doc = "Master Configuration Register 2"]
pub mod mcfg2;
#[doc = "MCFG3 (rw) register accessor: Master Configuration Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`mcfg3::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcfg3`]
module"]
#[doc(alias = "MCFG3")]
pub type Mcfg3 = crate::Reg<mcfg3::Mcfg3Spec>;
#[doc = "Master Configuration Register 3"]
pub mod mcfg3;
#[doc = "MCFG4 (rw) register accessor: Master Configuration Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`mcfg4::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcfg4`]
module"]
#[doc(alias = "MCFG4")]
pub type Mcfg4 = crate::Reg<mcfg4::Mcfg4Spec>;
#[doc = "Master Configuration Register 4"]
pub mod mcfg4;
#[doc = "SCFG0 (rw) register accessor: Slave Configuration Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg0`]
module"]
#[doc(alias = "SCFG0")]
pub type Scfg0 = crate::Reg<scfg0::Scfg0Spec>;
#[doc = "Slave Configuration Register 0"]
pub mod scfg0;
#[doc = "SCFG1 (rw) register accessor: Slave Configuration Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg1`]
module"]
#[doc(alias = "SCFG1")]
pub type Scfg1 = crate::Reg<scfg1::Scfg1Spec>;
#[doc = "Slave Configuration Register 1"]
pub mod scfg1;
#[doc = "SCFG2 (rw) register accessor: Slave Configuration Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg2`]
module"]
#[doc(alias = "SCFG2")]
pub type Scfg2 = crate::Reg<scfg2::Scfg2Spec>;
#[doc = "Slave Configuration Register 2"]
pub mod scfg2;
#[doc = "SCFG3 (rw) register accessor: Slave Configuration Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg3::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg3`]
module"]
#[doc(alias = "SCFG3")]
pub type Scfg3 = crate::Reg<scfg3::Scfg3Spec>;
#[doc = "Slave Configuration Register 3"]
pub mod scfg3;
#[doc = "SCFG4 (rw) register accessor: Slave Configuration Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg4::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg4`]
module"]
#[doc(alias = "SCFG4")]
pub type Scfg4 = crate::Reg<scfg4::Scfg4Spec>;
#[doc = "Slave Configuration Register 4"]
pub mod scfg4;
#[doc = "SCFG5 (rw) register accessor: Slave Configuration Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg5::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg5`]
module"]
#[doc(alias = "SCFG5")]
pub type Scfg5 = crate::Reg<scfg5::Scfg5Spec>;
#[doc = "Slave Configuration Register 5"]
pub mod scfg5;
#[doc = "SCFG6 (rw) register accessor: Slave Configuration Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg6::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg6`]
module"]
#[doc(alias = "SCFG6")]
pub type Scfg6 = crate::Reg<scfg6::Scfg6Spec>;
#[doc = "Slave Configuration Register 6"]
pub mod scfg6;
#[doc = "SCFG7 (rw) register accessor: Slave Configuration Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg7::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg7`]
module"]
#[doc(alias = "SCFG7")]
pub type Scfg7 = crate::Reg<scfg7::Scfg7Spec>;
#[doc = "Slave Configuration Register 7"]
pub mod scfg7;
#[doc = "SCFG8 (rw) register accessor: Slave Configuration Register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg8::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg8`]
module"]
#[doc(alias = "SCFG8")]
pub type Scfg8 = crate::Reg<scfg8::Scfg8Spec>;
#[doc = "Slave Configuration Register 8"]
pub mod scfg8;
#[doc = "SCFG9 (rw) register accessor: Slave Configuration Register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg9::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg9`]
module"]
#[doc(alias = "SCFG9")]
pub type Scfg9 = crate::Reg<scfg9::Scfg9Spec>;
#[doc = "Slave Configuration Register 9"]
pub mod scfg9;
#[doc = "PRAS0 (rw) register accessor: Priority Register A for Slave 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pras0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pras0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras0`]
module"]
#[doc(alias = "PRAS0")]
pub type Pras0 = crate::Reg<pras0::Pras0Spec>;
#[doc = "Priority Register A for Slave 0"]
pub mod pras0;
#[doc = "PRAS1 (rw) register accessor: Priority Register A for Slave 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pras1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pras1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras1`]
module"]
#[doc(alias = "PRAS1")]
pub type Pras1 = crate::Reg<pras1::Pras1Spec>;
#[doc = "Priority Register A for Slave 1"]
pub mod pras1;
#[doc = "PRAS2 (rw) register accessor: Priority Register A for Slave 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pras2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pras2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras2`]
module"]
#[doc(alias = "PRAS2")]
pub type Pras2 = crate::Reg<pras2::Pras2Spec>;
#[doc = "Priority Register A for Slave 2"]
pub mod pras2;
#[doc = "PRAS3 (rw) register accessor: Priority Register A for Slave 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pras3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pras3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras3`]
module"]
#[doc(alias = "PRAS3")]
pub type Pras3 = crate::Reg<pras3::Pras3Spec>;
#[doc = "Priority Register A for Slave 3"]
pub mod pras3;
#[doc = "PRAS4 (rw) register accessor: Priority Register A for Slave 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pras4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pras4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras4`]
module"]
#[doc(alias = "PRAS4")]
pub type Pras4 = crate::Reg<pras4::Pras4Spec>;
#[doc = "Priority Register A for Slave 4"]
pub mod pras4;
#[doc = "PRAS5 (rw) register accessor: Priority Register A for Slave 5\n\nYou can [`read`](crate::Reg::read) this register and get [`pras5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pras5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras5`]
module"]
#[doc(alias = "PRAS5")]
pub type Pras5 = crate::Reg<pras5::Pras5Spec>;
#[doc = "Priority Register A for Slave 5"]
pub mod pras5;
#[doc = "PRAS6 (rw) register accessor: Priority Register A for Slave 6\n\nYou can [`read`](crate::Reg::read) this register and get [`pras6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pras6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras6`]
module"]
#[doc(alias = "PRAS6")]
pub type Pras6 = crate::Reg<pras6::Pras6Spec>;
#[doc = "Priority Register A for Slave 6"]
pub mod pras6;
#[doc = "PRAS7 (rw) register accessor: Priority Register A for Slave 7\n\nYou can [`read`](crate::Reg::read) this register and get [`pras7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pras7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras7`]
module"]
#[doc(alias = "PRAS7")]
pub type Pras7 = crate::Reg<pras7::Pras7Spec>;
#[doc = "Priority Register A for Slave 7"]
pub mod pras7;
#[doc = "PRAS8 (rw) register accessor: Priority Register A for Slave 8\n\nYou can [`read`](crate::Reg::read) this register and get [`pras8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pras8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras8`]
module"]
#[doc(alias = "PRAS8")]
pub type Pras8 = crate::Reg<pras8::Pras8Spec>;
#[doc = "Priority Register A for Slave 8"]
pub mod pras8;
#[doc = "PRAS9 (rw) register accessor: Priority Register A for Slave 9\n\nYou can [`read`](crate::Reg::read) this register and get [`pras9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pras9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras9`]
module"]
#[doc(alias = "PRAS9")]
pub type Pras9 = crate::Reg<pras9::Pras9Spec>;
#[doc = "Priority Register A for Slave 9"]
pub mod pras9;
#[doc = "MRCR (rw) register accessor: Master Remap Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrcr`]
module"]
#[doc(alias = "MRCR")]
pub type Mrcr = crate::Reg<mrcr::MrcrSpec>;
#[doc = "Master Remap Control Register"]
pub mod mrcr;
#[doc = "WPMR (rw) register accessor: Write Protect Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`]
module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: Write Protect Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`]
module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "Write Protect Status Register"]
pub mod wpsr;
