#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pmc_scer: PMC_SCER,
    pmc_scdr: PMC_SCDR,
    pmc_scsr: PMC_SCSR,
    _reserved3: [u8; 0x04],
    pmc_pcer0: PMC_PCER0,
    pmc_pcdr0: PMC_PCDR0,
    pmc_pcsr0: PMC_PCSR0,
    _reserved6: [u8; 0x04],
    ckgr_mor: CKGR_MOR,
    ckgr_mcfr: CKGR_MCFR,
    ckgr_pllar: CKGR_PLLAR,
    _reserved9: [u8; 0x04],
    pmc_mckr: PMC_MCKR,
    _reserved10: [u8; 0x0c],
    pmc_pck0: PMC_PCK0,
    pmc_pck1: PMC_PCK1,
    pmc_pck2: PMC_PCK2,
    _reserved13: [u8; 0x14],
    pmc_ier: PMC_IER,
    pmc_idr: PMC_IDR,
    pmc_sr: PMC_SR,
    pmc_imr: PMC_IMR,
    pmc_fsmr: PMC_FSMR,
    pmc_fspr: PMC_FSPR,
    pmc_focr: PMC_FOCR,
    _reserved20: [u8; 0x68],
    pmc_wpmr: PMC_WPMR,
    pmc_wpsr: PMC_WPSR,
}
impl RegisterBlock {
    #[doc = "0x00 - System Clock Enable Register"]
    #[inline(always)]
    pub const fn pmc_scer(&self) -> &PMC_SCER {
        &self.pmc_scer
    }
    #[doc = "0x04 - System Clock Disable Register"]
    #[inline(always)]
    pub const fn pmc_scdr(&self) -> &PMC_SCDR {
        &self.pmc_scdr
    }
    #[doc = "0x08 - System Clock Status Register"]
    #[inline(always)]
    pub const fn pmc_scsr(&self) -> &PMC_SCSR {
        &self.pmc_scsr
    }
    #[doc = "0x10 - Peripheral Clock Enable Register 0"]
    #[inline(always)]
    pub const fn pmc_pcer0(&self) -> &PMC_PCER0 {
        &self.pmc_pcer0
    }
    #[doc = "0x14 - Peripheral Clock Disable Register 0"]
    #[inline(always)]
    pub const fn pmc_pcdr0(&self) -> &PMC_PCDR0 {
        &self.pmc_pcdr0
    }
    #[doc = "0x18 - Peripheral Clock Status Register 0"]
    #[inline(always)]
    pub const fn pmc_pcsr0(&self) -> &PMC_PCSR0 {
        &self.pmc_pcsr0
    }
    #[doc = "0x20 - Main Oscillator Register"]
    #[inline(always)]
    pub const fn ckgr_mor(&self) -> &CKGR_MOR {
        &self.ckgr_mor
    }
    #[doc = "0x24 - Main Clock Frequency Register"]
    #[inline(always)]
    pub const fn ckgr_mcfr(&self) -> &CKGR_MCFR {
        &self.ckgr_mcfr
    }
    #[doc = "0x28 - PLLA Register"]
    #[inline(always)]
    pub const fn ckgr_pllar(&self) -> &CKGR_PLLAR {
        &self.ckgr_pllar
    }
    #[doc = "0x30 - Master Clock Register"]
    #[inline(always)]
    pub const fn pmc_mckr(&self) -> &PMC_MCKR {
        &self.pmc_mckr
    }
    #[doc = "0x40 - Programmable Clock 0 Register 0"]
    #[inline(always)]
    pub const fn pmc_pck0(&self) -> &PMC_PCK0 {
        &self.pmc_pck0
    }
    #[doc = "0x44 - Programmable Clock 0 Register 1"]
    #[inline(always)]
    pub const fn pmc_pck1(&self) -> &PMC_PCK1 {
        &self.pmc_pck1
    }
    #[doc = "0x48 - Programmable Clock 0 Register 2"]
    #[inline(always)]
    pub const fn pmc_pck2(&self) -> &PMC_PCK2 {
        &self.pmc_pck2
    }
    #[doc = "0x60 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn pmc_ier(&self) -> &PMC_IER {
        &self.pmc_ier
    }
    #[doc = "0x64 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn pmc_idr(&self) -> &PMC_IDR {
        &self.pmc_idr
    }
    #[doc = "0x68 - Status Register"]
    #[inline(always)]
    pub const fn pmc_sr(&self) -> &PMC_SR {
        &self.pmc_sr
    }
    #[doc = "0x6c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn pmc_imr(&self) -> &PMC_IMR {
        &self.pmc_imr
    }
    #[doc = "0x70 - Fast Start-up Mode Register"]
    #[inline(always)]
    pub const fn pmc_fsmr(&self) -> &PMC_FSMR {
        &self.pmc_fsmr
    }
    #[doc = "0x74 - Fast Start-up Polarity Register"]
    #[inline(always)]
    pub const fn pmc_fspr(&self) -> &PMC_FSPR {
        &self.pmc_fspr
    }
    #[doc = "0x78 - Fault Output Clear Register"]
    #[inline(always)]
    pub const fn pmc_focr(&self) -> &PMC_FOCR {
        &self.pmc_focr
    }
    #[doc = "0xe4 - Write Protect Mode Register"]
    #[inline(always)]
    pub const fn pmc_wpmr(&self) -> &PMC_WPMR {
        &self.pmc_wpmr
    }
    #[doc = "0xe8 - Write Protect Status Register"]
    #[inline(always)]
    pub const fn pmc_wpsr(&self) -> &PMC_WPSR {
        &self.pmc_wpsr
    }
}
#[doc = "PMC_SCER (w) register accessor: System Clock Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_scer::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_scer`]
module"]
pub type PMC_SCER = crate::Reg<pmc_scer::PMC_SCER_SPEC>;
#[doc = "System Clock Enable Register"]
pub mod pmc_scer;
#[doc = "PMC_SCDR (w) register accessor: System Clock Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_scdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_scdr`]
module"]
pub type PMC_SCDR = crate::Reg<pmc_scdr::PMC_SCDR_SPEC>;
#[doc = "System Clock Disable Register"]
pub mod pmc_scdr;
#[doc = "PMC_SCSR (r) register accessor: System Clock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_scsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_scsr`]
module"]
pub type PMC_SCSR = crate::Reg<pmc_scsr::PMC_SCSR_SPEC>;
#[doc = "System Clock Status Register"]
pub mod pmc_scsr;
#[doc = "PMC_PCER0 (w) register accessor: Peripheral Clock Enable Register 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_pcer0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_pcer0`]
module"]
pub type PMC_PCER0 = crate::Reg<pmc_pcer0::PMC_PCER0_SPEC>;
#[doc = "Peripheral Clock Enable Register 0"]
pub mod pmc_pcer0;
#[doc = "PMC_PCDR0 (w) register accessor: Peripheral Clock Disable Register 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_pcdr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_pcdr0`]
module"]
pub type PMC_PCDR0 = crate::Reg<pmc_pcdr0::PMC_PCDR0_SPEC>;
#[doc = "Peripheral Clock Disable Register 0"]
pub mod pmc_pcdr0;
#[doc = "PMC_PCSR0 (r) register accessor: Peripheral Clock Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_pcsr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_pcsr0`]
module"]
pub type PMC_PCSR0 = crate::Reg<pmc_pcsr0::PMC_PCSR0_SPEC>;
#[doc = "Peripheral Clock Status Register 0"]
pub mod pmc_pcsr0;
#[doc = "CKGR_MOR (rw) register accessor: Main Oscillator Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckgr_mor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckgr_mor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckgr_mor`]
module"]
pub type CKGR_MOR = crate::Reg<ckgr_mor::CKGR_MOR_SPEC>;
#[doc = "Main Oscillator Register"]
pub mod ckgr_mor;
#[doc = "CKGR_MCFR (r) register accessor: Main Clock Frequency Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckgr_mcfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckgr_mcfr`]
module"]
pub type CKGR_MCFR = crate::Reg<ckgr_mcfr::CKGR_MCFR_SPEC>;
#[doc = "Main Clock Frequency Register"]
pub mod ckgr_mcfr;
#[doc = "CKGR_PLLAR (rw) register accessor: PLLA Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckgr_pllar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckgr_pllar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckgr_pllar`]
module"]
pub type CKGR_PLLAR = crate::Reg<ckgr_pllar::CKGR_PLLAR_SPEC>;
#[doc = "PLLA Register"]
pub mod ckgr_pllar;
#[doc = "PMC_MCKR (rw) register accessor: Master Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_mckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_mckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_mckr`]
module"]
pub type PMC_MCKR = crate::Reg<pmc_mckr::PMC_MCKR_SPEC>;
#[doc = "Master Clock Register"]
pub mod pmc_mckr;
#[doc = "PMC_PCK0 (rw) register accessor: Programmable Clock 0 Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_pck0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_pck0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_pck0`]
module"]
pub type PMC_PCK0 = crate::Reg<pmc_pck0::PMC_PCK0_SPEC>;
#[doc = "Programmable Clock 0 Register 0"]
pub mod pmc_pck0;
#[doc = "PMC_PCK1 (rw) register accessor: Programmable Clock 0 Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_pck1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_pck1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_pck1`]
module"]
pub type PMC_PCK1 = crate::Reg<pmc_pck1::PMC_PCK1_SPEC>;
#[doc = "Programmable Clock 0 Register 1"]
pub mod pmc_pck1;
#[doc = "PMC_PCK2 (rw) register accessor: Programmable Clock 0 Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_pck2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_pck2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_pck2`]
module"]
pub type PMC_PCK2 = crate::Reg<pmc_pck2::PMC_PCK2_SPEC>;
#[doc = "Programmable Clock 0 Register 2"]
pub mod pmc_pck2;
#[doc = "PMC_IER (w) register accessor: Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_ier`]
module"]
pub type PMC_IER = crate::Reg<pmc_ier::PMC_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod pmc_ier;
#[doc = "PMC_IDR (w) register accessor: Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_idr`]
module"]
pub type PMC_IDR = crate::Reg<pmc_idr::PMC_IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod pmc_idr;
#[doc = "PMC_SR (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_sr`]
module"]
pub type PMC_SR = crate::Reg<pmc_sr::PMC_SR_SPEC>;
#[doc = "Status Register"]
pub mod pmc_sr;
#[doc = "PMC_IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_imr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_imr`]
module"]
pub type PMC_IMR = crate::Reg<pmc_imr::PMC_IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod pmc_imr;
#[doc = "PMC_FSMR (rw) register accessor: Fast Start-up Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_fsmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_fsmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_fsmr`]
module"]
pub type PMC_FSMR = crate::Reg<pmc_fsmr::PMC_FSMR_SPEC>;
#[doc = "Fast Start-up Mode Register"]
pub mod pmc_fsmr;
#[doc = "PMC_FSPR (rw) register accessor: Fast Start-up Polarity Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_fspr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_fspr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_fspr`]
module"]
pub type PMC_FSPR = crate::Reg<pmc_fspr::PMC_FSPR_SPEC>;
#[doc = "Fast Start-up Polarity Register"]
pub mod pmc_fspr;
#[doc = "PMC_FOCR (w) register accessor: Fault Output Clear Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_focr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_focr`]
module"]
pub type PMC_FOCR = crate::Reg<pmc_focr::PMC_FOCR_SPEC>;
#[doc = "Fault Output Clear Register"]
pub mod pmc_focr;
#[doc = "PMC_WPMR (rw) register accessor: Write Protect Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_wpmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_wpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_wpmr`]
module"]
pub type PMC_WPMR = crate::Reg<pmc_wpmr::PMC_WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod pmc_wpmr;
#[doc = "PMC_WPSR (r) register accessor: Write Protect Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_wpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_wpsr`]
module"]
pub type PMC_WPSR = crate::Reg<pmc_wpsr::PMC_WPSR_SPEC>;
#[doc = "Write Protect Status Register"]
pub mod pmc_wpsr;
