#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Mode Register"]
    pub mr: MR,
    #[doc = "0x04 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x08 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x0c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x10 - Status Register"]
    pub sr: SR,
    #[doc = "0x14 - Baudrate Register"]
    pub br: BR,
    #[doc = "0x18 - Timer Register"]
    pub tim: TIM,
    #[doc = "0x1c - Timestamp Register"]
    pub timestp: TIMESTP,
    #[doc = "0x20 - Error Counter Register"]
    pub ecr: ECR,
    #[doc = "0x24 - Transfer Command Register"]
    pub tcr: TCR,
    #[doc = "0x28 - Abort Command Register"]
    pub acr: ACR,
    _reserved11: [u8; 0xb8],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub wpsr: WPSR,
    _reserved13: [u8; 0x0114],
    #[doc = "0x200 - Mailbox Mode Register (MB = 0)"]
    pub mmr0: MMR0,
    #[doc = "0x204 - Mailbox Acceptance Mask Register (MB = 0)"]
    pub mam0: MAM0,
    #[doc = "0x208 - Mailbox ID Register (MB = 0)"]
    pub mid0: MID0,
    #[doc = "0x20c - Mailbox Family ID Register (MB = 0)"]
    pub mfid0: MFID0,
    #[doc = "0x210 - Mailbox Status Register (MB = 0)"]
    pub msr0: MSR0,
    #[doc = "0x214 - Mailbox Data Low Register (MB = 0)"]
    pub mdl0: MDL0,
    #[doc = "0x218 - Mailbox Data High Register (MB = 0)"]
    pub mdh0: MDH0,
    #[doc = "0x21c - Mailbox Control Register (MB = 0)"]
    pub mcr0: MCR0,
    #[doc = "0x220 - Mailbox Mode Register (MB = 1)"]
    pub mmr1: MMR1,
    #[doc = "0x224 - Mailbox Acceptance Mask Register (MB = 1)"]
    pub mam1: MAM1,
    #[doc = "0x228 - Mailbox ID Register (MB = 1)"]
    pub mid1: MID1,
    #[doc = "0x22c - Mailbox Family ID Register (MB = 1)"]
    pub mfid1: MFID1,
    #[doc = "0x230 - Mailbox Status Register (MB = 1)"]
    pub msr1: MSR1,
    #[doc = "0x234 - Mailbox Data Low Register (MB = 1)"]
    pub mdl1: MDL1,
    #[doc = "0x238 - Mailbox Data High Register (MB = 1)"]
    pub mdh1: MDH1,
    #[doc = "0x23c - Mailbox Control Register (MB = 1)"]
    pub mcr1: MCR1,
    #[doc = "0x240 - Mailbox Mode Register (MB = 2)"]
    pub mmr2: MMR2,
    #[doc = "0x244 - Mailbox Acceptance Mask Register (MB = 2)"]
    pub mam2: MAM2,
    #[doc = "0x248 - Mailbox ID Register (MB = 2)"]
    pub mid2: MID2,
    #[doc = "0x24c - Mailbox Family ID Register (MB = 2)"]
    pub mfid2: MFID2,
    #[doc = "0x250 - Mailbox Status Register (MB = 2)"]
    pub msr2: MSR2,
    #[doc = "0x254 - Mailbox Data Low Register (MB = 2)"]
    pub mdl2: MDL2,
    #[doc = "0x258 - Mailbox Data High Register (MB = 2)"]
    pub mdh2: MDH2,
    #[doc = "0x25c - Mailbox Control Register (MB = 2)"]
    pub mcr2: MCR2,
    #[doc = "0x260 - Mailbox Mode Register (MB = 3)"]
    pub mmr3: MMR3,
    #[doc = "0x264 - Mailbox Acceptance Mask Register (MB = 3)"]
    pub mam3: MAM3,
    #[doc = "0x268 - Mailbox ID Register (MB = 3)"]
    pub mid3: MID3,
    #[doc = "0x26c - Mailbox Family ID Register (MB = 3)"]
    pub mfid3: MFID3,
    #[doc = "0x270 - Mailbox Status Register (MB = 3)"]
    pub msr3: MSR3,
    #[doc = "0x274 - Mailbox Data Low Register (MB = 3)"]
    pub mdl3: MDL3,
    #[doc = "0x278 - Mailbox Data High Register (MB = 3)"]
    pub mdh3: MDH3,
    #[doc = "0x27c - Mailbox Control Register (MB = 3)"]
    pub mcr3: MCR3,
    #[doc = "0x280 - Mailbox Mode Register (MB = 4)"]
    pub mmr4: MMR4,
    #[doc = "0x284 - Mailbox Acceptance Mask Register (MB = 4)"]
    pub mam4: MAM4,
    #[doc = "0x288 - Mailbox ID Register (MB = 4)"]
    pub mid4: MID4,
    #[doc = "0x28c - Mailbox Family ID Register (MB = 4)"]
    pub mfid4: MFID4,
    #[doc = "0x290 - Mailbox Status Register (MB = 4)"]
    pub msr4: MSR4,
    #[doc = "0x294 - Mailbox Data Low Register (MB = 4)"]
    pub mdl4: MDL4,
    #[doc = "0x298 - Mailbox Data High Register (MB = 4)"]
    pub mdh4: MDH4,
    #[doc = "0x29c - Mailbox Control Register (MB = 4)"]
    pub mcr4: MCR4,
    #[doc = "0x2a0 - Mailbox Mode Register (MB = 5)"]
    pub mmr5: MMR5,
    #[doc = "0x2a4 - Mailbox Acceptance Mask Register (MB = 5)"]
    pub mam5: MAM5,
    #[doc = "0x2a8 - Mailbox ID Register (MB = 5)"]
    pub mid5: MID5,
    #[doc = "0x2ac - Mailbox Family ID Register (MB = 5)"]
    pub mfid5: MFID5,
    #[doc = "0x2b0 - Mailbox Status Register (MB = 5)"]
    pub msr5: MSR5,
    #[doc = "0x2b4 - Mailbox Data Low Register (MB = 5)"]
    pub mdl5: MDL5,
    #[doc = "0x2b8 - Mailbox Data High Register (MB = 5)"]
    pub mdh5: MDH5,
    #[doc = "0x2bc - Mailbox Control Register (MB = 5)"]
    pub mcr5: MCR5,
    #[doc = "0x2c0 - Mailbox Mode Register (MB = 6)"]
    pub mmr6: MMR6,
    #[doc = "0x2c4 - Mailbox Acceptance Mask Register (MB = 6)"]
    pub mam6: MAM6,
    #[doc = "0x2c8 - Mailbox ID Register (MB = 6)"]
    pub mid6: MID6,
    #[doc = "0x2cc - Mailbox Family ID Register (MB = 6)"]
    pub mfid6: MFID6,
    #[doc = "0x2d0 - Mailbox Status Register (MB = 6)"]
    pub msr6: MSR6,
    #[doc = "0x2d4 - Mailbox Data Low Register (MB = 6)"]
    pub mdl6: MDL6,
    #[doc = "0x2d8 - Mailbox Data High Register (MB = 6)"]
    pub mdh6: MDH6,
    #[doc = "0x2dc - Mailbox Control Register (MB = 6)"]
    pub mcr6: MCR6,
    #[doc = "0x2e0 - Mailbox Mode Register (MB = 7)"]
    pub mmr7: MMR7,
    #[doc = "0x2e4 - Mailbox Acceptance Mask Register (MB = 7)"]
    pub mam7: MAM7,
    #[doc = "0x2e8 - Mailbox ID Register (MB = 7)"]
    pub mid7: MID7,
    #[doc = "0x2ec - Mailbox Family ID Register (MB = 7)"]
    pub mfid7: MFID7,
    #[doc = "0x2f0 - Mailbox Status Register (MB = 7)"]
    pub msr7: MSR7,
    #[doc = "0x2f4 - Mailbox Data Low Register (MB = 7)"]
    pub mdl7: MDL7,
    #[doc = "0x2f8 - Mailbox Data High Register (MB = 7)"]
    pub mdh7: MDH7,
    #[doc = "0x2fc - Mailbox Control Register (MB = 7)"]
    pub mcr7: MCR7,
}
#[doc = "MR (rw) register accessor: Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mr`]
module"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "IER (w) register accessor: Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`idr`]
module"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`imr`]
module"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "BR (rw) register accessor: Baudrate Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`br`]
module"]
pub type BR = crate::Reg<br::BR_SPEC>;
#[doc = "Baudrate Register"]
pub mod br;
#[doc = "TIM (r) register accessor: Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tim`]
module"]
pub type TIM = crate::Reg<tim::TIM_SPEC>;
#[doc = "Timer Register"]
pub mod tim;
#[doc = "TIMESTP (r) register accessor: Timestamp Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timestp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timestp`]
module"]
pub type TIMESTP = crate::Reg<timestp::TIMESTP_SPEC>;
#[doc = "Timestamp Register"]
pub mod timestp;
#[doc = "ECR (r) register accessor: Error Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ecr`]
module"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "Error Counter Register"]
pub mod ecr;
#[doc = "TCR (w) register accessor: Transfer Command Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tcr`]
module"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Transfer Command Register"]
pub mod tcr;
#[doc = "ACR (w) register accessor: Abort Command Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`acr`]
module"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Abort Command Register"]
pub mod acr;
#[doc = "WPMR (rw) register accessor: Write Protect Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wpmr`]
module"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: Write Protect Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wpsr`]
module"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protect Status Register"]
pub mod wpsr;
#[doc = "MMR0 (rw) register accessor: Mailbox Mode Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmr0`]
module"]
pub type MMR0 = crate::Reg<mmr0::MMR0_SPEC>;
#[doc = "Mailbox Mode Register (MB = 0)"]
pub mod mmr0;
#[doc = "MAM0 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mam0`]
module"]
pub type MAM0 = crate::Reg<mam0::MAM0_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 0)"]
pub mod mam0;
#[doc = "MID0 (rw) register accessor: Mailbox ID Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mid0`]
module"]
pub type MID0 = crate::Reg<mid0::MID0_SPEC>;
#[doc = "Mailbox ID Register (MB = 0)"]
pub mod mid0;
#[doc = "MFID0 (r) register accessor: Mailbox Family ID Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mfid0`]
module"]
pub type MFID0 = crate::Reg<mfid0::MFID0_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 0)"]
pub mod mfid0;
#[doc = "MSR0 (r) register accessor: Mailbox Status Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msr0`]
module"]
pub type MSR0 = crate::Reg<msr0::MSR0_SPEC>;
#[doc = "Mailbox Status Register (MB = 0)"]
pub mod msr0;
#[doc = "MDL0 (rw) register accessor: Mailbox Data Low Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdl0`]
module"]
pub type MDL0 = crate::Reg<mdl0::MDL0_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 0)"]
pub mod mdl0;
#[doc = "MDH0 (rw) register accessor: Mailbox Data High Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdh0`]
module"]
pub type MDH0 = crate::Reg<mdh0::MDH0_SPEC>;
#[doc = "Mailbox Data High Register (MB = 0)"]
pub mod mdh0;
#[doc = "MCR0 (w) register accessor: Mailbox Control Register (MB = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mcr0`]
module"]
pub type MCR0 = crate::Reg<mcr0::MCR0_SPEC>;
#[doc = "Mailbox Control Register (MB = 0)"]
pub mod mcr0;
#[doc = "MMR1 (rw) register accessor: Mailbox Mode Register (MB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmr1`]
module"]
pub type MMR1 = crate::Reg<mmr1::MMR1_SPEC>;
#[doc = "Mailbox Mode Register (MB = 1)"]
pub mod mmr1;
#[doc = "MAM1 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mam1`]
module"]
pub type MAM1 = crate::Reg<mam1::MAM1_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 1)"]
pub mod mam1;
#[doc = "MID1 (rw) register accessor: Mailbox ID Register (MB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mid1`]
module"]
pub type MID1 = crate::Reg<mid1::MID1_SPEC>;
#[doc = "Mailbox ID Register (MB = 1)"]
pub mod mid1;
#[doc = "MFID1 (r) register accessor: Mailbox Family ID Register (MB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mfid1`]
module"]
pub type MFID1 = crate::Reg<mfid1::MFID1_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 1)"]
pub mod mfid1;
#[doc = "MSR1 (r) register accessor: Mailbox Status Register (MB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msr1`]
module"]
pub type MSR1 = crate::Reg<msr1::MSR1_SPEC>;
#[doc = "Mailbox Status Register (MB = 1)"]
pub mod msr1;
#[doc = "MDL1 (rw) register accessor: Mailbox Data Low Register (MB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdl1`]
module"]
pub type MDL1 = crate::Reg<mdl1::MDL1_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 1)"]
pub mod mdl1;
#[doc = "MDH1 (rw) register accessor: Mailbox Data High Register (MB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdh1`]
module"]
pub type MDH1 = crate::Reg<mdh1::MDH1_SPEC>;
#[doc = "Mailbox Data High Register (MB = 1)"]
pub mod mdh1;
#[doc = "MCR1 (w) register accessor: Mailbox Control Register (MB = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mcr1`]
module"]
pub type MCR1 = crate::Reg<mcr1::MCR1_SPEC>;
#[doc = "Mailbox Control Register (MB = 1)"]
pub mod mcr1;
#[doc = "MMR2 (rw) register accessor: Mailbox Mode Register (MB = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmr2`]
module"]
pub type MMR2 = crate::Reg<mmr2::MMR2_SPEC>;
#[doc = "Mailbox Mode Register (MB = 2)"]
pub mod mmr2;
#[doc = "MAM2 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mam2`]
module"]
pub type MAM2 = crate::Reg<mam2::MAM2_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 2)"]
pub mod mam2;
#[doc = "MID2 (rw) register accessor: Mailbox ID Register (MB = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mid2`]
module"]
pub type MID2 = crate::Reg<mid2::MID2_SPEC>;
#[doc = "Mailbox ID Register (MB = 2)"]
pub mod mid2;
#[doc = "MFID2 (r) register accessor: Mailbox Family ID Register (MB = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mfid2`]
module"]
pub type MFID2 = crate::Reg<mfid2::MFID2_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 2)"]
pub mod mfid2;
#[doc = "MSR2 (r) register accessor: Mailbox Status Register (MB = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msr2`]
module"]
pub type MSR2 = crate::Reg<msr2::MSR2_SPEC>;
#[doc = "Mailbox Status Register (MB = 2)"]
pub mod msr2;
#[doc = "MDL2 (rw) register accessor: Mailbox Data Low Register (MB = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdl2`]
module"]
pub type MDL2 = crate::Reg<mdl2::MDL2_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 2)"]
pub mod mdl2;
#[doc = "MDH2 (rw) register accessor: Mailbox Data High Register (MB = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdh2`]
module"]
pub type MDH2 = crate::Reg<mdh2::MDH2_SPEC>;
#[doc = "Mailbox Data High Register (MB = 2)"]
pub mod mdh2;
#[doc = "MCR2 (w) register accessor: Mailbox Control Register (MB = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mcr2`]
module"]
pub type MCR2 = crate::Reg<mcr2::MCR2_SPEC>;
#[doc = "Mailbox Control Register (MB = 2)"]
pub mod mcr2;
#[doc = "MMR3 (rw) register accessor: Mailbox Mode Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmr3`]
module"]
pub type MMR3 = crate::Reg<mmr3::MMR3_SPEC>;
#[doc = "Mailbox Mode Register (MB = 3)"]
pub mod mmr3;
#[doc = "MAM3 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mam3`]
module"]
pub type MAM3 = crate::Reg<mam3::MAM3_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 3)"]
pub mod mam3;
#[doc = "MID3 (rw) register accessor: Mailbox ID Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mid3`]
module"]
pub type MID3 = crate::Reg<mid3::MID3_SPEC>;
#[doc = "Mailbox ID Register (MB = 3)"]
pub mod mid3;
#[doc = "MFID3 (r) register accessor: Mailbox Family ID Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mfid3`]
module"]
pub type MFID3 = crate::Reg<mfid3::MFID3_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 3)"]
pub mod mfid3;
#[doc = "MSR3 (r) register accessor: Mailbox Status Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msr3`]
module"]
pub type MSR3 = crate::Reg<msr3::MSR3_SPEC>;
#[doc = "Mailbox Status Register (MB = 3)"]
pub mod msr3;
#[doc = "MDL3 (rw) register accessor: Mailbox Data Low Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdl3`]
module"]
pub type MDL3 = crate::Reg<mdl3::MDL3_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 3)"]
pub mod mdl3;
#[doc = "MDH3 (rw) register accessor: Mailbox Data High Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdh3`]
module"]
pub type MDH3 = crate::Reg<mdh3::MDH3_SPEC>;
#[doc = "Mailbox Data High Register (MB = 3)"]
pub mod mdh3;
#[doc = "MCR3 (w) register accessor: Mailbox Control Register (MB = 3)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mcr3`]
module"]
pub type MCR3 = crate::Reg<mcr3::MCR3_SPEC>;
#[doc = "Mailbox Control Register (MB = 3)"]
pub mod mcr3;
#[doc = "MMR4 (rw) register accessor: Mailbox Mode Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmr4`]
module"]
pub type MMR4 = crate::Reg<mmr4::MMR4_SPEC>;
#[doc = "Mailbox Mode Register (MB = 4)"]
pub mod mmr4;
#[doc = "MAM4 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mam4`]
module"]
pub type MAM4 = crate::Reg<mam4::MAM4_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 4)"]
pub mod mam4;
#[doc = "MID4 (rw) register accessor: Mailbox ID Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mid4`]
module"]
pub type MID4 = crate::Reg<mid4::MID4_SPEC>;
#[doc = "Mailbox ID Register (MB = 4)"]
pub mod mid4;
#[doc = "MFID4 (r) register accessor: Mailbox Family ID Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mfid4`]
module"]
pub type MFID4 = crate::Reg<mfid4::MFID4_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 4)"]
pub mod mfid4;
#[doc = "MSR4 (r) register accessor: Mailbox Status Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msr4`]
module"]
pub type MSR4 = crate::Reg<msr4::MSR4_SPEC>;
#[doc = "Mailbox Status Register (MB = 4)"]
pub mod msr4;
#[doc = "MDL4 (rw) register accessor: Mailbox Data Low Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdl4`]
module"]
pub type MDL4 = crate::Reg<mdl4::MDL4_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 4)"]
pub mod mdl4;
#[doc = "MDH4 (rw) register accessor: Mailbox Data High Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdh4`]
module"]
pub type MDH4 = crate::Reg<mdh4::MDH4_SPEC>;
#[doc = "Mailbox Data High Register (MB = 4)"]
pub mod mdh4;
#[doc = "MCR4 (w) register accessor: Mailbox Control Register (MB = 4)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mcr4`]
module"]
pub type MCR4 = crate::Reg<mcr4::MCR4_SPEC>;
#[doc = "Mailbox Control Register (MB = 4)"]
pub mod mcr4;
#[doc = "MMR5 (rw) register accessor: Mailbox Mode Register (MB = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmr5`]
module"]
pub type MMR5 = crate::Reg<mmr5::MMR5_SPEC>;
#[doc = "Mailbox Mode Register (MB = 5)"]
pub mod mmr5;
#[doc = "MAM5 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mam5`]
module"]
pub type MAM5 = crate::Reg<mam5::MAM5_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 5)"]
pub mod mam5;
#[doc = "MID5 (rw) register accessor: Mailbox ID Register (MB = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mid5`]
module"]
pub type MID5 = crate::Reg<mid5::MID5_SPEC>;
#[doc = "Mailbox ID Register (MB = 5)"]
pub mod mid5;
#[doc = "MFID5 (r) register accessor: Mailbox Family ID Register (MB = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mfid5`]
module"]
pub type MFID5 = crate::Reg<mfid5::MFID5_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 5)"]
pub mod mfid5;
#[doc = "MSR5 (r) register accessor: Mailbox Status Register (MB = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msr5`]
module"]
pub type MSR5 = crate::Reg<msr5::MSR5_SPEC>;
#[doc = "Mailbox Status Register (MB = 5)"]
pub mod msr5;
#[doc = "MDL5 (rw) register accessor: Mailbox Data Low Register (MB = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdl5`]
module"]
pub type MDL5 = crate::Reg<mdl5::MDL5_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 5)"]
pub mod mdl5;
#[doc = "MDH5 (rw) register accessor: Mailbox Data High Register (MB = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdh5`]
module"]
pub type MDH5 = crate::Reg<mdh5::MDH5_SPEC>;
#[doc = "Mailbox Data High Register (MB = 5)"]
pub mod mdh5;
#[doc = "MCR5 (w) register accessor: Mailbox Control Register (MB = 5)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mcr5`]
module"]
pub type MCR5 = crate::Reg<mcr5::MCR5_SPEC>;
#[doc = "Mailbox Control Register (MB = 5)"]
pub mod mcr5;
#[doc = "MMR6 (rw) register accessor: Mailbox Mode Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmr6`]
module"]
pub type MMR6 = crate::Reg<mmr6::MMR6_SPEC>;
#[doc = "Mailbox Mode Register (MB = 6)"]
pub mod mmr6;
#[doc = "MAM6 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mam6`]
module"]
pub type MAM6 = crate::Reg<mam6::MAM6_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 6)"]
pub mod mam6;
#[doc = "MID6 (rw) register accessor: Mailbox ID Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mid6`]
module"]
pub type MID6 = crate::Reg<mid6::MID6_SPEC>;
#[doc = "Mailbox ID Register (MB = 6)"]
pub mod mid6;
#[doc = "MFID6 (r) register accessor: Mailbox Family ID Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mfid6`]
module"]
pub type MFID6 = crate::Reg<mfid6::MFID6_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 6)"]
pub mod mfid6;
#[doc = "MSR6 (r) register accessor: Mailbox Status Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msr6`]
module"]
pub type MSR6 = crate::Reg<msr6::MSR6_SPEC>;
#[doc = "Mailbox Status Register (MB = 6)"]
pub mod msr6;
#[doc = "MDL6 (rw) register accessor: Mailbox Data Low Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdl6`]
module"]
pub type MDL6 = crate::Reg<mdl6::MDL6_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 6)"]
pub mod mdl6;
#[doc = "MDH6 (rw) register accessor: Mailbox Data High Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdh6`]
module"]
pub type MDH6 = crate::Reg<mdh6::MDH6_SPEC>;
#[doc = "Mailbox Data High Register (MB = 6)"]
pub mod mdh6;
#[doc = "MCR6 (w) register accessor: Mailbox Control Register (MB = 6)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mcr6`]
module"]
pub type MCR6 = crate::Reg<mcr6::MCR6_SPEC>;
#[doc = "Mailbox Control Register (MB = 6)"]
pub mod mcr6;
#[doc = "MMR7 (rw) register accessor: Mailbox Mode Register (MB = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmr7`]
module"]
pub type MMR7 = crate::Reg<mmr7::MMR7_SPEC>;
#[doc = "Mailbox Mode Register (MB = 7)"]
pub mod mmr7;
#[doc = "MAM7 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mam7`]
module"]
pub type MAM7 = crate::Reg<mam7::MAM7_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 7)"]
pub mod mam7;
#[doc = "MID7 (rw) register accessor: Mailbox ID Register (MB = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mid7`]
module"]
pub type MID7 = crate::Reg<mid7::MID7_SPEC>;
#[doc = "Mailbox ID Register (MB = 7)"]
pub mod mid7;
#[doc = "MFID7 (r) register accessor: Mailbox Family ID Register (MB = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mfid7`]
module"]
pub type MFID7 = crate::Reg<mfid7::MFID7_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 7)"]
pub mod mfid7;
#[doc = "MSR7 (r) register accessor: Mailbox Status Register (MB = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msr7`]
module"]
pub type MSR7 = crate::Reg<msr7::MSR7_SPEC>;
#[doc = "Mailbox Status Register (MB = 7)"]
pub mod msr7;
#[doc = "MDL7 (rw) register accessor: Mailbox Data Low Register (MB = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdl7`]
module"]
pub type MDL7 = crate::Reg<mdl7::MDL7_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 7)"]
pub mod mdl7;
#[doc = "MDH7 (rw) register accessor: Mailbox Data High Register (MB = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdh7`]
module"]
pub type MDH7 = crate::Reg<mdh7::MDH7_SPEC>;
#[doc = "Mailbox Data High Register (MB = 7)"]
pub mod mdh7;
#[doc = "MCR7 (w) register accessor: Mailbox Control Register (MB = 7)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mcr7`]
module"]
pub type MCR7 = crate::Reg<mcr7::MCR7_SPEC>;
#[doc = "Mailbox Control Register (MB = 7)"]
pub mod mcr7;
