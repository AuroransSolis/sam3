#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ccr0: CCR0,
    _reserved_1_cmr0: [u8; 0x04],
    smmr0: SMMR0,
    _reserved3: [u8; 0x04],
    cv0: CV0,
    ra0: RA0,
    rb0: RB0,
    rc0: RC0,
    sr0: SR0,
    ier0: IER0,
    idr0: IDR0,
    imr0: IMR0,
    _reserved11: [u8; 0x10],
    ccr1: CCR1,
    _reserved_12_cmr1: [u8; 0x04],
    smmr1: SMMR1,
    _reserved14: [u8; 0x04],
    cv1: CV1,
    ra1: RA1,
    rb1: RB1,
    rc1: RC1,
    sr1: SR1,
    ier1: IER1,
    idr1: IDR1,
    imr1: IMR1,
    _reserved22: [u8; 0x10],
    ccr2: CCR2,
    _reserved_23_cmr2: [u8; 0x04],
    smmr2: SMMR2,
    _reserved25: [u8; 0x04],
    cv2: CV2,
    ra2: RA2,
    rb2: RB2,
    rc2: RC2,
    sr2: SR2,
    ier2: IER2,
    idr2: IDR2,
    imr2: IMR2,
    _reserved33: [u8; 0x10],
    bcr: BCR,
    bmr: BMR,
    qier: QIER,
    qidr: QIDR,
    qimr: QIMR,
    qisr: QISR,
    fmr: FMR,
    _reserved40: [u8; 0x08],
    wpmr: WPMR,
}
impl RegisterBlock {
    #[doc = "0x00 - Channel Control Register (channel = 0)"]
    #[inline(always)]
    pub const fn ccr0(&self) -> &CCR0 {
        &self.ccr0
    }
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub const fn wave_eq_1_cmr0_wave_eq_1(&self) -> &WAVE_EQ_1_CMR0_WAVE_EQ_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub const fn cmr0(&self) -> &CMR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - Stepper Motor Mode Register (channel = 0)"]
    #[inline(always)]
    pub const fn smmr0(&self) -> &SMMR0 {
        &self.smmr0
    }
    #[doc = "0x10 - Counter Value (channel = 0)"]
    #[inline(always)]
    pub const fn cv0(&self) -> &CV0 {
        &self.cv0
    }
    #[doc = "0x14 - Register A (channel = 0)"]
    #[inline(always)]
    pub const fn ra0(&self) -> &RA0 {
        &self.ra0
    }
    #[doc = "0x18 - Register B (channel = 0)"]
    #[inline(always)]
    pub const fn rb0(&self) -> &RB0 {
        &self.rb0
    }
    #[doc = "0x1c - Register C (channel = 0)"]
    #[inline(always)]
    pub const fn rc0(&self) -> &RC0 {
        &self.rc0
    }
    #[doc = "0x20 - Status Register (channel = 0)"]
    #[inline(always)]
    pub const fn sr0(&self) -> &SR0 {
        &self.sr0
    }
    #[doc = "0x24 - Interrupt Enable Register (channel = 0)"]
    #[inline(always)]
    pub const fn ier0(&self) -> &IER0 {
        &self.ier0
    }
    #[doc = "0x28 - Interrupt Disable Register (channel = 0)"]
    #[inline(always)]
    pub const fn idr0(&self) -> &IDR0 {
        &self.idr0
    }
    #[doc = "0x2c - Interrupt Mask Register (channel = 0)"]
    #[inline(always)]
    pub const fn imr0(&self) -> &IMR0 {
        &self.imr0
    }
    #[doc = "0x40 - Channel Control Register (channel = 1)"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &CCR1 {
        &self.ccr1
    }
    #[doc = "0x44 - Channel Mode Register (channel = 1)"]
    #[inline(always)]
    pub const fn wave_eq_1_cmr1_wave_eq_1(&self) -> &WAVE_EQ_1_CMR1_WAVE_EQ_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x44 - Channel Mode Register (channel = 1)"]
    #[inline(always)]
    pub const fn cmr1(&self) -> &CMR1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x48 - Stepper Motor Mode Register (channel = 1)"]
    #[inline(always)]
    pub const fn smmr1(&self) -> &SMMR1 {
        &self.smmr1
    }
    #[doc = "0x50 - Counter Value (channel = 1)"]
    #[inline(always)]
    pub const fn cv1(&self) -> &CV1 {
        &self.cv1
    }
    #[doc = "0x54 - Register A (channel = 1)"]
    #[inline(always)]
    pub const fn ra1(&self) -> &RA1 {
        &self.ra1
    }
    #[doc = "0x58 - Register B (channel = 1)"]
    #[inline(always)]
    pub const fn rb1(&self) -> &RB1 {
        &self.rb1
    }
    #[doc = "0x5c - Register C (channel = 1)"]
    #[inline(always)]
    pub const fn rc1(&self) -> &RC1 {
        &self.rc1
    }
    #[doc = "0x60 - Status Register (channel = 1)"]
    #[inline(always)]
    pub const fn sr1(&self) -> &SR1 {
        &self.sr1
    }
    #[doc = "0x64 - Interrupt Enable Register (channel = 1)"]
    #[inline(always)]
    pub const fn ier1(&self) -> &IER1 {
        &self.ier1
    }
    #[doc = "0x68 - Interrupt Disable Register (channel = 1)"]
    #[inline(always)]
    pub const fn idr1(&self) -> &IDR1 {
        &self.idr1
    }
    #[doc = "0x6c - Interrupt Mask Register (channel = 1)"]
    #[inline(always)]
    pub const fn imr1(&self) -> &IMR1 {
        &self.imr1
    }
    #[doc = "0x80 - Channel Control Register (channel = 2)"]
    #[inline(always)]
    pub const fn ccr2(&self) -> &CCR2 {
        &self.ccr2
    }
    #[doc = "0x84 - Channel Mode Register (channel = 2)"]
    #[inline(always)]
    pub const fn wave_eq_1_cmr2_wave_eq_1(&self) -> &WAVE_EQ_1_CMR2_WAVE_EQ_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(132).cast() }
    }
    #[doc = "0x84 - Channel Mode Register (channel = 2)"]
    #[inline(always)]
    pub const fn cmr2(&self) -> &CMR2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(132).cast() }
    }
    #[doc = "0x88 - Stepper Motor Mode Register (channel = 2)"]
    #[inline(always)]
    pub const fn smmr2(&self) -> &SMMR2 {
        &self.smmr2
    }
    #[doc = "0x90 - Counter Value (channel = 2)"]
    #[inline(always)]
    pub const fn cv2(&self) -> &CV2 {
        &self.cv2
    }
    #[doc = "0x94 - Register A (channel = 2)"]
    #[inline(always)]
    pub const fn ra2(&self) -> &RA2 {
        &self.ra2
    }
    #[doc = "0x98 - Register B (channel = 2)"]
    #[inline(always)]
    pub const fn rb2(&self) -> &RB2 {
        &self.rb2
    }
    #[doc = "0x9c - Register C (channel = 2)"]
    #[inline(always)]
    pub const fn rc2(&self) -> &RC2 {
        &self.rc2
    }
    #[doc = "0xa0 - Status Register (channel = 2)"]
    #[inline(always)]
    pub const fn sr2(&self) -> &SR2 {
        &self.sr2
    }
    #[doc = "0xa4 - Interrupt Enable Register (channel = 2)"]
    #[inline(always)]
    pub const fn ier2(&self) -> &IER2 {
        &self.ier2
    }
    #[doc = "0xa8 - Interrupt Disable Register (channel = 2)"]
    #[inline(always)]
    pub const fn idr2(&self) -> &IDR2 {
        &self.idr2
    }
    #[doc = "0xac - Interrupt Mask Register (channel = 2)"]
    #[inline(always)]
    pub const fn imr2(&self) -> &IMR2 {
        &self.imr2
    }
    #[doc = "0xc0 - Block Control Register"]
    #[inline(always)]
    pub const fn bcr(&self) -> &BCR {
        &self.bcr
    }
    #[doc = "0xc4 - Block Mode Register"]
    #[inline(always)]
    pub const fn bmr(&self) -> &BMR {
        &self.bmr
    }
    #[doc = "0xc8 - QDEC Interrupt Enable Register"]
    #[inline(always)]
    pub const fn qier(&self) -> &QIER {
        &self.qier
    }
    #[doc = "0xcc - QDEC Interrupt Disable Register"]
    #[inline(always)]
    pub const fn qidr(&self) -> &QIDR {
        &self.qidr
    }
    #[doc = "0xd0 - QDEC Interrupt Mask Register"]
    #[inline(always)]
    pub const fn qimr(&self) -> &QIMR {
        &self.qimr
    }
    #[doc = "0xd4 - QDEC Interrupt Status Register"]
    #[inline(always)]
    pub const fn qisr(&self) -> &QISR {
        &self.qisr
    }
    #[doc = "0xd8 - Fault Mode Register"]
    #[inline(always)]
    pub const fn fmr(&self) -> &FMR {
        &self.fmr
    }
    #[doc = "0xe4 - Write Protect Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &WPMR {
        &self.wpmr
    }
}
#[doc = "CCR0 (w) register accessor: Channel Control Register (channel = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr0`]
module"]
pub type CCR0 = crate::Reg<ccr0::CCR0_SPEC>;
#[doc = "Channel Control Register (channel = 0)"]
pub mod ccr0;
#[doc = "CMR0 (rw) register accessor: Channel Mode Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr0`]
module"]
pub type CMR0 = crate::Reg<cmr0::CMR0_SPEC>;
#[doc = "Channel Mode Register (channel = 0)"]
pub mod cmr0;
#[doc = "WAVE_EQ_1_CMR0_WAVE_EQ_1 (rw) register accessor: Channel Mode Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wave_eq_1_cmr0_wave_eq_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wave_eq_1_cmr0_wave_eq_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wave_eq_1_cmr0_wave_eq_1`]
module"]
pub type WAVE_EQ_1_CMR0_WAVE_EQ_1 =
    crate::Reg<wave_eq_1_cmr0_wave_eq_1::WAVE_EQ_1_CMR0_WAVE_EQ_1_SPEC>;
#[doc = "Channel Mode Register (channel = 0)"]
pub mod wave_eq_1_cmr0_wave_eq_1;
#[doc = "SMMR0 (rw) register accessor: Stepper Motor Mode Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smmr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smmr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smmr0`]
module"]
pub type SMMR0 = crate::Reg<smmr0::SMMR0_SPEC>;
#[doc = "Stepper Motor Mode Register (channel = 0)"]
pub mod smmr0;
#[doc = "CV0 (r) register accessor: Counter Value (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cv0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cv0`]
module"]
pub type CV0 = crate::Reg<cv0::CV0_SPEC>;
#[doc = "Counter Value (channel = 0)"]
pub mod cv0;
#[doc = "RA0 (rw) register accessor: Register A (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ra0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ra0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ra0`]
module"]
pub type RA0 = crate::Reg<ra0::RA0_SPEC>;
#[doc = "Register A (channel = 0)"]
pub mod ra0;
#[doc = "RB0 (rw) register accessor: Register B (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rb0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rb0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rb0`]
module"]
pub type RB0 = crate::Reg<rb0::RB0_SPEC>;
#[doc = "Register B (channel = 0)"]
pub mod rb0;
#[doc = "RC0 (rw) register accessor: Register C (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc0`]
module"]
pub type RC0 = crate::Reg<rc0::RC0_SPEC>;
#[doc = "Register C (channel = 0)"]
pub mod rc0;
#[doc = "SR0 (r) register accessor: Status Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr0`]
module"]
pub type SR0 = crate::Reg<sr0::SR0_SPEC>;
#[doc = "Status Register (channel = 0)"]
pub mod sr0;
#[doc = "IER0 (w) register accessor: Interrupt Enable Register (channel = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier0`]
module"]
pub type IER0 = crate::Reg<ier0::IER0_SPEC>;
#[doc = "Interrupt Enable Register (channel = 0)"]
pub mod ier0;
#[doc = "IDR0 (w) register accessor: Interrupt Disable Register (channel = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr0`]
module"]
pub type IDR0 = crate::Reg<idr0::IDR0_SPEC>;
#[doc = "Interrupt Disable Register (channel = 0)"]
pub mod idr0;
#[doc = "IMR0 (r) register accessor: Interrupt Mask Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr0`]
module"]
pub type IMR0 = crate::Reg<imr0::IMR0_SPEC>;
#[doc = "Interrupt Mask Register (channel = 0)"]
pub mod imr0;
#[doc = "CCR1 (w) register accessor: Channel Control Register (channel = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`]
module"]
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
#[doc = "Channel Control Register (channel = 1)"]
pub mod ccr1;
#[doc = "CMR1 (rw) register accessor: Channel Mode Register (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr1`]
module"]
pub type CMR1 = crate::Reg<cmr1::CMR1_SPEC>;
#[doc = "Channel Mode Register (channel = 1)"]
pub mod cmr1;
#[doc = "WAVE_EQ_1_CMR1_WAVE_EQ_1 (rw) register accessor: Channel Mode Register (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wave_eq_1_cmr1_wave_eq_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wave_eq_1_cmr1_wave_eq_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wave_eq_1_cmr1_wave_eq_1`]
module"]
pub type WAVE_EQ_1_CMR1_WAVE_EQ_1 =
    crate::Reg<wave_eq_1_cmr1_wave_eq_1::WAVE_EQ_1_CMR1_WAVE_EQ_1_SPEC>;
#[doc = "Channel Mode Register (channel = 1)"]
pub mod wave_eq_1_cmr1_wave_eq_1;
#[doc = "SMMR1 (rw) register accessor: Stepper Motor Mode Register (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smmr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smmr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smmr1`]
module"]
pub type SMMR1 = crate::Reg<smmr1::SMMR1_SPEC>;
#[doc = "Stepper Motor Mode Register (channel = 1)"]
pub mod smmr1;
#[doc = "CV1 (r) register accessor: Counter Value (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cv1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cv1`]
module"]
pub type CV1 = crate::Reg<cv1::CV1_SPEC>;
#[doc = "Counter Value (channel = 1)"]
pub mod cv1;
#[doc = "RA1 (rw) register accessor: Register A (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ra1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ra1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ra1`]
module"]
pub type RA1 = crate::Reg<ra1::RA1_SPEC>;
#[doc = "Register A (channel = 1)"]
pub mod ra1;
#[doc = "RB1 (rw) register accessor: Register B (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rb1`]
module"]
pub type RB1 = crate::Reg<rb1::RB1_SPEC>;
#[doc = "Register B (channel = 1)"]
pub mod rb1;
#[doc = "RC1 (rw) register accessor: Register C (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc1`]
module"]
pub type RC1 = crate::Reg<rc1::RC1_SPEC>;
#[doc = "Register C (channel = 1)"]
pub mod rc1;
#[doc = "SR1 (r) register accessor: Status Register (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr1`]
module"]
pub type SR1 = crate::Reg<sr1::SR1_SPEC>;
#[doc = "Status Register (channel = 1)"]
pub mod sr1;
#[doc = "IER1 (w) register accessor: Interrupt Enable Register (channel = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier1`]
module"]
pub type IER1 = crate::Reg<ier1::IER1_SPEC>;
#[doc = "Interrupt Enable Register (channel = 1)"]
pub mod ier1;
#[doc = "IDR1 (w) register accessor: Interrupt Disable Register (channel = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr1`]
module"]
pub type IDR1 = crate::Reg<idr1::IDR1_SPEC>;
#[doc = "Interrupt Disable Register (channel = 1)"]
pub mod idr1;
#[doc = "IMR1 (r) register accessor: Interrupt Mask Register (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr1`]
module"]
pub type IMR1 = crate::Reg<imr1::IMR1_SPEC>;
#[doc = "Interrupt Mask Register (channel = 1)"]
pub mod imr1;
#[doc = "CCR2 (w) register accessor: Channel Control Register (channel = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2`]
module"]
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
#[doc = "Channel Control Register (channel = 2)"]
pub mod ccr2;
#[doc = "CMR2 (rw) register accessor: Channel Mode Register (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr2`]
module"]
pub type CMR2 = crate::Reg<cmr2::CMR2_SPEC>;
#[doc = "Channel Mode Register (channel = 2)"]
pub mod cmr2;
#[doc = "WAVE_EQ_1_CMR2_WAVE_EQ_1 (rw) register accessor: Channel Mode Register (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wave_eq_1_cmr2_wave_eq_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wave_eq_1_cmr2_wave_eq_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wave_eq_1_cmr2_wave_eq_1`]
module"]
pub type WAVE_EQ_1_CMR2_WAVE_EQ_1 =
    crate::Reg<wave_eq_1_cmr2_wave_eq_1::WAVE_EQ_1_CMR2_WAVE_EQ_1_SPEC>;
#[doc = "Channel Mode Register (channel = 2)"]
pub mod wave_eq_1_cmr2_wave_eq_1;
#[doc = "SMMR2 (rw) register accessor: Stepper Motor Mode Register (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smmr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smmr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smmr2`]
module"]
pub type SMMR2 = crate::Reg<smmr2::SMMR2_SPEC>;
#[doc = "Stepper Motor Mode Register (channel = 2)"]
pub mod smmr2;
#[doc = "CV2 (r) register accessor: Counter Value (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cv2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cv2`]
module"]
pub type CV2 = crate::Reg<cv2::CV2_SPEC>;
#[doc = "Counter Value (channel = 2)"]
pub mod cv2;
#[doc = "RA2 (rw) register accessor: Register A (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ra2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ra2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ra2`]
module"]
pub type RA2 = crate::Reg<ra2::RA2_SPEC>;
#[doc = "Register A (channel = 2)"]
pub mod ra2;
#[doc = "RB2 (rw) register accessor: Register B (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rb2`]
module"]
pub type RB2 = crate::Reg<rb2::RB2_SPEC>;
#[doc = "Register B (channel = 2)"]
pub mod rb2;
#[doc = "RC2 (rw) register accessor: Register C (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc2`]
module"]
pub type RC2 = crate::Reg<rc2::RC2_SPEC>;
#[doc = "Register C (channel = 2)"]
pub mod rc2;
#[doc = "SR2 (r) register accessor: Status Register (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr2`]
module"]
pub type SR2 = crate::Reg<sr2::SR2_SPEC>;
#[doc = "Status Register (channel = 2)"]
pub mod sr2;
#[doc = "IER2 (w) register accessor: Interrupt Enable Register (channel = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier2`]
module"]
pub type IER2 = crate::Reg<ier2::IER2_SPEC>;
#[doc = "Interrupt Enable Register (channel = 2)"]
pub mod ier2;
#[doc = "IDR2 (w) register accessor: Interrupt Disable Register (channel = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr2`]
module"]
pub type IDR2 = crate::Reg<idr2::IDR2_SPEC>;
#[doc = "Interrupt Disable Register (channel = 2)"]
pub mod idr2;
#[doc = "IMR2 (r) register accessor: Interrupt Mask Register (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr2`]
module"]
pub type IMR2 = crate::Reg<imr2::IMR2_SPEC>;
#[doc = "Interrupt Mask Register (channel = 2)"]
pub mod imr2;
#[doc = "BCR (w) register accessor: Block Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr`]
module"]
pub type BCR = crate::Reg<bcr::BCR_SPEC>;
#[doc = "Block Control Register"]
pub mod bcr;
#[doc = "BMR (rw) register accessor: Block Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmr`]
module"]
pub type BMR = crate::Reg<bmr::BMR_SPEC>;
#[doc = "Block Mode Register"]
pub mod bmr;
#[doc = "QIER (w) register accessor: QDEC Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qier`]
module"]
pub type QIER = crate::Reg<qier::QIER_SPEC>;
#[doc = "QDEC Interrupt Enable Register"]
pub mod qier;
#[doc = "QIDR (w) register accessor: QDEC Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qidr`]
module"]
pub type QIDR = crate::Reg<qidr::QIDR_SPEC>;
#[doc = "QDEC Interrupt Disable Register"]
pub mod qidr;
#[doc = "QIMR (r) register accessor: QDEC Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qimr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qimr`]
module"]
pub type QIMR = crate::Reg<qimr::QIMR_SPEC>;
#[doc = "QDEC Interrupt Mask Register"]
pub mod qimr;
#[doc = "QISR (r) register accessor: QDEC Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qisr`]
module"]
pub type QISR = crate::Reg<qisr::QISR_SPEC>;
#[doc = "QDEC Interrupt Status Register"]
pub mod qisr;
#[doc = "FMR (rw) register accessor: Fault Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmr`]
module"]
pub type FMR = crate::Reg<fmr::FMR_SPEC>;
#[doc = "Fault Mode Register"]
pub mod fmr;
#[doc = "WPMR (rw) register accessor: Write Protect Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`]
module"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
