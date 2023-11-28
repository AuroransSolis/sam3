#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cr: CR,
    mmr: MMR,
    smr: SMR,
    iadr: IADR,
    cwgr: CWGR,
    _reserved5: [u8; 0x0c],
    sr: SR,
    ier: IER,
    idr: IDR,
    imr: IMR,
    rhr: RHR,
    thr: THR,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - Master Mode Register"]
    #[inline(always)]
    pub const fn mmr(&self) -> &MMR {
        &self.mmr
    }
    #[doc = "0x08 - Slave Mode Register"]
    #[inline(always)]
    pub const fn smr(&self) -> &SMR {
        &self.smr
    }
    #[doc = "0x0c - Internal Address Register"]
    #[inline(always)]
    pub const fn iadr(&self) -> &IADR {
        &self.iadr
    }
    #[doc = "0x10 - Clock Waveform Generator Register"]
    #[inline(always)]
    pub const fn cwgr(&self) -> &CWGR {
        &self.cwgr
    }
    #[doc = "0x20 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x24 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x28 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    #[doc = "0x2c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    #[doc = "0x30 - Receive Holding Register"]
    #[inline(always)]
    pub const fn rhr(&self) -> &RHR {
        &self.rhr
    }
    #[doc = "0x34 - Transmit Holding Register"]
    #[inline(always)]
    pub const fn thr(&self) -> &THR {
        &self.thr
    }
}
#[doc = "CR (w) register accessor: Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MMR (rw) register accessor: Master Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr`]
module"]
pub type MMR = crate::Reg<mmr::MMR_SPEC>;
#[doc = "Master Mode Register"]
pub mod mmr;
#[doc = "SMR (rw) register accessor: Slave Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smr`]
module"]
pub type SMR = crate::Reg<smr::SMR_SPEC>;
#[doc = "Slave Mode Register"]
pub mod smr;
#[doc = "IADR (rw) register accessor: Internal Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iadr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iadr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iadr`]
module"]
pub type IADR = crate::Reg<iadr::IADR_SPEC>;
#[doc = "Internal Address Register"]
pub mod iadr;
#[doc = "CWGR (rw) register accessor: Clock Waveform Generator Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwgr`]
module"]
pub type CWGR = crate::Reg<cwgr::CWGR_SPEC>;
#[doc = "Clock Waveform Generator Register"]
pub mod cwgr;
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
#[doc = "RHR (r) register accessor: Receive Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rhr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rhr`]
module"]
pub type RHR = crate::Reg<rhr::RHR_SPEC>;
#[doc = "Receive Holding Register"]
pub mod rhr;
#[doc = "THR (w) register accessor: Transmit Holding Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thr`]
module"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "Transmit Holding Register"]
pub mod thr;
