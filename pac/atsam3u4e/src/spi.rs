#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    mr: Mr,
    rdr: Rdr,
    tdr: Tdr,
    sr: Sr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    _reserved8: [u8; 0x10],
    csr0: Csr0,
    csr1: Csr1,
    csr2: Csr2,
    csr3: Csr3,
    _reserved12: [u8; 0xa4],
    wpmr: Wpmr,
    wpsr: Wpsr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &Mr {
        &self.mr
    }
    #[doc = "0x08 - Receive Data Register"]
    #[inline(always)]
    pub const fn rdr(&self) -> &Rdr {
        &self.rdr
    }
    #[doc = "0x0c - Transmit Data Register"]
    #[inline(always)]
    pub const fn tdr(&self) -> &Tdr {
        &self.tdr
    }
    #[doc = "0x10 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x14 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x18 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x1c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x30 - Chip Select Register 0"]
    #[inline(always)]
    pub const fn csr0(&self) -> &Csr0 {
        &self.csr0
    }
    #[doc = "0x34 - Chip Select Register 1"]
    #[inline(always)]
    pub const fn csr1(&self) -> &Csr1 {
        &self.csr1
    }
    #[doc = "0x38 - Chip Select Register 2"]
    #[inline(always)]
    pub const fn csr2(&self) -> &Csr2 {
        &self.csr2
    }
    #[doc = "0x3c - Chip Select Register 3"]
    #[inline(always)]
    pub const fn csr3(&self) -> &Csr3 {
        &self.csr3
    }
    #[doc = "0xe4 - Write Protection Control Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &Wpmr {
        &self.wpmr
    }
    #[doc = "0xe8 - Write Protection Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &Wpsr {
        &self.wpsr
    }
}
#[doc = "CR (w) register accessor: Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MR (rw) register accessor: Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`]
module"]
#[doc(alias = "MR")]
pub type Mr = crate::Reg<mr::MrSpec>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "RDR (r) register accessor: Receive Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdr`]
module"]
#[doc(alias = "RDR")]
pub type Rdr = crate::Reg<rdr::RdrSpec>;
#[doc = "Receive Data Register"]
pub mod rdr;
#[doc = "TDR (w) register accessor: Transmit Data Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr`]
module"]
#[doc(alias = "TDR")]
pub type Tdr = crate::Reg<tdr::TdrSpec>;
#[doc = "Transmit Data Register"]
pub mod tdr;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register"]
pub mod sr;
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
#[doc = "CSR0 (rw) register accessor: Chip Select Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr0`]
module"]
#[doc(alias = "CSR0")]
pub type Csr0 = crate::Reg<csr0::Csr0Spec>;
#[doc = "Chip Select Register 0"]
pub mod csr0;
#[doc = "CSR1 (rw) register accessor: Chip Select Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr1`]
module"]
#[doc(alias = "CSR1")]
pub type Csr1 = crate::Reg<csr1::Csr1Spec>;
#[doc = "Chip Select Register 1"]
pub mod csr1;
#[doc = "CSR2 (rw) register accessor: Chip Select Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr2`]
module"]
#[doc(alias = "CSR2")]
pub type Csr2 = crate::Reg<csr2::Csr2Spec>;
#[doc = "Chip Select Register 2"]
pub mod csr2;
#[doc = "CSR3 (rw) register accessor: Chip Select Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr3`]
module"]
#[doc(alias = "CSR3")]
pub type Csr3 = crate::Reg<csr3::Csr3Spec>;
#[doc = "Chip Select Register 3"]
pub mod csr3;
#[doc = "WPMR (rw) register accessor: Write Protection Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`]
module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "Write Protection Control Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: Write Protection Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`]
module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
