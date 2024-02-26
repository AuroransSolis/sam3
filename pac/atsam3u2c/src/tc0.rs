#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ccr0: Ccr0,
    _reserved_1_cmr0: [u8; 0x04],
    _reserved2: [u8; 0x08],
    cv0: Cv0,
    ra0: Ra0,
    rb0: Rb0,
    rc0: Rc0,
    sr0: Sr0,
    ier0: Ier0,
    idr0: Idr0,
    imr0: Imr0,
    _reserved10: [u8; 0x10],
    ccr1: Ccr1,
    _reserved_11_cmr1: [u8; 0x04],
    _reserved12: [u8; 0x08],
    cv1: Cv1,
    ra1: Ra1,
    rb1: Rb1,
    rc1: Rc1,
    sr1: Sr1,
    ier1: Ier1,
    idr1: Idr1,
    imr1: Imr1,
    _reserved20: [u8; 0x10],
    ccr2: Ccr2,
    _reserved_21_cmr2: [u8; 0x04],
    _reserved22: [u8; 0x08],
    cv2: Cv2,
    ra2: Ra2,
    rb2: Rb2,
    rc2: Rc2,
    sr2: Sr2,
    ier2: Ier2,
    idr2: Idr2,
    imr2: Imr2,
    _reserved30: [u8; 0x10],
    bcr: Bcr,
    bmr: Bmr,
    qier: Qier,
    qidr: Qidr,
    qimr: Qimr,
    qisr: Qisr,
}
impl RegisterBlock {
    #[doc = "0x00 - Channel Control Register (channel = 0)"]
    #[inline(always)]
    pub const fn ccr0(&self) -> &Ccr0 {
        &self.ccr0
    }
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub const fn waveform_mode_cmr0_waveform_mode(&self) -> &WaveformModeCmr0WaveformMode {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub const fn cmr0(&self) -> &Cmr0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x10 - Counter Value (channel = 0)"]
    #[inline(always)]
    pub const fn cv0(&self) -> &Cv0 {
        &self.cv0
    }
    #[doc = "0x14 - Register A (channel = 0)"]
    #[inline(always)]
    pub const fn ra0(&self) -> &Ra0 {
        &self.ra0
    }
    #[doc = "0x18 - Register B (channel = 0)"]
    #[inline(always)]
    pub const fn rb0(&self) -> &Rb0 {
        &self.rb0
    }
    #[doc = "0x1c - Register C (channel = 0)"]
    #[inline(always)]
    pub const fn rc0(&self) -> &Rc0 {
        &self.rc0
    }
    #[doc = "0x20 - Status Register (channel = 0)"]
    #[inline(always)]
    pub const fn sr0(&self) -> &Sr0 {
        &self.sr0
    }
    #[doc = "0x24 - Interrupt Enable Register (channel = 0)"]
    #[inline(always)]
    pub const fn ier0(&self) -> &Ier0 {
        &self.ier0
    }
    #[doc = "0x28 - Interrupt Disable Register (channel = 0)"]
    #[inline(always)]
    pub const fn idr0(&self) -> &Idr0 {
        &self.idr0
    }
    #[doc = "0x2c - Interrupt Mask Register (channel = 0)"]
    #[inline(always)]
    pub const fn imr0(&self) -> &Imr0 {
        &self.imr0
    }
    #[doc = "0x40 - Channel Control Register (channel = 1)"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &Ccr1 {
        &self.ccr1
    }
    #[doc = "0x44 - Channel Mode Register (channel = 1)"]
    #[inline(always)]
    pub const fn waveform_mode_cmr1_waveform_mode(&self) -> &WaveformModeCmr1WaveformMode {
        unsafe { &*(self as *const Self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x44 - Channel Mode Register (channel = 1)"]
    #[inline(always)]
    pub const fn cmr1(&self) -> &Cmr1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x50 - Counter Value (channel = 1)"]
    #[inline(always)]
    pub const fn cv1(&self) -> &Cv1 {
        &self.cv1
    }
    #[doc = "0x54 - Register A (channel = 1)"]
    #[inline(always)]
    pub const fn ra1(&self) -> &Ra1 {
        &self.ra1
    }
    #[doc = "0x58 - Register B (channel = 1)"]
    #[inline(always)]
    pub const fn rb1(&self) -> &Rb1 {
        &self.rb1
    }
    #[doc = "0x5c - Register C (channel = 1)"]
    #[inline(always)]
    pub const fn rc1(&self) -> &Rc1 {
        &self.rc1
    }
    #[doc = "0x60 - Status Register (channel = 1)"]
    #[inline(always)]
    pub const fn sr1(&self) -> &Sr1 {
        &self.sr1
    }
    #[doc = "0x64 - Interrupt Enable Register (channel = 1)"]
    #[inline(always)]
    pub const fn ier1(&self) -> &Ier1 {
        &self.ier1
    }
    #[doc = "0x68 - Interrupt Disable Register (channel = 1)"]
    #[inline(always)]
    pub const fn idr1(&self) -> &Idr1 {
        &self.idr1
    }
    #[doc = "0x6c - Interrupt Mask Register (channel = 1)"]
    #[inline(always)]
    pub const fn imr1(&self) -> &Imr1 {
        &self.imr1
    }
    #[doc = "0x80 - Channel Control Register (channel = 2)"]
    #[inline(always)]
    pub const fn ccr2(&self) -> &Ccr2 {
        &self.ccr2
    }
    #[doc = "0x84 - Channel Mode Register (channel = 2)"]
    #[inline(always)]
    pub const fn waveform_mode_cmr2_waveform_mode(&self) -> &WaveformModeCmr2WaveformMode {
        unsafe { &*(self as *const Self).cast::<u8>().add(132).cast() }
    }
    #[doc = "0x84 - Channel Mode Register (channel = 2)"]
    #[inline(always)]
    pub const fn cmr2(&self) -> &Cmr2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(132).cast() }
    }
    #[doc = "0x90 - Counter Value (channel = 2)"]
    #[inline(always)]
    pub const fn cv2(&self) -> &Cv2 {
        &self.cv2
    }
    #[doc = "0x94 - Register A (channel = 2)"]
    #[inline(always)]
    pub const fn ra2(&self) -> &Ra2 {
        &self.ra2
    }
    #[doc = "0x98 - Register B (channel = 2)"]
    #[inline(always)]
    pub const fn rb2(&self) -> &Rb2 {
        &self.rb2
    }
    #[doc = "0x9c - Register C (channel = 2)"]
    #[inline(always)]
    pub const fn rc2(&self) -> &Rc2 {
        &self.rc2
    }
    #[doc = "0xa0 - Status Register (channel = 2)"]
    #[inline(always)]
    pub const fn sr2(&self) -> &Sr2 {
        &self.sr2
    }
    #[doc = "0xa4 - Interrupt Enable Register (channel = 2)"]
    #[inline(always)]
    pub const fn ier2(&self) -> &Ier2 {
        &self.ier2
    }
    #[doc = "0xa8 - Interrupt Disable Register (channel = 2)"]
    #[inline(always)]
    pub const fn idr2(&self) -> &Idr2 {
        &self.idr2
    }
    #[doc = "0xac - Interrupt Mask Register (channel = 2)"]
    #[inline(always)]
    pub const fn imr2(&self) -> &Imr2 {
        &self.imr2
    }
    #[doc = "0xc0 - Block Control Register"]
    #[inline(always)]
    pub const fn bcr(&self) -> &Bcr {
        &self.bcr
    }
    #[doc = "0xc4 - Block Mode Register"]
    #[inline(always)]
    pub const fn bmr(&self) -> &Bmr {
        &self.bmr
    }
    #[doc = "0xc8 - QDEC Interrupt Enable Register"]
    #[inline(always)]
    pub const fn qier(&self) -> &Qier {
        &self.qier
    }
    #[doc = "0xcc - QDEC Interrupt Disable Register"]
    #[inline(always)]
    pub const fn qidr(&self) -> &Qidr {
        &self.qidr
    }
    #[doc = "0xd0 - QDEC Interrupt Mask Register"]
    #[inline(always)]
    pub const fn qimr(&self) -> &Qimr {
        &self.qimr
    }
    #[doc = "0xd4 - QDEC Interrupt Status Register"]
    #[inline(always)]
    pub const fn qisr(&self) -> &Qisr {
        &self.qisr
    }
}
#[doc = "CCR0 (w) register accessor: Channel Control Register (channel = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr0`]
module"]
#[doc(alias = "CCR0")]
pub type Ccr0 = crate::Reg<ccr0::Ccr0Spec>;
#[doc = "Channel Control Register (channel = 0)"]
pub mod ccr0;
#[doc = "CMR0 (rw) register accessor: Channel Mode Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr0`]
module"]
#[doc(alias = "CMR0")]
pub type Cmr0 = crate::Reg<cmr0::Cmr0Spec>;
#[doc = "Channel Mode Register (channel = 0)"]
pub mod cmr0;
#[doc = "WAVEFORM_MODE_CMR0_WAVEFORM_MODE (rw) register accessor: Channel Mode Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`waveform_mode_cmr0_waveform_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`waveform_mode_cmr0_waveform_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@waveform_mode_cmr0_waveform_mode`]
module"]
#[doc(alias = "WAVEFORM_MODE_CMR0_WAVEFORM_MODE")]
pub type WaveformModeCmr0WaveformMode =
    crate::Reg<waveform_mode_cmr0_waveform_mode::WaveformModeCmr0WaveformModeSpec>;
#[doc = "Channel Mode Register (channel = 0)"]
pub mod waveform_mode_cmr0_waveform_mode;
#[doc = "CV0 (r) register accessor: Counter Value (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cv0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cv0`]
module"]
#[doc(alias = "CV0")]
pub type Cv0 = crate::Reg<cv0::Cv0Spec>;
#[doc = "Counter Value (channel = 0)"]
pub mod cv0;
#[doc = "RA0 (rw) register accessor: Register A (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ra0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ra0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ra0`]
module"]
#[doc(alias = "RA0")]
pub type Ra0 = crate::Reg<ra0::Ra0Spec>;
#[doc = "Register A (channel = 0)"]
pub mod ra0;
#[doc = "RB0 (rw) register accessor: Register B (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rb0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rb0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rb0`]
module"]
#[doc(alias = "RB0")]
pub type Rb0 = crate::Reg<rb0::Rb0Spec>;
#[doc = "Register B (channel = 0)"]
pub mod rb0;
#[doc = "RC0 (rw) register accessor: Register C (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc0`]
module"]
#[doc(alias = "RC0")]
pub type Rc0 = crate::Reg<rc0::Rc0Spec>;
#[doc = "Register C (channel = 0)"]
pub mod rc0;
#[doc = "SR0 (r) register accessor: Status Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr0`]
module"]
#[doc(alias = "SR0")]
pub type Sr0 = crate::Reg<sr0::Sr0Spec>;
#[doc = "Status Register (channel = 0)"]
pub mod sr0;
#[doc = "IER0 (w) register accessor: Interrupt Enable Register (channel = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier0`]
module"]
#[doc(alias = "IER0")]
pub type Ier0 = crate::Reg<ier0::Ier0Spec>;
#[doc = "Interrupt Enable Register (channel = 0)"]
pub mod ier0;
#[doc = "IDR0 (w) register accessor: Interrupt Disable Register (channel = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr0`]
module"]
#[doc(alias = "IDR0")]
pub type Idr0 = crate::Reg<idr0::Idr0Spec>;
#[doc = "Interrupt Disable Register (channel = 0)"]
pub mod idr0;
#[doc = "IMR0 (r) register accessor: Interrupt Mask Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr0`]
module"]
#[doc(alias = "IMR0")]
pub type Imr0 = crate::Reg<imr0::Imr0Spec>;
#[doc = "Interrupt Mask Register (channel = 0)"]
pub mod imr0;
#[doc = "CCR1 (w) register accessor: Channel Control Register (channel = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`]
module"]
#[doc(alias = "CCR1")]
pub type Ccr1 = crate::Reg<ccr1::Ccr1Spec>;
#[doc = "Channel Control Register (channel = 1)"]
pub mod ccr1;
#[doc = "CMR1 (rw) register accessor: Channel Mode Register (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr1`]
module"]
#[doc(alias = "CMR1")]
pub type Cmr1 = crate::Reg<cmr1::Cmr1Spec>;
#[doc = "Channel Mode Register (channel = 1)"]
pub mod cmr1;
#[doc = "WAVEFORM_MODE_CMR1_WAVEFORM_MODE (rw) register accessor: Channel Mode Register (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`waveform_mode_cmr1_waveform_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`waveform_mode_cmr1_waveform_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@waveform_mode_cmr1_waveform_mode`]
module"]
#[doc(alias = "WAVEFORM_MODE_CMR1_WAVEFORM_MODE")]
pub type WaveformModeCmr1WaveformMode =
    crate::Reg<waveform_mode_cmr1_waveform_mode::WaveformModeCmr1WaveformModeSpec>;
#[doc = "Channel Mode Register (channel = 1)"]
pub mod waveform_mode_cmr1_waveform_mode;
#[doc = "CV1 (r) register accessor: Counter Value (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cv1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cv1`]
module"]
#[doc(alias = "CV1")]
pub type Cv1 = crate::Reg<cv1::Cv1Spec>;
#[doc = "Counter Value (channel = 1)"]
pub mod cv1;
#[doc = "RA1 (rw) register accessor: Register A (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ra1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ra1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ra1`]
module"]
#[doc(alias = "RA1")]
pub type Ra1 = crate::Reg<ra1::Ra1Spec>;
#[doc = "Register A (channel = 1)"]
pub mod ra1;
#[doc = "RB1 (rw) register accessor: Register B (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rb1`]
module"]
#[doc(alias = "RB1")]
pub type Rb1 = crate::Reg<rb1::Rb1Spec>;
#[doc = "Register B (channel = 1)"]
pub mod rb1;
#[doc = "RC1 (rw) register accessor: Register C (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc1`]
module"]
#[doc(alias = "RC1")]
pub type Rc1 = crate::Reg<rc1::Rc1Spec>;
#[doc = "Register C (channel = 1)"]
pub mod rc1;
#[doc = "SR1 (r) register accessor: Status Register (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr1`]
module"]
#[doc(alias = "SR1")]
pub type Sr1 = crate::Reg<sr1::Sr1Spec>;
#[doc = "Status Register (channel = 1)"]
pub mod sr1;
#[doc = "IER1 (w) register accessor: Interrupt Enable Register (channel = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier1`]
module"]
#[doc(alias = "IER1")]
pub type Ier1 = crate::Reg<ier1::Ier1Spec>;
#[doc = "Interrupt Enable Register (channel = 1)"]
pub mod ier1;
#[doc = "IDR1 (w) register accessor: Interrupt Disable Register (channel = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr1`]
module"]
#[doc(alias = "IDR1")]
pub type Idr1 = crate::Reg<idr1::Idr1Spec>;
#[doc = "Interrupt Disable Register (channel = 1)"]
pub mod idr1;
#[doc = "IMR1 (r) register accessor: Interrupt Mask Register (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr1`]
module"]
#[doc(alias = "IMR1")]
pub type Imr1 = crate::Reg<imr1::Imr1Spec>;
#[doc = "Interrupt Mask Register (channel = 1)"]
pub mod imr1;
#[doc = "CCR2 (w) register accessor: Channel Control Register (channel = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2`]
module"]
#[doc(alias = "CCR2")]
pub type Ccr2 = crate::Reg<ccr2::Ccr2Spec>;
#[doc = "Channel Control Register (channel = 2)"]
pub mod ccr2;
#[doc = "CMR2 (rw) register accessor: Channel Mode Register (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr2`]
module"]
#[doc(alias = "CMR2")]
pub type Cmr2 = crate::Reg<cmr2::Cmr2Spec>;
#[doc = "Channel Mode Register (channel = 2)"]
pub mod cmr2;
#[doc = "WAVEFORM_MODE_CMR2_WAVEFORM_MODE (rw) register accessor: Channel Mode Register (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`waveform_mode_cmr2_waveform_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`waveform_mode_cmr2_waveform_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@waveform_mode_cmr2_waveform_mode`]
module"]
#[doc(alias = "WAVEFORM_MODE_CMR2_WAVEFORM_MODE")]
pub type WaveformModeCmr2WaveformMode =
    crate::Reg<waveform_mode_cmr2_waveform_mode::WaveformModeCmr2WaveformModeSpec>;
#[doc = "Channel Mode Register (channel = 2)"]
pub mod waveform_mode_cmr2_waveform_mode;
#[doc = "CV2 (r) register accessor: Counter Value (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cv2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cv2`]
module"]
#[doc(alias = "CV2")]
pub type Cv2 = crate::Reg<cv2::Cv2Spec>;
#[doc = "Counter Value (channel = 2)"]
pub mod cv2;
#[doc = "RA2 (rw) register accessor: Register A (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ra2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ra2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ra2`]
module"]
#[doc(alias = "RA2")]
pub type Ra2 = crate::Reg<ra2::Ra2Spec>;
#[doc = "Register A (channel = 2)"]
pub mod ra2;
#[doc = "RB2 (rw) register accessor: Register B (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rb2`]
module"]
#[doc(alias = "RB2")]
pub type Rb2 = crate::Reg<rb2::Rb2Spec>;
#[doc = "Register B (channel = 2)"]
pub mod rb2;
#[doc = "RC2 (rw) register accessor: Register C (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc2`]
module"]
#[doc(alias = "RC2")]
pub type Rc2 = crate::Reg<rc2::Rc2Spec>;
#[doc = "Register C (channel = 2)"]
pub mod rc2;
#[doc = "SR2 (r) register accessor: Status Register (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr2`]
module"]
#[doc(alias = "SR2")]
pub type Sr2 = crate::Reg<sr2::Sr2Spec>;
#[doc = "Status Register (channel = 2)"]
pub mod sr2;
#[doc = "IER2 (w) register accessor: Interrupt Enable Register (channel = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier2`]
module"]
#[doc(alias = "IER2")]
pub type Ier2 = crate::Reg<ier2::Ier2Spec>;
#[doc = "Interrupt Enable Register (channel = 2)"]
pub mod ier2;
#[doc = "IDR2 (w) register accessor: Interrupt Disable Register (channel = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr2`]
module"]
#[doc(alias = "IDR2")]
pub type Idr2 = crate::Reg<idr2::Idr2Spec>;
#[doc = "Interrupt Disable Register (channel = 2)"]
pub mod idr2;
#[doc = "IMR2 (r) register accessor: Interrupt Mask Register (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr2`]
module"]
#[doc(alias = "IMR2")]
pub type Imr2 = crate::Reg<imr2::Imr2Spec>;
#[doc = "Interrupt Mask Register (channel = 2)"]
pub mod imr2;
#[doc = "BCR (w) register accessor: Block Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr`]
module"]
#[doc(alias = "BCR")]
pub type Bcr = crate::Reg<bcr::BcrSpec>;
#[doc = "Block Control Register"]
pub mod bcr;
#[doc = "BMR (rw) register accessor: Block Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmr`]
module"]
#[doc(alias = "BMR")]
pub type Bmr = crate::Reg<bmr::BmrSpec>;
#[doc = "Block Mode Register"]
pub mod bmr;
#[doc = "QIER (w) register accessor: QDEC Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qier`]
module"]
#[doc(alias = "QIER")]
pub type Qier = crate::Reg<qier::QierSpec>;
#[doc = "QDEC Interrupt Enable Register"]
pub mod qier;
#[doc = "QIDR (w) register accessor: QDEC Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qidr`]
module"]
#[doc(alias = "QIDR")]
pub type Qidr = crate::Reg<qidr::QidrSpec>;
#[doc = "QDEC Interrupt Disable Register"]
pub mod qidr;
#[doc = "QIMR (r) register accessor: QDEC Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qimr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qimr`]
module"]
#[doc(alias = "QIMR")]
pub type Qimr = crate::Reg<qimr::QimrSpec>;
#[doc = "QDEC Interrupt Mask Register"]
pub mod qimr;
#[doc = "QISR (r) register accessor: QDEC Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qisr`]
module"]
#[doc(alias = "QISR")]
pub type Qisr = crate::Reg<qisr::QisrSpec>;
#[doc = "QDEC Interrupt Status Register"]
pub mod qisr;
