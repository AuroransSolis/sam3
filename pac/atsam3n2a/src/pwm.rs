#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mr: Mr,
    ena: Ena,
    dis: Dis,
    sr: Sr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    isr: Isr,
    _reserved8: [u8; 0x01e0],
    cmr0: Cmr0,
    cdty0: Cdty0,
    cprd0: Cprd0,
    ccnt0: Ccnt0,
    cupd0: Cupd0,
    _reserved13: [u8; 0x0c],
    cmr1: Cmr1,
    cdty1: Cdty1,
    cprd1: Cprd1,
    ccnt1: Ccnt1,
    cupd1: Cupd1,
    _reserved18: [u8; 0x0c],
    cmr2: Cmr2,
    cdty2: Cdty2,
    cprd2: Cprd2,
    ccnt2: Ccnt2,
    cupd2: Cupd2,
    _reserved23: [u8; 0x0c],
    cmr3: Cmr3,
    cdty3: Cdty3,
    cprd3: Cprd3,
    ccnt3: Ccnt3,
    cupd3: Cupd3,
}
impl RegisterBlock {
    #[doc = "0x00 - PWM Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &Mr {
        &self.mr
    }
    #[doc = "0x04 - PWM Enable Register"]
    #[inline(always)]
    pub const fn ena(&self) -> &Ena {
        &self.ena
    }
    #[doc = "0x08 - PWM Disable Register"]
    #[inline(always)]
    pub const fn dis(&self) -> &Dis {
        &self.dis
    }
    #[doc = "0x0c - PWM Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x10 - PWM Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x14 - PWM Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x18 - PWM Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x1c - PWM Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x200 - PWM Channel Mode Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn cmr0(&self) -> &Cmr0 {
        &self.cmr0
    }
    #[doc = "0x204 - PWM Channel Duty Cycle Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn cdty0(&self) -> &Cdty0 {
        &self.cdty0
    }
    #[doc = "0x208 - PWM Channel Period Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn cprd0(&self) -> &Cprd0 {
        &self.cprd0
    }
    #[doc = "0x20c - PWM Channel Counter Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn ccnt0(&self) -> &Ccnt0 {
        &self.ccnt0
    }
    #[doc = "0x210 - PWM Channel Update Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn cupd0(&self) -> &Cupd0 {
        &self.cupd0
    }
    #[doc = "0x220 - PWM Channel Mode Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn cmr1(&self) -> &Cmr1 {
        &self.cmr1
    }
    #[doc = "0x224 - PWM Channel Duty Cycle Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn cdty1(&self) -> &Cdty1 {
        &self.cdty1
    }
    #[doc = "0x228 - PWM Channel Period Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn cprd1(&self) -> &Cprd1 {
        &self.cprd1
    }
    #[doc = "0x22c - PWM Channel Counter Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn ccnt1(&self) -> &Ccnt1 {
        &self.ccnt1
    }
    #[doc = "0x230 - PWM Channel Update Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn cupd1(&self) -> &Cupd1 {
        &self.cupd1
    }
    #[doc = "0x240 - PWM Channel Mode Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn cmr2(&self) -> &Cmr2 {
        &self.cmr2
    }
    #[doc = "0x244 - PWM Channel Duty Cycle Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn cdty2(&self) -> &Cdty2 {
        &self.cdty2
    }
    #[doc = "0x248 - PWM Channel Period Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn cprd2(&self) -> &Cprd2 {
        &self.cprd2
    }
    #[doc = "0x24c - PWM Channel Counter Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn ccnt2(&self) -> &Ccnt2 {
        &self.ccnt2
    }
    #[doc = "0x250 - PWM Channel Update Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn cupd2(&self) -> &Cupd2 {
        &self.cupd2
    }
    #[doc = "0x260 - PWM Channel Mode Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn cmr3(&self) -> &Cmr3 {
        &self.cmr3
    }
    #[doc = "0x264 - PWM Channel Duty Cycle Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn cdty3(&self) -> &Cdty3 {
        &self.cdty3
    }
    #[doc = "0x268 - PWM Channel Period Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn cprd3(&self) -> &Cprd3 {
        &self.cprd3
    }
    #[doc = "0x26c - PWM Channel Counter Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn ccnt3(&self) -> &Ccnt3 {
        &self.ccnt3
    }
    #[doc = "0x270 - PWM Channel Update Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn cupd3(&self) -> &Cupd3 {
        &self.cupd3
    }
}
#[doc = "MR (rw) register accessor: PWM Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`]
module"]
#[doc(alias = "MR")]
pub type Mr = crate::Reg<mr::MrSpec>;
#[doc = "PWM Mode Register"]
pub mod mr;
#[doc = "ENA (w) register accessor: PWM Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ena::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ena`]
module"]
#[doc(alias = "ENA")]
pub type Ena = crate::Reg<ena::EnaSpec>;
#[doc = "PWM Enable Register"]
pub mod ena;
#[doc = "DIS (w) register accessor: PWM Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dis::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dis`]
module"]
#[doc(alias = "DIS")]
pub type Dis = crate::Reg<dis::DisSpec>;
#[doc = "PWM Disable Register"]
pub mod dis;
#[doc = "SR (r) register accessor: PWM Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "PWM Status Register"]
pub mod sr;
#[doc = "IER (w) register accessor: PWM Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "PWM Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: PWM Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "PWM Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: PWM Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "PWM Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: PWM Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "PWM Interrupt Status Register"]
pub mod isr;
#[doc = "CMR0 (rw) register accessor: PWM Channel Mode Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr0`]
module"]
#[doc(alias = "CMR0")]
pub type Cmr0 = crate::Reg<cmr0::Cmr0Spec>;
#[doc = "PWM Channel Mode Register (ch_num = 0)"]
pub mod cmr0;
#[doc = "CDTY0 (rw) register accessor: PWM Channel Duty Cycle Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdty0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdty0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdty0`]
module"]
#[doc(alias = "CDTY0")]
pub type Cdty0 = crate::Reg<cdty0::Cdty0Spec>;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 0)"]
pub mod cdty0;
#[doc = "CPRD0 (rw) register accessor: PWM Channel Period Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cprd0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprd0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cprd0`]
module"]
#[doc(alias = "CPRD0")]
pub type Cprd0 = crate::Reg<cprd0::Cprd0Spec>;
#[doc = "PWM Channel Period Register (ch_num = 0)"]
pub mod cprd0;
#[doc = "CCNT0 (r) register accessor: PWM Channel Counter Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccnt0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccnt0`]
module"]
#[doc(alias = "CCNT0")]
pub type Ccnt0 = crate::Reg<ccnt0::Ccnt0Spec>;
#[doc = "PWM Channel Counter Register (ch_num = 0)"]
pub mod ccnt0;
#[doc = "CUPD0 (w) register accessor: PWM Channel Update Register (ch_num = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cupd0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cupd0`]
module"]
#[doc(alias = "CUPD0")]
pub type Cupd0 = crate::Reg<cupd0::Cupd0Spec>;
#[doc = "PWM Channel Update Register (ch_num = 0)"]
pub mod cupd0;
#[doc = "CMR1 (rw) register accessor: PWM Channel Mode Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr1`]
module"]
#[doc(alias = "CMR1")]
pub type Cmr1 = crate::Reg<cmr1::Cmr1Spec>;
#[doc = "PWM Channel Mode Register (ch_num = 1)"]
pub mod cmr1;
#[doc = "CDTY1 (rw) register accessor: PWM Channel Duty Cycle Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdty1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdty1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdty1`]
module"]
#[doc(alias = "CDTY1")]
pub type Cdty1 = crate::Reg<cdty1::Cdty1Spec>;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 1)"]
pub mod cdty1;
#[doc = "CPRD1 (rw) register accessor: PWM Channel Period Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cprd1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprd1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cprd1`]
module"]
#[doc(alias = "CPRD1")]
pub type Cprd1 = crate::Reg<cprd1::Cprd1Spec>;
#[doc = "PWM Channel Period Register (ch_num = 1)"]
pub mod cprd1;
#[doc = "CCNT1 (r) register accessor: PWM Channel Counter Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccnt1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccnt1`]
module"]
#[doc(alias = "CCNT1")]
pub type Ccnt1 = crate::Reg<ccnt1::Ccnt1Spec>;
#[doc = "PWM Channel Counter Register (ch_num = 1)"]
pub mod ccnt1;
#[doc = "CUPD1 (w) register accessor: PWM Channel Update Register (ch_num = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cupd1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cupd1`]
module"]
#[doc(alias = "CUPD1")]
pub type Cupd1 = crate::Reg<cupd1::Cupd1Spec>;
#[doc = "PWM Channel Update Register (ch_num = 1)"]
pub mod cupd1;
#[doc = "CMR2 (rw) register accessor: PWM Channel Mode Register (ch_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr2`]
module"]
#[doc(alias = "CMR2")]
pub type Cmr2 = crate::Reg<cmr2::Cmr2Spec>;
#[doc = "PWM Channel Mode Register (ch_num = 2)"]
pub mod cmr2;
#[doc = "CDTY2 (rw) register accessor: PWM Channel Duty Cycle Register (ch_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdty2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdty2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdty2`]
module"]
#[doc(alias = "CDTY2")]
pub type Cdty2 = crate::Reg<cdty2::Cdty2Spec>;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 2)"]
pub mod cdty2;
#[doc = "CPRD2 (rw) register accessor: PWM Channel Period Register (ch_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cprd2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprd2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cprd2`]
module"]
#[doc(alias = "CPRD2")]
pub type Cprd2 = crate::Reg<cprd2::Cprd2Spec>;
#[doc = "PWM Channel Period Register (ch_num = 2)"]
pub mod cprd2;
#[doc = "CCNT2 (r) register accessor: PWM Channel Counter Register (ch_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccnt2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccnt2`]
module"]
#[doc(alias = "CCNT2")]
pub type Ccnt2 = crate::Reg<ccnt2::Ccnt2Spec>;
#[doc = "PWM Channel Counter Register (ch_num = 2)"]
pub mod ccnt2;
#[doc = "CUPD2 (w) register accessor: PWM Channel Update Register (ch_num = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cupd2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cupd2`]
module"]
#[doc(alias = "CUPD2")]
pub type Cupd2 = crate::Reg<cupd2::Cupd2Spec>;
#[doc = "PWM Channel Update Register (ch_num = 2)"]
pub mod cupd2;
#[doc = "CMR3 (rw) register accessor: PWM Channel Mode Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr3`]
module"]
#[doc(alias = "CMR3")]
pub type Cmr3 = crate::Reg<cmr3::Cmr3Spec>;
#[doc = "PWM Channel Mode Register (ch_num = 3)"]
pub mod cmr3;
#[doc = "CDTY3 (rw) register accessor: PWM Channel Duty Cycle Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdty3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdty3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdty3`]
module"]
#[doc(alias = "CDTY3")]
pub type Cdty3 = crate::Reg<cdty3::Cdty3Spec>;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 3)"]
pub mod cdty3;
#[doc = "CPRD3 (rw) register accessor: PWM Channel Period Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cprd3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprd3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cprd3`]
module"]
#[doc(alias = "CPRD3")]
pub type Cprd3 = crate::Reg<cprd3::Cprd3Spec>;
#[doc = "PWM Channel Period Register (ch_num = 3)"]
pub mod cprd3;
#[doc = "CCNT3 (r) register accessor: PWM Channel Counter Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccnt3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccnt3`]
module"]
#[doc(alias = "CCNT3")]
pub type Ccnt3 = crate::Reg<ccnt3::Ccnt3Spec>;
#[doc = "PWM Channel Counter Register (ch_num = 3)"]
pub mod ccnt3;
#[doc = "CUPD3 (w) register accessor: PWM Channel Update Register (ch_num = 3)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cupd3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cupd3`]
module"]
#[doc(alias = "CUPD3")]
pub type Cupd3 = crate::Reg<cupd3::Cupd3Spec>;
#[doc = "PWM Channel Update Register (ch_num = 3)"]
pub mod cupd3;
