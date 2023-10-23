#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Configuration Register 0"]
    pub matrix_mcfg0: MATRIX_MCFG0,
    #[doc = "0x04 - Master Configuration Register 1"]
    pub matrix_mcfg1: MATRIX_MCFG1,
    #[doc = "0x08 - Master Configuration Register 2"]
    pub matrix_mcfg2: MATRIX_MCFG2,
    #[doc = "0x0c - Master Configuration Register 3"]
    pub matrix_mcfg3: MATRIX_MCFG3,
    #[doc = "0x10 - Master Configuration Register 4"]
    pub matrix_mcfg4: MATRIX_MCFG4,
    #[doc = "0x14 - Master Configuration Register 5"]
    pub matrix_mcfg5: MATRIX_MCFG5,
    _reserved6: [u8; 0x28],
    #[doc = "0x40 - Slave Configuration Register 0"]
    pub matrix_scfg0: MATRIX_SCFG0,
    #[doc = "0x44 - Slave Configuration Register 1"]
    pub matrix_scfg1: MATRIX_SCFG1,
    #[doc = "0x48 - Slave Configuration Register 2"]
    pub matrix_scfg2: MATRIX_SCFG2,
    #[doc = "0x4c - Slave Configuration Register 3"]
    pub matrix_scfg3: MATRIX_SCFG3,
    #[doc = "0x50 - Slave Configuration Register 4"]
    pub matrix_scfg4: MATRIX_SCFG4,
    #[doc = "0x54 - Slave Configuration Register 5"]
    pub matrix_scfg5: MATRIX_SCFG5,
    #[doc = "0x58 - Slave Configuration Register 6"]
    pub matrix_scfg6: MATRIX_SCFG6,
    #[doc = "0x5c - Slave Configuration Register 7"]
    pub matrix_scfg7: MATRIX_SCFG7,
    #[doc = "0x60 - Slave Configuration Register 8"]
    pub matrix_scfg8: MATRIX_SCFG8,
    _reserved15: [u8; 0x1c],
    #[doc = "0x80 - Priority Register A for Slave 0"]
    pub matrix_pras0: MATRIX_PRAS0,
    _reserved16: [u8; 0x04],
    #[doc = "0x88 - Priority Register A for Slave 1"]
    pub matrix_pras1: MATRIX_PRAS1,
    _reserved17: [u8; 0x04],
    #[doc = "0x90 - Priority Register A for Slave 2"]
    pub matrix_pras2: MATRIX_PRAS2,
    _reserved18: [u8; 0x04],
    #[doc = "0x98 - Priority Register A for Slave 3"]
    pub matrix_pras3: MATRIX_PRAS3,
    _reserved19: [u8; 0x04],
    #[doc = "0xa0 - Priority Register A for Slave 4"]
    pub matrix_pras4: MATRIX_PRAS4,
    _reserved20: [u8; 0x04],
    #[doc = "0xa8 - Priority Register A for Slave 5"]
    pub matrix_pras5: MATRIX_PRAS5,
    _reserved21: [u8; 0x04],
    #[doc = "0xb0 - Priority Register A for Slave 6"]
    pub matrix_pras6: MATRIX_PRAS6,
    _reserved22: [u8; 0x04],
    #[doc = "0xb8 - Priority Register A for Slave 7"]
    pub matrix_pras7: MATRIX_PRAS7,
    _reserved23: [u8; 0x04],
    #[doc = "0xc0 - Priority Register A for Slave 8"]
    pub matrix_pras8: MATRIX_PRAS8,
    _reserved24: [u8; 0x3c],
    #[doc = "0x100 - Master Remap Control Register"]
    pub matrix_mrcr: MATRIX_MRCR,
    _reserved25: [u8; 0x10],
    #[doc = "0x114 - System I/O Configuration register"]
    pub ccfg_sysio: CCFG_SYSIO,
    _reserved26: [u8; 0xcc],
    #[doc = "0x1e4 - Write Protect Mode Register"]
    pub matrix_wpmr: MATRIX_WPMR,
    #[doc = "0x1e8 - Write Protect Status Register"]
    pub matrix_wpsr: MATRIX_WPSR,
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
#[doc = "MATRIX_MCFG3 (rw) register accessor: Master Configuration Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_mcfg3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_mcfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_mcfg3`]
module"]
pub type MATRIX_MCFG3 = crate::Reg<matrix_mcfg3::MATRIX_MCFG3_SPEC>;
#[doc = "Master Configuration Register 3"]
pub mod matrix_mcfg3;
#[doc = "MATRIX_MCFG4 (rw) register accessor: Master Configuration Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_mcfg4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_mcfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_mcfg4`]
module"]
pub type MATRIX_MCFG4 = crate::Reg<matrix_mcfg4::MATRIX_MCFG4_SPEC>;
#[doc = "Master Configuration Register 4"]
pub mod matrix_mcfg4;
#[doc = "MATRIX_MCFG5 (rw) register accessor: Master Configuration Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_mcfg5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_mcfg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_mcfg5`]
module"]
pub type MATRIX_MCFG5 = crate::Reg<matrix_mcfg5::MATRIX_MCFG5_SPEC>;
#[doc = "Master Configuration Register 5"]
pub mod matrix_mcfg5;
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
#[doc = "MATRIX_SCFG4 (rw) register accessor: Slave Configuration Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_scfg4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_scfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_scfg4`]
module"]
pub type MATRIX_SCFG4 = crate::Reg<matrix_scfg4::MATRIX_SCFG4_SPEC>;
#[doc = "Slave Configuration Register 4"]
pub mod matrix_scfg4;
#[doc = "MATRIX_SCFG5 (rw) register accessor: Slave Configuration Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_scfg5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_scfg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_scfg5`]
module"]
pub type MATRIX_SCFG5 = crate::Reg<matrix_scfg5::MATRIX_SCFG5_SPEC>;
#[doc = "Slave Configuration Register 5"]
pub mod matrix_scfg5;
#[doc = "MATRIX_SCFG6 (rw) register accessor: Slave Configuration Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_scfg6::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_scfg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_scfg6`]
module"]
pub type MATRIX_SCFG6 = crate::Reg<matrix_scfg6::MATRIX_SCFG6_SPEC>;
#[doc = "Slave Configuration Register 6"]
pub mod matrix_scfg6;
#[doc = "MATRIX_SCFG7 (rw) register accessor: Slave Configuration Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_scfg7::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_scfg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_scfg7`]
module"]
pub type MATRIX_SCFG7 = crate::Reg<matrix_scfg7::MATRIX_SCFG7_SPEC>;
#[doc = "Slave Configuration Register 7"]
pub mod matrix_scfg7;
#[doc = "MATRIX_SCFG8 (rw) register accessor: Slave Configuration Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_scfg8::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_scfg8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_scfg8`]
module"]
pub type MATRIX_SCFG8 = crate::Reg<matrix_scfg8::MATRIX_SCFG8_SPEC>;
#[doc = "Slave Configuration Register 8"]
pub mod matrix_scfg8;
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
#[doc = "MATRIX_PRAS4 (rw) register accessor: Priority Register A for Slave 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_pras4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_pras4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras4`]
module"]
pub type MATRIX_PRAS4 = crate::Reg<matrix_pras4::MATRIX_PRAS4_SPEC>;
#[doc = "Priority Register A for Slave 4"]
pub mod matrix_pras4;
#[doc = "MATRIX_PRAS5 (rw) register accessor: Priority Register A for Slave 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_pras5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_pras5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras5`]
module"]
pub type MATRIX_PRAS5 = crate::Reg<matrix_pras5::MATRIX_PRAS5_SPEC>;
#[doc = "Priority Register A for Slave 5"]
pub mod matrix_pras5;
#[doc = "MATRIX_PRAS6 (rw) register accessor: Priority Register A for Slave 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_pras6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_pras6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras6`]
module"]
pub type MATRIX_PRAS6 = crate::Reg<matrix_pras6::MATRIX_PRAS6_SPEC>;
#[doc = "Priority Register A for Slave 6"]
pub mod matrix_pras6;
#[doc = "MATRIX_PRAS7 (rw) register accessor: Priority Register A for Slave 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_pras7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_pras7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras7`]
module"]
pub type MATRIX_PRAS7 = crate::Reg<matrix_pras7::MATRIX_PRAS7_SPEC>;
#[doc = "Priority Register A for Slave 7"]
pub mod matrix_pras7;
#[doc = "MATRIX_PRAS8 (rw) register accessor: Priority Register A for Slave 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_pras8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_pras8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras8`]
module"]
pub type MATRIX_PRAS8 = crate::Reg<matrix_pras8::MATRIX_PRAS8_SPEC>;
#[doc = "Priority Register A for Slave 8"]
pub mod matrix_pras8;
#[doc = "MATRIX_MRCR (rw) register accessor: Master Remap Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_mrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_mrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_mrcr`]
module"]
pub type MATRIX_MRCR = crate::Reg<matrix_mrcr::MATRIX_MRCR_SPEC>;
#[doc = "Master Remap Control Register"]
pub mod matrix_mrcr;
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
