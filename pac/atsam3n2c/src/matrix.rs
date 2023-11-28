#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    matrix_mcfg0: MATRIX_MCFG0,
    matrix_mcfg1: MATRIX_MCFG1,
    matrix_mcfg2: MATRIX_MCFG2,
    _reserved3: [u8; 0x34],
    matrix_scfg0: MATRIX_SCFG0,
    matrix_scfg1: MATRIX_SCFG1,
    matrix_scfg2: MATRIX_SCFG2,
    matrix_scfg3: MATRIX_SCFG3,
    _reserved7: [u8; 0x30],
    matrix_pras0: MATRIX_PRAS0,
    _reserved8: [u8; 0x04],
    matrix_pras1: MATRIX_PRAS1,
    _reserved9: [u8; 0x04],
    matrix_pras2: MATRIX_PRAS2,
    _reserved10: [u8; 0x04],
    matrix_pras3: MATRIX_PRAS3,
    _reserved11: [u8; 0x78],
    ccfg_sysio: CCFG_SYSIO,
    _reserved12: [u8; 0xcc],
    matrix_wpmr: MATRIX_WPMR,
    matrix_wpsr: MATRIX_WPSR,
}
impl RegisterBlock {
    #[doc = "0x00 - Master Configuration Register 0"]
    #[inline(always)]
    pub const fn matrix_mcfg0(&self) -> &MATRIX_MCFG0 {
        &self.matrix_mcfg0
    }
    #[doc = "0x04 - Master Configuration Register 1"]
    #[inline(always)]
    pub const fn matrix_mcfg1(&self) -> &MATRIX_MCFG1 {
        &self.matrix_mcfg1
    }
    #[doc = "0x08 - Master Configuration Register 2"]
    #[inline(always)]
    pub const fn matrix_mcfg2(&self) -> &MATRIX_MCFG2 {
        &self.matrix_mcfg2
    }
    #[doc = "0x40 - Slave Configuration Register 0"]
    #[inline(always)]
    pub const fn matrix_scfg0(&self) -> &MATRIX_SCFG0 {
        &self.matrix_scfg0
    }
    #[doc = "0x44 - Slave Configuration Register 1"]
    #[inline(always)]
    pub const fn matrix_scfg1(&self) -> &MATRIX_SCFG1 {
        &self.matrix_scfg1
    }
    #[doc = "0x48 - Slave Configuration Register 2"]
    #[inline(always)]
    pub const fn matrix_scfg2(&self) -> &MATRIX_SCFG2 {
        &self.matrix_scfg2
    }
    #[doc = "0x4c - Slave Configuration Register 3"]
    #[inline(always)]
    pub const fn matrix_scfg3(&self) -> &MATRIX_SCFG3 {
        &self.matrix_scfg3
    }
    #[doc = "0x80 - Priority Register A for Slave 0"]
    #[inline(always)]
    pub const fn matrix_pras0(&self) -> &MATRIX_PRAS0 {
        &self.matrix_pras0
    }
    #[doc = "0x88 - Priority Register A for Slave 1"]
    #[inline(always)]
    pub const fn matrix_pras1(&self) -> &MATRIX_PRAS1 {
        &self.matrix_pras1
    }
    #[doc = "0x90 - Priority Register A for Slave 2"]
    #[inline(always)]
    pub const fn matrix_pras2(&self) -> &MATRIX_PRAS2 {
        &self.matrix_pras2
    }
    #[doc = "0x98 - Priority Register A for Slave 3"]
    #[inline(always)]
    pub const fn matrix_pras3(&self) -> &MATRIX_PRAS3 {
        &self.matrix_pras3
    }
    #[doc = "0x114 - System I/O Configuration register"]
    #[inline(always)]
    pub const fn ccfg_sysio(&self) -> &CCFG_SYSIO {
        &self.ccfg_sysio
    }
    #[doc = "0x1e4 - Write Protect Mode Register"]
    #[inline(always)]
    pub const fn matrix_wpmr(&self) -> &MATRIX_WPMR {
        &self.matrix_wpmr
    }
    #[doc = "0x1e8 - Write Protect Status Register"]
    #[inline(always)]
    pub const fn matrix_wpsr(&self) -> &MATRIX_WPSR {
        &self.matrix_wpsr
    }
}
#[doc = "MATRIX_MCFG0 (rw) register accessor: Master Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_mcfg0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_mcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_mcfg0`]
module"]
pub type MATRIX_MCFG0 = crate::Reg<matrix_mcfg0::MATRIX_MCFG0_SPEC>;
#[doc = "Master Configuration Register 0"]
pub mod matrix_mcfg0;
#[doc = "MATRIX_MCFG1 (rw) register accessor: Master Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_mcfg1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_mcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_mcfg1`]
module"]
pub type MATRIX_MCFG1 = crate::Reg<matrix_mcfg1::MATRIX_MCFG1_SPEC>;
#[doc = "Master Configuration Register 1"]
pub mod matrix_mcfg1;
#[doc = "MATRIX_MCFG2 (rw) register accessor: Master Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_mcfg2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_mcfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_mcfg2`]
module"]
pub type MATRIX_MCFG2 = crate::Reg<matrix_mcfg2::MATRIX_MCFG2_SPEC>;
#[doc = "Master Configuration Register 2"]
pub mod matrix_mcfg2;
#[doc = "MATRIX_SCFG0 (rw) register accessor: Slave Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_scfg0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_scfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_scfg0`]
module"]
pub type MATRIX_SCFG0 = crate::Reg<matrix_scfg0::MATRIX_SCFG0_SPEC>;
#[doc = "Slave Configuration Register 0"]
pub mod matrix_scfg0;
#[doc = "MATRIX_SCFG1 (rw) register accessor: Slave Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_scfg1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_scfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_scfg1`]
module"]
pub type MATRIX_SCFG1 = crate::Reg<matrix_scfg1::MATRIX_SCFG1_SPEC>;
#[doc = "Slave Configuration Register 1"]
pub mod matrix_scfg1;
#[doc = "MATRIX_SCFG2 (rw) register accessor: Slave Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_scfg2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_scfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_scfg2`]
module"]
pub type MATRIX_SCFG2 = crate::Reg<matrix_scfg2::MATRIX_SCFG2_SPEC>;
#[doc = "Slave Configuration Register 2"]
pub mod matrix_scfg2;
#[doc = "MATRIX_SCFG3 (rw) register accessor: Slave Configuration Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_scfg3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_scfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_scfg3`]
module"]
pub type MATRIX_SCFG3 = crate::Reg<matrix_scfg3::MATRIX_SCFG3_SPEC>;
#[doc = "Slave Configuration Register 3"]
pub mod matrix_scfg3;
#[doc = "MATRIX_PRAS0 (rw) register accessor: Priority Register A for Slave 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_pras0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_pras0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras0`]
module"]
pub type MATRIX_PRAS0 = crate::Reg<matrix_pras0::MATRIX_PRAS0_SPEC>;
#[doc = "Priority Register A for Slave 0"]
pub mod matrix_pras0;
#[doc = "MATRIX_PRAS1 (rw) register accessor: Priority Register A for Slave 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_pras1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_pras1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras1`]
module"]
pub type MATRIX_PRAS1 = crate::Reg<matrix_pras1::MATRIX_PRAS1_SPEC>;
#[doc = "Priority Register A for Slave 1"]
pub mod matrix_pras1;
#[doc = "MATRIX_PRAS2 (rw) register accessor: Priority Register A for Slave 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_pras2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_pras2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras2`]
module"]
pub type MATRIX_PRAS2 = crate::Reg<matrix_pras2::MATRIX_PRAS2_SPEC>;
#[doc = "Priority Register A for Slave 2"]
pub mod matrix_pras2;
#[doc = "MATRIX_PRAS3 (rw) register accessor: Priority Register A for Slave 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_pras3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_pras3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras3`]
module"]
pub type MATRIX_PRAS3 = crate::Reg<matrix_pras3::MATRIX_PRAS3_SPEC>;
#[doc = "Priority Register A for Slave 3"]
pub mod matrix_pras3;
#[doc = "CCFG_SYSIO (rw) register accessor: System I/O Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_sysio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_sysio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_sysio`]
module"]
pub type CCFG_SYSIO = crate::Reg<ccfg_sysio::CCFG_SYSIO_SPEC>;
#[doc = "System I/O Configuration register"]
pub mod ccfg_sysio;
#[doc = "MATRIX_WPMR (rw) register accessor: Write Protect Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_wpmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_wpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_wpmr`]
module"]
pub type MATRIX_WPMR = crate::Reg<matrix_wpmr::MATRIX_WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod matrix_wpmr;
#[doc = "MATRIX_WPSR (r) register accessor: Write Protect Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_wpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_wpsr`]
module"]
pub type MATRIX_WPSR = crate::Reg<matrix_wpsr::MATRIX_WPSR_SPEC>;
#[doc = "Write Protect Status Register"]
pub mod matrix_wpsr;
