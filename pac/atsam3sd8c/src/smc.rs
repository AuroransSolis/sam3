#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    setup0: Setup0,
    pulse0: Pulse0,
    cycle0: Cycle0,
    mode0: Mode0,
    setup1: Setup1,
    pulse1: Pulse1,
    cycle1: Cycle1,
    mode1: Mode1,
    setup2: Setup2,
    pulse2: Pulse2,
    cycle2: Cycle2,
    mode2: Mode2,
    setup3: Setup3,
    pulse3: Pulse3,
    cycle3: Cycle3,
    mode3: Mode3,
    _reserved16: [u8; 0x40],
    ocms: Ocms,
    key1: Key1,
    key2: Key2,
    _reserved19: [u8; 0x58],
    wpmr: Wpmr,
    wpsr: Wpsr,
}
impl RegisterBlock {
    #[doc = "0x00 - SMC Setup Register (CS_number = 0)"]
    #[inline(always)]
    pub const fn setup0(&self) -> &Setup0 {
        &self.setup0
    }
    #[doc = "0x04 - SMC Pulse Register (CS_number = 0)"]
    #[inline(always)]
    pub const fn pulse0(&self) -> &Pulse0 {
        &self.pulse0
    }
    #[doc = "0x08 - SMC Cycle Register (CS_number = 0)"]
    #[inline(always)]
    pub const fn cycle0(&self) -> &Cycle0 {
        &self.cycle0
    }
    #[doc = "0x0c - SMC Mode Register (CS_number = 0)"]
    #[inline(always)]
    pub const fn mode0(&self) -> &Mode0 {
        &self.mode0
    }
    #[doc = "0x10 - SMC Setup Register (CS_number = 1)"]
    #[inline(always)]
    pub const fn setup1(&self) -> &Setup1 {
        &self.setup1
    }
    #[doc = "0x14 - SMC Pulse Register (CS_number = 1)"]
    #[inline(always)]
    pub const fn pulse1(&self) -> &Pulse1 {
        &self.pulse1
    }
    #[doc = "0x18 - SMC Cycle Register (CS_number = 1)"]
    #[inline(always)]
    pub const fn cycle1(&self) -> &Cycle1 {
        &self.cycle1
    }
    #[doc = "0x1c - SMC Mode Register (CS_number = 1)"]
    #[inline(always)]
    pub const fn mode1(&self) -> &Mode1 {
        &self.mode1
    }
    #[doc = "0x20 - SMC Setup Register (CS_number = 2)"]
    #[inline(always)]
    pub const fn setup2(&self) -> &Setup2 {
        &self.setup2
    }
    #[doc = "0x24 - SMC Pulse Register (CS_number = 2)"]
    #[inline(always)]
    pub const fn pulse2(&self) -> &Pulse2 {
        &self.pulse2
    }
    #[doc = "0x28 - SMC Cycle Register (CS_number = 2)"]
    #[inline(always)]
    pub const fn cycle2(&self) -> &Cycle2 {
        &self.cycle2
    }
    #[doc = "0x2c - SMC Mode Register (CS_number = 2)"]
    #[inline(always)]
    pub const fn mode2(&self) -> &Mode2 {
        &self.mode2
    }
    #[doc = "0x30 - SMC Setup Register (CS_number = 3)"]
    #[inline(always)]
    pub const fn setup3(&self) -> &Setup3 {
        &self.setup3
    }
    #[doc = "0x34 - SMC Pulse Register (CS_number = 3)"]
    #[inline(always)]
    pub const fn pulse3(&self) -> &Pulse3 {
        &self.pulse3
    }
    #[doc = "0x38 - SMC Cycle Register (CS_number = 3)"]
    #[inline(always)]
    pub const fn cycle3(&self) -> &Cycle3 {
        &self.cycle3
    }
    #[doc = "0x3c - SMC Mode Register (CS_number = 3)"]
    #[inline(always)]
    pub const fn mode3(&self) -> &Mode3 {
        &self.mode3
    }
    #[doc = "0x80 - SMC OCMS MODE Register"]
    #[inline(always)]
    pub const fn ocms(&self) -> &Ocms {
        &self.ocms
    }
    #[doc = "0x84 - SMC OCMS KEY1 Register"]
    #[inline(always)]
    pub const fn key1(&self) -> &Key1 {
        &self.key1
    }
    #[doc = "0x88 - SMC OCMS KEY2 Register"]
    #[inline(always)]
    pub const fn key2(&self) -> &Key2 {
        &self.key2
    }
    #[doc = "0xe4 - SMC Write Protect Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &Wpmr {
        &self.wpmr
    }
    #[doc = "0xe8 - SMC Write Protect Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &Wpsr {
        &self.wpsr
    }
}
#[doc = "SETUP0 (rw) register accessor: SMC Setup Register (CS_number = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`setup0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setup0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup0`]
module"]
#[doc(alias = "SETUP0")]
pub type Setup0 = crate::Reg<setup0::Setup0Spec>;
#[doc = "SMC Setup Register (CS_number = 0)"]
pub mod setup0;
#[doc = "PULSE0 (rw) register accessor: SMC Pulse Register (CS_number = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`pulse0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pulse0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pulse0`]
module"]
#[doc(alias = "PULSE0")]
pub type Pulse0 = crate::Reg<pulse0::Pulse0Spec>;
#[doc = "SMC Pulse Register (CS_number = 0)"]
pub mod pulse0;
#[doc = "CYCLE0 (rw) register accessor: SMC Cycle Register (CS_number = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`cycle0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cycle0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cycle0`]
module"]
#[doc(alias = "CYCLE0")]
pub type Cycle0 = crate::Reg<cycle0::Cycle0Spec>;
#[doc = "SMC Cycle Register (CS_number = 0)"]
pub mod cycle0;
#[doc = "MODE0 (rw) register accessor: SMC Mode Register (CS_number = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`mode0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode0`]
module"]
#[doc(alias = "MODE0")]
pub type Mode0 = crate::Reg<mode0::Mode0Spec>;
#[doc = "SMC Mode Register (CS_number = 0)"]
pub mod mode0;
#[doc = "SETUP1 (rw) register accessor: SMC Setup Register (CS_number = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`setup1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setup1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup1`]
module"]
#[doc(alias = "SETUP1")]
pub type Setup1 = crate::Reg<setup1::Setup1Spec>;
#[doc = "SMC Setup Register (CS_number = 1)"]
pub mod setup1;
#[doc = "PULSE1 (rw) register accessor: SMC Pulse Register (CS_number = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`pulse1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pulse1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pulse1`]
module"]
#[doc(alias = "PULSE1")]
pub type Pulse1 = crate::Reg<pulse1::Pulse1Spec>;
#[doc = "SMC Pulse Register (CS_number = 1)"]
pub mod pulse1;
#[doc = "CYCLE1 (rw) register accessor: SMC Cycle Register (CS_number = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`cycle1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cycle1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cycle1`]
module"]
#[doc(alias = "CYCLE1")]
pub type Cycle1 = crate::Reg<cycle1::Cycle1Spec>;
#[doc = "SMC Cycle Register (CS_number = 1)"]
pub mod cycle1;
#[doc = "MODE1 (rw) register accessor: SMC Mode Register (CS_number = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`mode1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode1`]
module"]
#[doc(alias = "MODE1")]
pub type Mode1 = crate::Reg<mode1::Mode1Spec>;
#[doc = "SMC Mode Register (CS_number = 1)"]
pub mod mode1;
#[doc = "SETUP2 (rw) register accessor: SMC Setup Register (CS_number = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`setup2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setup2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup2`]
module"]
#[doc(alias = "SETUP2")]
pub type Setup2 = crate::Reg<setup2::Setup2Spec>;
#[doc = "SMC Setup Register (CS_number = 2)"]
pub mod setup2;
#[doc = "PULSE2 (rw) register accessor: SMC Pulse Register (CS_number = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`pulse2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pulse2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pulse2`]
module"]
#[doc(alias = "PULSE2")]
pub type Pulse2 = crate::Reg<pulse2::Pulse2Spec>;
#[doc = "SMC Pulse Register (CS_number = 2)"]
pub mod pulse2;
#[doc = "CYCLE2 (rw) register accessor: SMC Cycle Register (CS_number = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`cycle2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cycle2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cycle2`]
module"]
#[doc(alias = "CYCLE2")]
pub type Cycle2 = crate::Reg<cycle2::Cycle2Spec>;
#[doc = "SMC Cycle Register (CS_number = 2)"]
pub mod cycle2;
#[doc = "MODE2 (rw) register accessor: SMC Mode Register (CS_number = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`mode2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode2`]
module"]
#[doc(alias = "MODE2")]
pub type Mode2 = crate::Reg<mode2::Mode2Spec>;
#[doc = "SMC Mode Register (CS_number = 2)"]
pub mod mode2;
#[doc = "SETUP3 (rw) register accessor: SMC Setup Register (CS_number = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`setup3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setup3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup3`]
module"]
#[doc(alias = "SETUP3")]
pub type Setup3 = crate::Reg<setup3::Setup3Spec>;
#[doc = "SMC Setup Register (CS_number = 3)"]
pub mod setup3;
#[doc = "PULSE3 (rw) register accessor: SMC Pulse Register (CS_number = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`pulse3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pulse3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pulse3`]
module"]
#[doc(alias = "PULSE3")]
pub type Pulse3 = crate::Reg<pulse3::Pulse3Spec>;
#[doc = "SMC Pulse Register (CS_number = 3)"]
pub mod pulse3;
#[doc = "CYCLE3 (rw) register accessor: SMC Cycle Register (CS_number = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`cycle3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cycle3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cycle3`]
module"]
#[doc(alias = "CYCLE3")]
pub type Cycle3 = crate::Reg<cycle3::Cycle3Spec>;
#[doc = "SMC Cycle Register (CS_number = 3)"]
pub mod cycle3;
#[doc = "MODE3 (rw) register accessor: SMC Mode Register (CS_number = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`mode3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode3`]
module"]
#[doc(alias = "MODE3")]
pub type Mode3 = crate::Reg<mode3::Mode3Spec>;
#[doc = "SMC Mode Register (CS_number = 3)"]
pub mod mode3;
#[doc = "OCMS (rw) register accessor: SMC OCMS MODE Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ocms::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocms::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocms`]
module"]
#[doc(alias = "OCMS")]
pub type Ocms = crate::Reg<ocms::OcmsSpec>;
#[doc = "SMC OCMS MODE Register"]
pub mod ocms;
#[doc = "KEY1 (w) register accessor: SMC OCMS KEY1 Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key1`]
module"]
#[doc(alias = "KEY1")]
pub type Key1 = crate::Reg<key1::Key1Spec>;
#[doc = "SMC OCMS KEY1 Register"]
pub mod key1;
#[doc = "KEY2 (w) register accessor: SMC OCMS KEY2 Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key2`]
module"]
#[doc(alias = "KEY2")]
pub type Key2 = crate::Reg<key2::Key2Spec>;
#[doc = "SMC OCMS KEY2 Register"]
pub mod key2;
#[doc = "WPMR (rw) register accessor: SMC Write Protect Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`]
module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "SMC Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: SMC Write Protect Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`]
module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "SMC Write Protect Status Register"]
pub mod wpsr;
