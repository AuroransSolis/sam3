#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    matrix_mcfg0: MatrixMcfg0,
    matrix_mcfg1: MatrixMcfg1,
    matrix_mcfg2: MatrixMcfg2,
    matrix_mcfg3: MatrixMcfg3,
    _reserved4: [u8; 0x30],
    matrix_scfg0: MatrixScfg0,
    matrix_scfg1: MatrixScfg1,
    matrix_scfg2: MatrixScfg2,
    matrix_scfg3: MatrixScfg3,
    matrix_scfg4: MatrixScfg4,
    _reserved9: [u8; 0x2c],
    matrix_pras0: MatrixPras0,
    _reserved10: [u8; 0x04],
    matrix_pras1: MatrixPras1,
    _reserved11: [u8; 0x04],
    matrix_pras2: MatrixPras2,
    _reserved12: [u8; 0x04],
    matrix_pras3: MatrixPras3,
    _reserved13: [u8; 0x04],
    matrix_pras4: MatrixPras4,
    _reserved14: [u8; 0x70],
    ccfg_sysio: CcfgSysio,
    _reserved15: [u8; 0x04],
    ccfg_smcnfcs: CcfgSmcnfcs,
    _reserved16: [u8; 0xc4],
    matrix_wpmr: MatrixWpmr,
    matrix_wpsr: MatrixWpsr,
}
impl RegisterBlock {
    #[doc = "0x00 - Master Configuration Register 0"]
    #[inline(always)]
    pub const fn matrix_mcfg0(&self) -> &MatrixMcfg0 {
        &self.matrix_mcfg0
    }
    #[doc = "0x04 - Master Configuration Register 1"]
    #[inline(always)]
    pub const fn matrix_mcfg1(&self) -> &MatrixMcfg1 {
        &self.matrix_mcfg1
    }
    #[doc = "0x08 - Master Configuration Register 2"]
    #[inline(always)]
    pub const fn matrix_mcfg2(&self) -> &MatrixMcfg2 {
        &self.matrix_mcfg2
    }
    #[doc = "0x0c - Master Configuration Register 3"]
    #[inline(always)]
    pub const fn matrix_mcfg3(&self) -> &MatrixMcfg3 {
        &self.matrix_mcfg3
    }
    #[doc = "0x40 - Slave Configuration Register 0"]
    #[inline(always)]
    pub const fn matrix_scfg0(&self) -> &MatrixScfg0 {
        &self.matrix_scfg0
    }
    #[doc = "0x44 - Slave Configuration Register 1"]
    #[inline(always)]
    pub const fn matrix_scfg1(&self) -> &MatrixScfg1 {
        &self.matrix_scfg1
    }
    #[doc = "0x48 - Slave Configuration Register 2"]
    #[inline(always)]
    pub const fn matrix_scfg2(&self) -> &MatrixScfg2 {
        &self.matrix_scfg2
    }
    #[doc = "0x4c - Slave Configuration Register 3"]
    #[inline(always)]
    pub const fn matrix_scfg3(&self) -> &MatrixScfg3 {
        &self.matrix_scfg3
    }
    #[doc = "0x50 - Slave Configuration Register 4"]
    #[inline(always)]
    pub const fn matrix_scfg4(&self) -> &MatrixScfg4 {
        &self.matrix_scfg4
    }
    #[doc = "0x80 - Priority Register A for Slave 0"]
    #[inline(always)]
    pub const fn matrix_pras0(&self) -> &MatrixPras0 {
        &self.matrix_pras0
    }
    #[doc = "0x88 - Priority Register A for Slave 1"]
    #[inline(always)]
    pub const fn matrix_pras1(&self) -> &MatrixPras1 {
        &self.matrix_pras1
    }
    #[doc = "0x90 - Priority Register A for Slave 2"]
    #[inline(always)]
    pub const fn matrix_pras2(&self) -> &MatrixPras2 {
        &self.matrix_pras2
    }
    #[doc = "0x98 - Priority Register A for Slave 3"]
    #[inline(always)]
    pub const fn matrix_pras3(&self) -> &MatrixPras3 {
        &self.matrix_pras3
    }
    #[doc = "0xa0 - Priority Register A for Slave 4"]
    #[inline(always)]
    pub const fn matrix_pras4(&self) -> &MatrixPras4 {
        &self.matrix_pras4
    }
    #[doc = "0x114 - System I/O Configuration register"]
    #[inline(always)]
    pub const fn ccfg_sysio(&self) -> &CcfgSysio {
        &self.ccfg_sysio
    }
    #[doc = "0x11c - SMC Chip Select NAND Flash Assignment Register"]
    #[inline(always)]
    pub const fn ccfg_smcnfcs(&self) -> &CcfgSmcnfcs {
        &self.ccfg_smcnfcs
    }
    #[doc = "0x1e4 - Write Protect Mode Register"]
    #[inline(always)]
    pub const fn matrix_wpmr(&self) -> &MatrixWpmr {
        &self.matrix_wpmr
    }
    #[doc = "0x1e8 - Write Protect Status Register"]
    #[inline(always)]
    pub const fn matrix_wpsr(&self) -> &MatrixWpsr {
        &self.matrix_wpsr
    }
}
#[doc = "MATRIX_MCFG0 (rw) register accessor: Master Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_mcfg0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_mcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_mcfg0`]
module"]
#[doc(alias = "MATRIX_MCFG0")]
pub type MatrixMcfg0 = crate::Reg<matrix_mcfg0::MatrixMcfg0Spec>;
#[doc = "Master Configuration Register 0"]
pub mod matrix_mcfg0;
#[doc = "MATRIX_MCFG1 (rw) register accessor: Master Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_mcfg1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_mcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_mcfg1`]
module"]
#[doc(alias = "MATRIX_MCFG1")]
pub type MatrixMcfg1 = crate::Reg<matrix_mcfg1::MatrixMcfg1Spec>;
#[doc = "Master Configuration Register 1"]
pub mod matrix_mcfg1;
#[doc = "MATRIX_MCFG2 (rw) register accessor: Master Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_mcfg2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_mcfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_mcfg2`]
module"]
#[doc(alias = "MATRIX_MCFG2")]
pub type MatrixMcfg2 = crate::Reg<matrix_mcfg2::MatrixMcfg2Spec>;
#[doc = "Master Configuration Register 2"]
pub mod matrix_mcfg2;
#[doc = "MATRIX_MCFG3 (rw) register accessor: Master Configuration Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_mcfg3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_mcfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_mcfg3`]
module"]
#[doc(alias = "MATRIX_MCFG3")]
pub type MatrixMcfg3 = crate::Reg<matrix_mcfg3::MatrixMcfg3Spec>;
#[doc = "Master Configuration Register 3"]
pub mod matrix_mcfg3;
#[doc = "MATRIX_SCFG0 (rw) register accessor: Slave Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_scfg0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_scfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_scfg0`]
module"]
#[doc(alias = "MATRIX_SCFG0")]
pub type MatrixScfg0 = crate::Reg<matrix_scfg0::MatrixScfg0Spec>;
#[doc = "Slave Configuration Register 0"]
pub mod matrix_scfg0;
#[doc = "MATRIX_SCFG1 (rw) register accessor: Slave Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_scfg1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_scfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_scfg1`]
module"]
#[doc(alias = "MATRIX_SCFG1")]
pub type MatrixScfg1 = crate::Reg<matrix_scfg1::MatrixScfg1Spec>;
#[doc = "Slave Configuration Register 1"]
pub mod matrix_scfg1;
#[doc = "MATRIX_SCFG2 (rw) register accessor: Slave Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_scfg2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_scfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_scfg2`]
module"]
#[doc(alias = "MATRIX_SCFG2")]
pub type MatrixScfg2 = crate::Reg<matrix_scfg2::MatrixScfg2Spec>;
#[doc = "Slave Configuration Register 2"]
pub mod matrix_scfg2;
#[doc = "MATRIX_SCFG3 (rw) register accessor: Slave Configuration Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_scfg3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_scfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_scfg3`]
module"]
#[doc(alias = "MATRIX_SCFG3")]
pub type MatrixScfg3 = crate::Reg<matrix_scfg3::MatrixScfg3Spec>;
#[doc = "Slave Configuration Register 3"]
pub mod matrix_scfg3;
#[doc = "MATRIX_SCFG4 (rw) register accessor: Slave Configuration Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_scfg4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_scfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_scfg4`]
module"]
#[doc(alias = "MATRIX_SCFG4")]
pub type MatrixScfg4 = crate::Reg<matrix_scfg4::MatrixScfg4Spec>;
#[doc = "Slave Configuration Register 4"]
pub mod matrix_scfg4;
#[doc = "MATRIX_PRAS0 (rw) register accessor: Priority Register A for Slave 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_pras0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_pras0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras0`]
module"]
#[doc(alias = "MATRIX_PRAS0")]
pub type MatrixPras0 = crate::Reg<matrix_pras0::MatrixPras0Spec>;
#[doc = "Priority Register A for Slave 0"]
pub mod matrix_pras0;
#[doc = "MATRIX_PRAS1 (rw) register accessor: Priority Register A for Slave 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_pras1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_pras1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras1`]
module"]
#[doc(alias = "MATRIX_PRAS1")]
pub type MatrixPras1 = crate::Reg<matrix_pras1::MatrixPras1Spec>;
#[doc = "Priority Register A for Slave 1"]
pub mod matrix_pras1;
#[doc = "MATRIX_PRAS2 (rw) register accessor: Priority Register A for Slave 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_pras2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_pras2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras2`]
module"]
#[doc(alias = "MATRIX_PRAS2")]
pub type MatrixPras2 = crate::Reg<matrix_pras2::MatrixPras2Spec>;
#[doc = "Priority Register A for Slave 2"]
pub mod matrix_pras2;
#[doc = "MATRIX_PRAS3 (rw) register accessor: Priority Register A for Slave 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_pras3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_pras3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras3`]
module"]
#[doc(alias = "MATRIX_PRAS3")]
pub type MatrixPras3 = crate::Reg<matrix_pras3::MatrixPras3Spec>;
#[doc = "Priority Register A for Slave 3"]
pub mod matrix_pras3;
#[doc = "MATRIX_PRAS4 (rw) register accessor: Priority Register A for Slave 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_pras4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_pras4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras4`]
module"]
#[doc(alias = "MATRIX_PRAS4")]
pub type MatrixPras4 = crate::Reg<matrix_pras4::MatrixPras4Spec>;
#[doc = "Priority Register A for Slave 4"]
pub mod matrix_pras4;
#[doc = "CCFG_SYSIO (rw) register accessor: System I/O Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_sysio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_sysio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_sysio`]
module"]
#[doc(alias = "CCFG_SYSIO")]
pub type CcfgSysio = crate::Reg<ccfg_sysio::CcfgSysioSpec>;
#[doc = "System I/O Configuration register"]
pub mod ccfg_sysio;
#[doc = "CCFG_SMCNFCS (rw) register accessor: SMC Chip Select NAND Flash Assignment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_smcnfcs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_smcnfcs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_smcnfcs`]
module"]
#[doc(alias = "CCFG_SMCNFCS")]
pub type CcfgSmcnfcs = crate::Reg<ccfg_smcnfcs::CcfgSmcnfcsSpec>;
#[doc = "SMC Chip Select NAND Flash Assignment Register"]
pub mod ccfg_smcnfcs;
#[doc = "MATRIX_WPMR (rw) register accessor: Write Protect Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_wpmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_wpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_wpmr`]
module"]
#[doc(alias = "MATRIX_WPMR")]
pub type MatrixWpmr = crate::Reg<matrix_wpmr::MatrixWpmrSpec>;
#[doc = "Write Protect Mode Register"]
pub mod matrix_wpmr;
#[doc = "MATRIX_WPSR (r) register accessor: Write Protect Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_wpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_wpsr`]
module"]
#[doc(alias = "MATRIX_WPSR")]
pub type MatrixWpsr = crate::Reg<matrix_wpsr::MatrixWpsrSpec>;
#[doc = "Write Protect Status Register"]
pub mod matrix_wpsr;
