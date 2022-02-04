#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x18 - Master Configuration Register"]
    pub matrix_mcfg: [crate::Reg<matrix_mcfg::MATRIX_MCFG_SPEC>; 6],
    _reserved1: [u8; 0x28],
    #[doc = "0x40..0x64 - Slave Configuration Register"]
    pub matrix_scfg: [crate::Reg<matrix_scfg::MATRIX_SCFG_SPEC>; 9],
    _reserved2: [u8; 0x1c],
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
    _reserved7: [u8; 0x04],
    #[doc = "0xa8 - Priority Register A for Slave 5"]
    pub matrix_pras5: crate::Reg<matrix_pras5::MATRIX_PRAS5_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0xb0 - Priority Register A for Slave 6"]
    pub matrix_pras6: crate::Reg<matrix_pras6::MATRIX_PRAS6_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0xb8 - Priority Register A for Slave 7"]
    pub matrix_pras7: crate::Reg<matrix_pras7::MATRIX_PRAS7_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0xc0 - Priority Register A for Slave 8"]
    pub matrix_pras8: crate::Reg<matrix_pras8::MATRIX_PRAS8_SPEC>,
    _reserved11: [u8; 0x3c],
    #[doc = "0x100 - Master Remap Control Register"]
    pub matrix_mrcr: crate::Reg<matrix_mrcr::MATRIX_MRCR_SPEC>,
    _reserved12: [u8; 0x10],
    #[doc = "0x114 - System I/O Configuration register"]
    pub ccfg_sysio: crate::Reg<ccfg_sysio::CCFG_SYSIO_SPEC>,
    _reserved13: [u8; 0xcc],
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
#[doc = "MATRIX_PRAS5 register accessor: an alias for `Reg<MATRIX_PRAS5_SPEC>`"]
pub type MATRIX_PRAS5 = crate::Reg<matrix_pras5::MATRIX_PRAS5_SPEC>;
#[doc = "Priority Register A for Slave 5"]
pub mod matrix_pras5;
#[doc = "MATRIX_PRAS6 register accessor: an alias for `Reg<MATRIX_PRAS6_SPEC>`"]
pub type MATRIX_PRAS6 = crate::Reg<matrix_pras6::MATRIX_PRAS6_SPEC>;
#[doc = "Priority Register A for Slave 6"]
pub mod matrix_pras6;
#[doc = "MATRIX_PRAS7 register accessor: an alias for `Reg<MATRIX_PRAS7_SPEC>`"]
pub type MATRIX_PRAS7 = crate::Reg<matrix_pras7::MATRIX_PRAS7_SPEC>;
#[doc = "Priority Register A for Slave 7"]
pub mod matrix_pras7;
#[doc = "MATRIX_PRAS8 register accessor: an alias for `Reg<MATRIX_PRAS8_SPEC>`"]
pub type MATRIX_PRAS8 = crate::Reg<matrix_pras8::MATRIX_PRAS8_SPEC>;
#[doc = "Priority Register A for Slave 8"]
pub mod matrix_pras8;
#[doc = "MATRIX_MRCR register accessor: an alias for `Reg<MATRIX_MRCR_SPEC>`"]
pub type MATRIX_MRCR = crate::Reg<matrix_mrcr::MATRIX_MRCR_SPEC>;
#[doc = "Master Remap Control Register"]
pub mod matrix_mrcr;
#[doc = "CCFG_SYSIO register accessor: an alias for `Reg<CCFG_SYSIO_SPEC>`"]
pub type CCFG_SYSIO = crate::Reg<ccfg_sysio::CCFG_SYSIO_SPEC>;
#[doc = "System I/O Configuration register"]
pub mod ccfg_sysio;
#[doc = "MATRIX_WPMR register accessor: an alias for `Reg<MATRIX_WPMR_SPEC>`"]
pub type MATRIX_WPMR = crate::Reg<matrix_wpmr::MATRIX_WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod matrix_wpmr;
#[doc = "MATRIX_WPSR register accessor: an alias for `Reg<MATRIX_WPSR_SPEC>`"]
pub type MATRIX_WPSR = crate::Reg<matrix_wpsr::MATRIX_WPSR_SPEC>;
#[doc = "Write Protect Status Register"]
pub mod matrix_wpsr;
