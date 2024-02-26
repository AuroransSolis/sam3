#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mr: Mr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    sr: Sr,
    br: Br,
    tim: Tim,
    timestp: Timestp,
    ecr: Ecr,
    tcr: Tcr,
    acr: Acr,
    _reserved11: [u8; 0xb8],
    wpmr: Wpmr,
    wpsr: Wpsr,
    _reserved13: [u8; 0x0114],
    mmr0: Mmr0,
    mam0: Mam0,
    mid0: Mid0,
    mfid0: Mfid0,
    msr0: Msr0,
    mdl0: Mdl0,
    mdh0: Mdh0,
    mcr0: Mcr0,
    mmr1: Mmr1,
    mam1: Mam1,
    mid1: Mid1,
    mfid1: Mfid1,
    msr1: Msr1,
    mdl1: Mdl1,
    mdh1: Mdh1,
    mcr1: Mcr1,
    mmr2: Mmr2,
    mam2: Mam2,
    mid2: Mid2,
    mfid2: Mfid2,
    msr2: Msr2,
    mdl2: Mdl2,
    mdh2: Mdh2,
    mcr2: Mcr2,
    mmr3: Mmr3,
    mam3: Mam3,
    mid3: Mid3,
    mfid3: Mfid3,
    msr3: Msr3,
    mdl3: Mdl3,
    mdh3: Mdh3,
    mcr3: Mcr3,
    mmr4: Mmr4,
    mam4: Mam4,
    mid4: Mid4,
    mfid4: Mfid4,
    msr4: Msr4,
    mdl4: Mdl4,
    mdh4: Mdh4,
    mcr4: Mcr4,
    mmr5: Mmr5,
    mam5: Mam5,
    mid5: Mid5,
    mfid5: Mfid5,
    msr5: Msr5,
    mdl5: Mdl5,
    mdh5: Mdh5,
    mcr5: Mcr5,
    mmr6: Mmr6,
    mam6: Mam6,
    mid6: Mid6,
    mfid6: Mfid6,
    msr6: Msr6,
    mdl6: Mdl6,
    mdh6: Mdh6,
    mcr6: Mcr6,
    mmr7: Mmr7,
    mam7: Mam7,
    mid7: Mid7,
    mfid7: Mfid7,
    msr7: Msr7,
    mdl7: Mdl7,
    mdh7: Mdh7,
    mcr7: Mcr7,
}
impl RegisterBlock {
    #[doc = "0x00 - Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &Mr {
        &self.mr
    }
    #[doc = "0x04 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x08 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x0c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x10 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x14 - Baudrate Register"]
    #[inline(always)]
    pub const fn br(&self) -> &Br {
        &self.br
    }
    #[doc = "0x18 - Timer Register"]
    #[inline(always)]
    pub const fn tim(&self) -> &Tim {
        &self.tim
    }
    #[doc = "0x1c - Timestamp Register"]
    #[inline(always)]
    pub const fn timestp(&self) -> &Timestp {
        &self.timestp
    }
    #[doc = "0x20 - Error Counter Register"]
    #[inline(always)]
    pub const fn ecr(&self) -> &Ecr {
        &self.ecr
    }
    #[doc = "0x24 - Transfer Command Register"]
    #[inline(always)]
    pub const fn tcr(&self) -> &Tcr {
        &self.tcr
    }
    #[doc = "0x28 - Abort Command Register"]
    #[inline(always)]
    pub const fn acr(&self) -> &Acr {
        &self.acr
    }
    #[doc = "0xe4 - Write Protect Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &Wpmr {
        &self.wpmr
    }
    #[doc = "0xe8 - Write Protect Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &Wpsr {
        &self.wpsr
    }
    #[doc = "0x200 - Mailbox Mode Register (MB = 0)"]
    #[inline(always)]
    pub const fn mmr0(&self) -> &Mmr0 {
        &self.mmr0
    }
    #[doc = "0x204 - Mailbox Acceptance Mask Register (MB = 0)"]
    #[inline(always)]
    pub const fn mam0(&self) -> &Mam0 {
        &self.mam0
    }
    #[doc = "0x208 - Mailbox ID Register (MB = 0)"]
    #[inline(always)]
    pub const fn mid0(&self) -> &Mid0 {
        &self.mid0
    }
    #[doc = "0x20c - Mailbox Family ID Register (MB = 0)"]
    #[inline(always)]
    pub const fn mfid0(&self) -> &Mfid0 {
        &self.mfid0
    }
    #[doc = "0x210 - Mailbox Status Register (MB = 0)"]
    #[inline(always)]
    pub const fn msr0(&self) -> &Msr0 {
        &self.msr0
    }
    #[doc = "0x214 - Mailbox Data Low Register (MB = 0)"]
    #[inline(always)]
    pub const fn mdl0(&self) -> &Mdl0 {
        &self.mdl0
    }
    #[doc = "0x218 - Mailbox Data High Register (MB = 0)"]
    #[inline(always)]
    pub const fn mdh0(&self) -> &Mdh0 {
        &self.mdh0
    }
    #[doc = "0x21c - Mailbox Control Register (MB = 0)"]
    #[inline(always)]
    pub const fn mcr0(&self) -> &Mcr0 {
        &self.mcr0
    }
    #[doc = "0x220 - Mailbox Mode Register (MB = 1)"]
    #[inline(always)]
    pub const fn mmr1(&self) -> &Mmr1 {
        &self.mmr1
    }
    #[doc = "0x224 - Mailbox Acceptance Mask Register (MB = 1)"]
    #[inline(always)]
    pub const fn mam1(&self) -> &Mam1 {
        &self.mam1
    }
    #[doc = "0x228 - Mailbox ID Register (MB = 1)"]
    #[inline(always)]
    pub const fn mid1(&self) -> &Mid1 {
        &self.mid1
    }
    #[doc = "0x22c - Mailbox Family ID Register (MB = 1)"]
    #[inline(always)]
    pub const fn mfid1(&self) -> &Mfid1 {
        &self.mfid1
    }
    #[doc = "0x230 - Mailbox Status Register (MB = 1)"]
    #[inline(always)]
    pub const fn msr1(&self) -> &Msr1 {
        &self.msr1
    }
    #[doc = "0x234 - Mailbox Data Low Register (MB = 1)"]
    #[inline(always)]
    pub const fn mdl1(&self) -> &Mdl1 {
        &self.mdl1
    }
    #[doc = "0x238 - Mailbox Data High Register (MB = 1)"]
    #[inline(always)]
    pub const fn mdh1(&self) -> &Mdh1 {
        &self.mdh1
    }
    #[doc = "0x23c - Mailbox Control Register (MB = 1)"]
    #[inline(always)]
    pub const fn mcr1(&self) -> &Mcr1 {
        &self.mcr1
    }
    #[doc = "0x240 - Mailbox Mode Register (MB = 2)"]
    #[inline(always)]
    pub const fn mmr2(&self) -> &Mmr2 {
        &self.mmr2
    }
    #[doc = "0x244 - Mailbox Acceptance Mask Register (MB = 2)"]
    #[inline(always)]
    pub const fn mam2(&self) -> &Mam2 {
        &self.mam2
    }
    #[doc = "0x248 - Mailbox ID Register (MB = 2)"]
    #[inline(always)]
    pub const fn mid2(&self) -> &Mid2 {
        &self.mid2
    }
    #[doc = "0x24c - Mailbox Family ID Register (MB = 2)"]
    #[inline(always)]
    pub const fn mfid2(&self) -> &Mfid2 {
        &self.mfid2
    }
    #[doc = "0x250 - Mailbox Status Register (MB = 2)"]
    #[inline(always)]
    pub const fn msr2(&self) -> &Msr2 {
        &self.msr2
    }
    #[doc = "0x254 - Mailbox Data Low Register (MB = 2)"]
    #[inline(always)]
    pub const fn mdl2(&self) -> &Mdl2 {
        &self.mdl2
    }
    #[doc = "0x258 - Mailbox Data High Register (MB = 2)"]
    #[inline(always)]
    pub const fn mdh2(&self) -> &Mdh2 {
        &self.mdh2
    }
    #[doc = "0x25c - Mailbox Control Register (MB = 2)"]
    #[inline(always)]
    pub const fn mcr2(&self) -> &Mcr2 {
        &self.mcr2
    }
    #[doc = "0x260 - Mailbox Mode Register (MB = 3)"]
    #[inline(always)]
    pub const fn mmr3(&self) -> &Mmr3 {
        &self.mmr3
    }
    #[doc = "0x264 - Mailbox Acceptance Mask Register (MB = 3)"]
    #[inline(always)]
    pub const fn mam3(&self) -> &Mam3 {
        &self.mam3
    }
    #[doc = "0x268 - Mailbox ID Register (MB = 3)"]
    #[inline(always)]
    pub const fn mid3(&self) -> &Mid3 {
        &self.mid3
    }
    #[doc = "0x26c - Mailbox Family ID Register (MB = 3)"]
    #[inline(always)]
    pub const fn mfid3(&self) -> &Mfid3 {
        &self.mfid3
    }
    #[doc = "0x270 - Mailbox Status Register (MB = 3)"]
    #[inline(always)]
    pub const fn msr3(&self) -> &Msr3 {
        &self.msr3
    }
    #[doc = "0x274 - Mailbox Data Low Register (MB = 3)"]
    #[inline(always)]
    pub const fn mdl3(&self) -> &Mdl3 {
        &self.mdl3
    }
    #[doc = "0x278 - Mailbox Data High Register (MB = 3)"]
    #[inline(always)]
    pub const fn mdh3(&self) -> &Mdh3 {
        &self.mdh3
    }
    #[doc = "0x27c - Mailbox Control Register (MB = 3)"]
    #[inline(always)]
    pub const fn mcr3(&self) -> &Mcr3 {
        &self.mcr3
    }
    #[doc = "0x280 - Mailbox Mode Register (MB = 4)"]
    #[inline(always)]
    pub const fn mmr4(&self) -> &Mmr4 {
        &self.mmr4
    }
    #[doc = "0x284 - Mailbox Acceptance Mask Register (MB = 4)"]
    #[inline(always)]
    pub const fn mam4(&self) -> &Mam4 {
        &self.mam4
    }
    #[doc = "0x288 - Mailbox ID Register (MB = 4)"]
    #[inline(always)]
    pub const fn mid4(&self) -> &Mid4 {
        &self.mid4
    }
    #[doc = "0x28c - Mailbox Family ID Register (MB = 4)"]
    #[inline(always)]
    pub const fn mfid4(&self) -> &Mfid4 {
        &self.mfid4
    }
    #[doc = "0x290 - Mailbox Status Register (MB = 4)"]
    #[inline(always)]
    pub const fn msr4(&self) -> &Msr4 {
        &self.msr4
    }
    #[doc = "0x294 - Mailbox Data Low Register (MB = 4)"]
    #[inline(always)]
    pub const fn mdl4(&self) -> &Mdl4 {
        &self.mdl4
    }
    #[doc = "0x298 - Mailbox Data High Register (MB = 4)"]
    #[inline(always)]
    pub const fn mdh4(&self) -> &Mdh4 {
        &self.mdh4
    }
    #[doc = "0x29c - Mailbox Control Register (MB = 4)"]
    #[inline(always)]
    pub const fn mcr4(&self) -> &Mcr4 {
        &self.mcr4
    }
    #[doc = "0x2a0 - Mailbox Mode Register (MB = 5)"]
    #[inline(always)]
    pub const fn mmr5(&self) -> &Mmr5 {
        &self.mmr5
    }
    #[doc = "0x2a4 - Mailbox Acceptance Mask Register (MB = 5)"]
    #[inline(always)]
    pub const fn mam5(&self) -> &Mam5 {
        &self.mam5
    }
    #[doc = "0x2a8 - Mailbox ID Register (MB = 5)"]
    #[inline(always)]
    pub const fn mid5(&self) -> &Mid5 {
        &self.mid5
    }
    #[doc = "0x2ac - Mailbox Family ID Register (MB = 5)"]
    #[inline(always)]
    pub const fn mfid5(&self) -> &Mfid5 {
        &self.mfid5
    }
    #[doc = "0x2b0 - Mailbox Status Register (MB = 5)"]
    #[inline(always)]
    pub const fn msr5(&self) -> &Msr5 {
        &self.msr5
    }
    #[doc = "0x2b4 - Mailbox Data Low Register (MB = 5)"]
    #[inline(always)]
    pub const fn mdl5(&self) -> &Mdl5 {
        &self.mdl5
    }
    #[doc = "0x2b8 - Mailbox Data High Register (MB = 5)"]
    #[inline(always)]
    pub const fn mdh5(&self) -> &Mdh5 {
        &self.mdh5
    }
    #[doc = "0x2bc - Mailbox Control Register (MB = 5)"]
    #[inline(always)]
    pub const fn mcr5(&self) -> &Mcr5 {
        &self.mcr5
    }
    #[doc = "0x2c0 - Mailbox Mode Register (MB = 6)"]
    #[inline(always)]
    pub const fn mmr6(&self) -> &Mmr6 {
        &self.mmr6
    }
    #[doc = "0x2c4 - Mailbox Acceptance Mask Register (MB = 6)"]
    #[inline(always)]
    pub const fn mam6(&self) -> &Mam6 {
        &self.mam6
    }
    #[doc = "0x2c8 - Mailbox ID Register (MB = 6)"]
    #[inline(always)]
    pub const fn mid6(&self) -> &Mid6 {
        &self.mid6
    }
    #[doc = "0x2cc - Mailbox Family ID Register (MB = 6)"]
    #[inline(always)]
    pub const fn mfid6(&self) -> &Mfid6 {
        &self.mfid6
    }
    #[doc = "0x2d0 - Mailbox Status Register (MB = 6)"]
    #[inline(always)]
    pub const fn msr6(&self) -> &Msr6 {
        &self.msr6
    }
    #[doc = "0x2d4 - Mailbox Data Low Register (MB = 6)"]
    #[inline(always)]
    pub const fn mdl6(&self) -> &Mdl6 {
        &self.mdl6
    }
    #[doc = "0x2d8 - Mailbox Data High Register (MB = 6)"]
    #[inline(always)]
    pub const fn mdh6(&self) -> &Mdh6 {
        &self.mdh6
    }
    #[doc = "0x2dc - Mailbox Control Register (MB = 6)"]
    #[inline(always)]
    pub const fn mcr6(&self) -> &Mcr6 {
        &self.mcr6
    }
    #[doc = "0x2e0 - Mailbox Mode Register (MB = 7)"]
    #[inline(always)]
    pub const fn mmr7(&self) -> &Mmr7 {
        &self.mmr7
    }
    #[doc = "0x2e4 - Mailbox Acceptance Mask Register (MB = 7)"]
    #[inline(always)]
    pub const fn mam7(&self) -> &Mam7 {
        &self.mam7
    }
    #[doc = "0x2e8 - Mailbox ID Register (MB = 7)"]
    #[inline(always)]
    pub const fn mid7(&self) -> &Mid7 {
        &self.mid7
    }
    #[doc = "0x2ec - Mailbox Family ID Register (MB = 7)"]
    #[inline(always)]
    pub const fn mfid7(&self) -> &Mfid7 {
        &self.mfid7
    }
    #[doc = "0x2f0 - Mailbox Status Register (MB = 7)"]
    #[inline(always)]
    pub const fn msr7(&self) -> &Msr7 {
        &self.msr7
    }
    #[doc = "0x2f4 - Mailbox Data Low Register (MB = 7)"]
    #[inline(always)]
    pub const fn mdl7(&self) -> &Mdl7 {
        &self.mdl7
    }
    #[doc = "0x2f8 - Mailbox Data High Register (MB = 7)"]
    #[inline(always)]
    pub const fn mdh7(&self) -> &Mdh7 {
        &self.mdh7
    }
    #[doc = "0x2fc - Mailbox Control Register (MB = 7)"]
    #[inline(always)]
    pub const fn mcr7(&self) -> &Mcr7 {
        &self.mcr7
    }
}
#[doc = "MR (rw) register accessor: Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`]
module"]
#[doc(alias = "MR")]
pub type Mr = crate::Reg<mr::MrSpec>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "IER (w) register accessor: Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "BR (rw) register accessor: Baudrate Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@br`]
module"]
#[doc(alias = "BR")]
pub type Br = crate::Reg<br::BrSpec>;
#[doc = "Baudrate Register"]
pub mod br;
#[doc = "TIM (r) register accessor: Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim`]
module"]
#[doc(alias = "TIM")]
pub type Tim = crate::Reg<tim::TimSpec>;
#[doc = "Timer Register"]
pub mod tim;
#[doc = "TIMESTP (r) register accessor: Timestamp Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timestp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timestp`]
module"]
#[doc(alias = "TIMESTP")]
pub type Timestp = crate::Reg<timestp::TimestpSpec>;
#[doc = "Timestamp Register"]
pub mod timestp;
#[doc = "ECR (r) register accessor: Error Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`]
module"]
#[doc(alias = "ECR")]
pub type Ecr = crate::Reg<ecr::EcrSpec>;
#[doc = "Error Counter Register"]
pub mod ecr;
#[doc = "TCR (w) register accessor: Transfer Command Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr`]
module"]
#[doc(alias = "TCR")]
pub type Tcr = crate::Reg<tcr::TcrSpec>;
#[doc = "Transfer Command Register"]
pub mod tcr;
#[doc = "ACR (w) register accessor: Abort Command Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`]
module"]
#[doc(alias = "ACR")]
pub type Acr = crate::Reg<acr::AcrSpec>;
#[doc = "Abort Command Register"]
pub mod acr;
#[doc = "WPMR (rw) register accessor: Write Protect Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`]
module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: Write Protect Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`]
module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "Write Protect Status Register"]
pub mod wpsr;
#[doc = "MMR0 (rw) register accessor: Mailbox Mode Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr0`]
module"]
#[doc(alias = "MMR0")]
pub type Mmr0 = crate::Reg<mmr0::Mmr0Spec>;
#[doc = "Mailbox Mode Register (MB = 0)"]
pub mod mmr0;
#[doc = "MAM0 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mam0`]
module"]
#[doc(alias = "MAM0")]
pub type Mam0 = crate::Reg<mam0::Mam0Spec>;
#[doc = "Mailbox Acceptance Mask Register (MB = 0)"]
pub mod mam0;
#[doc = "MID0 (rw) register accessor: Mailbox ID Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mid0`]
module"]
#[doc(alias = "MID0")]
pub type Mid0 = crate::Reg<mid0::Mid0Spec>;
#[doc = "Mailbox ID Register (MB = 0)"]
pub mod mid0;
#[doc = "MFID0 (r) register accessor: Mailbox Family ID Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfid0`]
module"]
#[doc(alias = "MFID0")]
pub type Mfid0 = crate::Reg<mfid0::Mfid0Spec>;
#[doc = "Mailbox Family ID Register (MB = 0)"]
pub mod mfid0;
#[doc = "MSR0 (r) register accessor: Mailbox Status Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr0`]
module"]
#[doc(alias = "MSR0")]
pub type Msr0 = crate::Reg<msr0::Msr0Spec>;
#[doc = "Mailbox Status Register (MB = 0)"]
pub mod msr0;
#[doc = "MDL0 (rw) register accessor: Mailbox Data Low Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdl0`]
module"]
#[doc(alias = "MDL0")]
pub type Mdl0 = crate::Reg<mdl0::Mdl0Spec>;
#[doc = "Mailbox Data Low Register (MB = 0)"]
pub mod mdl0;
#[doc = "MDH0 (rw) register accessor: Mailbox Data High Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdh0`]
module"]
#[doc(alias = "MDH0")]
pub type Mdh0 = crate::Reg<mdh0::Mdh0Spec>;
#[doc = "Mailbox Data High Register (MB = 0)"]
pub mod mdh0;
#[doc = "MCR0 (w) register accessor: Mailbox Control Register (MB = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr0`]
module"]
#[doc(alias = "MCR0")]
pub type Mcr0 = crate::Reg<mcr0::Mcr0Spec>;
#[doc = "Mailbox Control Register (MB = 0)"]
pub mod mcr0;
#[doc = "MMR1 (rw) register accessor: Mailbox Mode Register (MB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr1`]
module"]
#[doc(alias = "MMR1")]
pub type Mmr1 = crate::Reg<mmr1::Mmr1Spec>;
#[doc = "Mailbox Mode Register (MB = 1)"]
pub mod mmr1;
#[doc = "MAM1 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mam1`]
module"]
#[doc(alias = "MAM1")]
pub type Mam1 = crate::Reg<mam1::Mam1Spec>;
#[doc = "Mailbox Acceptance Mask Register (MB = 1)"]
pub mod mam1;
#[doc = "MID1 (rw) register accessor: Mailbox ID Register (MB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mid1`]
module"]
#[doc(alias = "MID1")]
pub type Mid1 = crate::Reg<mid1::Mid1Spec>;
#[doc = "Mailbox ID Register (MB = 1)"]
pub mod mid1;
#[doc = "MFID1 (r) register accessor: Mailbox Family ID Register (MB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfid1`]
module"]
#[doc(alias = "MFID1")]
pub type Mfid1 = crate::Reg<mfid1::Mfid1Spec>;
#[doc = "Mailbox Family ID Register (MB = 1)"]
pub mod mfid1;
#[doc = "MSR1 (r) register accessor: Mailbox Status Register (MB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr1`]
module"]
#[doc(alias = "MSR1")]
pub type Msr1 = crate::Reg<msr1::Msr1Spec>;
#[doc = "Mailbox Status Register (MB = 1)"]
pub mod msr1;
#[doc = "MDL1 (rw) register accessor: Mailbox Data Low Register (MB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdl1`]
module"]
#[doc(alias = "MDL1")]
pub type Mdl1 = crate::Reg<mdl1::Mdl1Spec>;
#[doc = "Mailbox Data Low Register (MB = 1)"]
pub mod mdl1;
#[doc = "MDH1 (rw) register accessor: Mailbox Data High Register (MB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdh1`]
module"]
#[doc(alias = "MDH1")]
pub type Mdh1 = crate::Reg<mdh1::Mdh1Spec>;
#[doc = "Mailbox Data High Register (MB = 1)"]
pub mod mdh1;
#[doc = "MCR1 (w) register accessor: Mailbox Control Register (MB = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr1`]
module"]
#[doc(alias = "MCR1")]
pub type Mcr1 = crate::Reg<mcr1::Mcr1Spec>;
#[doc = "Mailbox Control Register (MB = 1)"]
pub mod mcr1;
#[doc = "MMR2 (rw) register accessor: Mailbox Mode Register (MB = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr2`]
module"]
#[doc(alias = "MMR2")]
pub type Mmr2 = crate::Reg<mmr2::Mmr2Spec>;
#[doc = "Mailbox Mode Register (MB = 2)"]
pub mod mmr2;
#[doc = "MAM2 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mam2`]
module"]
#[doc(alias = "MAM2")]
pub type Mam2 = crate::Reg<mam2::Mam2Spec>;
#[doc = "Mailbox Acceptance Mask Register (MB = 2)"]
pub mod mam2;
#[doc = "MID2 (rw) register accessor: Mailbox ID Register (MB = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mid2`]
module"]
#[doc(alias = "MID2")]
pub type Mid2 = crate::Reg<mid2::Mid2Spec>;
#[doc = "Mailbox ID Register (MB = 2)"]
pub mod mid2;
#[doc = "MFID2 (r) register accessor: Mailbox Family ID Register (MB = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfid2`]
module"]
#[doc(alias = "MFID2")]
pub type Mfid2 = crate::Reg<mfid2::Mfid2Spec>;
#[doc = "Mailbox Family ID Register (MB = 2)"]
pub mod mfid2;
#[doc = "MSR2 (r) register accessor: Mailbox Status Register (MB = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr2`]
module"]
#[doc(alias = "MSR2")]
pub type Msr2 = crate::Reg<msr2::Msr2Spec>;
#[doc = "Mailbox Status Register (MB = 2)"]
pub mod msr2;
#[doc = "MDL2 (rw) register accessor: Mailbox Data Low Register (MB = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdl2`]
module"]
#[doc(alias = "MDL2")]
pub type Mdl2 = crate::Reg<mdl2::Mdl2Spec>;
#[doc = "Mailbox Data Low Register (MB = 2)"]
pub mod mdl2;
#[doc = "MDH2 (rw) register accessor: Mailbox Data High Register (MB = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdh2`]
module"]
#[doc(alias = "MDH2")]
pub type Mdh2 = crate::Reg<mdh2::Mdh2Spec>;
#[doc = "Mailbox Data High Register (MB = 2)"]
pub mod mdh2;
#[doc = "MCR2 (w) register accessor: Mailbox Control Register (MB = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr2`]
module"]
#[doc(alias = "MCR2")]
pub type Mcr2 = crate::Reg<mcr2::Mcr2Spec>;
#[doc = "Mailbox Control Register (MB = 2)"]
pub mod mcr2;
#[doc = "MMR3 (rw) register accessor: Mailbox Mode Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr3`]
module"]
#[doc(alias = "MMR3")]
pub type Mmr3 = crate::Reg<mmr3::Mmr3Spec>;
#[doc = "Mailbox Mode Register (MB = 3)"]
pub mod mmr3;
#[doc = "MAM3 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mam3`]
module"]
#[doc(alias = "MAM3")]
pub type Mam3 = crate::Reg<mam3::Mam3Spec>;
#[doc = "Mailbox Acceptance Mask Register (MB = 3)"]
pub mod mam3;
#[doc = "MID3 (rw) register accessor: Mailbox ID Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mid3`]
module"]
#[doc(alias = "MID3")]
pub type Mid3 = crate::Reg<mid3::Mid3Spec>;
#[doc = "Mailbox ID Register (MB = 3)"]
pub mod mid3;
#[doc = "MFID3 (r) register accessor: Mailbox Family ID Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfid3`]
module"]
#[doc(alias = "MFID3")]
pub type Mfid3 = crate::Reg<mfid3::Mfid3Spec>;
#[doc = "Mailbox Family ID Register (MB = 3)"]
pub mod mfid3;
#[doc = "MSR3 (r) register accessor: Mailbox Status Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr3`]
module"]
#[doc(alias = "MSR3")]
pub type Msr3 = crate::Reg<msr3::Msr3Spec>;
#[doc = "Mailbox Status Register (MB = 3)"]
pub mod msr3;
#[doc = "MDL3 (rw) register accessor: Mailbox Data Low Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdl3`]
module"]
#[doc(alias = "MDL3")]
pub type Mdl3 = crate::Reg<mdl3::Mdl3Spec>;
#[doc = "Mailbox Data Low Register (MB = 3)"]
pub mod mdl3;
#[doc = "MDH3 (rw) register accessor: Mailbox Data High Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdh3`]
module"]
#[doc(alias = "MDH3")]
pub type Mdh3 = crate::Reg<mdh3::Mdh3Spec>;
#[doc = "Mailbox Data High Register (MB = 3)"]
pub mod mdh3;
#[doc = "MCR3 (w) register accessor: Mailbox Control Register (MB = 3)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr3`]
module"]
#[doc(alias = "MCR3")]
pub type Mcr3 = crate::Reg<mcr3::Mcr3Spec>;
#[doc = "Mailbox Control Register (MB = 3)"]
pub mod mcr3;
#[doc = "MMR4 (rw) register accessor: Mailbox Mode Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr4`]
module"]
#[doc(alias = "MMR4")]
pub type Mmr4 = crate::Reg<mmr4::Mmr4Spec>;
#[doc = "Mailbox Mode Register (MB = 4)"]
pub mod mmr4;
#[doc = "MAM4 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mam4`]
module"]
#[doc(alias = "MAM4")]
pub type Mam4 = crate::Reg<mam4::Mam4Spec>;
#[doc = "Mailbox Acceptance Mask Register (MB = 4)"]
pub mod mam4;
#[doc = "MID4 (rw) register accessor: Mailbox ID Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mid4`]
module"]
#[doc(alias = "MID4")]
pub type Mid4 = crate::Reg<mid4::Mid4Spec>;
#[doc = "Mailbox ID Register (MB = 4)"]
pub mod mid4;
#[doc = "MFID4 (r) register accessor: Mailbox Family ID Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfid4`]
module"]
#[doc(alias = "MFID4")]
pub type Mfid4 = crate::Reg<mfid4::Mfid4Spec>;
#[doc = "Mailbox Family ID Register (MB = 4)"]
pub mod mfid4;
#[doc = "MSR4 (r) register accessor: Mailbox Status Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr4`]
module"]
#[doc(alias = "MSR4")]
pub type Msr4 = crate::Reg<msr4::Msr4Spec>;
#[doc = "Mailbox Status Register (MB = 4)"]
pub mod msr4;
#[doc = "MDL4 (rw) register accessor: Mailbox Data Low Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdl4`]
module"]
#[doc(alias = "MDL4")]
pub type Mdl4 = crate::Reg<mdl4::Mdl4Spec>;
#[doc = "Mailbox Data Low Register (MB = 4)"]
pub mod mdl4;
#[doc = "MDH4 (rw) register accessor: Mailbox Data High Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdh4`]
module"]
#[doc(alias = "MDH4")]
pub type Mdh4 = crate::Reg<mdh4::Mdh4Spec>;
#[doc = "Mailbox Data High Register (MB = 4)"]
pub mod mdh4;
#[doc = "MCR4 (w) register accessor: Mailbox Control Register (MB = 4)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr4`]
module"]
#[doc(alias = "MCR4")]
pub type Mcr4 = crate::Reg<mcr4::Mcr4Spec>;
#[doc = "Mailbox Control Register (MB = 4)"]
pub mod mcr4;
#[doc = "MMR5 (rw) register accessor: Mailbox Mode Register (MB = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr5`]
module"]
#[doc(alias = "MMR5")]
pub type Mmr5 = crate::Reg<mmr5::Mmr5Spec>;
#[doc = "Mailbox Mode Register (MB = 5)"]
pub mod mmr5;
#[doc = "MAM5 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mam5`]
module"]
#[doc(alias = "MAM5")]
pub type Mam5 = crate::Reg<mam5::Mam5Spec>;
#[doc = "Mailbox Acceptance Mask Register (MB = 5)"]
pub mod mam5;
#[doc = "MID5 (rw) register accessor: Mailbox ID Register (MB = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mid5`]
module"]
#[doc(alias = "MID5")]
pub type Mid5 = crate::Reg<mid5::Mid5Spec>;
#[doc = "Mailbox ID Register (MB = 5)"]
pub mod mid5;
#[doc = "MFID5 (r) register accessor: Mailbox Family ID Register (MB = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfid5`]
module"]
#[doc(alias = "MFID5")]
pub type Mfid5 = crate::Reg<mfid5::Mfid5Spec>;
#[doc = "Mailbox Family ID Register (MB = 5)"]
pub mod mfid5;
#[doc = "MSR5 (r) register accessor: Mailbox Status Register (MB = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr5`]
module"]
#[doc(alias = "MSR5")]
pub type Msr5 = crate::Reg<msr5::Msr5Spec>;
#[doc = "Mailbox Status Register (MB = 5)"]
pub mod msr5;
#[doc = "MDL5 (rw) register accessor: Mailbox Data Low Register (MB = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdl5`]
module"]
#[doc(alias = "MDL5")]
pub type Mdl5 = crate::Reg<mdl5::Mdl5Spec>;
#[doc = "Mailbox Data Low Register (MB = 5)"]
pub mod mdl5;
#[doc = "MDH5 (rw) register accessor: Mailbox Data High Register (MB = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdh5`]
module"]
#[doc(alias = "MDH5")]
pub type Mdh5 = crate::Reg<mdh5::Mdh5Spec>;
#[doc = "Mailbox Data High Register (MB = 5)"]
pub mod mdh5;
#[doc = "MCR5 (w) register accessor: Mailbox Control Register (MB = 5)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr5`]
module"]
#[doc(alias = "MCR5")]
pub type Mcr5 = crate::Reg<mcr5::Mcr5Spec>;
#[doc = "Mailbox Control Register (MB = 5)"]
pub mod mcr5;
#[doc = "MMR6 (rw) register accessor: Mailbox Mode Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr6`]
module"]
#[doc(alias = "MMR6")]
pub type Mmr6 = crate::Reg<mmr6::Mmr6Spec>;
#[doc = "Mailbox Mode Register (MB = 6)"]
pub mod mmr6;
#[doc = "MAM6 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mam6`]
module"]
#[doc(alias = "MAM6")]
pub type Mam6 = crate::Reg<mam6::Mam6Spec>;
#[doc = "Mailbox Acceptance Mask Register (MB = 6)"]
pub mod mam6;
#[doc = "MID6 (rw) register accessor: Mailbox ID Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mid6`]
module"]
#[doc(alias = "MID6")]
pub type Mid6 = crate::Reg<mid6::Mid6Spec>;
#[doc = "Mailbox ID Register (MB = 6)"]
pub mod mid6;
#[doc = "MFID6 (r) register accessor: Mailbox Family ID Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfid6`]
module"]
#[doc(alias = "MFID6")]
pub type Mfid6 = crate::Reg<mfid6::Mfid6Spec>;
#[doc = "Mailbox Family ID Register (MB = 6)"]
pub mod mfid6;
#[doc = "MSR6 (r) register accessor: Mailbox Status Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr6`]
module"]
#[doc(alias = "MSR6")]
pub type Msr6 = crate::Reg<msr6::Msr6Spec>;
#[doc = "Mailbox Status Register (MB = 6)"]
pub mod msr6;
#[doc = "MDL6 (rw) register accessor: Mailbox Data Low Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdl6`]
module"]
#[doc(alias = "MDL6")]
pub type Mdl6 = crate::Reg<mdl6::Mdl6Spec>;
#[doc = "Mailbox Data Low Register (MB = 6)"]
pub mod mdl6;
#[doc = "MDH6 (rw) register accessor: Mailbox Data High Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdh6`]
module"]
#[doc(alias = "MDH6")]
pub type Mdh6 = crate::Reg<mdh6::Mdh6Spec>;
#[doc = "Mailbox Data High Register (MB = 6)"]
pub mod mdh6;
#[doc = "MCR6 (w) register accessor: Mailbox Control Register (MB = 6)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr6`]
module"]
#[doc(alias = "MCR6")]
pub type Mcr6 = crate::Reg<mcr6::Mcr6Spec>;
#[doc = "Mailbox Control Register (MB = 6)"]
pub mod mcr6;
#[doc = "MMR7 (rw) register accessor: Mailbox Mode Register (MB = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr7`]
module"]
#[doc(alias = "MMR7")]
pub type Mmr7 = crate::Reg<mmr7::Mmr7Spec>;
#[doc = "Mailbox Mode Register (MB = 7)"]
pub mod mmr7;
#[doc = "MAM7 (rw) register accessor: Mailbox Acceptance Mask Register (MB = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mam7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mam7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mam7`]
module"]
#[doc(alias = "MAM7")]
pub type Mam7 = crate::Reg<mam7::Mam7Spec>;
#[doc = "Mailbox Acceptance Mask Register (MB = 7)"]
pub mod mam7;
#[doc = "MID7 (rw) register accessor: Mailbox ID Register (MB = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mid7`]
module"]
#[doc(alias = "MID7")]
pub type Mid7 = crate::Reg<mid7::Mid7Spec>;
#[doc = "Mailbox ID Register (MB = 7)"]
pub mod mid7;
#[doc = "MFID7 (r) register accessor: Mailbox Family ID Register (MB = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfid7`]
module"]
#[doc(alias = "MFID7")]
pub type Mfid7 = crate::Reg<mfid7::Mfid7Spec>;
#[doc = "Mailbox Family ID Register (MB = 7)"]
pub mod mfid7;
#[doc = "MSR7 (r) register accessor: Mailbox Status Register (MB = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr7`]
module"]
#[doc(alias = "MSR7")]
pub type Msr7 = crate::Reg<msr7::Msr7Spec>;
#[doc = "Mailbox Status Register (MB = 7)"]
pub mod msr7;
#[doc = "MDL7 (rw) register accessor: Mailbox Data Low Register (MB = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdl7`]
module"]
#[doc(alias = "MDL7")]
pub type Mdl7 = crate::Reg<mdl7::Mdl7Spec>;
#[doc = "Mailbox Data Low Register (MB = 7)"]
pub mod mdl7;
#[doc = "MDH7 (rw) register accessor: Mailbox Data High Register (MB = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdh7`]
module"]
#[doc(alias = "MDH7")]
pub type Mdh7 = crate::Reg<mdh7::Mdh7Spec>;
#[doc = "Mailbox Data High Register (MB = 7)"]
pub mod mdh7;
#[doc = "MCR7 (w) register accessor: Mailbox Control Register (MB = 7)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr7`]
module"]
#[doc(alias = "MCR7")]
pub type Mcr7 = crate::Reg<mcr7::Mcr7Spec>;
#[doc = "Mailbox Control Register (MB = 7)"]
pub mod mcr7;
