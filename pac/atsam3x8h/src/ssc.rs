#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    cmr: Cmr,
    _reserved2: [u8; 0x08],
    rcmr: Rcmr,
    rfmr: Rfmr,
    tcmr: Tcmr,
    tfmr: Tfmr,
    rhr: Rhr,
    thr: Thr,
    _reserved8: [u8; 0x08],
    rshr: Rshr,
    tshr: Tshr,
    rc0r: Rc0r,
    rc1r: Rc1r,
    sr: Sr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    _reserved16: [u8; 0x94],
    wpmr: Wpmr,
    wpsr: Wpsr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - Clock Mode Register"]
    #[inline(always)]
    pub const fn cmr(&self) -> &Cmr {
        &self.cmr
    }
    #[doc = "0x10 - Receive Clock Mode Register"]
    #[inline(always)]
    pub const fn rcmr(&self) -> &Rcmr {
        &self.rcmr
    }
    #[doc = "0x14 - Receive Frame Mode Register"]
    #[inline(always)]
    pub const fn rfmr(&self) -> &Rfmr {
        &self.rfmr
    }
    #[doc = "0x18 - Transmit Clock Mode Register"]
    #[inline(always)]
    pub const fn tcmr(&self) -> &Tcmr {
        &self.tcmr
    }
    #[doc = "0x1c - Transmit Frame Mode Register"]
    #[inline(always)]
    pub const fn tfmr(&self) -> &Tfmr {
        &self.tfmr
    }
    #[doc = "0x20 - Receive Holding Register"]
    #[inline(always)]
    pub const fn rhr(&self) -> &Rhr {
        &self.rhr
    }
    #[doc = "0x24 - Transmit Holding Register"]
    #[inline(always)]
    pub const fn thr(&self) -> &Thr {
        &self.thr
    }
    #[doc = "0x30 - Receive Sync. Holding Register"]
    #[inline(always)]
    pub const fn rshr(&self) -> &Rshr {
        &self.rshr
    }
    #[doc = "0x34 - Transmit Sync. Holding Register"]
    #[inline(always)]
    pub const fn tshr(&self) -> &Tshr {
        &self.tshr
    }
    #[doc = "0x38 - Receive Compare 0 Register"]
    #[inline(always)]
    pub const fn rc0r(&self) -> &Rc0r {
        &self.rc0r
    }
    #[doc = "0x3c - Receive Compare 1 Register"]
    #[inline(always)]
    pub const fn rc1r(&self) -> &Rc1r {
        &self.rc1r
    }
    #[doc = "0x40 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x44 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x48 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x4c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
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
}
#[doc = "CR (w) register accessor: Control Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "CMR (rw) register accessor: Clock Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr`]
module"]
#[doc(alias = "CMR")]
pub type Cmr = crate::Reg<cmr::CmrSpec>;
#[doc = "Clock Mode Register"]
pub mod cmr;
#[doc = "RCMR (rw) register accessor: Receive Clock Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcmr`]
module"]
#[doc(alias = "RCMR")]
pub type Rcmr = crate::Reg<rcmr::RcmrSpec>;
#[doc = "Receive Clock Mode Register"]
pub mod rcmr;
#[doc = "RFMR (rw) register accessor: Receive Frame Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfmr`]
module"]
#[doc(alias = "RFMR")]
pub type Rfmr = crate::Reg<rfmr::RfmrSpec>;
#[doc = "Receive Frame Mode Register"]
pub mod rfmr;
#[doc = "TCMR (rw) register accessor: Transmit Clock Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcmr`]
module"]
#[doc(alias = "TCMR")]
pub type Tcmr = crate::Reg<tcmr::TcmrSpec>;
#[doc = "Transmit Clock Mode Register"]
pub mod tcmr;
#[doc = "TFMR (rw) register accessor: Transmit Frame Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tfmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfmr`]
module"]
#[doc(alias = "TFMR")]
pub type Tfmr = crate::Reg<tfmr::TfmrSpec>;
#[doc = "Transmit Frame Mode Register"]
pub mod tfmr;
#[doc = "RHR (r) register accessor: Receive Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rhr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rhr`]
module"]
#[doc(alias = "RHR")]
pub type Rhr = crate::Reg<rhr::RhrSpec>;
#[doc = "Receive Holding Register"]
pub mod rhr;
#[doc = "THR (w) register accessor: Transmit Holding Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thr`]
module"]
#[doc(alias = "THR")]
pub type Thr = crate::Reg<thr::ThrSpec>;
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "RSHR (r) register accessor: Receive Sync. Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rshr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rshr`]
module"]
#[doc(alias = "RSHR")]
pub type Rshr = crate::Reg<rshr::RshrSpec>;
#[doc = "Receive Sync. Holding Register"]
pub mod rshr;
#[doc = "TSHR (rw) register accessor: Transmit Sync. Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tshr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tshr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tshr`]
module"]
#[doc(alias = "TSHR")]
pub type Tshr = crate::Reg<tshr::TshrSpec>;
#[doc = "Transmit Sync. Holding Register"]
pub mod tshr;
#[doc = "RC0R (rw) register accessor: Receive Compare 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rc0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc0r`]
module"]
#[doc(alias = "RC0R")]
pub type Rc0r = crate::Reg<rc0r::Rc0rSpec>;
#[doc = "Receive Compare 0 Register"]
pub mod rc0r;
#[doc = "RC1R (rw) register accessor: Receive Compare 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rc1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc1r`]
module"]
#[doc(alias = "RC1R")]
pub type Rc1r = crate::Reg<rc1r::Rc1rSpec>;
#[doc = "Receive Compare 1 Register"]
pub mod rc1r;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "IER (w) register accessor: Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "WPMR (rw) register accessor: Write Protect Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`]
module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: Write Protect Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`]
module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "Write Protect Status Register"]
pub mod wpsr;
