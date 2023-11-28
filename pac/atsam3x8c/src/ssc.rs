#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cr: CR,
    cmr: CMR,
    _reserved2: [u8; 0x08],
    rcmr: RCMR,
    rfmr: RFMR,
    tcmr: TCMR,
    tfmr: TFMR,
    rhr: RHR,
    thr: THR,
    _reserved8: [u8; 0x08],
    rshr: RSHR,
    tshr: TSHR,
    rc0r: RC0R,
    rc1r: RC1R,
    sr: SR,
    ier: IER,
    idr: IDR,
    imr: IMR,
    _reserved16: [u8; 0x94],
    wpmr: WPMR,
    wpsr: WPSR,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - Clock Mode Register"]
    #[inline(always)]
    pub const fn cmr(&self) -> &CMR {
        &self.cmr
    }
    #[doc = "0x10 - Receive Clock Mode Register"]
    #[inline(always)]
    pub const fn rcmr(&self) -> &RCMR {
        &self.rcmr
    }
    #[doc = "0x14 - Receive Frame Mode Register"]
    #[inline(always)]
    pub const fn rfmr(&self) -> &RFMR {
        &self.rfmr
    }
    #[doc = "0x18 - Transmit Clock Mode Register"]
    #[inline(always)]
    pub const fn tcmr(&self) -> &TCMR {
        &self.tcmr
    }
    #[doc = "0x1c - Transmit Frame Mode Register"]
    #[inline(always)]
    pub const fn tfmr(&self) -> &TFMR {
        &self.tfmr
    }
    #[doc = "0x20 - Receive Holding Register"]
    #[inline(always)]
    pub const fn rhr(&self) -> &RHR {
        &self.rhr
    }
    #[doc = "0x24 - Transmit Holding Register"]
    #[inline(always)]
    pub const fn thr(&self) -> &THR {
        &self.thr
    }
    #[doc = "0x30 - Receive Sync. Holding Register"]
    #[inline(always)]
    pub const fn rshr(&self) -> &RSHR {
        &self.rshr
    }
    #[doc = "0x34 - Transmit Sync. Holding Register"]
    #[inline(always)]
    pub const fn tshr(&self) -> &TSHR {
        &self.tshr
    }
    #[doc = "0x38 - Receive Compare 0 Register"]
    #[inline(always)]
    pub const fn rc0r(&self) -> &RC0R {
        &self.rc0r
    }
    #[doc = "0x3c - Receive Compare 1 Register"]
    #[inline(always)]
    pub const fn rc1r(&self) -> &RC1R {
        &self.rc1r
    }
    #[doc = "0x40 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x44 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x48 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    #[doc = "0x4c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
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
}
#[doc = "CR (w) register accessor: Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "CMR (rw) register accessor: Clock Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr`]
module"]
pub type CMR = crate::Reg<cmr::CMR_SPEC>;
#[doc = "Clock Mode Register"]
pub mod cmr;
#[doc = "RCMR (rw) register accessor: Receive Clock Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcmr`]
module"]
pub type RCMR = crate::Reg<rcmr::RCMR_SPEC>;
#[doc = "Receive Clock Mode Register"]
pub mod rcmr;
#[doc = "RFMR (rw) register accessor: Receive Frame Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfmr`]
module"]
pub type RFMR = crate::Reg<rfmr::RFMR_SPEC>;
#[doc = "Receive Frame Mode Register"]
pub mod rfmr;
#[doc = "TCMR (rw) register accessor: Transmit Clock Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcmr`]
module"]
pub type TCMR = crate::Reg<tcmr::TCMR_SPEC>;
#[doc = "Transmit Clock Mode Register"]
pub mod tcmr;
#[doc = "TFMR (rw) register accessor: Transmit Frame Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfmr`]
module"]
pub type TFMR = crate::Reg<tfmr::TFMR_SPEC>;
#[doc = "Transmit Frame Mode Register"]
pub mod tfmr;
#[doc = "RHR (r) register accessor: Receive Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rhr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rhr`]
module"]
pub type RHR = crate::Reg<rhr::RHR_SPEC>;
#[doc = "Receive Holding Register"]
pub mod rhr;
#[doc = "THR (w) register accessor: Transmit Holding Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thr`]
module"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "RSHR (r) register accessor: Receive Sync. Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rshr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rshr`]
module"]
pub type RSHR = crate::Reg<rshr::RSHR_SPEC>;
#[doc = "Receive Sync. Holding Register"]
pub mod rshr;
#[doc = "TSHR (rw) register accessor: Transmit Sync. Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tshr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tshr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tshr`]
module"]
pub type TSHR = crate::Reg<tshr::TSHR_SPEC>;
#[doc = "Transmit Sync. Holding Register"]
pub mod tshr;
#[doc = "RC0R (rw) register accessor: Receive Compare 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc0r`]
module"]
pub type RC0R = crate::Reg<rc0r::RC0R_SPEC>;
#[doc = "Receive Compare 0 Register"]
pub mod rc0r;
#[doc = "RC1R (rw) register accessor: Receive Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc1r`]
module"]
pub type RC1R = crate::Reg<rc1r::RC1R_SPEC>;
#[doc = "Receive Compare 1 Register"]
pub mod rc1r;
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
