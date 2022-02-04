#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Mode Register"]
    pub mr: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x04 - PWM Enable Register"]
    pub ena: crate::Reg<ena::ENA_SPEC>,
    #[doc = "0x08 - PWM Disable Register"]
    pub dis: crate::Reg<dis::DIS_SPEC>,
    #[doc = "0x0c - PWM Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x10 - PWM Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x14 - PWM Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x18 - PWM Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x1c - PWM Interrupt Status Register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    _reserved8: [u8; 0x01e0],
    #[doc = "0x200 - PWM Channel Mode Register (ch_num = 0)"]
    pub cmr0: crate::Reg<cmr0::CMR0_SPEC>,
    #[doc = "0x204 - PWM Channel Duty Cycle Register (ch_num = 0)"]
    pub cdty0: crate::Reg<cdty0::CDTY0_SPEC>,
    #[doc = "0x208 - PWM Channel Period Register (ch_num = 0)"]
    pub cprd0: crate::Reg<cprd0::CPRD0_SPEC>,
    #[doc = "0x20c - PWM Channel Counter Register (ch_num = 0)"]
    pub ccnt0: crate::Reg<ccnt0::CCNT0_SPEC>,
    #[doc = "0x210 - PWM Channel Update Register (ch_num = 0)"]
    pub cupd0: crate::Reg<cupd0::CUPD0_SPEC>,
    _reserved13: [u8; 0x0c],
    #[doc = "0x220 - PWM Channel Mode Register (ch_num = 1)"]
    pub cmr1: crate::Reg<cmr1::CMR1_SPEC>,
    #[doc = "0x224 - PWM Channel Duty Cycle Register (ch_num = 1)"]
    pub cdty1: crate::Reg<cdty1::CDTY1_SPEC>,
    #[doc = "0x228 - PWM Channel Period Register (ch_num = 1)"]
    pub cprd1: crate::Reg<cprd1::CPRD1_SPEC>,
    #[doc = "0x22c - PWM Channel Counter Register (ch_num = 1)"]
    pub ccnt1: crate::Reg<ccnt1::CCNT1_SPEC>,
    #[doc = "0x230 - PWM Channel Update Register (ch_num = 1)"]
    pub cupd1: crate::Reg<cupd1::CUPD1_SPEC>,
    _reserved18: [u8; 0x0c],
    #[doc = "0x240 - PWM Channel Mode Register (ch_num = 2)"]
    pub cmr2: crate::Reg<cmr2::CMR2_SPEC>,
    #[doc = "0x244 - PWM Channel Duty Cycle Register (ch_num = 2)"]
    pub cdty2: crate::Reg<cdty2::CDTY2_SPEC>,
    #[doc = "0x248 - PWM Channel Period Register (ch_num = 2)"]
    pub cprd2: crate::Reg<cprd2::CPRD2_SPEC>,
    #[doc = "0x24c - PWM Channel Counter Register (ch_num = 2)"]
    pub ccnt2: crate::Reg<ccnt2::CCNT2_SPEC>,
    #[doc = "0x250 - PWM Channel Update Register (ch_num = 2)"]
    pub cupd2: crate::Reg<cupd2::CUPD2_SPEC>,
    _reserved23: [u8; 0x0c],
    #[doc = "0x260 - PWM Channel Mode Register (ch_num = 3)"]
    pub cmr3: crate::Reg<cmr3::CMR3_SPEC>,
    #[doc = "0x264 - PWM Channel Duty Cycle Register (ch_num = 3)"]
    pub cdty3: crate::Reg<cdty3::CDTY3_SPEC>,
    #[doc = "0x268 - PWM Channel Period Register (ch_num = 3)"]
    pub cprd3: crate::Reg<cprd3::CPRD3_SPEC>,
    #[doc = "0x26c - PWM Channel Counter Register (ch_num = 3)"]
    pub ccnt3: crate::Reg<ccnt3::CCNT3_SPEC>,
    #[doc = "0x270 - PWM Channel Update Register (ch_num = 3)"]
    pub cupd3: crate::Reg<cupd3::CUPD3_SPEC>,
}
#[doc = "MR register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "PWM Mode Register"]
pub mod mr;
#[doc = "ENA register accessor: an alias for `Reg<ENA_SPEC>`"]
pub type ENA = crate::Reg<ena::ENA_SPEC>;
#[doc = "PWM Enable Register"]
pub mod ena;
#[doc = "DIS register accessor: an alias for `Reg<DIS_SPEC>`"]
pub type DIS = crate::Reg<dis::DIS_SPEC>;
#[doc = "PWM Disable Register"]
pub mod dis;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "PWM Status Register"]
pub mod sr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "PWM Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "PWM Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "PWM Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "PWM Interrupt Status Register"]
pub mod isr;
#[doc = "CMR0 register accessor: an alias for `Reg<CMR0_SPEC>`"]
pub type CMR0 = crate::Reg<cmr0::CMR0_SPEC>;
#[doc = "PWM Channel Mode Register (ch_num = 0)"]
pub mod cmr0;
#[doc = "CDTY0 register accessor: an alias for `Reg<CDTY0_SPEC>`"]
pub type CDTY0 = crate::Reg<cdty0::CDTY0_SPEC>;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 0)"]
pub mod cdty0;
#[doc = "CPRD0 register accessor: an alias for `Reg<CPRD0_SPEC>`"]
pub type CPRD0 = crate::Reg<cprd0::CPRD0_SPEC>;
#[doc = "PWM Channel Period Register (ch_num = 0)"]
pub mod cprd0;
#[doc = "CCNT0 register accessor: an alias for `Reg<CCNT0_SPEC>`"]
pub type CCNT0 = crate::Reg<ccnt0::CCNT0_SPEC>;
#[doc = "PWM Channel Counter Register (ch_num = 0)"]
pub mod ccnt0;
#[doc = "CUPD0 register accessor: an alias for `Reg<CUPD0_SPEC>`"]
pub type CUPD0 = crate::Reg<cupd0::CUPD0_SPEC>;
#[doc = "PWM Channel Update Register (ch_num = 0)"]
pub mod cupd0;
#[doc = "CMR1 register accessor: an alias for `Reg<CMR1_SPEC>`"]
pub type CMR1 = crate::Reg<cmr1::CMR1_SPEC>;
#[doc = "PWM Channel Mode Register (ch_num = 1)"]
pub mod cmr1;
#[doc = "CDTY1 register accessor: an alias for `Reg<CDTY1_SPEC>`"]
pub type CDTY1 = crate::Reg<cdty1::CDTY1_SPEC>;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 1)"]
pub mod cdty1;
#[doc = "CPRD1 register accessor: an alias for `Reg<CPRD1_SPEC>`"]
pub type CPRD1 = crate::Reg<cprd1::CPRD1_SPEC>;
#[doc = "PWM Channel Period Register (ch_num = 1)"]
pub mod cprd1;
#[doc = "CCNT1 register accessor: an alias for `Reg<CCNT1_SPEC>`"]
pub type CCNT1 = crate::Reg<ccnt1::CCNT1_SPEC>;
#[doc = "PWM Channel Counter Register (ch_num = 1)"]
pub mod ccnt1;
#[doc = "CUPD1 register accessor: an alias for `Reg<CUPD1_SPEC>`"]
pub type CUPD1 = crate::Reg<cupd1::CUPD1_SPEC>;
#[doc = "PWM Channel Update Register (ch_num = 1)"]
pub mod cupd1;
#[doc = "CMR2 register accessor: an alias for `Reg<CMR2_SPEC>`"]
pub type CMR2 = crate::Reg<cmr2::CMR2_SPEC>;
#[doc = "PWM Channel Mode Register (ch_num = 2)"]
pub mod cmr2;
#[doc = "CDTY2 register accessor: an alias for `Reg<CDTY2_SPEC>`"]
pub type CDTY2 = crate::Reg<cdty2::CDTY2_SPEC>;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 2)"]
pub mod cdty2;
#[doc = "CPRD2 register accessor: an alias for `Reg<CPRD2_SPEC>`"]
pub type CPRD2 = crate::Reg<cprd2::CPRD2_SPEC>;
#[doc = "PWM Channel Period Register (ch_num = 2)"]
pub mod cprd2;
#[doc = "CCNT2 register accessor: an alias for `Reg<CCNT2_SPEC>`"]
pub type CCNT2 = crate::Reg<ccnt2::CCNT2_SPEC>;
#[doc = "PWM Channel Counter Register (ch_num = 2)"]
pub mod ccnt2;
#[doc = "CUPD2 register accessor: an alias for `Reg<CUPD2_SPEC>`"]
pub type CUPD2 = crate::Reg<cupd2::CUPD2_SPEC>;
#[doc = "PWM Channel Update Register (ch_num = 2)"]
pub mod cupd2;
#[doc = "CMR3 register accessor: an alias for `Reg<CMR3_SPEC>`"]
pub type CMR3 = crate::Reg<cmr3::CMR3_SPEC>;
#[doc = "PWM Channel Mode Register (ch_num = 3)"]
pub mod cmr3;
#[doc = "CDTY3 register accessor: an alias for `Reg<CDTY3_SPEC>`"]
pub type CDTY3 = crate::Reg<cdty3::CDTY3_SPEC>;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 3)"]
pub mod cdty3;
#[doc = "CPRD3 register accessor: an alias for `Reg<CPRD3_SPEC>`"]
pub type CPRD3 = crate::Reg<cprd3::CPRD3_SPEC>;
#[doc = "PWM Channel Period Register (ch_num = 3)"]
pub mod cprd3;
#[doc = "CCNT3 register accessor: an alias for `Reg<CCNT3_SPEC>`"]
pub type CCNT3 = crate::Reg<ccnt3::CCNT3_SPEC>;
#[doc = "PWM Channel Counter Register (ch_num = 3)"]
pub mod ccnt3;
#[doc = "CUPD3 register accessor: an alias for `Reg<CUPD3_SPEC>`"]
pub type CUPD3 = crate::Reg<cupd3::CUPD3_SPEC>;
#[doc = "PWM Channel Update Register (ch_num = 3)"]
pub mod cupd3;
