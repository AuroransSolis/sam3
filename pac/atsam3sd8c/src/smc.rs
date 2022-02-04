#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SMC Setup Register (CS_number = 0)"]
    pub setup0: crate::Reg<setup0::SETUP0_SPEC>,
    #[doc = "0x04 - SMC Pulse Register (CS_number = 0)"]
    pub pulse0: crate::Reg<pulse0::PULSE0_SPEC>,
    #[doc = "0x08 - SMC Cycle Register (CS_number = 0)"]
    pub cycle0: crate::Reg<cycle0::CYCLE0_SPEC>,
    #[doc = "0x0c - SMC Mode Register (CS_number = 0)"]
    pub mode0: crate::Reg<mode0::MODE0_SPEC>,
    #[doc = "0x10 - SMC Setup Register (CS_number = 1)"]
    pub setup1: crate::Reg<setup1::SETUP1_SPEC>,
    #[doc = "0x14 - SMC Pulse Register (CS_number = 1)"]
    pub pulse1: crate::Reg<pulse1::PULSE1_SPEC>,
    #[doc = "0x18 - SMC Cycle Register (CS_number = 1)"]
    pub cycle1: crate::Reg<cycle1::CYCLE1_SPEC>,
    #[doc = "0x1c - SMC Mode Register (CS_number = 1)"]
    pub mode1: crate::Reg<mode1::MODE1_SPEC>,
    #[doc = "0x20 - SMC Setup Register (CS_number = 2)"]
    pub setup2: crate::Reg<setup2::SETUP2_SPEC>,
    #[doc = "0x24 - SMC Pulse Register (CS_number = 2)"]
    pub pulse2: crate::Reg<pulse2::PULSE2_SPEC>,
    #[doc = "0x28 - SMC Cycle Register (CS_number = 2)"]
    pub cycle2: crate::Reg<cycle2::CYCLE2_SPEC>,
    #[doc = "0x2c - SMC Mode Register (CS_number = 2)"]
    pub mode2: crate::Reg<mode2::MODE2_SPEC>,
    #[doc = "0x30 - SMC Setup Register (CS_number = 3)"]
    pub setup3: crate::Reg<setup3::SETUP3_SPEC>,
    #[doc = "0x34 - SMC Pulse Register (CS_number = 3)"]
    pub pulse3: crate::Reg<pulse3::PULSE3_SPEC>,
    #[doc = "0x38 - SMC Cycle Register (CS_number = 3)"]
    pub cycle3: crate::Reg<cycle3::CYCLE3_SPEC>,
    #[doc = "0x3c - SMC Mode Register (CS_number = 3)"]
    pub mode3: crate::Reg<mode3::MODE3_SPEC>,
    _reserved16: [u8; 0x40],
    #[doc = "0x80 - SMC OCMS MODE Register"]
    pub ocms: crate::Reg<ocms::OCMS_SPEC>,
    #[doc = "0x84 - SMC OCMS KEY1 Register"]
    pub key1: crate::Reg<key1::KEY1_SPEC>,
    #[doc = "0x88 - SMC OCMS KEY2 Register"]
    pub key2: crate::Reg<key2::KEY2_SPEC>,
    _reserved19: [u8; 0x58],
    #[doc = "0xe4 - SMC Write Protect Mode Register"]
    pub wpmr: crate::Reg<wpmr::WPMR_SPEC>,
    #[doc = "0xe8 - SMC Write Protect Status Register"]
    pub wpsr: crate::Reg<wpsr::WPSR_SPEC>,
}
#[doc = "SETUP0 register accessor: an alias for `Reg<SETUP0_SPEC>`"]
pub type SETUP0 = crate::Reg<setup0::SETUP0_SPEC>;
#[doc = "SMC Setup Register (CS_number = 0)"]
pub mod setup0;
#[doc = "PULSE0 register accessor: an alias for `Reg<PULSE0_SPEC>`"]
pub type PULSE0 = crate::Reg<pulse0::PULSE0_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 0)"]
pub mod pulse0;
#[doc = "CYCLE0 register accessor: an alias for `Reg<CYCLE0_SPEC>`"]
pub type CYCLE0 = crate::Reg<cycle0::CYCLE0_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 0)"]
pub mod cycle0;
#[doc = "MODE0 register accessor: an alias for `Reg<MODE0_SPEC>`"]
pub type MODE0 = crate::Reg<mode0::MODE0_SPEC>;
#[doc = "SMC Mode Register (CS_number = 0)"]
pub mod mode0;
#[doc = "SETUP1 register accessor: an alias for `Reg<SETUP1_SPEC>`"]
pub type SETUP1 = crate::Reg<setup1::SETUP1_SPEC>;
#[doc = "SMC Setup Register (CS_number = 1)"]
pub mod setup1;
#[doc = "PULSE1 register accessor: an alias for `Reg<PULSE1_SPEC>`"]
pub type PULSE1 = crate::Reg<pulse1::PULSE1_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 1)"]
pub mod pulse1;
#[doc = "CYCLE1 register accessor: an alias for `Reg<CYCLE1_SPEC>`"]
pub type CYCLE1 = crate::Reg<cycle1::CYCLE1_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 1)"]
pub mod cycle1;
#[doc = "MODE1 register accessor: an alias for `Reg<MODE1_SPEC>`"]
pub type MODE1 = crate::Reg<mode1::MODE1_SPEC>;
#[doc = "SMC Mode Register (CS_number = 1)"]
pub mod mode1;
#[doc = "SETUP2 register accessor: an alias for `Reg<SETUP2_SPEC>`"]
pub type SETUP2 = crate::Reg<setup2::SETUP2_SPEC>;
#[doc = "SMC Setup Register (CS_number = 2)"]
pub mod setup2;
#[doc = "PULSE2 register accessor: an alias for `Reg<PULSE2_SPEC>`"]
pub type PULSE2 = crate::Reg<pulse2::PULSE2_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 2)"]
pub mod pulse2;
#[doc = "CYCLE2 register accessor: an alias for `Reg<CYCLE2_SPEC>`"]
pub type CYCLE2 = crate::Reg<cycle2::CYCLE2_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 2)"]
pub mod cycle2;
#[doc = "MODE2 register accessor: an alias for `Reg<MODE2_SPEC>`"]
pub type MODE2 = crate::Reg<mode2::MODE2_SPEC>;
#[doc = "SMC Mode Register (CS_number = 2)"]
pub mod mode2;
#[doc = "SETUP3 register accessor: an alias for `Reg<SETUP3_SPEC>`"]
pub type SETUP3 = crate::Reg<setup3::SETUP3_SPEC>;
#[doc = "SMC Setup Register (CS_number = 3)"]
pub mod setup3;
#[doc = "PULSE3 register accessor: an alias for `Reg<PULSE3_SPEC>`"]
pub type PULSE3 = crate::Reg<pulse3::PULSE3_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 3)"]
pub mod pulse3;
#[doc = "CYCLE3 register accessor: an alias for `Reg<CYCLE3_SPEC>`"]
pub type CYCLE3 = crate::Reg<cycle3::CYCLE3_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 3)"]
pub mod cycle3;
#[doc = "MODE3 register accessor: an alias for `Reg<MODE3_SPEC>`"]
pub type MODE3 = crate::Reg<mode3::MODE3_SPEC>;
#[doc = "SMC Mode Register (CS_number = 3)"]
pub mod mode3;
#[doc = "OCMS register accessor: an alias for `Reg<OCMS_SPEC>`"]
pub type OCMS = crate::Reg<ocms::OCMS_SPEC>;
#[doc = "SMC OCMS MODE Register"]
pub mod ocms;
#[doc = "KEY1 register accessor: an alias for `Reg<KEY1_SPEC>`"]
pub type KEY1 = crate::Reg<key1::KEY1_SPEC>;
#[doc = "SMC OCMS KEY1 Register"]
pub mod key1;
#[doc = "KEY2 register accessor: an alias for `Reg<KEY2_SPEC>`"]
pub type KEY2 = crate::Reg<key2::KEY2_SPEC>;
#[doc = "SMC OCMS KEY2 Register"]
pub mod key2;
#[doc = "WPMR register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "SMC Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "SMC Write Protect Status Register"]
pub mod wpsr;
