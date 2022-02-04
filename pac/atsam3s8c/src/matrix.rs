#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x10 - Master Configuration Register"]
    pub matrix_mcfg: [crate::Reg<matrix_mcfg::MATRIX_MCFG_SPEC>; 4],
    _reserved1: [u8; 0x30],
    #[doc = "0x40..0x54 - Slave Configuration Register"]
    pub matrix_scfg: [crate::Reg<matrix_scfg::MATRIX_SCFG_SPEC>; 5],
    _reserved2: [u8; 0x2c],
    #[doc = "0x80 - Priority Register A for Slave 0"]
    pub matrix_pras0: crate::Reg<matrix_pras0::MATRIX_PRAS0_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x88 - Priority Register A for Slave 1"]
    pub matrix_pras1: crate::Reg<matrix_pras1::MATRIX_PRAS1_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x90 - Priority Register A for Slave 2"]
    pub matrix_pras2: crate::Reg<matrix_pras2::MATRIX_PRAS2_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x98 - Priority Register A for Slave 3"]
    pub matrix_pras3: crate::Reg<matrix_pras3::MATRIX_PRAS3_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0xa0 - Priority Register A for Slave 4"]
    pub matrix_pras4: crate::Reg<matrix_pras4::MATRIX_PRAS4_SPEC>,
    _reserved7: [u8; 0x70],
    #[doc = "0x114 - System I/O Configuration register"]
    pub ccfg_sysio: crate::Reg<ccfg_sysio::CCFG_SYSIO_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x11c - SMC Chip Select NAND Flash Assignment Register"]
    pub ccfg_smcnfcs: crate::Reg<ccfg_smcnfcs::CCFG_SMCNFCS_SPEC>,
    _reserved9: [u8; 0xc4],
    #[doc = "0x1e4 - Write Protect Mode Register"]
    pub matrix_wpmr: crate::Reg<matrix_wpmr::MATRIX_WPMR_SPEC>,
    #[doc = "0x1e8 - Write Protect Status Register"]
    pub matrix_wpsr: crate::Reg<matrix_wpsr::MATRIX_WPSR_SPEC>,
}
#[doc = "MATRIX_MCFG register accessor: an alias for `Reg<MATRIX_MCFG_SPEC>`"]
pub type MATRIX_MCFG = crate::Reg<matrix_mcfg::MATRIX_MCFG_SPEC>;
#[doc = "Master Configuration Register"]
pub mod matrix_mcfg;
#[doc = "MATRIX_SCFG register accessor: an alias for `Reg<MATRIX_SCFG_SPEC>`"]
pub type MATRIX_SCFG = crate::Reg<matrix_scfg::MATRIX_SCFG_SPEC>;
#[doc = "Slave Configuration Register"]
pub mod matrix_scfg;
#[doc = "MATRIX_PRAS0 register accessor: an alias for `Reg<MATRIX_PRAS0_SPEC>`"]
pub type MATRIX_PRAS0 = crate::Reg<matrix_pras0::MATRIX_PRAS0_SPEC>;
#[doc = "Priority Register A for Slave 0"]
pub mod matrix_pras0;
#[doc = "MATRIX_PRAS1 register accessor: an alias for `Reg<MATRIX_PRAS1_SPEC>`"]
pub type MATRIX_PRAS1 = crate::Reg<matrix_pras1::MATRIX_PRAS1_SPEC>;
#[doc = "Priority Register A for Slave 1"]
pub mod matrix_pras1;
#[doc = "MATRIX_PRAS2 register accessor: an alias for `Reg<MATRIX_PRAS2_SPEC>`"]
pub type MATRIX_PRAS2 = crate::Reg<matrix_pras2::MATRIX_PRAS2_SPEC>;
#[doc = "Priority Register A for Slave 2"]
pub mod matrix_pras2;
#[doc = "MATRIX_PRAS3 register accessor: an alias for `Reg<MATRIX_PRAS3_SPEC>`"]
pub type MATRIX_PRAS3 = crate::Reg<matrix_pras3::MATRIX_PRAS3_SPEC>;
#[doc = "Priority Register A for Slave 3"]
pub mod matrix_pras3;
#[doc = "MATRIX_PRAS4 register accessor: an alias for `Reg<MATRIX_PRAS4_SPEC>`"]
pub type MATRIX_PRAS4 = crate::Reg<matrix_pras4::MATRIX_PRAS4_SPEC>;
#[doc = "Priority Register A for Slave 4"]
pub mod matrix_pras4;
#[doc = "CCFG_SYSIO register accessor: an alias for `Reg<CCFG_SYSIO_SPEC>`"]
pub type CCFG_SYSIO = crate::Reg<ccfg_sysio::CCFG_SYSIO_SPEC>;
#[doc = "System I/O Configuration register"]
pub mod ccfg_sysio;
#[doc = "CCFG_SMCNFCS register accessor: an alias for `Reg<CCFG_SMCNFCS_SPEC>`"]
pub type CCFG_SMCNFCS = crate::Reg<ccfg_smcnfcs::CCFG_SMCNFCS_SPEC>;
#[doc = "SMC Chip Select NAND Flash Assignment Register"]
pub mod ccfg_smcnfcs;
#[doc = "MATRIX_WPMR register accessor: an alias for `Reg<MATRIX_WPMR_SPEC>`"]
pub type MATRIX_WPMR = crate::Reg<matrix_wpmr::MATRIX_WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod matrix_wpmr;
#[doc = "MATRIX_WPSR register accessor: an alias for `Reg<MATRIX_WPSR_SPEC>`"]
pub type MATRIX_WPSR = crate::Reg<matrix_wpsr::MATRIX_WPSR_SPEC>;
#[doc = "Write Protect Status Register"]
pub mod matrix_wpsr;
