#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Clock Register"]
    pub clk: crate::Reg<clk::CLK_SPEC>,
    #[doc = "0x04 - PWM Enable Register"]
    pub ena: crate::Reg<ena::ENA_SPEC>,
    #[doc = "0x08 - PWM Disable Register"]
    pub dis: crate::Reg<dis::DIS_SPEC>,
    #[doc = "0x0c - PWM Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x10 - PWM Interrupt Enable Register 1"]
    pub ier1: crate::Reg<ier1::IER1_SPEC>,
    #[doc = "0x14 - PWM Interrupt Disable Register 1"]
    pub idr1: crate::Reg<idr1::IDR1_SPEC>,
    #[doc = "0x18 - PWM Interrupt Mask Register 1"]
    pub imr1: crate::Reg<imr1::IMR1_SPEC>,
    #[doc = "0x1c - PWM Interrupt Status Register 1"]
    pub isr1: crate::Reg<isr1::ISR1_SPEC>,
    #[doc = "0x20 - PWM Sync Channels Mode Register"]
    pub scm: crate::Reg<scm::SCM_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x28 - PWM Sync Channels Update Control Register"]
    pub scuc: crate::Reg<scuc::SCUC_SPEC>,
    #[doc = "0x2c - PWM Sync Channels Update Period Register"]
    pub scup: crate::Reg<scup::SCUP_SPEC>,
    #[doc = "0x30 - PWM Sync Channels Update Period Update Register"]
    pub scupupd: crate::Reg<scupupd::SCUPUPD_SPEC>,
    #[doc = "0x34 - PWM Interrupt Enable Register 2"]
    pub ier2: crate::Reg<ier2::IER2_SPEC>,
    #[doc = "0x38 - PWM Interrupt Disable Register 2"]
    pub idr2: crate::Reg<idr2::IDR2_SPEC>,
    #[doc = "0x3c - PWM Interrupt Mask Register 2"]
    pub imr2: crate::Reg<imr2::IMR2_SPEC>,
    #[doc = "0x40 - PWM Interrupt Status Register 2"]
    pub isr2: crate::Reg<isr2::ISR2_SPEC>,
    #[doc = "0x44 - PWM Output Override Value Register"]
    pub oov: crate::Reg<oov::OOV_SPEC>,
    #[doc = "0x48 - PWM Output Selection Register"]
    pub os: crate::Reg<os::OS_SPEC>,
    #[doc = "0x4c - PWM Output Selection Set Register"]
    pub oss: crate::Reg<oss::OSS_SPEC>,
    #[doc = "0x50 - PWM Output Selection Clear Register"]
    pub osc: crate::Reg<osc::OSC_SPEC>,
    #[doc = "0x54 - PWM Output Selection Set Update Register"]
    pub ossupd: crate::Reg<ossupd::OSSUPD_SPEC>,
    #[doc = "0x58 - PWM Output Selection Clear Update Register"]
    pub oscupd: crate::Reg<oscupd::OSCUPD_SPEC>,
    #[doc = "0x5c - PWM Fault Mode Register"]
    pub fmr: crate::Reg<fmr::FMR_SPEC>,
    #[doc = "0x60 - PWM Fault Status Register"]
    pub fsr: crate::Reg<fsr::FSR_SPEC>,
    #[doc = "0x64 - PWM Fault Clear Register"]
    pub fcr: crate::Reg<fcr::FCR_SPEC>,
    #[doc = "0x68 - PWM Fault Protection Value Register"]
    pub fpv: crate::Reg<fpv::FPV_SPEC>,
    #[doc = "0x6c - PWM Fault Protection Enable Register"]
    pub fpe: crate::Reg<fpe::FPE_SPEC>,
    _reserved27: [u8; 0x0c],
    #[doc = "0x7c..0x84 - PWM Event Line 0 Mode Register"]
    pub elmr: [crate::Reg<elmr::ELMR_SPEC>; 2],
    _reserved28: [u8; 0x2c],
    #[doc = "0xb0 - PWM Stepper Motor Mode Register"]
    pub smmr: crate::Reg<smmr::SMMR_SPEC>,
    _reserved29: [u8; 0x30],
    #[doc = "0xe4 - PWM Write Protect Control Register"]
    pub wpcr: crate::Reg<wpcr::WPCR_SPEC>,
    #[doc = "0xe8 - PWM Write Protect Status Register"]
    pub wpsr: crate::Reg<wpsr::WPSR_SPEC>,
    _reserved31: [u8; 0x1c],
    #[doc = "0x108 - Transmit Pointer Register"]
    pub tpr: crate::Reg<tpr::TPR_SPEC>,
    #[doc = "0x10c - Transmit Counter Register"]
    pub tcr: crate::Reg<tcr::TCR_SPEC>,
    _reserved33: [u8; 0x08],
    #[doc = "0x118 - Transmit Next Pointer Register"]
    pub tnpr: crate::Reg<tnpr::TNPR_SPEC>,
    #[doc = "0x11c - Transmit Next Counter Register"]
    pub tncr: crate::Reg<tncr::TNCR_SPEC>,
    #[doc = "0x120 - Transfer Control Register"]
    pub ptcr: crate::Reg<ptcr::PTCR_SPEC>,
    #[doc = "0x124 - Transfer Status Register"]
    pub ptsr: crate::Reg<ptsr::PTSR_SPEC>,
    _reserved37: [u8; 0x08],
    #[doc = "0x130 - PWM Comparison 0 Value Register"]
    pub cmpv0: crate::Reg<cmpv0::CMPV0_SPEC>,
    #[doc = "0x134 - PWM Comparison 0 Value Update Register"]
    pub cmpvupd0: crate::Reg<cmpvupd0::CMPVUPD0_SPEC>,
    #[doc = "0x138 - PWM Comparison 0 Mode Register"]
    pub cmpm0: crate::Reg<cmpm0::CMPM0_SPEC>,
    #[doc = "0x13c - PWM Comparison 0 Mode Update Register"]
    pub cmpmupd0: crate::Reg<cmpmupd0::CMPMUPD0_SPEC>,
    #[doc = "0x140 - PWM Comparison 1 Value Register"]
    pub cmpv1: crate::Reg<cmpv1::CMPV1_SPEC>,
    #[doc = "0x144 - PWM Comparison 1 Value Update Register"]
    pub cmpvupd1: crate::Reg<cmpvupd1::CMPVUPD1_SPEC>,
    #[doc = "0x148 - PWM Comparison 1 Mode Register"]
    pub cmpm1: crate::Reg<cmpm1::CMPM1_SPEC>,
    #[doc = "0x14c - PWM Comparison 1 Mode Update Register"]
    pub cmpmupd1: crate::Reg<cmpmupd1::CMPMUPD1_SPEC>,
    #[doc = "0x150 - PWM Comparison 2 Value Register"]
    pub cmpv2: crate::Reg<cmpv2::CMPV2_SPEC>,
    #[doc = "0x154 - PWM Comparison 2 Value Update Register"]
    pub cmpvupd2: crate::Reg<cmpvupd2::CMPVUPD2_SPEC>,
    #[doc = "0x158 - PWM Comparison 2 Mode Register"]
    pub cmpm2: crate::Reg<cmpm2::CMPM2_SPEC>,
    #[doc = "0x15c - PWM Comparison 2 Mode Update Register"]
    pub cmpmupd2: crate::Reg<cmpmupd2::CMPMUPD2_SPEC>,
    #[doc = "0x160 - PWM Comparison 3 Value Register"]
    pub cmpv3: crate::Reg<cmpv3::CMPV3_SPEC>,
    #[doc = "0x164 - PWM Comparison 3 Value Update Register"]
    pub cmpvupd3: crate::Reg<cmpvupd3::CMPVUPD3_SPEC>,
    #[doc = "0x168 - PWM Comparison 3 Mode Register"]
    pub cmpm3: crate::Reg<cmpm3::CMPM3_SPEC>,
    #[doc = "0x16c - PWM Comparison 3 Mode Update Register"]
    pub cmpmupd3: crate::Reg<cmpmupd3::CMPMUPD3_SPEC>,
    #[doc = "0x170 - PWM Comparison 4 Value Register"]
    pub cmpv4: crate::Reg<cmpv4::CMPV4_SPEC>,
    #[doc = "0x174 - PWM Comparison 4 Value Update Register"]
    pub cmpvupd4: crate::Reg<cmpvupd4::CMPVUPD4_SPEC>,
    #[doc = "0x178 - PWM Comparison 4 Mode Register"]
    pub cmpm4: crate::Reg<cmpm4::CMPM4_SPEC>,
    #[doc = "0x17c - PWM Comparison 4 Mode Update Register"]
    pub cmpmupd4: crate::Reg<cmpmupd4::CMPMUPD4_SPEC>,
    #[doc = "0x180 - PWM Comparison 5 Value Register"]
    pub cmpv5: crate::Reg<cmpv5::CMPV5_SPEC>,
    #[doc = "0x184 - PWM Comparison 5 Value Update Register"]
    pub cmpvupd5: crate::Reg<cmpvupd5::CMPVUPD5_SPEC>,
    #[doc = "0x188 - PWM Comparison 5 Mode Register"]
    pub cmpm5: crate::Reg<cmpm5::CMPM5_SPEC>,
    #[doc = "0x18c - PWM Comparison 5 Mode Update Register"]
    pub cmpmupd5: crate::Reg<cmpmupd5::CMPMUPD5_SPEC>,
    #[doc = "0x190 - PWM Comparison 6 Value Register"]
    pub cmpv6: crate::Reg<cmpv6::CMPV6_SPEC>,
    #[doc = "0x194 - PWM Comparison 6 Value Update Register"]
    pub cmpvupd6: crate::Reg<cmpvupd6::CMPVUPD6_SPEC>,
    #[doc = "0x198 - PWM Comparison 6 Mode Register"]
    pub cmpm6: crate::Reg<cmpm6::CMPM6_SPEC>,
    #[doc = "0x19c - PWM Comparison 6 Mode Update Register"]
    pub cmpmupd6: crate::Reg<cmpmupd6::CMPMUPD6_SPEC>,
    #[doc = "0x1a0 - PWM Comparison 7 Value Register"]
    pub cmpv7: crate::Reg<cmpv7::CMPV7_SPEC>,
    #[doc = "0x1a4 - PWM Comparison 7 Value Update Register"]
    pub cmpvupd7: crate::Reg<cmpvupd7::CMPVUPD7_SPEC>,
    #[doc = "0x1a8 - PWM Comparison 7 Mode Register"]
    pub cmpm7: crate::Reg<cmpm7::CMPM7_SPEC>,
    #[doc = "0x1ac - PWM Comparison 7 Mode Update Register"]
    pub cmpmupd7: crate::Reg<cmpmupd7::CMPMUPD7_SPEC>,
    _reserved69: [u8; 0x50],
    #[doc = "0x200 - PWM Channel Mode Register (ch_num = 0)"]
    pub cmr0: crate::Reg<cmr0::CMR0_SPEC>,
    #[doc = "0x204 - PWM Channel Duty Cycle Register (ch_num = 0)"]
    pub cdty0: crate::Reg<cdty0::CDTY0_SPEC>,
    #[doc = "0x208 - PWM Channel Duty Cycle Update Register (ch_num = 0)"]
    pub cdtyupd0: crate::Reg<cdtyupd0::CDTYUPD0_SPEC>,
    #[doc = "0x20c - PWM Channel Period Register (ch_num = 0)"]
    pub cprd0: crate::Reg<cprd0::CPRD0_SPEC>,
    #[doc = "0x210 - PWM Channel Period Update Register (ch_num = 0)"]
    pub cprdupd0: crate::Reg<cprdupd0::CPRDUPD0_SPEC>,
    #[doc = "0x214 - PWM Channel Counter Register (ch_num = 0)"]
    pub ccnt0: crate::Reg<ccnt0::CCNT0_SPEC>,
    #[doc = "0x218 - PWM Channel Dead Time Register (ch_num = 0)"]
    pub dt0: crate::Reg<dt0::DT0_SPEC>,
    #[doc = "0x21c - PWM Channel Dead Time Update Register (ch_num = 0)"]
    pub dtupd0: crate::Reg<dtupd0::DTUPD0_SPEC>,
    #[doc = "0x220 - PWM Channel Mode Register (ch_num = 1)"]
    pub cmr1: crate::Reg<cmr1::CMR1_SPEC>,
    #[doc = "0x224 - PWM Channel Duty Cycle Register (ch_num = 1)"]
    pub cdty1: crate::Reg<cdty1::CDTY1_SPEC>,
    #[doc = "0x228 - PWM Channel Duty Cycle Update Register (ch_num = 1)"]
    pub cdtyupd1: crate::Reg<cdtyupd1::CDTYUPD1_SPEC>,
    #[doc = "0x22c - PWM Channel Period Register (ch_num = 1)"]
    pub cprd1: crate::Reg<cprd1::CPRD1_SPEC>,
    #[doc = "0x230 - PWM Channel Period Update Register (ch_num = 1)"]
    pub cprdupd1: crate::Reg<cprdupd1::CPRDUPD1_SPEC>,
    #[doc = "0x234 - PWM Channel Counter Register (ch_num = 1)"]
    pub ccnt1: crate::Reg<ccnt1::CCNT1_SPEC>,
    #[doc = "0x238 - PWM Channel Dead Time Register (ch_num = 1)"]
    pub dt1: crate::Reg<dt1::DT1_SPEC>,
    #[doc = "0x23c - PWM Channel Dead Time Update Register (ch_num = 1)"]
    pub dtupd1: crate::Reg<dtupd1::DTUPD1_SPEC>,
    #[doc = "0x240 - PWM Channel Mode Register (ch_num = 2)"]
    pub cmr2: crate::Reg<cmr2::CMR2_SPEC>,
    #[doc = "0x244 - PWM Channel Duty Cycle Register (ch_num = 2)"]
    pub cdty2: crate::Reg<cdty2::CDTY2_SPEC>,
    #[doc = "0x248 - PWM Channel Duty Cycle Update Register (ch_num = 2)"]
    pub cdtyupd2: crate::Reg<cdtyupd2::CDTYUPD2_SPEC>,
    #[doc = "0x24c - PWM Channel Period Register (ch_num = 2)"]
    pub cprd2: crate::Reg<cprd2::CPRD2_SPEC>,
    #[doc = "0x250 - PWM Channel Period Update Register (ch_num = 2)"]
    pub cprdupd2: crate::Reg<cprdupd2::CPRDUPD2_SPEC>,
    #[doc = "0x254 - PWM Channel Counter Register (ch_num = 2)"]
    pub ccnt2: crate::Reg<ccnt2::CCNT2_SPEC>,
    #[doc = "0x258 - PWM Channel Dead Time Register (ch_num = 2)"]
    pub dt2: crate::Reg<dt2::DT2_SPEC>,
    #[doc = "0x25c - PWM Channel Dead Time Update Register (ch_num = 2)"]
    pub dtupd2: crate::Reg<dtupd2::DTUPD2_SPEC>,
    #[doc = "0x260 - PWM Channel Mode Register (ch_num = 3)"]
    pub cmr3: crate::Reg<cmr3::CMR3_SPEC>,
    #[doc = "0x264 - PWM Channel Duty Cycle Register (ch_num = 3)"]
    pub cdty3: crate::Reg<cdty3::CDTY3_SPEC>,
    #[doc = "0x268 - PWM Channel Duty Cycle Update Register (ch_num = 3)"]
    pub cdtyupd3: crate::Reg<cdtyupd3::CDTYUPD3_SPEC>,
    #[doc = "0x26c - PWM Channel Period Register (ch_num = 3)"]
    pub cprd3: crate::Reg<cprd3::CPRD3_SPEC>,
    #[doc = "0x270 - PWM Channel Period Update Register (ch_num = 3)"]
    pub cprdupd3: crate::Reg<cprdupd3::CPRDUPD3_SPEC>,
    #[doc = "0x274 - PWM Channel Counter Register (ch_num = 3)"]
    pub ccnt3: crate::Reg<ccnt3::CCNT3_SPEC>,
    #[doc = "0x278 - PWM Channel Dead Time Register (ch_num = 3)"]
    pub dt3: crate::Reg<dt3::DT3_SPEC>,
    #[doc = "0x27c - PWM Channel Dead Time Update Register (ch_num = 3)"]
    pub dtupd3: crate::Reg<dtupd3::DTUPD3_SPEC>,
}
#[doc = "CLK register accessor: an alias for `Reg<CLK_SPEC>`"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "PWM Clock Register"]
pub mod clk;
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
#[doc = "IER1 register accessor: an alias for `Reg<IER1_SPEC>`"]
pub type IER1 = crate::Reg<ier1::IER1_SPEC>;
#[doc = "PWM Interrupt Enable Register 1"]
pub mod ier1;
#[doc = "IDR1 register accessor: an alias for `Reg<IDR1_SPEC>`"]
pub type IDR1 = crate::Reg<idr1::IDR1_SPEC>;
#[doc = "PWM Interrupt Disable Register 1"]
pub mod idr1;
#[doc = "IMR1 register accessor: an alias for `Reg<IMR1_SPEC>`"]
pub type IMR1 = crate::Reg<imr1::IMR1_SPEC>;
#[doc = "PWM Interrupt Mask Register 1"]
pub mod imr1;
#[doc = "ISR1 register accessor: an alias for `Reg<ISR1_SPEC>`"]
pub type ISR1 = crate::Reg<isr1::ISR1_SPEC>;
#[doc = "PWM Interrupt Status Register 1"]
pub mod isr1;
#[doc = "SCM register accessor: an alias for `Reg<SCM_SPEC>`"]
pub type SCM = crate::Reg<scm::SCM_SPEC>;
#[doc = "PWM Sync Channels Mode Register"]
pub mod scm;
#[doc = "SCUC register accessor: an alias for `Reg<SCUC_SPEC>`"]
pub type SCUC = crate::Reg<scuc::SCUC_SPEC>;
#[doc = "PWM Sync Channels Update Control Register"]
pub mod scuc;
#[doc = "SCUP register accessor: an alias for `Reg<SCUP_SPEC>`"]
pub type SCUP = crate::Reg<scup::SCUP_SPEC>;
#[doc = "PWM Sync Channels Update Period Register"]
pub mod scup;
#[doc = "SCUPUPD register accessor: an alias for `Reg<SCUPUPD_SPEC>`"]
pub type SCUPUPD = crate::Reg<scupupd::SCUPUPD_SPEC>;
#[doc = "PWM Sync Channels Update Period Update Register"]
pub mod scupupd;
#[doc = "IER2 register accessor: an alias for `Reg<IER2_SPEC>`"]
pub type IER2 = crate::Reg<ier2::IER2_SPEC>;
#[doc = "PWM Interrupt Enable Register 2"]
pub mod ier2;
#[doc = "IDR2 register accessor: an alias for `Reg<IDR2_SPEC>`"]
pub type IDR2 = crate::Reg<idr2::IDR2_SPEC>;
#[doc = "PWM Interrupt Disable Register 2"]
pub mod idr2;
#[doc = "IMR2 register accessor: an alias for `Reg<IMR2_SPEC>`"]
pub type IMR2 = crate::Reg<imr2::IMR2_SPEC>;
#[doc = "PWM Interrupt Mask Register 2"]
pub mod imr2;
#[doc = "ISR2 register accessor: an alias for `Reg<ISR2_SPEC>`"]
pub type ISR2 = crate::Reg<isr2::ISR2_SPEC>;
#[doc = "PWM Interrupt Status Register 2"]
pub mod isr2;
#[doc = "OOV register accessor: an alias for `Reg<OOV_SPEC>`"]
pub type OOV = crate::Reg<oov::OOV_SPEC>;
#[doc = "PWM Output Override Value Register"]
pub mod oov;
#[doc = "OS register accessor: an alias for `Reg<OS_SPEC>`"]
pub type OS = crate::Reg<os::OS_SPEC>;
#[doc = "PWM Output Selection Register"]
pub mod os;
#[doc = "OSS register accessor: an alias for `Reg<OSS_SPEC>`"]
pub type OSS = crate::Reg<oss::OSS_SPEC>;
#[doc = "PWM Output Selection Set Register"]
pub mod oss;
#[doc = "OSC register accessor: an alias for `Reg<OSC_SPEC>`"]
pub type OSC = crate::Reg<osc::OSC_SPEC>;
#[doc = "PWM Output Selection Clear Register"]
pub mod osc;
#[doc = "OSSUPD register accessor: an alias for `Reg<OSSUPD_SPEC>`"]
pub type OSSUPD = crate::Reg<ossupd::OSSUPD_SPEC>;
#[doc = "PWM Output Selection Set Update Register"]
pub mod ossupd;
#[doc = "OSCUPD register accessor: an alias for `Reg<OSCUPD_SPEC>`"]
pub type OSCUPD = crate::Reg<oscupd::OSCUPD_SPEC>;
#[doc = "PWM Output Selection Clear Update Register"]
pub mod oscupd;
#[doc = "FMR register accessor: an alias for `Reg<FMR_SPEC>`"]
pub type FMR = crate::Reg<fmr::FMR_SPEC>;
#[doc = "PWM Fault Mode Register"]
pub mod fmr;
#[doc = "FSR register accessor: an alias for `Reg<FSR_SPEC>`"]
pub type FSR = crate::Reg<fsr::FSR_SPEC>;
#[doc = "PWM Fault Status Register"]
pub mod fsr;
#[doc = "FCR register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "PWM Fault Clear Register"]
pub mod fcr;
#[doc = "FPV register accessor: an alias for `Reg<FPV_SPEC>`"]
pub type FPV = crate::Reg<fpv::FPV_SPEC>;
#[doc = "PWM Fault Protection Value Register"]
pub mod fpv;
#[doc = "FPE register accessor: an alias for `Reg<FPE_SPEC>`"]
pub type FPE = crate::Reg<fpe::FPE_SPEC>;
#[doc = "PWM Fault Protection Enable Register"]
pub mod fpe;
#[doc = "ELMR register accessor: an alias for `Reg<ELMR_SPEC>`"]
pub type ELMR = crate::Reg<elmr::ELMR_SPEC>;
#[doc = "PWM Event Line 0 Mode Register"]
pub mod elmr;
#[doc = "SMMR register accessor: an alias for `Reg<SMMR_SPEC>`"]
pub type SMMR = crate::Reg<smmr::SMMR_SPEC>;
#[doc = "PWM Stepper Motor Mode Register"]
pub mod smmr;
#[doc = "WPCR register accessor: an alias for `Reg<WPCR_SPEC>`"]
pub type WPCR = crate::Reg<wpcr::WPCR_SPEC>;
#[doc = "PWM Write Protect Control Register"]
pub mod wpcr;
#[doc = "WPSR register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "PWM Write Protect Status Register"]
pub mod wpsr;
#[doc = "CMPV0 register accessor: an alias for `Reg<CMPV0_SPEC>`"]
pub type CMPV0 = crate::Reg<cmpv0::CMPV0_SPEC>;
#[doc = "PWM Comparison 0 Value Register"]
pub mod cmpv0;
#[doc = "CMPVUPD0 register accessor: an alias for `Reg<CMPVUPD0_SPEC>`"]
pub type CMPVUPD0 = crate::Reg<cmpvupd0::CMPVUPD0_SPEC>;
#[doc = "PWM Comparison 0 Value Update Register"]
pub mod cmpvupd0;
#[doc = "CMPM0 register accessor: an alias for `Reg<CMPM0_SPEC>`"]
pub type CMPM0 = crate::Reg<cmpm0::CMPM0_SPEC>;
#[doc = "PWM Comparison 0 Mode Register"]
pub mod cmpm0;
#[doc = "CMPMUPD0 register accessor: an alias for `Reg<CMPMUPD0_SPEC>`"]
pub type CMPMUPD0 = crate::Reg<cmpmupd0::CMPMUPD0_SPEC>;
#[doc = "PWM Comparison 0 Mode Update Register"]
pub mod cmpmupd0;
#[doc = "CMPV1 register accessor: an alias for `Reg<CMPV1_SPEC>`"]
pub type CMPV1 = crate::Reg<cmpv1::CMPV1_SPEC>;
#[doc = "PWM Comparison 1 Value Register"]
pub mod cmpv1;
#[doc = "CMPVUPD1 register accessor: an alias for `Reg<CMPVUPD1_SPEC>`"]
pub type CMPVUPD1 = crate::Reg<cmpvupd1::CMPVUPD1_SPEC>;
#[doc = "PWM Comparison 1 Value Update Register"]
pub mod cmpvupd1;
#[doc = "CMPM1 register accessor: an alias for `Reg<CMPM1_SPEC>`"]
pub type CMPM1 = crate::Reg<cmpm1::CMPM1_SPEC>;
#[doc = "PWM Comparison 1 Mode Register"]
pub mod cmpm1;
#[doc = "CMPMUPD1 register accessor: an alias for `Reg<CMPMUPD1_SPEC>`"]
pub type CMPMUPD1 = crate::Reg<cmpmupd1::CMPMUPD1_SPEC>;
#[doc = "PWM Comparison 1 Mode Update Register"]
pub mod cmpmupd1;
#[doc = "CMPV2 register accessor: an alias for `Reg<CMPV2_SPEC>`"]
pub type CMPV2 = crate::Reg<cmpv2::CMPV2_SPEC>;
#[doc = "PWM Comparison 2 Value Register"]
pub mod cmpv2;
#[doc = "CMPVUPD2 register accessor: an alias for `Reg<CMPVUPD2_SPEC>`"]
pub type CMPVUPD2 = crate::Reg<cmpvupd2::CMPVUPD2_SPEC>;
#[doc = "PWM Comparison 2 Value Update Register"]
pub mod cmpvupd2;
#[doc = "CMPM2 register accessor: an alias for `Reg<CMPM2_SPEC>`"]
pub type CMPM2 = crate::Reg<cmpm2::CMPM2_SPEC>;
#[doc = "PWM Comparison 2 Mode Register"]
pub mod cmpm2;
#[doc = "CMPMUPD2 register accessor: an alias for `Reg<CMPMUPD2_SPEC>`"]
pub type CMPMUPD2 = crate::Reg<cmpmupd2::CMPMUPD2_SPEC>;
#[doc = "PWM Comparison 2 Mode Update Register"]
pub mod cmpmupd2;
#[doc = "CMPV3 register accessor: an alias for `Reg<CMPV3_SPEC>`"]
pub type CMPV3 = crate::Reg<cmpv3::CMPV3_SPEC>;
#[doc = "PWM Comparison 3 Value Register"]
pub mod cmpv3;
#[doc = "CMPVUPD3 register accessor: an alias for `Reg<CMPVUPD3_SPEC>`"]
pub type CMPVUPD3 = crate::Reg<cmpvupd3::CMPVUPD3_SPEC>;
#[doc = "PWM Comparison 3 Value Update Register"]
pub mod cmpvupd3;
#[doc = "CMPM3 register accessor: an alias for `Reg<CMPM3_SPEC>`"]
pub type CMPM3 = crate::Reg<cmpm3::CMPM3_SPEC>;
#[doc = "PWM Comparison 3 Mode Register"]
pub mod cmpm3;
#[doc = "CMPMUPD3 register accessor: an alias for `Reg<CMPMUPD3_SPEC>`"]
pub type CMPMUPD3 = crate::Reg<cmpmupd3::CMPMUPD3_SPEC>;
#[doc = "PWM Comparison 3 Mode Update Register"]
pub mod cmpmupd3;
#[doc = "CMPV4 register accessor: an alias for `Reg<CMPV4_SPEC>`"]
pub type CMPV4 = crate::Reg<cmpv4::CMPV4_SPEC>;
#[doc = "PWM Comparison 4 Value Register"]
pub mod cmpv4;
#[doc = "CMPVUPD4 register accessor: an alias for `Reg<CMPVUPD4_SPEC>`"]
pub type CMPVUPD4 = crate::Reg<cmpvupd4::CMPVUPD4_SPEC>;
#[doc = "PWM Comparison 4 Value Update Register"]
pub mod cmpvupd4;
#[doc = "CMPM4 register accessor: an alias for `Reg<CMPM4_SPEC>`"]
pub type CMPM4 = crate::Reg<cmpm4::CMPM4_SPEC>;
#[doc = "PWM Comparison 4 Mode Register"]
pub mod cmpm4;
#[doc = "CMPMUPD4 register accessor: an alias for `Reg<CMPMUPD4_SPEC>`"]
pub type CMPMUPD4 = crate::Reg<cmpmupd4::CMPMUPD4_SPEC>;
#[doc = "PWM Comparison 4 Mode Update Register"]
pub mod cmpmupd4;
#[doc = "CMPV5 register accessor: an alias for `Reg<CMPV5_SPEC>`"]
pub type CMPV5 = crate::Reg<cmpv5::CMPV5_SPEC>;
#[doc = "PWM Comparison 5 Value Register"]
pub mod cmpv5;
#[doc = "CMPVUPD5 register accessor: an alias for `Reg<CMPVUPD5_SPEC>`"]
pub type CMPVUPD5 = crate::Reg<cmpvupd5::CMPVUPD5_SPEC>;
#[doc = "PWM Comparison 5 Value Update Register"]
pub mod cmpvupd5;
#[doc = "CMPM5 register accessor: an alias for `Reg<CMPM5_SPEC>`"]
pub type CMPM5 = crate::Reg<cmpm5::CMPM5_SPEC>;
#[doc = "PWM Comparison 5 Mode Register"]
pub mod cmpm5;
#[doc = "CMPMUPD5 register accessor: an alias for `Reg<CMPMUPD5_SPEC>`"]
pub type CMPMUPD5 = crate::Reg<cmpmupd5::CMPMUPD5_SPEC>;
#[doc = "PWM Comparison 5 Mode Update Register"]
pub mod cmpmupd5;
#[doc = "CMPV6 register accessor: an alias for `Reg<CMPV6_SPEC>`"]
pub type CMPV6 = crate::Reg<cmpv6::CMPV6_SPEC>;
#[doc = "PWM Comparison 6 Value Register"]
pub mod cmpv6;
#[doc = "CMPVUPD6 register accessor: an alias for `Reg<CMPVUPD6_SPEC>`"]
pub type CMPVUPD6 = crate::Reg<cmpvupd6::CMPVUPD6_SPEC>;
#[doc = "PWM Comparison 6 Value Update Register"]
pub mod cmpvupd6;
#[doc = "CMPM6 register accessor: an alias for `Reg<CMPM6_SPEC>`"]
pub type CMPM6 = crate::Reg<cmpm6::CMPM6_SPEC>;
#[doc = "PWM Comparison 6 Mode Register"]
pub mod cmpm6;
#[doc = "CMPMUPD6 register accessor: an alias for `Reg<CMPMUPD6_SPEC>`"]
pub type CMPMUPD6 = crate::Reg<cmpmupd6::CMPMUPD6_SPEC>;
#[doc = "PWM Comparison 6 Mode Update Register"]
pub mod cmpmupd6;
#[doc = "CMPV7 register accessor: an alias for `Reg<CMPV7_SPEC>`"]
pub type CMPV7 = crate::Reg<cmpv7::CMPV7_SPEC>;
#[doc = "PWM Comparison 7 Value Register"]
pub mod cmpv7;
#[doc = "CMPVUPD7 register accessor: an alias for `Reg<CMPVUPD7_SPEC>`"]
pub type CMPVUPD7 = crate::Reg<cmpvupd7::CMPVUPD7_SPEC>;
#[doc = "PWM Comparison 7 Value Update Register"]
pub mod cmpvupd7;
#[doc = "CMPM7 register accessor: an alias for `Reg<CMPM7_SPEC>`"]
pub type CMPM7 = crate::Reg<cmpm7::CMPM7_SPEC>;
#[doc = "PWM Comparison 7 Mode Register"]
pub mod cmpm7;
#[doc = "CMPMUPD7 register accessor: an alias for `Reg<CMPMUPD7_SPEC>`"]
pub type CMPMUPD7 = crate::Reg<cmpmupd7::CMPMUPD7_SPEC>;
#[doc = "PWM Comparison 7 Mode Update Register"]
pub mod cmpmupd7;
#[doc = "CMR0 register accessor: an alias for `Reg<CMR0_SPEC>`"]
pub type CMR0 = crate::Reg<cmr0::CMR0_SPEC>;
#[doc = "PWM Channel Mode Register (ch_num = 0)"]
pub mod cmr0;
#[doc = "CDTY0 register accessor: an alias for `Reg<CDTY0_SPEC>`"]
pub type CDTY0 = crate::Reg<cdty0::CDTY0_SPEC>;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 0)"]
pub mod cdty0;
#[doc = "CDTYUPD0 register accessor: an alias for `Reg<CDTYUPD0_SPEC>`"]
pub type CDTYUPD0 = crate::Reg<cdtyupd0::CDTYUPD0_SPEC>;
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 0)"]
pub mod cdtyupd0;
#[doc = "CPRD0 register accessor: an alias for `Reg<CPRD0_SPEC>`"]
pub type CPRD0 = crate::Reg<cprd0::CPRD0_SPEC>;
#[doc = "PWM Channel Period Register (ch_num = 0)"]
pub mod cprd0;
#[doc = "CPRDUPD0 register accessor: an alias for `Reg<CPRDUPD0_SPEC>`"]
pub type CPRDUPD0 = crate::Reg<cprdupd0::CPRDUPD0_SPEC>;
#[doc = "PWM Channel Period Update Register (ch_num = 0)"]
pub mod cprdupd0;
#[doc = "CCNT0 register accessor: an alias for `Reg<CCNT0_SPEC>`"]
pub type CCNT0 = crate::Reg<ccnt0::CCNT0_SPEC>;
#[doc = "PWM Channel Counter Register (ch_num = 0)"]
pub mod ccnt0;
#[doc = "DT0 register accessor: an alias for `Reg<DT0_SPEC>`"]
pub type DT0 = crate::Reg<dt0::DT0_SPEC>;
#[doc = "PWM Channel Dead Time Register (ch_num = 0)"]
pub mod dt0;
#[doc = "DTUPD0 register accessor: an alias for `Reg<DTUPD0_SPEC>`"]
pub type DTUPD0 = crate::Reg<dtupd0::DTUPD0_SPEC>;
#[doc = "PWM Channel Dead Time Update Register (ch_num = 0)"]
pub mod dtupd0;
#[doc = "CMR1 register accessor: an alias for `Reg<CMR1_SPEC>`"]
pub type CMR1 = crate::Reg<cmr1::CMR1_SPEC>;
#[doc = "PWM Channel Mode Register (ch_num = 1)"]
pub mod cmr1;
#[doc = "CDTY1 register accessor: an alias for `Reg<CDTY1_SPEC>`"]
pub type CDTY1 = crate::Reg<cdty1::CDTY1_SPEC>;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 1)"]
pub mod cdty1;
#[doc = "CDTYUPD1 register accessor: an alias for `Reg<CDTYUPD1_SPEC>`"]
pub type CDTYUPD1 = crate::Reg<cdtyupd1::CDTYUPD1_SPEC>;
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 1)"]
pub mod cdtyupd1;
#[doc = "CPRD1 register accessor: an alias for `Reg<CPRD1_SPEC>`"]
pub type CPRD1 = crate::Reg<cprd1::CPRD1_SPEC>;
#[doc = "PWM Channel Period Register (ch_num = 1)"]
pub mod cprd1;
#[doc = "CPRDUPD1 register accessor: an alias for `Reg<CPRDUPD1_SPEC>`"]
pub type CPRDUPD1 = crate::Reg<cprdupd1::CPRDUPD1_SPEC>;
#[doc = "PWM Channel Period Update Register (ch_num = 1)"]
pub mod cprdupd1;
#[doc = "CCNT1 register accessor: an alias for `Reg<CCNT1_SPEC>`"]
pub type CCNT1 = crate::Reg<ccnt1::CCNT1_SPEC>;
#[doc = "PWM Channel Counter Register (ch_num = 1)"]
pub mod ccnt1;
#[doc = "DT1 register accessor: an alias for `Reg<DT1_SPEC>`"]
pub type DT1 = crate::Reg<dt1::DT1_SPEC>;
#[doc = "PWM Channel Dead Time Register (ch_num = 1)"]
pub mod dt1;
#[doc = "DTUPD1 register accessor: an alias for `Reg<DTUPD1_SPEC>`"]
pub type DTUPD1 = crate::Reg<dtupd1::DTUPD1_SPEC>;
#[doc = "PWM Channel Dead Time Update Register (ch_num = 1)"]
pub mod dtupd1;
#[doc = "CMR2 register accessor: an alias for `Reg<CMR2_SPEC>`"]
pub type CMR2 = crate::Reg<cmr2::CMR2_SPEC>;
#[doc = "PWM Channel Mode Register (ch_num = 2)"]
pub mod cmr2;
#[doc = "CDTY2 register accessor: an alias for `Reg<CDTY2_SPEC>`"]
pub type CDTY2 = crate::Reg<cdty2::CDTY2_SPEC>;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 2)"]
pub mod cdty2;
#[doc = "CDTYUPD2 register accessor: an alias for `Reg<CDTYUPD2_SPEC>`"]
pub type CDTYUPD2 = crate::Reg<cdtyupd2::CDTYUPD2_SPEC>;
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 2)"]
pub mod cdtyupd2;
#[doc = "CPRD2 register accessor: an alias for `Reg<CPRD2_SPEC>`"]
pub type CPRD2 = crate::Reg<cprd2::CPRD2_SPEC>;
#[doc = "PWM Channel Period Register (ch_num = 2)"]
pub mod cprd2;
#[doc = "CPRDUPD2 register accessor: an alias for `Reg<CPRDUPD2_SPEC>`"]
pub type CPRDUPD2 = crate::Reg<cprdupd2::CPRDUPD2_SPEC>;
#[doc = "PWM Channel Period Update Register (ch_num = 2)"]
pub mod cprdupd2;
#[doc = "CCNT2 register accessor: an alias for `Reg<CCNT2_SPEC>`"]
pub type CCNT2 = crate::Reg<ccnt2::CCNT2_SPEC>;
#[doc = "PWM Channel Counter Register (ch_num = 2)"]
pub mod ccnt2;
#[doc = "DT2 register accessor: an alias for `Reg<DT2_SPEC>`"]
pub type DT2 = crate::Reg<dt2::DT2_SPEC>;
#[doc = "PWM Channel Dead Time Register (ch_num = 2)"]
pub mod dt2;
#[doc = "DTUPD2 register accessor: an alias for `Reg<DTUPD2_SPEC>`"]
pub type DTUPD2 = crate::Reg<dtupd2::DTUPD2_SPEC>;
#[doc = "PWM Channel Dead Time Update Register (ch_num = 2)"]
pub mod dtupd2;
#[doc = "CMR3 register accessor: an alias for `Reg<CMR3_SPEC>`"]
pub type CMR3 = crate::Reg<cmr3::CMR3_SPEC>;
#[doc = "PWM Channel Mode Register (ch_num = 3)"]
pub mod cmr3;
#[doc = "CDTY3 register accessor: an alias for `Reg<CDTY3_SPEC>`"]
pub type CDTY3 = crate::Reg<cdty3::CDTY3_SPEC>;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 3)"]
pub mod cdty3;
#[doc = "CDTYUPD3 register accessor: an alias for `Reg<CDTYUPD3_SPEC>`"]
pub type CDTYUPD3 = crate::Reg<cdtyupd3::CDTYUPD3_SPEC>;
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 3)"]
pub mod cdtyupd3;
#[doc = "CPRD3 register accessor: an alias for `Reg<CPRD3_SPEC>`"]
pub type CPRD3 = crate::Reg<cprd3::CPRD3_SPEC>;
#[doc = "PWM Channel Period Register (ch_num = 3)"]
pub mod cprd3;
#[doc = "CPRDUPD3 register accessor: an alias for `Reg<CPRDUPD3_SPEC>`"]
pub type CPRDUPD3 = crate::Reg<cprdupd3::CPRDUPD3_SPEC>;
#[doc = "PWM Channel Period Update Register (ch_num = 3)"]
pub mod cprdupd3;
#[doc = "CCNT3 register accessor: an alias for `Reg<CCNT3_SPEC>`"]
pub type CCNT3 = crate::Reg<ccnt3::CCNT3_SPEC>;
#[doc = "PWM Channel Counter Register (ch_num = 3)"]
pub mod ccnt3;
#[doc = "DT3 register accessor: an alias for `Reg<DT3_SPEC>`"]
pub type DT3 = crate::Reg<dt3::DT3_SPEC>;
#[doc = "PWM Channel Dead Time Register (ch_num = 3)"]
pub mod dt3;
#[doc = "DTUPD3 register accessor: an alias for `Reg<DTUPD3_SPEC>`"]
pub type DTUPD3 = crate::Reg<dtupd3::DTUPD3_SPEC>;
#[doc = "PWM Channel Dead Time Update Register (ch_num = 3)"]
pub mod dtupd3;
#[doc = "TPR register accessor: an alias for `Reg<TPR_SPEC>`"]
pub type TPR = crate::Reg<tpr::TPR_SPEC>;
#[doc = "Transmit Pointer Register"]
pub mod tpr;
#[doc = "TCR register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Transmit Counter Register"]
pub mod tcr;
#[doc = "TNPR register accessor: an alias for `Reg<TNPR_SPEC>`"]
pub type TNPR = crate::Reg<tnpr::TNPR_SPEC>;
#[doc = "Transmit Next Pointer Register"]
pub mod tnpr;
#[doc = "TNCR register accessor: an alias for `Reg<TNCR_SPEC>`"]
pub type TNCR = crate::Reg<tncr::TNCR_SPEC>;
#[doc = "Transmit Next Counter Register"]
pub mod tncr;
#[doc = "PTCR register accessor: an alias for `Reg<PTCR_SPEC>`"]
pub type PTCR = crate::Reg<ptcr::PTCR_SPEC>;
#[doc = "Transfer Control Register"]
pub mod ptcr;
#[doc = "PTSR register accessor: an alias for `Reg<PTSR_SPEC>`"]
pub type PTSR = crate::Reg<ptsr::PTSR_SPEC>;
#[doc = "Transfer Status Register"]
pub mod ptsr;
