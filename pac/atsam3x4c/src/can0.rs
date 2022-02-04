#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Mode Register"]
    pub mr: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x04 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x08 - Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x0c - Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x10 - Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x14 - Baudrate Register"]
    pub br: crate::Reg<br::BR_SPEC>,
    #[doc = "0x18 - Timer Register"]
    pub tim: crate::Reg<tim::TIM_SPEC>,
    #[doc = "0x1c - Timestamp Register"]
    pub timestp: crate::Reg<timestp::TIMESTP_SPEC>,
    #[doc = "0x20 - Error Counter Register"]
    pub ecr: crate::Reg<ecr::ECR_SPEC>,
    #[doc = "0x24 - Transfer Command Register"]
    pub tcr: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x28 - Abort Command Register"]
    pub acr: crate::Reg<acr::ACR_SPEC>,
    _reserved11: [u8; 0xb8],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: crate::Reg<wpmr::WPMR_SPEC>,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub wpsr: crate::Reg<wpsr::WPSR_SPEC>,
    _reserved13: [u8; 0x0114],
    #[doc = "0x200 - Mailbox Mode Register (MB = 0)"]
    pub mmr0: crate::Reg<mmr0::MMR0_SPEC>,
    #[doc = "0x204 - Mailbox Acceptance Mask Register (MB = 0)"]
    pub mam0: crate::Reg<mam0::MAM0_SPEC>,
    #[doc = "0x208 - Mailbox ID Register (MB = 0)"]
    pub mid0: crate::Reg<mid0::MID0_SPEC>,
    #[doc = "0x20c - Mailbox Family ID Register (MB = 0)"]
    pub mfid0: crate::Reg<mfid0::MFID0_SPEC>,
    #[doc = "0x210 - Mailbox Status Register (MB = 0)"]
    pub msr0: crate::Reg<msr0::MSR0_SPEC>,
    #[doc = "0x214 - Mailbox Data Low Register (MB = 0)"]
    pub mdl0: crate::Reg<mdl0::MDL0_SPEC>,
    #[doc = "0x218 - Mailbox Data High Register (MB = 0)"]
    pub mdh0: crate::Reg<mdh0::MDH0_SPEC>,
    #[doc = "0x21c - Mailbox Control Register (MB = 0)"]
    pub mcr0: crate::Reg<mcr0::MCR0_SPEC>,
    #[doc = "0x220 - Mailbox Mode Register (MB = 1)"]
    pub mmr1: crate::Reg<mmr1::MMR1_SPEC>,
    #[doc = "0x224 - Mailbox Acceptance Mask Register (MB = 1)"]
    pub mam1: crate::Reg<mam1::MAM1_SPEC>,
    #[doc = "0x228 - Mailbox ID Register (MB = 1)"]
    pub mid1: crate::Reg<mid1::MID1_SPEC>,
    #[doc = "0x22c - Mailbox Family ID Register (MB = 1)"]
    pub mfid1: crate::Reg<mfid1::MFID1_SPEC>,
    #[doc = "0x230 - Mailbox Status Register (MB = 1)"]
    pub msr1: crate::Reg<msr1::MSR1_SPEC>,
    #[doc = "0x234 - Mailbox Data Low Register (MB = 1)"]
    pub mdl1: crate::Reg<mdl1::MDL1_SPEC>,
    #[doc = "0x238 - Mailbox Data High Register (MB = 1)"]
    pub mdh1: crate::Reg<mdh1::MDH1_SPEC>,
    #[doc = "0x23c - Mailbox Control Register (MB = 1)"]
    pub mcr1: crate::Reg<mcr1::MCR1_SPEC>,
    #[doc = "0x240 - Mailbox Mode Register (MB = 2)"]
    pub mmr2: crate::Reg<mmr2::MMR2_SPEC>,
    #[doc = "0x244 - Mailbox Acceptance Mask Register (MB = 2)"]
    pub mam2: crate::Reg<mam2::MAM2_SPEC>,
    #[doc = "0x248 - Mailbox ID Register (MB = 2)"]
    pub mid2: crate::Reg<mid2::MID2_SPEC>,
    #[doc = "0x24c - Mailbox Family ID Register (MB = 2)"]
    pub mfid2: crate::Reg<mfid2::MFID2_SPEC>,
    #[doc = "0x250 - Mailbox Status Register (MB = 2)"]
    pub msr2: crate::Reg<msr2::MSR2_SPEC>,
    #[doc = "0x254 - Mailbox Data Low Register (MB = 2)"]
    pub mdl2: crate::Reg<mdl2::MDL2_SPEC>,
    #[doc = "0x258 - Mailbox Data High Register (MB = 2)"]
    pub mdh2: crate::Reg<mdh2::MDH2_SPEC>,
    #[doc = "0x25c - Mailbox Control Register (MB = 2)"]
    pub mcr2: crate::Reg<mcr2::MCR2_SPEC>,
    #[doc = "0x260 - Mailbox Mode Register (MB = 3)"]
    pub mmr3: crate::Reg<mmr3::MMR3_SPEC>,
    #[doc = "0x264 - Mailbox Acceptance Mask Register (MB = 3)"]
    pub mam3: crate::Reg<mam3::MAM3_SPEC>,
    #[doc = "0x268 - Mailbox ID Register (MB = 3)"]
    pub mid3: crate::Reg<mid3::MID3_SPEC>,
    #[doc = "0x26c - Mailbox Family ID Register (MB = 3)"]
    pub mfid3: crate::Reg<mfid3::MFID3_SPEC>,
    #[doc = "0x270 - Mailbox Status Register (MB = 3)"]
    pub msr3: crate::Reg<msr3::MSR3_SPEC>,
    #[doc = "0x274 - Mailbox Data Low Register (MB = 3)"]
    pub mdl3: crate::Reg<mdl3::MDL3_SPEC>,
    #[doc = "0x278 - Mailbox Data High Register (MB = 3)"]
    pub mdh3: crate::Reg<mdh3::MDH3_SPEC>,
    #[doc = "0x27c - Mailbox Control Register (MB = 3)"]
    pub mcr3: crate::Reg<mcr3::MCR3_SPEC>,
    #[doc = "0x280 - Mailbox Mode Register (MB = 4)"]
    pub mmr4: crate::Reg<mmr4::MMR4_SPEC>,
    #[doc = "0x284 - Mailbox Acceptance Mask Register (MB = 4)"]
    pub mam4: crate::Reg<mam4::MAM4_SPEC>,
    #[doc = "0x288 - Mailbox ID Register (MB = 4)"]
    pub mid4: crate::Reg<mid4::MID4_SPEC>,
    #[doc = "0x28c - Mailbox Family ID Register (MB = 4)"]
    pub mfid4: crate::Reg<mfid4::MFID4_SPEC>,
    #[doc = "0x290 - Mailbox Status Register (MB = 4)"]
    pub msr4: crate::Reg<msr4::MSR4_SPEC>,
    #[doc = "0x294 - Mailbox Data Low Register (MB = 4)"]
    pub mdl4: crate::Reg<mdl4::MDL4_SPEC>,
    #[doc = "0x298 - Mailbox Data High Register (MB = 4)"]
    pub mdh4: crate::Reg<mdh4::MDH4_SPEC>,
    #[doc = "0x29c - Mailbox Control Register (MB = 4)"]
    pub mcr4: crate::Reg<mcr4::MCR4_SPEC>,
    #[doc = "0x2a0 - Mailbox Mode Register (MB = 5)"]
    pub mmr5: crate::Reg<mmr5::MMR5_SPEC>,
    #[doc = "0x2a4 - Mailbox Acceptance Mask Register (MB = 5)"]
    pub mam5: crate::Reg<mam5::MAM5_SPEC>,
    #[doc = "0x2a8 - Mailbox ID Register (MB = 5)"]
    pub mid5: crate::Reg<mid5::MID5_SPEC>,
    #[doc = "0x2ac - Mailbox Family ID Register (MB = 5)"]
    pub mfid5: crate::Reg<mfid5::MFID5_SPEC>,
    #[doc = "0x2b0 - Mailbox Status Register (MB = 5)"]
    pub msr5: crate::Reg<msr5::MSR5_SPEC>,
    #[doc = "0x2b4 - Mailbox Data Low Register (MB = 5)"]
    pub mdl5: crate::Reg<mdl5::MDL5_SPEC>,
    #[doc = "0x2b8 - Mailbox Data High Register (MB = 5)"]
    pub mdh5: crate::Reg<mdh5::MDH5_SPEC>,
    #[doc = "0x2bc - Mailbox Control Register (MB = 5)"]
    pub mcr5: crate::Reg<mcr5::MCR5_SPEC>,
    #[doc = "0x2c0 - Mailbox Mode Register (MB = 6)"]
    pub mmr6: crate::Reg<mmr6::MMR6_SPEC>,
    #[doc = "0x2c4 - Mailbox Acceptance Mask Register (MB = 6)"]
    pub mam6: crate::Reg<mam6::MAM6_SPEC>,
    #[doc = "0x2c8 - Mailbox ID Register (MB = 6)"]
    pub mid6: crate::Reg<mid6::MID6_SPEC>,
    #[doc = "0x2cc - Mailbox Family ID Register (MB = 6)"]
    pub mfid6: crate::Reg<mfid6::MFID6_SPEC>,
    #[doc = "0x2d0 - Mailbox Status Register (MB = 6)"]
    pub msr6: crate::Reg<msr6::MSR6_SPEC>,
    #[doc = "0x2d4 - Mailbox Data Low Register (MB = 6)"]
    pub mdl6: crate::Reg<mdl6::MDL6_SPEC>,
    #[doc = "0x2d8 - Mailbox Data High Register (MB = 6)"]
    pub mdh6: crate::Reg<mdh6::MDH6_SPEC>,
    #[doc = "0x2dc - Mailbox Control Register (MB = 6)"]
    pub mcr6: crate::Reg<mcr6::MCR6_SPEC>,
    #[doc = "0x2e0 - Mailbox Mode Register (MB = 7)"]
    pub mmr7: crate::Reg<mmr7::MMR7_SPEC>,
    #[doc = "0x2e4 - Mailbox Acceptance Mask Register (MB = 7)"]
    pub mam7: crate::Reg<mam7::MAM7_SPEC>,
    #[doc = "0x2e8 - Mailbox ID Register (MB = 7)"]
    pub mid7: crate::Reg<mid7::MID7_SPEC>,
    #[doc = "0x2ec - Mailbox Family ID Register (MB = 7)"]
    pub mfid7: crate::Reg<mfid7::MFID7_SPEC>,
    #[doc = "0x2f0 - Mailbox Status Register (MB = 7)"]
    pub msr7: crate::Reg<msr7::MSR7_SPEC>,
    #[doc = "0x2f4 - Mailbox Data Low Register (MB = 7)"]
    pub mdl7: crate::Reg<mdl7::MDL7_SPEC>,
    #[doc = "0x2f8 - Mailbox Data High Register (MB = 7)"]
    pub mdh7: crate::Reg<mdh7::MDH7_SPEC>,
    #[doc = "0x2fc - Mailbox Control Register (MB = 7)"]
    pub mcr7: crate::Reg<mcr7::MCR7_SPEC>,
}
#[doc = "MR register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "BR register accessor: an alias for `Reg<BR_SPEC>`"]
pub type BR = crate::Reg<br::BR_SPEC>;
#[doc = "Baudrate Register"]
pub mod br;
#[doc = "TIM register accessor: an alias for `Reg<TIM_SPEC>`"]
pub type TIM = crate::Reg<tim::TIM_SPEC>;
#[doc = "Timer Register"]
pub mod tim;
#[doc = "TIMESTP register accessor: an alias for `Reg<TIMESTP_SPEC>`"]
pub type TIMESTP = crate::Reg<timestp::TIMESTP_SPEC>;
#[doc = "Timestamp Register"]
pub mod timestp;
#[doc = "ECR register accessor: an alias for `Reg<ECR_SPEC>`"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "Error Counter Register"]
pub mod ecr;
#[doc = "TCR register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Transfer Command Register"]
pub mod tcr;
#[doc = "ACR register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Abort Command Register"]
pub mod acr;
#[doc = "WPMR register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protect Status Register"]
pub mod wpsr;
#[doc = "MMR0 register accessor: an alias for `Reg<MMR0_SPEC>`"]
pub type MMR0 = crate::Reg<mmr0::MMR0_SPEC>;
#[doc = "Mailbox Mode Register (MB = 0)"]
pub mod mmr0;
#[doc = "MAM0 register accessor: an alias for `Reg<MAM0_SPEC>`"]
pub type MAM0 = crate::Reg<mam0::MAM0_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 0)"]
pub mod mam0;
#[doc = "MID0 register accessor: an alias for `Reg<MID0_SPEC>`"]
pub type MID0 = crate::Reg<mid0::MID0_SPEC>;
#[doc = "Mailbox ID Register (MB = 0)"]
pub mod mid0;
#[doc = "MFID0 register accessor: an alias for `Reg<MFID0_SPEC>`"]
pub type MFID0 = crate::Reg<mfid0::MFID0_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 0)"]
pub mod mfid0;
#[doc = "MSR0 register accessor: an alias for `Reg<MSR0_SPEC>`"]
pub type MSR0 = crate::Reg<msr0::MSR0_SPEC>;
#[doc = "Mailbox Status Register (MB = 0)"]
pub mod msr0;
#[doc = "MDL0 register accessor: an alias for `Reg<MDL0_SPEC>`"]
pub type MDL0 = crate::Reg<mdl0::MDL0_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 0)"]
pub mod mdl0;
#[doc = "MDH0 register accessor: an alias for `Reg<MDH0_SPEC>`"]
pub type MDH0 = crate::Reg<mdh0::MDH0_SPEC>;
#[doc = "Mailbox Data High Register (MB = 0)"]
pub mod mdh0;
#[doc = "MCR0 register accessor: an alias for `Reg<MCR0_SPEC>`"]
pub type MCR0 = crate::Reg<mcr0::MCR0_SPEC>;
#[doc = "Mailbox Control Register (MB = 0)"]
pub mod mcr0;
#[doc = "MMR1 register accessor: an alias for `Reg<MMR1_SPEC>`"]
pub type MMR1 = crate::Reg<mmr1::MMR1_SPEC>;
#[doc = "Mailbox Mode Register (MB = 1)"]
pub mod mmr1;
#[doc = "MAM1 register accessor: an alias for `Reg<MAM1_SPEC>`"]
pub type MAM1 = crate::Reg<mam1::MAM1_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 1)"]
pub mod mam1;
#[doc = "MID1 register accessor: an alias for `Reg<MID1_SPEC>`"]
pub type MID1 = crate::Reg<mid1::MID1_SPEC>;
#[doc = "Mailbox ID Register (MB = 1)"]
pub mod mid1;
#[doc = "MFID1 register accessor: an alias for `Reg<MFID1_SPEC>`"]
pub type MFID1 = crate::Reg<mfid1::MFID1_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 1)"]
pub mod mfid1;
#[doc = "MSR1 register accessor: an alias for `Reg<MSR1_SPEC>`"]
pub type MSR1 = crate::Reg<msr1::MSR1_SPEC>;
#[doc = "Mailbox Status Register (MB = 1)"]
pub mod msr1;
#[doc = "MDL1 register accessor: an alias for `Reg<MDL1_SPEC>`"]
pub type MDL1 = crate::Reg<mdl1::MDL1_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 1)"]
pub mod mdl1;
#[doc = "MDH1 register accessor: an alias for `Reg<MDH1_SPEC>`"]
pub type MDH1 = crate::Reg<mdh1::MDH1_SPEC>;
#[doc = "Mailbox Data High Register (MB = 1)"]
pub mod mdh1;
#[doc = "MCR1 register accessor: an alias for `Reg<MCR1_SPEC>`"]
pub type MCR1 = crate::Reg<mcr1::MCR1_SPEC>;
#[doc = "Mailbox Control Register (MB = 1)"]
pub mod mcr1;
#[doc = "MMR2 register accessor: an alias for `Reg<MMR2_SPEC>`"]
pub type MMR2 = crate::Reg<mmr2::MMR2_SPEC>;
#[doc = "Mailbox Mode Register (MB = 2)"]
pub mod mmr2;
#[doc = "MAM2 register accessor: an alias for `Reg<MAM2_SPEC>`"]
pub type MAM2 = crate::Reg<mam2::MAM2_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 2)"]
pub mod mam2;
#[doc = "MID2 register accessor: an alias for `Reg<MID2_SPEC>`"]
pub type MID2 = crate::Reg<mid2::MID2_SPEC>;
#[doc = "Mailbox ID Register (MB = 2)"]
pub mod mid2;
#[doc = "MFID2 register accessor: an alias for `Reg<MFID2_SPEC>`"]
pub type MFID2 = crate::Reg<mfid2::MFID2_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 2)"]
pub mod mfid2;
#[doc = "MSR2 register accessor: an alias for `Reg<MSR2_SPEC>`"]
pub type MSR2 = crate::Reg<msr2::MSR2_SPEC>;
#[doc = "Mailbox Status Register (MB = 2)"]
pub mod msr2;
#[doc = "MDL2 register accessor: an alias for `Reg<MDL2_SPEC>`"]
pub type MDL2 = crate::Reg<mdl2::MDL2_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 2)"]
pub mod mdl2;
#[doc = "MDH2 register accessor: an alias for `Reg<MDH2_SPEC>`"]
pub type MDH2 = crate::Reg<mdh2::MDH2_SPEC>;
#[doc = "Mailbox Data High Register (MB = 2)"]
pub mod mdh2;
#[doc = "MCR2 register accessor: an alias for `Reg<MCR2_SPEC>`"]
pub type MCR2 = crate::Reg<mcr2::MCR2_SPEC>;
#[doc = "Mailbox Control Register (MB = 2)"]
pub mod mcr2;
#[doc = "MMR3 register accessor: an alias for `Reg<MMR3_SPEC>`"]
pub type MMR3 = crate::Reg<mmr3::MMR3_SPEC>;
#[doc = "Mailbox Mode Register (MB = 3)"]
pub mod mmr3;
#[doc = "MAM3 register accessor: an alias for `Reg<MAM3_SPEC>`"]
pub type MAM3 = crate::Reg<mam3::MAM3_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 3)"]
pub mod mam3;
#[doc = "MID3 register accessor: an alias for `Reg<MID3_SPEC>`"]
pub type MID3 = crate::Reg<mid3::MID3_SPEC>;
#[doc = "Mailbox ID Register (MB = 3)"]
pub mod mid3;
#[doc = "MFID3 register accessor: an alias for `Reg<MFID3_SPEC>`"]
pub type MFID3 = crate::Reg<mfid3::MFID3_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 3)"]
pub mod mfid3;
#[doc = "MSR3 register accessor: an alias for `Reg<MSR3_SPEC>`"]
pub type MSR3 = crate::Reg<msr3::MSR3_SPEC>;
#[doc = "Mailbox Status Register (MB = 3)"]
pub mod msr3;
#[doc = "MDL3 register accessor: an alias for `Reg<MDL3_SPEC>`"]
pub type MDL3 = crate::Reg<mdl3::MDL3_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 3)"]
pub mod mdl3;
#[doc = "MDH3 register accessor: an alias for `Reg<MDH3_SPEC>`"]
pub type MDH3 = crate::Reg<mdh3::MDH3_SPEC>;
#[doc = "Mailbox Data High Register (MB = 3)"]
pub mod mdh3;
#[doc = "MCR3 register accessor: an alias for `Reg<MCR3_SPEC>`"]
pub type MCR3 = crate::Reg<mcr3::MCR3_SPEC>;
#[doc = "Mailbox Control Register (MB = 3)"]
pub mod mcr3;
#[doc = "MMR4 register accessor: an alias for `Reg<MMR4_SPEC>`"]
pub type MMR4 = crate::Reg<mmr4::MMR4_SPEC>;
#[doc = "Mailbox Mode Register (MB = 4)"]
pub mod mmr4;
#[doc = "MAM4 register accessor: an alias for `Reg<MAM4_SPEC>`"]
pub type MAM4 = crate::Reg<mam4::MAM4_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 4)"]
pub mod mam4;
#[doc = "MID4 register accessor: an alias for `Reg<MID4_SPEC>`"]
pub type MID4 = crate::Reg<mid4::MID4_SPEC>;
#[doc = "Mailbox ID Register (MB = 4)"]
pub mod mid4;
#[doc = "MFID4 register accessor: an alias for `Reg<MFID4_SPEC>`"]
pub type MFID4 = crate::Reg<mfid4::MFID4_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 4)"]
pub mod mfid4;
#[doc = "MSR4 register accessor: an alias for `Reg<MSR4_SPEC>`"]
pub type MSR4 = crate::Reg<msr4::MSR4_SPEC>;
#[doc = "Mailbox Status Register (MB = 4)"]
pub mod msr4;
#[doc = "MDL4 register accessor: an alias for `Reg<MDL4_SPEC>`"]
pub type MDL4 = crate::Reg<mdl4::MDL4_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 4)"]
pub mod mdl4;
#[doc = "MDH4 register accessor: an alias for `Reg<MDH4_SPEC>`"]
pub type MDH4 = crate::Reg<mdh4::MDH4_SPEC>;
#[doc = "Mailbox Data High Register (MB = 4)"]
pub mod mdh4;
#[doc = "MCR4 register accessor: an alias for `Reg<MCR4_SPEC>`"]
pub type MCR4 = crate::Reg<mcr4::MCR4_SPEC>;
#[doc = "Mailbox Control Register (MB = 4)"]
pub mod mcr4;
#[doc = "MMR5 register accessor: an alias for `Reg<MMR5_SPEC>`"]
pub type MMR5 = crate::Reg<mmr5::MMR5_SPEC>;
#[doc = "Mailbox Mode Register (MB = 5)"]
pub mod mmr5;
#[doc = "MAM5 register accessor: an alias for `Reg<MAM5_SPEC>`"]
pub type MAM5 = crate::Reg<mam5::MAM5_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 5)"]
pub mod mam5;
#[doc = "MID5 register accessor: an alias for `Reg<MID5_SPEC>`"]
pub type MID5 = crate::Reg<mid5::MID5_SPEC>;
#[doc = "Mailbox ID Register (MB = 5)"]
pub mod mid5;
#[doc = "MFID5 register accessor: an alias for `Reg<MFID5_SPEC>`"]
pub type MFID5 = crate::Reg<mfid5::MFID5_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 5)"]
pub mod mfid5;
#[doc = "MSR5 register accessor: an alias for `Reg<MSR5_SPEC>`"]
pub type MSR5 = crate::Reg<msr5::MSR5_SPEC>;
#[doc = "Mailbox Status Register (MB = 5)"]
pub mod msr5;
#[doc = "MDL5 register accessor: an alias for `Reg<MDL5_SPEC>`"]
pub type MDL5 = crate::Reg<mdl5::MDL5_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 5)"]
pub mod mdl5;
#[doc = "MDH5 register accessor: an alias for `Reg<MDH5_SPEC>`"]
pub type MDH5 = crate::Reg<mdh5::MDH5_SPEC>;
#[doc = "Mailbox Data High Register (MB = 5)"]
pub mod mdh5;
#[doc = "MCR5 register accessor: an alias for `Reg<MCR5_SPEC>`"]
pub type MCR5 = crate::Reg<mcr5::MCR5_SPEC>;
#[doc = "Mailbox Control Register (MB = 5)"]
pub mod mcr5;
#[doc = "MMR6 register accessor: an alias for `Reg<MMR6_SPEC>`"]
pub type MMR6 = crate::Reg<mmr6::MMR6_SPEC>;
#[doc = "Mailbox Mode Register (MB = 6)"]
pub mod mmr6;
#[doc = "MAM6 register accessor: an alias for `Reg<MAM6_SPEC>`"]
pub type MAM6 = crate::Reg<mam6::MAM6_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 6)"]
pub mod mam6;
#[doc = "MID6 register accessor: an alias for `Reg<MID6_SPEC>`"]
pub type MID6 = crate::Reg<mid6::MID6_SPEC>;
#[doc = "Mailbox ID Register (MB = 6)"]
pub mod mid6;
#[doc = "MFID6 register accessor: an alias for `Reg<MFID6_SPEC>`"]
pub type MFID6 = crate::Reg<mfid6::MFID6_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 6)"]
pub mod mfid6;
#[doc = "MSR6 register accessor: an alias for `Reg<MSR6_SPEC>`"]
pub type MSR6 = crate::Reg<msr6::MSR6_SPEC>;
#[doc = "Mailbox Status Register (MB = 6)"]
pub mod msr6;
#[doc = "MDL6 register accessor: an alias for `Reg<MDL6_SPEC>`"]
pub type MDL6 = crate::Reg<mdl6::MDL6_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 6)"]
pub mod mdl6;
#[doc = "MDH6 register accessor: an alias for `Reg<MDH6_SPEC>`"]
pub type MDH6 = crate::Reg<mdh6::MDH6_SPEC>;
#[doc = "Mailbox Data High Register (MB = 6)"]
pub mod mdh6;
#[doc = "MCR6 register accessor: an alias for `Reg<MCR6_SPEC>`"]
pub type MCR6 = crate::Reg<mcr6::MCR6_SPEC>;
#[doc = "Mailbox Control Register (MB = 6)"]
pub mod mcr6;
#[doc = "MMR7 register accessor: an alias for `Reg<MMR7_SPEC>`"]
pub type MMR7 = crate::Reg<mmr7::MMR7_SPEC>;
#[doc = "Mailbox Mode Register (MB = 7)"]
pub mod mmr7;
#[doc = "MAM7 register accessor: an alias for `Reg<MAM7_SPEC>`"]
pub type MAM7 = crate::Reg<mam7::MAM7_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 7)"]
pub mod mam7;
#[doc = "MID7 register accessor: an alias for `Reg<MID7_SPEC>`"]
pub type MID7 = crate::Reg<mid7::MID7_SPEC>;
#[doc = "Mailbox ID Register (MB = 7)"]
pub mod mid7;
#[doc = "MFID7 register accessor: an alias for `Reg<MFID7_SPEC>`"]
pub type MFID7 = crate::Reg<mfid7::MFID7_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 7)"]
pub mod mfid7;
#[doc = "MSR7 register accessor: an alias for `Reg<MSR7_SPEC>`"]
pub type MSR7 = crate::Reg<msr7::MSR7_SPEC>;
#[doc = "Mailbox Status Register (MB = 7)"]
pub mod msr7;
#[doc = "MDL7 register accessor: an alias for `Reg<MDL7_SPEC>`"]
pub type MDL7 = crate::Reg<mdl7::MDL7_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 7)"]
pub mod mdl7;
#[doc = "MDH7 register accessor: an alias for `Reg<MDH7_SPEC>`"]
pub type MDH7 = crate::Reg<mdh7::MDH7_SPEC>;
#[doc = "Mailbox Data High Register (MB = 7)"]
pub mod mdh7;
#[doc = "MCR7 register accessor: an alias for `Reg<MCR7_SPEC>`"]
pub type MCR7 = crate::Reg<mcr7::MCR7_SPEC>;
#[doc = "Mailbox Control Register (MB = 7)"]
pub mod mcr7;
