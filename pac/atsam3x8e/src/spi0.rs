#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Mode Register"]
    pub mr: MR,
    #[doc = "0x08 - Receive Data Register"]
    pub rdr: RDR,
    #[doc = "0x0c - Transmit Data Register"]
    pub tdr: TDR,
    #[doc = "0x10 - Status Register"]
    pub sr: SR,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x18 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x1c - Interrupt Mask Register"]
    pub imr: IMR,
    _reserved8: [u8; 0x10],
    #[doc = "0x30 - Chip Select Register 0"]
    pub csr0: CSR0,
    #[doc = "0x34 - Chip Select Register 1"]
    pub csr1: CSR1,
    #[doc = "0x38 - Chip Select Register 2"]
    pub csr2: CSR2,
    #[doc = "0x3c - Chip Select Register 3"]
    pub csr3: CSR3,
    _reserved12: [u8; 0xa4],
    #[doc = "0xe4 - Write Protection Control Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub wpsr: WPSR,
}
#[doc = "CR (w) register accessor: Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MR (rw) register accessor: Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`]
module"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "RDR (r) register accessor: Receive Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdr`]
module"]
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
#[doc = "Receive Data Register"]
pub mod rdr;
#[doc = "TDR (w) register accessor: Transmit Data Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr`]
module"]
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
#[doc = "Transmit Data Register"]
pub mod tdr;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
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
#[doc = "CSR0 (rw) register accessor: Chip Select Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr0`]
module"]
pub type CSR0 = crate::Reg<csr0::CSR0_SPEC>;
#[doc = "Chip Select Register 0"]
pub mod csr0;
#[doc = "CSR1 (rw) register accessor: Chip Select Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr1`]
module"]
pub type CSR1 = crate::Reg<csr1::CSR1_SPEC>;
#[doc = "Chip Select Register 1"]
pub mod csr1;
#[doc = "CSR2 (rw) register accessor: Chip Select Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr2`]
module"]
pub type CSR2 = crate::Reg<csr2::CSR2_SPEC>;
#[doc = "Chip Select Register 2"]
pub mod csr2;
#[doc = "CSR3 (rw) register accessor: Chip Select Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr3`]
module"]
pub type CSR3 = crate::Reg<csr3::CSR3_SPEC>;
#[doc = "Chip Select Register 3"]
pub mod csr3;
#[doc = "WPMR (rw) register accessor: Write Protection Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`]
module"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protection Control Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: Write Protection Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`]
module"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
