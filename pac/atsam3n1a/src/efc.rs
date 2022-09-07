#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EEFC Flash Mode Register"]
    pub fmr: FMR,
    #[doc = "0x04 - EEFC Flash Command Register"]
    pub fcr: FCR,
    #[doc = "0x08 - EEFC Flash Status Register"]
    pub fsr: FSR,
    #[doc = "0x0c - EEFC Flash Result Register"]
    pub frr: FRR,
}
#[doc = "FMR (rw) register accessor: an alias for `Reg<FMR_SPEC>`"]
pub type FMR = crate::Reg<fmr::FMR_SPEC>;
#[doc = "EEFC Flash Mode Register"]
pub mod fmr;
#[doc = "FCR (w) register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "EEFC Flash Command Register"]
pub mod fcr;
#[doc = "FSR (r) register accessor: an alias for `Reg<FSR_SPEC>`"]
pub type FSR = crate::Reg<fsr::FSR_SPEC>;
#[doc = "EEFC Flash Status Register"]
pub mod fsr;
#[doc = "FRR (r) register accessor: an alias for `Reg<FRR_SPEC>`"]
pub type FRR = crate::Reg<frr::FRR_SPEC>;
#[doc = "EEFC Flash Result Register"]
pub mod frr;
