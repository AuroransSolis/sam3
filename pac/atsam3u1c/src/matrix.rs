#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x14 - Master Configuration Register"]
    pub mcfg: [crate::Reg<mcfg::MCFG_SPEC>; 5],
    _reserved1: [u8; 0x2c],
    #[doc = "0x40..0x68 - Slave Configuration Register"]
    pub scfg: [crate::Reg<scfg::SCFG_SPEC>; 10],
    _reserved2: [u8; 0x18],
    #[doc = "0x80 - Priority Register A for Slave 0"]
    pub pras0: crate::Reg<pras0::PRAS0_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x88 - Priority Register A for Slave 1"]
    pub pras1: crate::Reg<pras1::PRAS1_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x90 - Priority Register A for Slave 2"]
    pub pras2: crate::Reg<pras2::PRAS2_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x98 - Priority Register A for Slave 3"]
    pub pras3: crate::Reg<pras3::PRAS3_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0xa0 - Priority Register A for Slave 4"]
    pub pras4: crate::Reg<pras4::PRAS4_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0xa8 - Priority Register A for Slave 5"]
    pub pras5: crate::Reg<pras5::PRAS5_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0xb0 - Priority Register A for Slave 6"]
    pub pras6: crate::Reg<pras6::PRAS6_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0xb8 - Priority Register A for Slave 7"]
    pub pras7: crate::Reg<pras7::PRAS7_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0xc0 - Priority Register A for Slave 8"]
    pub pras8: crate::Reg<pras8::PRAS8_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0xc8 - Priority Register A for Slave 9"]
    pub pras9: crate::Reg<pras9::PRAS9_SPEC>,
    _reserved12: [u8; 0x34],
    #[doc = "0x100 - Master Remap Control Register"]
    pub mrcr: crate::Reg<mrcr::MRCR_SPEC>,
    _reserved13: [u8; 0xe0],
    #[doc = "0x1e4 - Write Protect Mode Register"]
    pub wpmr: crate::Reg<wpmr::WPMR_SPEC>,
    #[doc = "0x1e8 - Write Protect Status Register"]
    pub wpsr: crate::Reg<wpsr::WPSR_SPEC>,
}
#[doc = "MCFG register accessor: an alias for `Reg<MCFG_SPEC>`"]
pub type MCFG = crate::Reg<mcfg::MCFG_SPEC>;
#[doc = "Master Configuration Register"]
pub mod mcfg;
#[doc = "SCFG register accessor: an alias for `Reg<SCFG_SPEC>`"]
pub type SCFG = crate::Reg<scfg::SCFG_SPEC>;
#[doc = "Slave Configuration Register"]
pub mod scfg;
#[doc = "PRAS0 register accessor: an alias for `Reg<PRAS0_SPEC>`"]
pub type PRAS0 = crate::Reg<pras0::PRAS0_SPEC>;
#[doc = "Priority Register A for Slave 0"]
pub mod pras0;
#[doc = "PRAS1 register accessor: an alias for `Reg<PRAS1_SPEC>`"]
pub type PRAS1 = crate::Reg<pras1::PRAS1_SPEC>;
#[doc = "Priority Register A for Slave 1"]
pub mod pras1;
#[doc = "PRAS2 register accessor: an alias for `Reg<PRAS2_SPEC>`"]
pub type PRAS2 = crate::Reg<pras2::PRAS2_SPEC>;
#[doc = "Priority Register A for Slave 2"]
pub mod pras2;
#[doc = "PRAS3 register accessor: an alias for `Reg<PRAS3_SPEC>`"]
pub type PRAS3 = crate::Reg<pras3::PRAS3_SPEC>;
#[doc = "Priority Register A for Slave 3"]
pub mod pras3;
#[doc = "PRAS4 register accessor: an alias for `Reg<PRAS4_SPEC>`"]
pub type PRAS4 = crate::Reg<pras4::PRAS4_SPEC>;
#[doc = "Priority Register A for Slave 4"]
pub mod pras4;
#[doc = "PRAS5 register accessor: an alias for `Reg<PRAS5_SPEC>`"]
pub type PRAS5 = crate::Reg<pras5::PRAS5_SPEC>;
#[doc = "Priority Register A for Slave 5"]
pub mod pras5;
#[doc = "PRAS6 register accessor: an alias for `Reg<PRAS6_SPEC>`"]
pub type PRAS6 = crate::Reg<pras6::PRAS6_SPEC>;
#[doc = "Priority Register A for Slave 6"]
pub mod pras6;
#[doc = "PRAS7 register accessor: an alias for `Reg<PRAS7_SPEC>`"]
pub type PRAS7 = crate::Reg<pras7::PRAS7_SPEC>;
#[doc = "Priority Register A for Slave 7"]
pub mod pras7;
#[doc = "PRAS8 register accessor: an alias for `Reg<PRAS8_SPEC>`"]
pub type PRAS8 = crate::Reg<pras8::PRAS8_SPEC>;
#[doc = "Priority Register A for Slave 8"]
pub mod pras8;
#[doc = "PRAS9 register accessor: an alias for `Reg<PRAS9_SPEC>`"]
pub type PRAS9 = crate::Reg<pras9::PRAS9_SPEC>;
#[doc = "Priority Register A for Slave 9"]
pub mod pras9;
#[doc = "MRCR register accessor: an alias for `Reg<MRCR_SPEC>`"]
pub type MRCR = crate::Reg<mrcr::MRCR_SPEC>;
#[doc = "Master Remap Control Register"]
pub mod mrcr;
#[doc = "WPMR register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protect Status Register"]
pub mod wpsr;
