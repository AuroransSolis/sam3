#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    mr: MR,
    ier: IER,
    idr: IDR,
    imr: IMR,
    sr: SR,
    br: BR,
    tim: TIM,
    timestp: TIMESTP,
    ecr: ECR,
    tcr: TCR,
    acr: ACR,
    _reserved11: [u8; 0xb8],
    wpmr: WPMR,
    wpsr: WPSR,
    _reserved13: [u8; 0x0114],
    mmr0: MMR0,
    mam0: MAM0,
    mid0: MID0,
    mfid0: MFID0,
    msr0: MSR0,
    mdl0: MDL0,
    mdh0: MDH0,
    mcr0: MCR0,
    mmr1: MMR1,
    mam1: MAM1,
    mid1: MID1,
    mfid1: MFID1,
    msr1: MSR1,
    mdl1: MDL1,
    mdh1: MDH1,
    mcr1: MCR1,
    mmr2: MMR2,
    mam2: MAM2,
    mid2: MID2,
    mfid2: MFID2,
    msr2: MSR2,
    mdl2: MDL2,
    mdh2: MDH2,
    mcr2: MCR2,
    mmr3: MMR3,
    mam3: MAM3,
    mid3: MID3,
    mfid3: MFID3,
    msr3: MSR3,
    mdl3: MDL3,
    mdh3: MDH3,
    mcr3: MCR3,
    mmr4: MMR4,
    mam4: MAM4,
    mid4: MID4,
    mfid4: MFID4,
    msr4: MSR4,
    mdl4: MDL4,
    mdh4: MDH4,
    mcr4: MCR4,
    mmr5: MMR5,
    mam5: MAM5,
    mid5: MID5,
    mfid5: MFID5,
    msr5: MSR5,
    mdl5: MDL5,
    mdh5: MDH5,
    mcr5: MCR5,
    mmr6: MMR6,
    mam6: MAM6,
    mid6: MID6,
    mfid6: MFID6,
    msr6: MSR6,
    mdl6: MDL6,
    mdh6: MDH6,
    mcr6: MCR6,
    mmr7: MMR7,
    mam7: MAM7,
    mid7: MID7,
    mfid7: MFID7,
    msr7: MSR7,
    mdl7: MDL7,
    mdh7: MDH7,
    mcr7: MCR7,
}
impl RegisterBlock {
    #[doc = "0x00 - Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &MR {
        &self.mr
    }
    #[doc = "0x04 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x08 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    #[doc = "0x0c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    #[doc = "0x10 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x14 - Baudrate Register"]
    #[inline(always)]
    pub const fn br(&self) -> &BR {
        &self.br
    }
    #[doc = "0x18 - Timer Register"]
    #[inline(always)]
    pub const fn tim(&self) -> &TIM {
        &self.tim
    }
    #[doc = "0x1c - Timestamp Register"]
    #[inline(always)]
    pub const fn timestp(&self) -> &TIMESTP {
        &self.timestp
    }
    #[doc = "0x20 - Error Counter Register"]
    #[inline(always)]
    pub const fn ecr(&self) -> &ECR {
        &self.ecr
    }
    #[doc = "0x24 - Transfer Command Register"]
    #[inline(always)]
    pub const fn tcr(&self) -> &TCR {
        &self.tcr
    }
    #[doc = "0x28 - Abort Command Register"]
    #[inline(always)]
    pub const fn acr(&self) -> &ACR {
        &self.acr
    }
    #[doc = "0xe4 - Write Protect Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &WPMR {
        &self.wpmr
    }
    #[doc = "0xe8 - Write Protect Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &WPSR {
        &self.wpsr
    }
    #[doc = "0x200 - Mailbox Mode Register (MB = 0)"]
    #[inline(always)]
    pub const fn mmr0(&self) -> &MMR0 {
        &self.mmr0
    }
    #[doc = "0x204 - Mailbox Acceptance Mask Register (MB = 0)"]
    #[inline(always)]
    pub const fn mam0(&self) -> &MAM0 {
        &self.mam0
    }
    #[doc = "0x208 - Mailbox ID Register (MB = 0)"]
    #[inline(always)]
    pub const fn mid0(&self) -> &MID0 {
        &self.mid0
    }
    #[doc = "0x20c - Mailbox Family ID Register (MB = 0)"]
    #[inline(always)]
    pub const fn mfid0(&self) -> &MFID0 {
        &self.mfid0
    }
    #[doc = "0x210 - Mailbox Status Register (MB = 0)"]
    #[inline(always)]
    pub const fn msr0(&self) -> &MSR0 {
        &self.msr0
    }
    #[doc = "0x214 - Mailbox Data Low Register (MB = 0)"]
    #[inline(always)]
    pub const fn mdl0(&self) -> &MDL0 {
        &self.mdl0
    }
    #[doc = "0x218 - Mailbox Data High Register (MB = 0)"]
    #[inline(always)]
    pub const fn mdh0(&self) -> &MDH0 {
        &self.mdh0
    }
    #[doc = "0x21c - Mailbox Control Register (MB = 0)"]
    #[inline(always)]
    pub const fn mcr0(&self) -> &MCR0 {
        &self.mcr0
    }
    #[doc = "0x220 - Mailbox Mode Register (MB = 1)"]
    #[inline(always)]
    pub const fn mmr1(&self) -> &MMR1 {
        &self.mmr1
    }
    #[doc = "0x224 - Mailbox Acceptance Mask Register (MB = 1)"]
    #[inline(always)]
    pub const fn mam1(&self) -> &MAM1 {
        &self.mam1
    }
    #[doc = "0x228 - Mailbox ID Register (MB = 1)"]
    #[inline(always)]
    pub const fn mid1(&self) -> &MID1 {
        &self.mid1
    }
    #[doc = "0x22c - Mailbox Family ID Register (MB = 1)"]
    #[inline(always)]
    pub const fn mfid1(&self) -> &MFID1 {
        &self.mfid1
    }
    #[doc = "0x230 - Mailbox Status Register (MB = 1)"]
    #[inline(always)]
    pub const fn msr1(&self) -> &MSR1 {
        &self.msr1
    }
    #[doc = "0x234 - Mailbox Data Low Register (MB = 1)"]
    #[inline(always)]
    pub const fn mdl1(&self) -> &MDL1 {
        &self.mdl1
    }
    #[doc = "0x238 - Mailbox Data High Register (MB = 1)"]
    #[inline(always)]
    pub const fn mdh1(&self) -> &MDH1 {
        &self.mdh1
    }
    #[doc = "0x23c - Mailbox Control Register (MB = 1)"]
    #[inline(always)]
    pub const fn mcr1(&self) -> &MCR1 {
        &self.mcr1
    }
    #[doc = "0x240 - Mailbox Mode Register (MB = 2)"]
    #[inline(always)]
    pub const fn mmr2(&self) -> &MMR2 {
        &self.mmr2
    }
    #[doc = "0x244 - Mailbox Acceptance Mask Register (MB = 2)"]
    #[inline(always)]
    pub const fn mam2(&self) -> &MAM2 {
        &self.mam2
    }
    #[doc = "0x248 - Mailbox ID Register (MB = 2)"]
    #[inline(always)]
    pub const fn mid2(&self) -> &MID2 {
        &self.mid2
    }
    #[doc = "0x24c - Mailbox Family ID Register (MB = 2)"]
    #[inline(always)]
    pub const fn mfid2(&self) -> &MFID2 {
        &self.mfid2
    }
    #[doc = "0x250 - Mailbox Status Register (MB = 2)"]
    #[inline(always)]
    pub const fn msr2(&self) -> &MSR2 {
        &self.msr2
    }
    #[doc = "0x254 - Mailbox Data Low Register (MB = 2)"]
    #[inline(always)]
    pub const fn mdl2(&self) -> &MDL2 {
        &self.mdl2
    }
    #[doc = "0x258 - Mailbox Data High Register (MB = 2)"]
    #[inline(always)]
    pub const fn mdh2(&self) -> &MDH2 {
        &self.mdh2
    }
    #[doc = "0x25c - Mailbox Control Register (MB = 2)"]
    #[inline(always)]
    pub const fn mcr2(&self) -> &MCR2 {
        &self.mcr2
    }
    #[doc = "0x260 - Mailbox Mode Register (MB = 3)"]
    #[inline(always)]
    pub const fn mmr3(&self) -> &MMR3 {
        &self.mmr3
    }
    #[doc = "0x264 - Mailbox Acceptance Mask Register (MB = 3)"]
    #[inline(always)]
    pub const fn mam3(&self) -> &MAM3 {
        &self.mam3
    }
    #[doc = "0x268 - Mailbox ID Register (MB = 3)"]
    #[inline(always)]
    pub const fn mid3(&self) -> &MID3 {
        &self.mid3
    }
    #[doc = "0x26c - Mailbox Family ID Register (MB = 3)"]
    #[inline(always)]
    pub const fn mfid3(&self) -> &MFID3 {
        &self.mfid3
    }
    #[doc = "0x270 - Mailbox Status Register (MB = 3)"]
    #[inline(always)]
    pub const fn msr3(&self) -> &MSR3 {
        &self.msr3
    }
    #[doc = "0x274 - Mailbox Data Low Register (MB = 3)"]
    #[inline(always)]
    pub const fn mdl3(&self) -> &MDL3 {
        &self.mdl3
    }
    #[doc = "0x278 - Mailbox Data High Register (MB = 3)"]
    #[inline(always)]
    pub const fn mdh3(&self) -> &MDH3 {
        &self.mdh3
    }
    #[doc = "0x27c - Mailbox Control Register (MB = 3)"]
    #[inline(always)]
    pub const fn mcr3(&self) -> &MCR3 {
        &self.mcr3
    }
    #[doc = "0x280 - Mailbox Mode Register (MB = 4)"]
    #[inline(always)]
    pub const fn mmr4(&self) -> &MMR4 {
        &self.mmr4
    }
    #[doc = "0x284 - Mailbox Acceptance Mask Register (MB = 4)"]
    #[inline(always)]
    pub const fn mam4(&self) -> &MAM4 {
        &self.mam4
    }
    #[doc = "0x288 - Mailbox ID Register (MB = 4)"]
    #[inline(always)]
    pub const fn mid4(&self) -> &MID4 {
        &self.mid4
    }
    #[doc = "0x28c - Mailbox Family ID Register (MB = 4)"]
    #[inline(always)]
    pub const fn mfid4(&self) -> &MFID4 {
        &self.mfid4
    }
    #[doc = "0x290 - Mailbox Status Register (MB = 4)"]
    #[inline(always)]
    pub const fn msr4(&self) -> &MSR4 {
        &self.msr4
    }
    #[doc = "0x294 - Mailbox Data Low Register (MB = 4)"]
    #[inline(always)]
    pub const fn mdl4(&self) -> &MDL4 {
        &self.mdl4
    }
    #[doc = "0x298 - Mailbox Data High Register (MB = 4)"]
    #[inline(always)]
    pub const fn mdh4(&self) -> &MDH4 {
        &self.mdh4
    }
    #[doc = "0x29c - Mailbox Control Register (MB = 4)"]
    #[inline(always)]
    pub const fn mcr4(&self) -> &MCR4 {
        &self.mcr4
    }
    #[doc = "0x2a0 - Mailbox Mode Register (MB = 5)"]
    #[inline(always)]
    pub const fn mmr5(&self) -> &MMR5 {
        &self.mmr5
    }
    #[doc = "0x2a4 - Mailbox Acceptance Mask Register (MB = 5)"]
    #[inline(always)]
    pub const fn mam5(&self) -> &MAM5 {
        &self.mam5
    }
    #[doc = "0x2a8 - Mailbox ID Register (MB = 5)"]
    #[inline(always)]
    pub const fn mid5(&self) -> &MID5 {
        &self.mid5
    }
    #[doc = "0x2ac - Mailbox Family ID Register (MB = 5)"]
    #[inline(always)]
    pub const fn mfid5(&self) -> &MFID5 {
        &self.mfid5
    }
    #[doc = "0x2b0 - Mailbox Status Register (MB = 5)"]
    #[inline(always)]
    pub const fn msr5(&self) -> &MSR5 {
        &self.msr5
    }
    #[doc = "0x2b4 - Mailbox Data Low Register (MB = 5)"]
    #[inline(always)]
    pub const fn mdl5(&self) -> &MDL5 {
        &self.mdl5
    }
    #[doc = "0x2b8 - Mailbox Data High Register (MB = 5)"]
    #[inline(always)]
    pub const fn mdh5(&self) -> &MDH5 {
        &self.mdh5
    }
    #[doc = "0x2bc - Mailbox Control Register (MB = 5)"]
    #[inline(always)]
    pub const fn mcr5(&self) -> &MCR5 {
        &self.mcr5
    }
    #[doc = "0x2c0 - Mailbox Mode Register (MB = 6)"]
    #[inline(always)]
    pub const fn mmr6(&self) -> &MMR6 {
        &self.mmr6
    }
    #[doc = "0x2c4 - Mailbox Acceptance Mask Register (MB = 6)"]
    #[inline(always)]
    pub const fn mam6(&self) -> &MAM6 {
        &self.mam6
    }
    #[doc = "0x2c8 - Mailbox ID Register (MB = 6)"]
    #[inline(always)]
    pub const fn mid6(&self) -> &MID6 {
        &self.mid6
    }
    #[doc = "0x2cc - Mailbox Family ID Register (MB = 6)"]
    #[inline(always)]
    pub const fn mfid6(&self) -> &MFID6 {
        &self.mfid6
    }
    #[doc = "0x2d0 - Mailbox Status Register (MB = 6)"]
    #[inline(always)]
    pub const fn msr6(&self) -> &MSR6 {
        &self.msr6
    }
    #[doc = "0x2d4 - Mailbox Data Low Register (MB = 6)"]
    #[inline(always)]
    pub const fn mdl6(&self) -> &MDL6 {
        &self.mdl6
    }
    #[doc = "0x2d8 - Mailbox Data High Register (MB = 6)"]
    #[inline(always)]
    pub const fn mdh6(&self) -> &MDH6 {
        &self.mdh6
    }
    #[doc = "0x2dc - Mailbox Control Register (MB = 6)"]
    #[inline(always)]
    pub const fn mcr6(&self) -> &MCR6 {
        &self.mcr6
    }
    #[doc = "0x2e0 - Mailbox Mode Register (MB = 7)"]
    #[inline(always)]
    pub const fn mmr7(&self) -> &MMR7 {
        &self.mmr7
    }
    #[doc = "0x2e4 - Mailbox Acceptance Mask Register (MB = 7)"]
    #[inline(always)]
    pub const fn mam7(&self) -> &MAM7 {
        &self.mam7
    }
    #[doc = "0x2e8 - Mailbox ID Register (MB = 7)"]
    #[inline(always)]
    pub const fn mid7(&self) -> &MID7 {
        &self.mid7
    }
    #[doc = "0x2ec - Mailbox Family ID Register (MB = 7)"]
    #[inline(always)]
    pub const fn mfid7(&self) -> &MFID7 {
        &self.mfid7
    }
    #[doc = "0x2f0 - Mailbox Status Register (MB = 7)"]
    #[inline(always)]
    pub const fn msr7(&self) -> &MSR7 {
        &self.msr7
    }
    #[doc = "0x2f4 - Mailbox Data Low Register (MB = 7)"]
    #[inline(always)]
    pub const fn mdl7(&self) -> &MDL7 {
        &self.mdl7
    }
    #[doc = "0x2f8 - Mailbox Data High Register (MB = 7)"]
    #[inline(always)]
    pub const fn mdh7(&self) -> &MDH7 {
        &self.mdh7
    }
    #[doc = "0x2fc - Mailbox Control Register (MB = 7)"]
    #[inline(always)]
    pub const fn mcr7(&self) -> &MCR7 {
        &self.mcr7
    }
}
#[doc = "MR (rw) register accessor: Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`]
module"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "IER (w) register accessor: Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "BR (rw) register accessor: Baudrate Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@br`]
module"]
pub type BR = crate::Reg<br::BR_SPEC>;
#[doc = "Baudrate Register"]
pub mod br;
#[doc = "TIM (r) register accessor: Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim`]
module"]
pub type TIM = crate::Reg<tim::TIM_SPEC>;
#[doc = "Timer Register"]
pub mod tim;
#[doc = "TIMESTP (r) register accessor: Timestamp Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timestp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timestp`]
module"]
pub type TIMESTP = crate::Reg<timestp::TIMESTP_SPEC>;
#[doc = "Timestamp Register"]
pub mod timestp;
#[doc = "ECR (r) register accessor: Error Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`]
module"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "Error Counter Register"]
pub mod ecr;
#[doc = "TCR (w) register accessor: Transfer Command Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr`]
module"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Transfer Command Register"]
pub mod tcr;
#[doc = "ACR (w) register accessor: Abort Command Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`]
module"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Abort Command Register"]
pub mod acr;
#[doc = "WPMR (rw) register accessor: Write Protect Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`]
module"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: Write Protect Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`]
module"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protect Status Register"]
pub mod wpsr;
#[doc = "MMR0 (rw) register accessor: Mailbox Mode Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr0`]
module"]
pub type MMR0 = crate::Reg<mmr0::MMR0_SPEC>;
#[doc = "Mailbox Mode Register (MB = 0)"]
pub mod mmr0;
#[doc = "MAM0 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mam0`]
module"]
pub type MAM0 = crate::Reg<mam0::MAM0_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 0)"]
pub mod mam0;
#[doc = "MID0 (rw) register accessor: Mailbox ID Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mid0`]
module"]
pub type MID0 = crate::Reg<mid0::MID0_SPEC>;
#[doc = "Mailbox ID Register (MB = 0)"]
pub mod mid0;
#[doc = "MFID0 (r) register accessor: Mailbox Family ID Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfid0`]
module"]
pub type MFID0 = crate::Reg<mfid0::MFID0_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 0)"]
pub mod mfid0;
#[doc = "MSR0 (r) register accessor: Mailbox Status Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr0`]
module"]
pub type MSR0 = crate::Reg<msr0::MSR0_SPEC>;
#[doc = "Mailbox Status Register (MB = 0)"]
pub mod msr0;
#[doc = "MDL0 (rw) register accessor: Mailbox Data Low Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdl0`]
module"]
pub type MDL0 = crate::Reg<mdl0::MDL0_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 0)"]
pub mod mdl0;
#[doc = "MDH0 (rw) register accessor: Mailbox Data High Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdh0`]
module"]
pub type MDH0 = crate::Reg<mdh0::MDH0_SPEC>;
#[doc = "Mailbox Data High Register (MB = 0)"]
pub mod mdh0;
#[doc = "MCR0 (w) register accessor: Mailbox Control Register (MB = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr0`]
module"]
pub type MCR0 = crate::Reg<mcr0::MCR0_SPEC>;
#[doc = "Mailbox Control Register (MB = 0)"]
pub mod mcr0;
#[doc = "MMR1 (rw) register accessor: Mailbox Mode Register (MB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr1`]
module"]
pub type MMR1 = crate::Reg<mmr1::MMR1_SPEC>;
#[doc = "Mailbox Mode Register (MB = 1)"]
pub mod mmr1;
#[doc = "MAM1 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mam1`]
module"]
pub type MAM1 = crate::Reg<mam1::MAM1_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 1)"]
pub mod mam1;
#[doc = "MID1 (rw) register accessor: Mailbox ID Register (MB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mid1`]
module"]
pub type MID1 = crate::Reg<mid1::MID1_SPEC>;
#[doc = "Mailbox ID Register (MB = 1)"]
pub mod mid1;
#[doc = "MFID1 (r) register accessor: Mailbox Family ID Register (MB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfid1`]
module"]
pub type MFID1 = crate::Reg<mfid1::MFID1_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 1)"]
pub mod mfid1;
#[doc = "MSR1 (r) register accessor: Mailbox Status Register (MB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr1`]
module"]
pub type MSR1 = crate::Reg<msr1::MSR1_SPEC>;
#[doc = "Mailbox Status Register (MB = 1)"]
pub mod msr1;
#[doc = "MDL1 (rw) register accessor: Mailbox Data Low Register (MB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdl1`]
module"]
pub type MDL1 = crate::Reg<mdl1::MDL1_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 1)"]
pub mod mdl1;
#[doc = "MDH1 (rw) register accessor: Mailbox Data High Register (MB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdh1`]
module"]
pub type MDH1 = crate::Reg<mdh1::MDH1_SPEC>;
#[doc = "Mailbox Data High Register (MB = 1)"]
pub mod mdh1;
#[doc = "MCR1 (w) register accessor: Mailbox Control Register (MB = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr1`]
module"]
pub type MCR1 = crate::Reg<mcr1::MCR1_SPEC>;
#[doc = "Mailbox Control Register (MB = 1)"]
pub mod mcr1;
#[doc = "MMR2 (rw) register accessor: Mailbox Mode Register (MB = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr2`]
module"]
pub type MMR2 = crate::Reg<mmr2::MMR2_SPEC>;
#[doc = "Mailbox Mode Register (MB = 2)"]
pub mod mmr2;
#[doc = "MAM2 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mam2`]
module"]
pub type MAM2 = crate::Reg<mam2::MAM2_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 2)"]
pub mod mam2;
#[doc = "MID2 (rw) register accessor: Mailbox ID Register (MB = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mid2`]
module"]
pub type MID2 = crate::Reg<mid2::MID2_SPEC>;
#[doc = "Mailbox ID Register (MB = 2)"]
pub mod mid2;
#[doc = "MFID2 (r) register accessor: Mailbox Family ID Register (MB = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfid2`]
module"]
pub type MFID2 = crate::Reg<mfid2::MFID2_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 2)"]
pub mod mfid2;
#[doc = "MSR2 (r) register accessor: Mailbox Status Register (MB = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr2`]
module"]
pub type MSR2 = crate::Reg<msr2::MSR2_SPEC>;
#[doc = "Mailbox Status Register (MB = 2)"]
pub mod msr2;
#[doc = "MDL2 (rw) register accessor: Mailbox Data Low Register (MB = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdl2`]
module"]
pub type MDL2 = crate::Reg<mdl2::MDL2_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 2)"]
pub mod mdl2;
#[doc = "MDH2 (rw) register accessor: Mailbox Data High Register (MB = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdh2`]
module"]
pub type MDH2 = crate::Reg<mdh2::MDH2_SPEC>;
#[doc = "Mailbox Data High Register (MB = 2)"]
pub mod mdh2;
#[doc = "MCR2 (w) register accessor: Mailbox Control Register (MB = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr2`]
module"]
pub type MCR2 = crate::Reg<mcr2::MCR2_SPEC>;
#[doc = "Mailbox Control Register (MB = 2)"]
pub mod mcr2;
#[doc = "MMR3 (rw) register accessor: Mailbox Mode Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr3`]
module"]
pub type MMR3 = crate::Reg<mmr3::MMR3_SPEC>;
#[doc = "Mailbox Mode Register (MB = 3)"]
pub mod mmr3;
#[doc = "MAM3 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mam3`]
module"]
pub type MAM3 = crate::Reg<mam3::MAM3_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 3)"]
pub mod mam3;
#[doc = "MID3 (rw) register accessor: Mailbox ID Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mid3`]
module"]
pub type MID3 = crate::Reg<mid3::MID3_SPEC>;
#[doc = "Mailbox ID Register (MB = 3)"]
pub mod mid3;
#[doc = "MFID3 (r) register accessor: Mailbox Family ID Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfid3`]
module"]
pub type MFID3 = crate::Reg<mfid3::MFID3_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 3)"]
pub mod mfid3;
#[doc = "MSR3 (r) register accessor: Mailbox Status Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr3`]
module"]
pub type MSR3 = crate::Reg<msr3::MSR3_SPEC>;
#[doc = "Mailbox Status Register (MB = 3)"]
pub mod msr3;
#[doc = "MDL3 (rw) register accessor: Mailbox Data Low Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdl3`]
module"]
pub type MDL3 = crate::Reg<mdl3::MDL3_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 3)"]
pub mod mdl3;
#[doc = "MDH3 (rw) register accessor: Mailbox Data High Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdh3`]
module"]
pub type MDH3 = crate::Reg<mdh3::MDH3_SPEC>;
#[doc = "Mailbox Data High Register (MB = 3)"]
pub mod mdh3;
#[doc = "MCR3 (w) register accessor: Mailbox Control Register (MB = 3)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr3`]
module"]
pub type MCR3 = crate::Reg<mcr3::MCR3_SPEC>;
#[doc = "Mailbox Control Register (MB = 3)"]
pub mod mcr3;
#[doc = "MMR4 (rw) register accessor: Mailbox Mode Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr4`]
module"]
pub type MMR4 = crate::Reg<mmr4::MMR4_SPEC>;
#[doc = "Mailbox Mode Register (MB = 4)"]
pub mod mmr4;
#[doc = "MAM4 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mam4`]
module"]
pub type MAM4 = crate::Reg<mam4::MAM4_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 4)"]
pub mod mam4;
#[doc = "MID4 (rw) register accessor: Mailbox ID Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mid4`]
module"]
pub type MID4 = crate::Reg<mid4::MID4_SPEC>;
#[doc = "Mailbox ID Register (MB = 4)"]
pub mod mid4;
#[doc = "MFID4 (r) register accessor: Mailbox Family ID Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfid4`]
module"]
pub type MFID4 = crate::Reg<mfid4::MFID4_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 4)"]
pub mod mfid4;
#[doc = "MSR4 (r) register accessor: Mailbox Status Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr4`]
module"]
pub type MSR4 = crate::Reg<msr4::MSR4_SPEC>;
#[doc = "Mailbox Status Register (MB = 4)"]
pub mod msr4;
#[doc = "MDL4 (rw) register accessor: Mailbox Data Low Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdl4`]
module"]
pub type MDL4 = crate::Reg<mdl4::MDL4_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 4)"]
pub mod mdl4;
#[doc = "MDH4 (rw) register accessor: Mailbox Data High Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdh4`]
module"]
pub type MDH4 = crate::Reg<mdh4::MDH4_SPEC>;
#[doc = "Mailbox Data High Register (MB = 4)"]
pub mod mdh4;
#[doc = "MCR4 (w) register accessor: Mailbox Control Register (MB = 4)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr4`]
module"]
pub type MCR4 = crate::Reg<mcr4::MCR4_SPEC>;
#[doc = "Mailbox Control Register (MB = 4)"]
pub mod mcr4;
#[doc = "MMR5 (rw) register accessor: Mailbox Mode Register (MB = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr5`]
module"]
pub type MMR5 = crate::Reg<mmr5::MMR5_SPEC>;
#[doc = "Mailbox Mode Register (MB = 5)"]
pub mod mmr5;
#[doc = "MAM5 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mam5`]
module"]
pub type MAM5 = crate::Reg<mam5::MAM5_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 5)"]
pub mod mam5;
#[doc = "MID5 (rw) register accessor: Mailbox ID Register (MB = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mid5`]
module"]
pub type MID5 = crate::Reg<mid5::MID5_SPEC>;
#[doc = "Mailbox ID Register (MB = 5)"]
pub mod mid5;
#[doc = "MFID5 (r) register accessor: Mailbox Family ID Register (MB = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfid5`]
module"]
pub type MFID5 = crate::Reg<mfid5::MFID5_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 5)"]
pub mod mfid5;
#[doc = "MSR5 (r) register accessor: Mailbox Status Register (MB = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr5`]
module"]
pub type MSR5 = crate::Reg<msr5::MSR5_SPEC>;
#[doc = "Mailbox Status Register (MB = 5)"]
pub mod msr5;
#[doc = "MDL5 (rw) register accessor: Mailbox Data Low Register (MB = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdl5`]
module"]
pub type MDL5 = crate::Reg<mdl5::MDL5_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 5)"]
pub mod mdl5;
#[doc = "MDH5 (rw) register accessor: Mailbox Data High Register (MB = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdh5`]
module"]
pub type MDH5 = crate::Reg<mdh5::MDH5_SPEC>;
#[doc = "Mailbox Data High Register (MB = 5)"]
pub mod mdh5;
#[doc = "MCR5 (w) register accessor: Mailbox Control Register (MB = 5)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr5`]
module"]
pub type MCR5 = crate::Reg<mcr5::MCR5_SPEC>;
#[doc = "Mailbox Control Register (MB = 5)"]
pub mod mcr5;
#[doc = "MMR6 (rw) register accessor: Mailbox Mode Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr6`]
module"]
pub type MMR6 = crate::Reg<mmr6::MMR6_SPEC>;
#[doc = "Mailbox Mode Register (MB = 6)"]
pub mod mmr6;
#[doc = "MAM6 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mam6`]
module"]
pub type MAM6 = crate::Reg<mam6::MAM6_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 6)"]
pub mod mam6;
#[doc = "MID6 (rw) register accessor: Mailbox ID Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mid6`]
module"]
pub type MID6 = crate::Reg<mid6::MID6_SPEC>;
#[doc = "Mailbox ID Register (MB = 6)"]
pub mod mid6;
#[doc = "MFID6 (r) register accessor: Mailbox Family ID Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfid6`]
module"]
pub type MFID6 = crate::Reg<mfid6::MFID6_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 6)"]
pub mod mfid6;
#[doc = "MSR6 (r) register accessor: Mailbox Status Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr6`]
module"]
pub type MSR6 = crate::Reg<msr6::MSR6_SPEC>;
#[doc = "Mailbox Status Register (MB = 6)"]
pub mod msr6;
#[doc = "MDL6 (rw) register accessor: Mailbox Data Low Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdl6`]
module"]
pub type MDL6 = crate::Reg<mdl6::MDL6_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 6)"]
pub mod mdl6;
#[doc = "MDH6 (rw) register accessor: Mailbox Data High Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdh6`]
module"]
pub type MDH6 = crate::Reg<mdh6::MDH6_SPEC>;
#[doc = "Mailbox Data High Register (MB = 6)"]
pub mod mdh6;
#[doc = "MCR6 (w) register accessor: Mailbox Control Register (MB = 6)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr6`]
module"]
pub type MCR6 = crate::Reg<mcr6::MCR6_SPEC>;
#[doc = "Mailbox Control Register (MB = 6)"]
pub mod mcr6;
#[doc = "MMR7 (rw) register accessor: Mailbox Mode Register (MB = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr7`]
module"]
pub type MMR7 = crate::Reg<mmr7::MMR7_SPEC>;
#[doc = "Mailbox Mode Register (MB = 7)"]
pub mod mmr7;
#[doc = "MAM7 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mam7`]
module"]
pub type MAM7 = crate::Reg<mam7::MAM7_SPEC>;
#[doc = "Mailbox Acceptance Mask Register (MB = 7)"]
pub mod mam7;
#[doc = "MID7 (rw) register accessor: Mailbox ID Register (MB = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mid7`]
module"]
pub type MID7 = crate::Reg<mid7::MID7_SPEC>;
#[doc = "Mailbox ID Register (MB = 7)"]
pub mod mid7;
#[doc = "MFID7 (r) register accessor: Mailbox Family ID Register (MB = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfid7`]
module"]
pub type MFID7 = crate::Reg<mfid7::MFID7_SPEC>;
#[doc = "Mailbox Family ID Register (MB = 7)"]
pub mod mfid7;
#[doc = "MSR7 (r) register accessor: Mailbox Status Register (MB = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr7`]
module"]
pub type MSR7 = crate::Reg<msr7::MSR7_SPEC>;
#[doc = "Mailbox Status Register (MB = 7)"]
pub mod msr7;
#[doc = "MDL7 (rw) register accessor: Mailbox Data Low Register (MB = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdl7`]
module"]
pub type MDL7 = crate::Reg<mdl7::MDL7_SPEC>;
#[doc = "Mailbox Data Low Register (MB = 7)"]
pub mod mdl7;
#[doc = "MDH7 (rw) register accessor: Mailbox Data High Register (MB = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdh7`]
module"]
pub type MDH7 = crate::Reg<mdh7::MDH7_SPEC>;
#[doc = "Mailbox Data High Register (MB = 7)"]
pub mod mdh7;
#[doc = "MCR7 (w) register accessor: Mailbox Control Register (MB = 7)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr7`]
module"]
pub type MCR7 = crate::Reg<mcr7::MCR7_SPEC>;
#[doc = "Mailbox Control Register (MB = 7)"]
pub mod mcr7;
