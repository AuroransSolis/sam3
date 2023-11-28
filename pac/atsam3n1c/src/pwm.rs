#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    mr: MR,
    ena: ENA,
    dis: DIS,
    sr: SR,
    ier: IER,
    idr: IDR,
    imr: IMR,
    isr: ISR,
    _reserved8: [u8; 0x01e0],
    cmr0: CMR0,
    cdty0: CDTY0,
    cprd0: CPRD0,
    ccnt0: CCNT0,
    cupd0: CUPD0,
    _reserved13: [u8; 0x0c],
    cmr1: CMR1,
    cdty1: CDTY1,
    cprd1: CPRD1,
    ccnt1: CCNT1,
    cupd1: CUPD1,
    _reserved18: [u8; 0x0c],
    cmr2: CMR2,
    cdty2: CDTY2,
    cprd2: CPRD2,
    ccnt2: CCNT2,
    cupd2: CUPD2,
    _reserved23: [u8; 0x0c],
    cmr3: CMR3,
    cdty3: CDTY3,
    cprd3: CPRD3,
    ccnt3: CCNT3,
    cupd3: CUPD3,
}
impl RegisterBlock {
    #[doc = "0x00 - PWM Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &MR {
        &self.mr
    }
    #[doc = "0x04 - PWM Enable Register"]
    #[inline(always)]
    pub const fn ena(&self) -> &ENA {
        &self.ena
    }
    #[doc = "0x08 - PWM Disable Register"]
    #[inline(always)]
    pub const fn dis(&self) -> &DIS {
        &self.dis
    }
    #[doc = "0x0c - PWM Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x10 - PWM Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x14 - PWM Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    #[doc = "0x18 - PWM Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    #[doc = "0x1c - PWM Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x200 - PWM Channel Mode Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn cmr0(&self) -> &CMR0 {
        &self.cmr0
    }
    #[doc = "0x204 - PWM Channel Duty Cycle Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn cdty0(&self) -> &CDTY0 {
        &self.cdty0
    }
    #[doc = "0x208 - PWM Channel Period Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn cprd0(&self) -> &CPRD0 {
        &self.cprd0
    }
    #[doc = "0x20c - PWM Channel Counter Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn ccnt0(&self) -> &CCNT0 {
        &self.ccnt0
    }
    #[doc = "0x210 - PWM Channel Update Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn cupd0(&self) -> &CUPD0 {
        &self.cupd0
    }
    #[doc = "0x220 - PWM Channel Mode Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn cmr1(&self) -> &CMR1 {
        &self.cmr1
    }
    #[doc = "0x224 - PWM Channel Duty Cycle Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn cdty1(&self) -> &CDTY1 {
        &self.cdty1
    }
    #[doc = "0x228 - PWM Channel Period Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn cprd1(&self) -> &CPRD1 {
        &self.cprd1
    }
    #[doc = "0x22c - PWM Channel Counter Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn ccnt1(&self) -> &CCNT1 {
        &self.ccnt1
    }
    #[doc = "0x230 - PWM Channel Update Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn cupd1(&self) -> &CUPD1 {
        &self.cupd1
    }
    #[doc = "0x240 - PWM Channel Mode Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn cmr2(&self) -> &CMR2 {
        &self.cmr2
    }
    #[doc = "0x244 - PWM Channel Duty Cycle Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn cdty2(&self) -> &CDTY2 {
        &self.cdty2
    }
    #[doc = "0x248 - PWM Channel Period Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn cprd2(&self) -> &CPRD2 {
        &self.cprd2
    }
    #[doc = "0x24c - PWM Channel Counter Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn ccnt2(&self) -> &CCNT2 {
        &self.ccnt2
    }
    #[doc = "0x250 - PWM Channel Update Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn cupd2(&self) -> &CUPD2 {
        &self.cupd2
    }
    #[doc = "0x260 - PWM Channel Mode Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn cmr3(&self) -> &CMR3 {
        &self.cmr3
    }
    #[doc = "0x264 - PWM Channel Duty Cycle Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn cdty3(&self) -> &CDTY3 {
        &self.cdty3
    }
    #[doc = "0x268 - PWM Channel Period Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn cprd3(&self) -> &CPRD3 {
        &self.cprd3
    }
    #[doc = "0x26c - PWM Channel Counter Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn ccnt3(&self) -> &CCNT3 {
        &self.ccnt3
    }
    #[doc = "0x270 - PWM Channel Update Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn cupd3(&self) -> &CUPD3 {
        &self.cupd3
    }
}
#[doc = "MR (rw) register accessor: PWM Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`]
module"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "PWM Mode Register"]
pub mod mr;
#[doc = "ENA (w) register accessor: PWM Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ena::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ena`]
module"]
pub type ENA = crate::Reg<ena::ENA_SPEC>;
#[doc = "PWM Enable Register"]
pub mod ena;
#[doc = "DIS (w) register accessor: PWM Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dis::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dis`]
module"]
pub type DIS = crate::Reg<dis::DIS_SPEC>;
#[doc = "PWM Disable Register"]
pub mod dis;
#[doc = "SR (r) register accessor: PWM Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "PWM Status Register"]
pub mod sr;
#[doc = "IER (w) register accessor: PWM Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "PWM Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: PWM Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "PWM Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: PWM Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "PWM Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: PWM Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "PWM Interrupt Status Register"]
pub mod isr;
#[doc = "CMR0 (rw) register accessor: PWM Channel Mode Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr0`]
module"]
pub type CMR0 = crate::Reg<cmr0::CMR0_SPEC>;
#[doc = "PWM Channel Mode Register (ch_num = 0)"]
pub mod cmr0;
#[doc = "CDTY0 (rw) register accessor: PWM Channel Duty Cycle Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdty0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdty0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdty0`]
module"]
pub type CDTY0 = crate::Reg<cdty0::CDTY0_SPEC>;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 0)"]
pub mod cdty0;
#[doc = "CPRD0 (rw) register accessor: PWM Channel Period Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cprd0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprd0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cprd0`]
module"]
pub type CPRD0 = crate::Reg<cprd0::CPRD0_SPEC>;
#[doc = "PWM Channel Period Register (ch_num = 0)"]
pub mod cprd0;
#[doc = "CCNT0 (r) register accessor: PWM Channel Counter Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccnt0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccnt0`]
module"]
pub type CCNT0 = crate::Reg<ccnt0::CCNT0_SPEC>;
#[doc = "PWM Channel Counter Register (ch_num = 0)"]
pub mod ccnt0;
#[doc = "CUPD0 (w) register accessor: PWM Channel Update Register (ch_num = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cupd0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cupd0`]
module"]
pub type CUPD0 = crate::Reg<cupd0::CUPD0_SPEC>;
#[doc = "PWM Channel Update Register (ch_num = 0)"]
pub mod cupd0;
#[doc = "CMR1 (rw) register accessor: PWM Channel Mode Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr1`]
module"]
pub type CMR1 = crate::Reg<cmr1::CMR1_SPEC>;
#[doc = "PWM Channel Mode Register (ch_num = 1)"]
pub mod cmr1;
#[doc = "CDTY1 (rw) register accessor: PWM Channel Duty Cycle Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdty1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdty1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdty1`]
module"]
pub type CDTY1 = crate::Reg<cdty1::CDTY1_SPEC>;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 1)"]
pub mod cdty1;
#[doc = "CPRD1 (rw) register accessor: PWM Channel Period Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cprd1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprd1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cprd1`]
module"]
pub type CPRD1 = crate::Reg<cprd1::CPRD1_SPEC>;
#[doc = "PWM Channel Period Register (ch_num = 1)"]
pub mod cprd1;
#[doc = "CCNT1 (r) register accessor: PWM Channel Counter Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccnt1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccnt1`]
module"]
pub type CCNT1 = crate::Reg<ccnt1::CCNT1_SPEC>;
#[doc = "PWM Channel Counter Register (ch_num = 1)"]
pub mod ccnt1;
#[doc = "CUPD1 (w) register accessor: PWM Channel Update Register (ch_num = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cupd1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cupd1`]
module"]
pub type CUPD1 = crate::Reg<cupd1::CUPD1_SPEC>;
#[doc = "PWM Channel Update Register (ch_num = 1)"]
pub mod cupd1;
#[doc = "CMR2 (rw) register accessor: PWM Channel Mode Register (ch_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr2`]
module"]
pub type CMR2 = crate::Reg<cmr2::CMR2_SPEC>;
#[doc = "PWM Channel Mode Register (ch_num = 2)"]
pub mod cmr2;
#[doc = "CDTY2 (rw) register accessor: PWM Channel Duty Cycle Register (ch_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdty2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdty2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdty2`]
module"]
pub type CDTY2 = crate::Reg<cdty2::CDTY2_SPEC>;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 2)"]
pub mod cdty2;
#[doc = "CPRD2 (rw) register accessor: PWM Channel Period Register (ch_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cprd2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprd2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cprd2`]
module"]
pub type CPRD2 = crate::Reg<cprd2::CPRD2_SPEC>;
#[doc = "PWM Channel Period Register (ch_num = 2)"]
pub mod cprd2;
#[doc = "CCNT2 (r) register accessor: PWM Channel Counter Register (ch_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccnt2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccnt2`]
module"]
pub type CCNT2 = crate::Reg<ccnt2::CCNT2_SPEC>;
#[doc = "PWM Channel Counter Register (ch_num = 2)"]
pub mod ccnt2;
#[doc = "CUPD2 (w) register accessor: PWM Channel Update Register (ch_num = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cupd2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cupd2`]
module"]
pub type CUPD2 = crate::Reg<cupd2::CUPD2_SPEC>;
#[doc = "PWM Channel Update Register (ch_num = 2)"]
pub mod cupd2;
#[doc = "CMR3 (rw) register accessor: PWM Channel Mode Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr3`]
module"]
pub type CMR3 = crate::Reg<cmr3::CMR3_SPEC>;
#[doc = "PWM Channel Mode Register (ch_num = 3)"]
pub mod cmr3;
#[doc = "CDTY3 (rw) register accessor: PWM Channel Duty Cycle Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdty3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdty3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdty3`]
module"]
pub type CDTY3 = crate::Reg<cdty3::CDTY3_SPEC>;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 3)"]
pub mod cdty3;
#[doc = "CPRD3 (rw) register accessor: PWM Channel Period Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cprd3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprd3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cprd3`]
module"]
pub type CPRD3 = crate::Reg<cprd3::CPRD3_SPEC>;
#[doc = "PWM Channel Period Register (ch_num = 3)"]
pub mod cprd3;
#[doc = "CCNT3 (r) register accessor: PWM Channel Counter Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccnt3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccnt3`]
module"]
pub type CCNT3 = crate::Reg<ccnt3::CCNT3_SPEC>;
#[doc = "PWM Channel Counter Register (ch_num = 3)"]
pub mod ccnt3;
#[doc = "CUPD3 (w) register accessor: PWM Channel Update Register (ch_num = 3)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cupd3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cupd3`]
module"]
pub type CUPD3 = crate::Reg<cupd3::CUPD3_SPEC>;
#[doc = "PWM Channel Update Register (ch_num = 3)"]
pub mod cupd3;
