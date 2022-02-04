#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EEFC Flash Mode Register"]
    pub fmr: crate::Reg<fmr::FMR_SPEC>,
    #[doc = "0x04 - EEFC Flash Command Register"]
    pub fcr: crate::Reg<fcr::FCR_SPEC>,
    #[doc = "0x08 - EEFC Flash Status Register"]
    pub fsr: crate::Reg<fsr::FSR_SPEC>,
    #[doc = "0x0c - EEFC Flash Result Register"]
    pub frr: crate::Reg<frr::FRR_SPEC>,
}
#[doc = "FMR register accessor: an alias for `Reg<FMR_SPEC>`"]
pub type FMR = crate::Reg<fmr::FMR_SPEC>;
#[doc = "EEFC Flash Mode Register"]
pub mod fmr;
#[doc = "FCR register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "EEFC Flash Command Register"]
pub mod fcr;
#[doc = "FSR register accessor: an alias for `Reg<FSR_SPEC>`"]
pub type FSR = crate::Reg<fsr::FSR_SPEC>;
#[doc = "EEFC Flash Status Register"]
pub mod fsr;
#[doc = "FRR register accessor: an alias for `Reg<FRR_SPEC>`"]
pub type FRR = crate::Reg<frr::FRR_SPEC>;
#[doc = "EEFC Flash Result Register"]
pub mod frr;
