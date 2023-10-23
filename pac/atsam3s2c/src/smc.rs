#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SMC Setup Register (CS_number = 0)"]
    pub setup0: SETUP0,
    #[doc = "0x04 - SMC Pulse Register (CS_number = 0)"]
    pub pulse0: PULSE0,
    #[doc = "0x08 - SMC Cycle Register (CS_number = 0)"]
    pub cycle0: CYCLE0,
    #[doc = "0x0c - SMC Mode Register (CS_number = 0)"]
    pub mode0: MODE0,
    #[doc = "0x10 - SMC Setup Register (CS_number = 1)"]
    pub setup1: SETUP1,
    #[doc = "0x14 - SMC Pulse Register (CS_number = 1)"]
    pub pulse1: PULSE1,
    #[doc = "0x18 - SMC Cycle Register (CS_number = 1)"]
    pub cycle1: CYCLE1,
    #[doc = "0x1c - SMC Mode Register (CS_number = 1)"]
    pub mode1: MODE1,
    #[doc = "0x20 - SMC Setup Register (CS_number = 2)"]
    pub setup2: SETUP2,
    #[doc = "0x24 - SMC Pulse Register (CS_number = 2)"]
    pub pulse2: PULSE2,
    #[doc = "0x28 - SMC Cycle Register (CS_number = 2)"]
    pub cycle2: CYCLE2,
    #[doc = "0x2c - SMC Mode Register (CS_number = 2)"]
    pub mode2: MODE2,
    #[doc = "0x30 - SMC Setup Register (CS_number = 3)"]
    pub setup3: SETUP3,
    #[doc = "0x34 - SMC Pulse Register (CS_number = 3)"]
    pub pulse3: PULSE3,
    #[doc = "0x38 - SMC Cycle Register (CS_number = 3)"]
    pub cycle3: CYCLE3,
    #[doc = "0x3c - SMC Mode Register (CS_number = 3)"]
    pub mode3: MODE3,
    _reserved16: [u8; 0x40],
    #[doc = "0x80 - SMC OCMS MODE Register"]
    pub ocms: OCMS,
    #[doc = "0x84 - SMC OCMS KEY1 Register"]
    pub key1: KEY1,
    #[doc = "0x88 - SMC OCMS KEY2 Register"]
    pub key2: KEY2,
    _reserved19: [u8; 0x58],
    #[doc = "0xe4 - SMC Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - SMC Write Protect Status Register"]
    pub wpsr: WPSR,
}
#[doc = "SETUP0 (rw) register accessor: SMC Setup Register (CS_number = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setup0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup0`]
module"]
pub type SETUP0 = crate::Reg<setup0::SETUP0_SPEC>;
#[doc = "SMC Setup Register (CS_number = 0)"]
pub mod setup0;
#[doc = "PULSE0 (rw) register accessor: SMC Pulse Register (CS_number = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulse0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulse0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pulse0`]
module"]
pub type PULSE0 = crate::Reg<pulse0::PULSE0_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 0)"]
pub mod pulse0;
#[doc = "CYCLE0 (rw) register accessor: SMC Cycle Register (CS_number = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cycle0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cycle0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cycle0`]
module"]
pub type CYCLE0 = crate::Reg<cycle0::CYCLE0_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 0)"]
pub mod cycle0;
#[doc = "MODE0 (rw) register accessor: SMC Mode Register (CS_number = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode0`]
module"]
pub type MODE0 = crate::Reg<mode0::MODE0_SPEC>;
#[doc = "SMC Mode Register (CS_number = 0)"]
pub mod mode0;
#[doc = "SETUP1 (rw) register accessor: SMC Setup Register (CS_number = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setup1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup1`]
module"]
pub type SETUP1 = crate::Reg<setup1::SETUP1_SPEC>;
#[doc = "SMC Setup Register (CS_number = 1)"]
pub mod setup1;
#[doc = "PULSE1 (rw) register accessor: SMC Pulse Register (CS_number = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulse1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulse1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pulse1`]
module"]
pub type PULSE1 = crate::Reg<pulse1::PULSE1_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 1)"]
pub mod pulse1;
#[doc = "CYCLE1 (rw) register accessor: SMC Cycle Register (CS_number = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cycle1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cycle1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cycle1`]
module"]
pub type CYCLE1 = crate::Reg<cycle1::CYCLE1_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 1)"]
pub mod cycle1;
#[doc = "MODE1 (rw) register accessor: SMC Mode Register (CS_number = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode1`]
module"]
pub type MODE1 = crate::Reg<mode1::MODE1_SPEC>;
#[doc = "SMC Mode Register (CS_number = 1)"]
pub mod mode1;
#[doc = "SETUP2 (rw) register accessor: SMC Setup Register (CS_number = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setup2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup2`]
module"]
pub type SETUP2 = crate::Reg<setup2::SETUP2_SPEC>;
#[doc = "SMC Setup Register (CS_number = 2)"]
pub mod setup2;
#[doc = "PULSE2 (rw) register accessor: SMC Pulse Register (CS_number = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulse2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulse2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pulse2`]
module"]
pub type PULSE2 = crate::Reg<pulse2::PULSE2_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 2)"]
pub mod pulse2;
#[doc = "CYCLE2 (rw) register accessor: SMC Cycle Register (CS_number = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cycle2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cycle2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cycle2`]
module"]
pub type CYCLE2 = crate::Reg<cycle2::CYCLE2_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 2)"]
pub mod cycle2;
#[doc = "MODE2 (rw) register accessor: SMC Mode Register (CS_number = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode2`]
module"]
pub type MODE2 = crate::Reg<mode2::MODE2_SPEC>;
#[doc = "SMC Mode Register (CS_number = 2)"]
pub mod mode2;
#[doc = "SETUP3 (rw) register accessor: SMC Setup Register (CS_number = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setup3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup3`]
module"]
pub type SETUP3 = crate::Reg<setup3::SETUP3_SPEC>;
#[doc = "SMC Setup Register (CS_number = 3)"]
pub mod setup3;
#[doc = "PULSE3 (rw) register accessor: SMC Pulse Register (CS_number = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulse3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulse3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pulse3`]
module"]
pub type PULSE3 = crate::Reg<pulse3::PULSE3_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 3)"]
pub mod pulse3;
#[doc = "CYCLE3 (rw) register accessor: SMC Cycle Register (CS_number = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cycle3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cycle3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cycle3`]
module"]
pub type CYCLE3 = crate::Reg<cycle3::CYCLE3_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 3)"]
pub mod cycle3;
#[doc = "MODE3 (rw) register accessor: SMC Mode Register (CS_number = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode3`]
module"]
pub type MODE3 = crate::Reg<mode3::MODE3_SPEC>;
#[doc = "SMC Mode Register (CS_number = 3)"]
pub mod mode3;
#[doc = "OCMS (rw) register accessor: SMC OCMS MODE Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocms::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocms::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocms`]
module"]
pub type OCMS = crate::Reg<ocms::OCMS_SPEC>;
#[doc = "SMC OCMS MODE Register"]
pub mod ocms;
#[doc = "KEY1 (w) register accessor: SMC OCMS KEY1 Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key1`]
module"]
pub type KEY1 = crate::Reg<key1::KEY1_SPEC>;
#[doc = "SMC OCMS KEY1 Register"]
pub mod key1;
#[doc = "KEY2 (w) register accessor: SMC OCMS KEY2 Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key2`]
module"]
pub type KEY2 = crate::Reg<key2::KEY2_SPEC>;
#[doc = "SMC OCMS KEY2 Register"]
pub mod key2;
#[doc = "WPMR (rw) register accessor: SMC Write Protect Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`]
module"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "SMC Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: SMC Write Protect Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`]
module"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "SMC Write Protect Status Register"]
pub mod wpsr;
