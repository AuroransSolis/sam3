#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    mcfg0: MCFG0,
    mcfg1: MCFG1,
    mcfg2: MCFG2,
    mcfg3: MCFG3,
    mcfg4: MCFG4,
    _reserved5: [u8; 0x2c],
    scfg0: SCFG0,
    scfg1: SCFG1,
    scfg2: SCFG2,
    scfg3: SCFG3,
    scfg4: SCFG4,
    scfg5: SCFG5,
    scfg6: SCFG6,
    scfg7: SCFG7,
    scfg8: SCFG8,
    scfg9: SCFG9,
    _reserved15: [u8; 0x18],
    pras0: PRAS0,
    _reserved16: [u8; 0x04],
    pras1: PRAS1,
    _reserved17: [u8; 0x04],
    pras2: PRAS2,
    _reserved18: [u8; 0x04],
    pras3: PRAS3,
    _reserved19: [u8; 0x04],
    pras4: PRAS4,
    _reserved20: [u8; 0x04],
    pras5: PRAS5,
    _reserved21: [u8; 0x04],
    pras6: PRAS6,
    _reserved22: [u8; 0x04],
    pras7: PRAS7,
    _reserved23: [u8; 0x04],
    pras8: PRAS8,
    _reserved24: [u8; 0x04],
    pras9: PRAS9,
    _reserved25: [u8; 0x34],
    mrcr: MRCR,
    _reserved26: [u8; 0xe0],
    wpmr: WPMR,
    wpsr: WPSR,
}
impl RegisterBlock {
    #[doc = "0x00 - Master Configuration Register 0"]
    #[inline(always)]
    pub const fn mcfg0(&self) -> &MCFG0 {
        &self.mcfg0
    }
    #[doc = "0x04 - Master Configuration Register 1"]
    #[inline(always)]
    pub const fn mcfg1(&self) -> &MCFG1 {
        &self.mcfg1
    }
    #[doc = "0x08 - Master Configuration Register 2"]
    #[inline(always)]
    pub const fn mcfg2(&self) -> &MCFG2 {
        &self.mcfg2
    }
    #[doc = "0x0c - Master Configuration Register 3"]
    #[inline(always)]
    pub const fn mcfg3(&self) -> &MCFG3 {
        &self.mcfg3
    }
    #[doc = "0x10 - Master Configuration Register 4"]
    #[inline(always)]
    pub const fn mcfg4(&self) -> &MCFG4 {
        &self.mcfg4
    }
    #[doc = "0x40 - Slave Configuration Register 0"]
    #[inline(always)]
    pub const fn scfg0(&self) -> &SCFG0 {
        &self.scfg0
    }
    #[doc = "0x44 - Slave Configuration Register 1"]
    #[inline(always)]
    pub const fn scfg1(&self) -> &SCFG1 {
        &self.scfg1
    }
    #[doc = "0x48 - Slave Configuration Register 2"]
    #[inline(always)]
    pub const fn scfg2(&self) -> &SCFG2 {
        &self.scfg2
    }
    #[doc = "0x4c - Slave Configuration Register 3"]
    #[inline(always)]
    pub const fn scfg3(&self) -> &SCFG3 {
        &self.scfg3
    }
    #[doc = "0x50 - Slave Configuration Register 4"]
    #[inline(always)]
    pub const fn scfg4(&self) -> &SCFG4 {
        &self.scfg4
    }
    #[doc = "0x54 - Slave Configuration Register 5"]
    #[inline(always)]
    pub const fn scfg5(&self) -> &SCFG5 {
        &self.scfg5
    }
    #[doc = "0x58 - Slave Configuration Register 6"]
    #[inline(always)]
    pub const fn scfg6(&self) -> &SCFG6 {
        &self.scfg6
    }
    #[doc = "0x5c - Slave Configuration Register 7"]
    #[inline(always)]
    pub const fn scfg7(&self) -> &SCFG7 {
        &self.scfg7
    }
    #[doc = "0x60 - Slave Configuration Register 8"]
    #[inline(always)]
    pub const fn scfg8(&self) -> &SCFG8 {
        &self.scfg8
    }
    #[doc = "0x64 - Slave Configuration Register 9"]
    #[inline(always)]
    pub const fn scfg9(&self) -> &SCFG9 {
        &self.scfg9
    }
    #[doc = "0x80 - Priority Register A for Slave 0"]
    #[inline(always)]
    pub const fn pras0(&self) -> &PRAS0 {
        &self.pras0
    }
    #[doc = "0x88 - Priority Register A for Slave 1"]
    #[inline(always)]
    pub const fn pras1(&self) -> &PRAS1 {
        &self.pras1
    }
    #[doc = "0x90 - Priority Register A for Slave 2"]
    #[inline(always)]
    pub const fn pras2(&self) -> &PRAS2 {
        &self.pras2
    }
    #[doc = "0x98 - Priority Register A for Slave 3"]
    #[inline(always)]
    pub const fn pras3(&self) -> &PRAS3 {
        &self.pras3
    }
    #[doc = "0xa0 - Priority Register A for Slave 4"]
    #[inline(always)]
    pub const fn pras4(&self) -> &PRAS4 {
        &self.pras4
    }
    #[doc = "0xa8 - Priority Register A for Slave 5"]
    #[inline(always)]
    pub const fn pras5(&self) -> &PRAS5 {
        &self.pras5
    }
    #[doc = "0xb0 - Priority Register A for Slave 6"]
    #[inline(always)]
    pub const fn pras6(&self) -> &PRAS6 {
        &self.pras6
    }
    #[doc = "0xb8 - Priority Register A for Slave 7"]
    #[inline(always)]
    pub const fn pras7(&self) -> &PRAS7 {
        &self.pras7
    }
    #[doc = "0xc0 - Priority Register A for Slave 8"]
    #[inline(always)]
    pub const fn pras8(&self) -> &PRAS8 {
        &self.pras8
    }
    #[doc = "0xc8 - Priority Register A for Slave 9"]
    #[inline(always)]
    pub const fn pras9(&self) -> &PRAS9 {
        &self.pras9
    }
    #[doc = "0x100 - Master Remap Control Register"]
    #[inline(always)]
    pub const fn mrcr(&self) -> &MRCR {
        &self.mrcr
    }
    #[doc = "0x1e4 - Write Protect Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &WPMR {
        &self.wpmr
    }
    #[doc = "0x1e8 - Write Protect Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &WPSR {
        &self.wpsr
    }
}
#[doc = "MCFG0 (rw) register accessor: Master Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcfg0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcfg0`]
module"]
pub type MCFG0 = crate::Reg<mcfg0::MCFG0_SPEC>;
#[doc = "Master Configuration Register 0"]
pub mod mcfg0;
#[doc = "MCFG1 (rw) register accessor: Master Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcfg1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcfg1`]
module"]
pub type MCFG1 = crate::Reg<mcfg1::MCFG1_SPEC>;
#[doc = "Master Configuration Register 1"]
pub mod mcfg1;
#[doc = "MCFG2 (rw) register accessor: Master Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcfg2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcfg2`]
module"]
pub type MCFG2 = crate::Reg<mcfg2::MCFG2_SPEC>;
#[doc = "Master Configuration Register 2"]
pub mod mcfg2;
#[doc = "MCFG3 (rw) register accessor: Master Configuration Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcfg3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcfg3`]
module"]
pub type MCFG3 = crate::Reg<mcfg3::MCFG3_SPEC>;
#[doc = "Master Configuration Register 3"]
pub mod mcfg3;
#[doc = "MCFG4 (rw) register accessor: Master Configuration Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcfg4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcfg4`]
module"]
pub type MCFG4 = crate::Reg<mcfg4::MCFG4_SPEC>;
#[doc = "Master Configuration Register 4"]
pub mod mcfg4;
#[doc = "SCFG0 (rw) register accessor: Slave Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scfg0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg0`]
module"]
pub type SCFG0 = crate::Reg<scfg0::SCFG0_SPEC>;
#[doc = "Slave Configuration Register 0"]
pub mod scfg0;
#[doc = "SCFG1 (rw) register accessor: Slave Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scfg1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg1`]
module"]
pub type SCFG1 = crate::Reg<scfg1::SCFG1_SPEC>;
#[doc = "Slave Configuration Register 1"]
pub mod scfg1;
#[doc = "SCFG2 (rw) register accessor: Slave Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scfg2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg2`]
module"]
pub type SCFG2 = crate::Reg<scfg2::SCFG2_SPEC>;
#[doc = "Slave Configuration Register 2"]
pub mod scfg2;
#[doc = "SCFG3 (rw) register accessor: Slave Configuration Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scfg3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg3`]
module"]
pub type SCFG3 = crate::Reg<scfg3::SCFG3_SPEC>;
#[doc = "Slave Configuration Register 3"]
pub mod scfg3;
#[doc = "SCFG4 (rw) register accessor: Slave Configuration Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scfg4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg4`]
module"]
pub type SCFG4 = crate::Reg<scfg4::SCFG4_SPEC>;
#[doc = "Slave Configuration Register 4"]
pub mod scfg4;
#[doc = "SCFG5 (rw) register accessor: Slave Configuration Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scfg5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scfg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg5`]
module"]
pub type SCFG5 = crate::Reg<scfg5::SCFG5_SPEC>;
#[doc = "Slave Configuration Register 5"]
pub mod scfg5;
#[doc = "SCFG6 (rw) register accessor: Slave Configuration Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scfg6::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scfg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg6`]
module"]
pub type SCFG6 = crate::Reg<scfg6::SCFG6_SPEC>;
#[doc = "Slave Configuration Register 6"]
pub mod scfg6;
#[doc = "SCFG7 (rw) register accessor: Slave Configuration Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scfg7::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scfg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg7`]
module"]
pub type SCFG7 = crate::Reg<scfg7::SCFG7_SPEC>;
#[doc = "Slave Configuration Register 7"]
pub mod scfg7;
#[doc = "SCFG8 (rw) register accessor: Slave Configuration Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scfg8::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scfg8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg8`]
module"]
pub type SCFG8 = crate::Reg<scfg8::SCFG8_SPEC>;
#[doc = "Slave Configuration Register 8"]
pub mod scfg8;
#[doc = "SCFG9 (rw) register accessor: Slave Configuration Register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scfg9::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scfg9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg9`]
module"]
pub type SCFG9 = crate::Reg<scfg9::SCFG9_SPEC>;
#[doc = "Slave Configuration Register 9"]
pub mod scfg9;
#[doc = "PRAS0 (rw) register accessor: Priority Register A for Slave 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pras0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pras0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras0`]
module"]
pub type PRAS0 = crate::Reg<pras0::PRAS0_SPEC>;
#[doc = "Priority Register A for Slave 0"]
pub mod pras0;
#[doc = "PRAS1 (rw) register accessor: Priority Register A for Slave 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pras1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pras1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras1`]
module"]
pub type PRAS1 = crate::Reg<pras1::PRAS1_SPEC>;
#[doc = "Priority Register A for Slave 1"]
pub mod pras1;
#[doc = "PRAS2 (rw) register accessor: Priority Register A for Slave 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pras2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pras2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras2`]
module"]
pub type PRAS2 = crate::Reg<pras2::PRAS2_SPEC>;
#[doc = "Priority Register A for Slave 2"]
pub mod pras2;
#[doc = "PRAS3 (rw) register accessor: Priority Register A for Slave 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pras3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pras3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras3`]
module"]
pub type PRAS3 = crate::Reg<pras3::PRAS3_SPEC>;
#[doc = "Priority Register A for Slave 3"]
pub mod pras3;
#[doc = "PRAS4 (rw) register accessor: Priority Register A for Slave 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pras4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pras4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras4`]
module"]
pub type PRAS4 = crate::Reg<pras4::PRAS4_SPEC>;
#[doc = "Priority Register A for Slave 4"]
pub mod pras4;
#[doc = "PRAS5 (rw) register accessor: Priority Register A for Slave 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pras5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pras5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras5`]
module"]
pub type PRAS5 = crate::Reg<pras5::PRAS5_SPEC>;
#[doc = "Priority Register A for Slave 5"]
pub mod pras5;
#[doc = "PRAS6 (rw) register accessor: Priority Register A for Slave 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pras6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pras6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras6`]
module"]
pub type PRAS6 = crate::Reg<pras6::PRAS6_SPEC>;
#[doc = "Priority Register A for Slave 6"]
pub mod pras6;
#[doc = "PRAS7 (rw) register accessor: Priority Register A for Slave 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pras7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pras7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras7`]
module"]
pub type PRAS7 = crate::Reg<pras7::PRAS7_SPEC>;
#[doc = "Priority Register A for Slave 7"]
pub mod pras7;
#[doc = "PRAS8 (rw) register accessor: Priority Register A for Slave 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pras8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pras8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras8`]
module"]
pub type PRAS8 = crate::Reg<pras8::PRAS8_SPEC>;
#[doc = "Priority Register A for Slave 8"]
pub mod pras8;
#[doc = "PRAS9 (rw) register accessor: Priority Register A for Slave 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pras9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pras9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras9`]
module"]
pub type PRAS9 = crate::Reg<pras9::PRAS9_SPEC>;
#[doc = "Priority Register A for Slave 9"]
pub mod pras9;
#[doc = "MRCR (rw) register accessor: Master Remap Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrcr`]
module"]
pub type MRCR = crate::Reg<mrcr::MRCR_SPEC>;
#[doc = "Master Remap Control Register"]
pub mod mrcr;
#[doc = "WPMR (rw) register accessor: Write Protect Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`]
module"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: Write Protect Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`]
module"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protect Status Register"]
pub mod wpsr;
