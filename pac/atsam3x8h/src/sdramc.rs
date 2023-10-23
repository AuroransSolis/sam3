#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDRAMC Mode Register"]
    pub mr: MR,
    #[doc = "0x04 - SDRAMC Refresh Timer Register"]
    pub tr: TR,
    #[doc = "0x08 - SDRAMC Configuration Register"]
    pub cr: CR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - SDRAMC Low Power Register"]
    pub lpr: LPR,
    #[doc = "0x14 - SDRAMC Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x18 - SDRAMC Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x1c - SDRAMC Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x20 - SDRAMC Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x24 - SDRAMC Memory Device Register"]
    pub mdr: MDR,
    #[doc = "0x28 - SDRAMC Configuration Register 1"]
    pub cr1: CR1,
    #[doc = "0x2c - SDRAMC OCMS Register 1"]
    pub ocms: OCMS,
}
#[doc = "MR (rw) register accessor: SDRAMC Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`]
module"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "SDRAMC Mode Register"]
pub mod mr;
#[doc = "TR (rw) register accessor: SDRAMC Refresh Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr`]
module"]
pub type TR = crate::Reg<tr::TR_SPEC>;
#[doc = "SDRAMC Refresh Timer Register"]
pub mod tr;
#[doc = "CR (rw) register accessor: SDRAMC Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "SDRAMC Configuration Register"]
pub mod cr;
#[doc = "LPR (rw) register accessor: SDRAMC Low Power Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpr`]
module"]
pub type LPR = crate::Reg<lpr::LPR_SPEC>;
#[doc = "SDRAMC Low Power Register"]
pub mod lpr;
#[doc = "IER (w) register accessor: SDRAMC Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "SDRAMC Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: SDRAMC Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "SDRAMC Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: SDRAMC Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "SDRAMC Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: SDRAMC Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "SDRAMC Interrupt Status Register"]
pub mod isr;
#[doc = "MDR (rw) register accessor: SDRAMC Memory Device Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdr`]
module"]
pub type MDR = crate::Reg<mdr::MDR_SPEC>;
#[doc = "SDRAMC Memory Device Register"]
pub mod mdr;
#[doc = "CR1 (rw) register accessor: SDRAMC Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "SDRAMC Configuration Register 1"]
pub mod cr1;
#[doc = "OCMS (rw) register accessor: SDRAMC OCMS Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocms::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocms::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocms`]
module"]
pub type OCMS = crate::Reg<ocms::OCMS_SPEC>;
#[doc = "SDRAMC OCMS Register 1"]
pub mod ocms;
