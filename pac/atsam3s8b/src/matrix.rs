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
    _reserved4: [u8; 0x30],
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
    _reserved9: [u8; 0x2c],
    #[doc = "0x80 - Priority Register A for Slave 0"]
    pub matrix_pras0: MATRIX_PRAS0,
    _reserved10: [u8; 0x04],
    #[doc = "0x88 - Priority Register A for Slave 1"]
    pub matrix_pras1: MATRIX_PRAS1,
    _reserved11: [u8; 0x04],
    #[doc = "0x90 - Priority Register A for Slave 2"]
    pub matrix_pras2: MATRIX_PRAS2,
    _reserved12: [u8; 0x04],
    #[doc = "0x98 - Priority Register A for Slave 3"]
    pub matrix_pras3: MATRIX_PRAS3,
    _reserved13: [u8; 0x04],
    #[doc = "0xa0 - Priority Register A for Slave 4"]
    pub matrix_pras4: MATRIX_PRAS4,
    _reserved14: [u8; 0x70],
    #[doc = "0x114 - System I/O Configuration register"]
    pub ccfg_sysio: CCFG_SYSIO,
    _reserved15: [u8; 0x04],
    #[doc = "0x11c - SMC Chip Select NAND Flash Assignment Register"]
    pub ccfg_smcnfcs: CCFG_SMCNFCS,
    _reserved16: [u8; 0xc4],
    #[doc = "0x1e4 - Write Protect Mode Register"]
    pub matrix_wpmr: MATRIX_WPMR,
    #[doc = "0x1e8 - Write Protect Status Register"]
    pub matrix_wpsr: MATRIX_WPSR,
}
#[doc = "MATRIX_MCFG0 (rw) register accessor: Master Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_mcfg0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_mcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`matrix_mcfg0`]
module"]
pub type MATRIX_MCFG0 = crate::Reg<matrix_mcfg0::MATRIX_MCFG0_SPEC>;
#[doc = "Master Configuration Register 0"]
pub mod matrix_mcfg0;
#[doc = "MATRIX_MCFG1 (rw) register accessor: Master Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_mcfg1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_mcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`matrix_mcfg1`]
module"]
pub type MATRIX_MCFG1 = crate::Reg<matrix_mcfg1::MATRIX_MCFG1_SPEC>;
#[doc = "Master Configuration Register 1"]
pub mod matrix_mcfg1;
#[doc = "MATRIX_MCFG2 (rw) register accessor: Master Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_mcfg2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_mcfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`matrix_mcfg2`]
module"]
pub type MATRIX_MCFG2 = crate::Reg<matrix_mcfg2::MATRIX_MCFG2_SPEC>;
#[doc = "Master Configuration Register 2"]
pub mod matrix_mcfg2;
#[doc = "MATRIX_MCFG3 (rw) register accessor: Master Configuration Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_mcfg3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_mcfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`matrix_mcfg3`]
module"]
pub type MATRIX_MCFG3 = crate::Reg<matrix_mcfg3::MATRIX_MCFG3_SPEC>;
#[doc = "Master Configuration Register 3"]
pub mod matrix_mcfg3;
#[doc = "MATRIX_SCFG0 (rw) register accessor: Slave Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_scfg0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_scfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`matrix_scfg0`]
module"]
pub type MATRIX_SCFG0 = crate::Reg<matrix_scfg0::MATRIX_SCFG0_SPEC>;
#[doc = "Slave Configuration Register 0"]
pub mod matrix_scfg0;
#[doc = "MATRIX_SCFG1 (rw) register accessor: Slave Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_scfg1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_scfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`matrix_scfg1`]
module"]
pub type MATRIX_SCFG1 = crate::Reg<matrix_scfg1::MATRIX_SCFG1_SPEC>;
#[doc = "Slave Configuration Register 1"]
pub mod matrix_scfg1;
#[doc = "MATRIX_SCFG2 (rw) register accessor: Slave Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_scfg2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_scfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`matrix_scfg2`]
module"]
pub type MATRIX_SCFG2 = crate::Reg<matrix_scfg2::MATRIX_SCFG2_SPEC>;
#[doc = "Slave Configuration Register 2"]
pub mod matrix_scfg2;
#[doc = "MATRIX_SCFG3 (rw) register accessor: Slave Configuration Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_scfg3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_scfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`matrix_scfg3`]
module"]
pub type MATRIX_SCFG3 = crate::Reg<matrix_scfg3::MATRIX_SCFG3_SPEC>;
#[doc = "Slave Configuration Register 3"]
pub mod matrix_scfg3;
#[doc = "MATRIX_SCFG4 (rw) register accessor: Slave Configuration Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_scfg4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_scfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`matrix_scfg4`]
module"]
pub type MATRIX_SCFG4 = crate::Reg<matrix_scfg4::MATRIX_SCFG4_SPEC>;
#[doc = "Slave Configuration Register 4"]
pub mod matrix_scfg4;
#[doc = "MATRIX_PRAS0 (rw) register accessor: Priority Register A for Slave 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_pras0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_pras0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`matrix_pras0`]
module"]
pub type MATRIX_PRAS0 = crate::Reg<matrix_pras0::MATRIX_PRAS0_SPEC>;
#[doc = "Priority Register A for Slave 0"]
pub mod matrix_pras0;
#[doc = "MATRIX_PRAS1 (rw) register accessor: Priority Register A for Slave 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_pras1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_pras1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`matrix_pras1`]
module"]
pub type MATRIX_PRAS1 = crate::Reg<matrix_pras1::MATRIX_PRAS1_SPEC>;
#[doc = "Priority Register A for Slave 1"]
pub mod matrix_pras1;
#[doc = "MATRIX_PRAS2 (rw) register accessor: Priority Register A for Slave 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_pras2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_pras2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`matrix_pras2`]
module"]
pub type MATRIX_PRAS2 = crate::Reg<matrix_pras2::MATRIX_PRAS2_SPEC>;
#[doc = "Priority Register A for Slave 2"]
pub mod matrix_pras2;
#[doc = "MATRIX_PRAS3 (rw) register accessor: Priority Register A for Slave 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_pras3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_pras3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`matrix_pras3`]
module"]
pub type MATRIX_PRAS3 = crate::Reg<matrix_pras3::MATRIX_PRAS3_SPEC>;
#[doc = "Priority Register A for Slave 3"]
pub mod matrix_pras3;
#[doc = "MATRIX_PRAS4 (rw) register accessor: Priority Register A for Slave 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_pras4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_pras4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`matrix_pras4`]
module"]
pub type MATRIX_PRAS4 = crate::Reg<matrix_pras4::MATRIX_PRAS4_SPEC>;
#[doc = "Priority Register A for Slave 4"]
pub mod matrix_pras4;
#[doc = "CCFG_SYSIO (rw) register accessor: System I/O Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_sysio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_sysio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ccfg_sysio`]
module"]
pub type CCFG_SYSIO = crate::Reg<ccfg_sysio::CCFG_SYSIO_SPEC>;
#[doc = "System I/O Configuration register"]
pub mod ccfg_sysio;
#[doc = "CCFG_SMCNFCS (rw) register accessor: SMC Chip Select NAND Flash Assignment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_smcnfcs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_smcnfcs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ccfg_smcnfcs`]
module"]
pub type CCFG_SMCNFCS = crate::Reg<ccfg_smcnfcs::CCFG_SMCNFCS_SPEC>;
#[doc = "SMC Chip Select NAND Flash Assignment Register"]
pub mod ccfg_smcnfcs;
#[doc = "MATRIX_WPMR (rw) register accessor: Write Protect Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_wpmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_wpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`matrix_wpmr`]
module"]
pub type MATRIX_WPMR = crate::Reg<matrix_wpmr::MATRIX_WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod matrix_wpmr;
#[doc = "MATRIX_WPSR (r) register accessor: Write Protect Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_wpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`matrix_wpsr`]
module"]
pub type MATRIX_WPSR = crate::Reg<matrix_wpsr::MATRIX_WPSR_SPEC>;
#[doc = "Write Protect Status Register"]
pub mod matrix_wpsr;
