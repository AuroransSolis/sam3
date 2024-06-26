#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pmc_scer: PmcScer,
    pmc_scdr: PmcScdr,
    pmc_scsr: PmcScsr,
    _reserved3: [u8; 0x04],
    pmc_pcer0: PmcPcer0,
    pmc_pcdr0: PmcPcdr0,
    pmc_pcsr0: PmcPcsr0,
    _reserved6: [u8; 0x04],
    ckgr_mor: CkgrMor,
    ckgr_mcfr: CkgrMcfr,
    ckgr_pllar: CkgrPllar,
    _reserved9: [u8; 0x04],
    pmc_mckr: PmcMckr,
    _reserved10: [u8; 0x0c],
    pmc_pck0: PmcPck0,
    pmc_pck1: PmcPck1,
    pmc_pck2: PmcPck2,
    _reserved13: [u8; 0x14],
    pmc_ier: PmcIer,
    pmc_idr: PmcIdr,
    pmc_sr: PmcSr,
    pmc_imr: PmcImr,
    pmc_fsmr: PmcFsmr,
    pmc_fspr: PmcFspr,
    pmc_focr: PmcFocr,
    _reserved20: [u8; 0x68],
    pmc_wpmr: PmcWpmr,
    pmc_wpsr: PmcWpsr,
}
impl RegisterBlock {
    #[doc = "0x00 - System Clock Enable Register"]
    #[inline(always)]
    pub const fn pmc_scer(&self) -> &PmcScer {
        &self.pmc_scer
    }
    #[doc = "0x04 - System Clock Disable Register"]
    #[inline(always)]
    pub const fn pmc_scdr(&self) -> &PmcScdr {
        &self.pmc_scdr
    }
    #[doc = "0x08 - System Clock Status Register"]
    #[inline(always)]
    pub const fn pmc_scsr(&self) -> &PmcScsr {
        &self.pmc_scsr
    }
    #[doc = "0x10 - Peripheral Clock Enable Register 0"]
    #[inline(always)]
    pub const fn pmc_pcer0(&self) -> &PmcPcer0 {
        &self.pmc_pcer0
    }
    #[doc = "0x14 - Peripheral Clock Disable Register 0"]
    #[inline(always)]
    pub const fn pmc_pcdr0(&self) -> &PmcPcdr0 {
        &self.pmc_pcdr0
    }
    #[doc = "0x18 - Peripheral Clock Status Register 0"]
    #[inline(always)]
    pub const fn pmc_pcsr0(&self) -> &PmcPcsr0 {
        &self.pmc_pcsr0
    }
    #[doc = "0x20 - Main Oscillator Register"]
    #[inline(always)]
    pub const fn ckgr_mor(&self) -> &CkgrMor {
        &self.ckgr_mor
    }
    #[doc = "0x24 - Main Clock Frequency Register"]
    #[inline(always)]
    pub const fn ckgr_mcfr(&self) -> &CkgrMcfr {
        &self.ckgr_mcfr
    }
    #[doc = "0x28 - PLLA Register"]
    #[inline(always)]
    pub const fn ckgr_pllar(&self) -> &CkgrPllar {
        &self.ckgr_pllar
    }
    #[doc = "0x30 - Master Clock Register"]
    #[inline(always)]
    pub const fn pmc_mckr(&self) -> &PmcMckr {
        &self.pmc_mckr
    }
    #[doc = "0x40 - Programmable Clock 0 Register 0"]
    #[inline(always)]
    pub const fn pmc_pck0(&self) -> &PmcPck0 {
        &self.pmc_pck0
    }
    #[doc = "0x44 - Programmable Clock 0 Register 1"]
    #[inline(always)]
    pub const fn pmc_pck1(&self) -> &PmcPck1 {
        &self.pmc_pck1
    }
    #[doc = "0x48 - Programmable Clock 0 Register 2"]
    #[inline(always)]
    pub const fn pmc_pck2(&self) -> &PmcPck2 {
        &self.pmc_pck2
    }
    #[doc = "0x60 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn pmc_ier(&self) -> &PmcIer {
        &self.pmc_ier
    }
    #[doc = "0x64 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn pmc_idr(&self) -> &PmcIdr {
        &self.pmc_idr
    }
    #[doc = "0x68 - Status Register"]
    #[inline(always)]
    pub const fn pmc_sr(&self) -> &PmcSr {
        &self.pmc_sr
    }
    #[doc = "0x6c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn pmc_imr(&self) -> &PmcImr {
        &self.pmc_imr
    }
    #[doc = "0x70 - Fast Start-up Mode Register"]
    #[inline(always)]
    pub const fn pmc_fsmr(&self) -> &PmcFsmr {
        &self.pmc_fsmr
    }
    #[doc = "0x74 - Fast Start-up Polarity Register"]
    #[inline(always)]
    pub const fn pmc_fspr(&self) -> &PmcFspr {
        &self.pmc_fspr
    }
    #[doc = "0x78 - Fault Output Clear Register"]
    #[inline(always)]
    pub const fn pmc_focr(&self) -> &PmcFocr {
        &self.pmc_focr
    }
    #[doc = "0xe4 - Write Protect Mode Register"]
    #[inline(always)]
    pub const fn pmc_wpmr(&self) -> &PmcWpmr {
        &self.pmc_wpmr
    }
    #[doc = "0xe8 - Write Protect Status Register"]
    #[inline(always)]
    pub const fn pmc_wpsr(&self) -> &PmcWpsr {
        &self.pmc_wpsr
    }
}
#[doc = "PMC_SCER (w) register accessor: System Clock Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_scer::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_scer`]
module"]
#[doc(alias = "PMC_SCER")]
pub type PmcScer = crate::Reg<pmc_scer::PmcScerSpec>;
#[doc = "System Clock Enable Register"]
pub mod pmc_scer;
#[doc = "PMC_SCDR (w) register accessor: System Clock Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_scdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_scdr`]
module"]
#[doc(alias = "PMC_SCDR")]
pub type PmcScdr = crate::Reg<pmc_scdr::PmcScdrSpec>;
#[doc = "System Clock Disable Register"]
pub mod pmc_scdr;
#[doc = "PMC_SCSR (r) register accessor: System Clock Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmc_scsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_scsr`]
module"]
#[doc(alias = "PMC_SCSR")]
pub type PmcScsr = crate::Reg<pmc_scsr::PmcScsrSpec>;
#[doc = "System Clock Status Register"]
pub mod pmc_scsr;
#[doc = "PMC_PCER0 (w) register accessor: Peripheral Clock Enable Register 0\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_pcer0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_pcer0`]
module"]
#[doc(alias = "PMC_PCER0")]
pub type PmcPcer0 = crate::Reg<pmc_pcer0::PmcPcer0Spec>;
#[doc = "Peripheral Clock Enable Register 0"]
pub mod pmc_pcer0;
#[doc = "PMC_PCDR0 (w) register accessor: Peripheral Clock Disable Register 0\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_pcdr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_pcdr0`]
module"]
#[doc(alias = "PMC_PCDR0")]
pub type PmcPcdr0 = crate::Reg<pmc_pcdr0::PmcPcdr0Spec>;
#[doc = "Peripheral Clock Disable Register 0"]
pub mod pmc_pcdr0;
#[doc = "PMC_PCSR0 (r) register accessor: Peripheral Clock Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pmc_pcsr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_pcsr0`]
module"]
#[doc(alias = "PMC_PCSR0")]
pub type PmcPcsr0 = crate::Reg<pmc_pcsr0::PmcPcsr0Spec>;
#[doc = "Peripheral Clock Status Register 0"]
pub mod pmc_pcsr0;
#[doc = "CKGR_MOR (rw) register accessor: Main Oscillator Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckgr_mor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgr_mor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckgr_mor`]
module"]
#[doc(alias = "CKGR_MOR")]
pub type CkgrMor = crate::Reg<ckgr_mor::CkgrMorSpec>;
#[doc = "Main Oscillator Register"]
pub mod ckgr_mor;
#[doc = "CKGR_MCFR (r) register accessor: Main Clock Frequency Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckgr_mcfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckgr_mcfr`]
module"]
#[doc(alias = "CKGR_MCFR")]
pub type CkgrMcfr = crate::Reg<ckgr_mcfr::CkgrMcfrSpec>;
#[doc = "Main Clock Frequency Register"]
pub mod ckgr_mcfr;
#[doc = "CKGR_PLLAR (rw) register accessor: PLLA Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckgr_pllar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgr_pllar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckgr_pllar`]
module"]
#[doc(alias = "CKGR_PLLAR")]
pub type CkgrPllar = crate::Reg<ckgr_pllar::CkgrPllarSpec>;
#[doc = "PLLA Register"]
pub mod ckgr_pllar;
#[doc = "PMC_MCKR (rw) register accessor: Master Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmc_mckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_mckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_mckr`]
module"]
#[doc(alias = "PMC_MCKR")]
pub type PmcMckr = crate::Reg<pmc_mckr::PmcMckrSpec>;
#[doc = "Master Clock Register"]
pub mod pmc_mckr;
#[doc = "PMC_PCK0 (rw) register accessor: Programmable Clock 0 Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pmc_pck0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_pck0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_pck0`]
module"]
#[doc(alias = "PMC_PCK0")]
pub type PmcPck0 = crate::Reg<pmc_pck0::PmcPck0Spec>;
#[doc = "Programmable Clock 0 Register 0"]
pub mod pmc_pck0;
#[doc = "PMC_PCK1 (rw) register accessor: Programmable Clock 0 Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pmc_pck1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_pck1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_pck1`]
module"]
#[doc(alias = "PMC_PCK1")]
pub type PmcPck1 = crate::Reg<pmc_pck1::PmcPck1Spec>;
#[doc = "Programmable Clock 0 Register 1"]
pub mod pmc_pck1;
#[doc = "PMC_PCK2 (rw) register accessor: Programmable Clock 0 Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pmc_pck2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_pck2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_pck2`]
module"]
#[doc(alias = "PMC_PCK2")]
pub type PmcPck2 = crate::Reg<pmc_pck2::PmcPck2Spec>;
#[doc = "Programmable Clock 0 Register 2"]
pub mod pmc_pck2;
#[doc = "PMC_IER (w) register accessor: Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_ier`]
module"]
#[doc(alias = "PMC_IER")]
pub type PmcIer = crate::Reg<pmc_ier::PmcIerSpec>;
#[doc = "Interrupt Enable Register"]
pub mod pmc_ier;
#[doc = "PMC_IDR (w) register accessor: Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_idr`]
module"]
#[doc(alias = "PMC_IDR")]
pub type PmcIdr = crate::Reg<pmc_idr::PmcIdrSpec>;
#[doc = "Interrupt Disable Register"]
pub mod pmc_idr;
#[doc = "PMC_SR (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmc_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_sr`]
module"]
#[doc(alias = "PMC_SR")]
pub type PmcSr = crate::Reg<pmc_sr::PmcSrSpec>;
#[doc = "Status Register"]
pub mod pmc_sr;
#[doc = "PMC_IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmc_imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_imr`]
module"]
#[doc(alias = "PMC_IMR")]
pub type PmcImr = crate::Reg<pmc_imr::PmcImrSpec>;
#[doc = "Interrupt Mask Register"]
pub mod pmc_imr;
#[doc = "PMC_FSMR (rw) register accessor: Fast Start-up Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmc_fsmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_fsmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_fsmr`]
module"]
#[doc(alias = "PMC_FSMR")]
pub type PmcFsmr = crate::Reg<pmc_fsmr::PmcFsmrSpec>;
#[doc = "Fast Start-up Mode Register"]
pub mod pmc_fsmr;
#[doc = "PMC_FSPR (rw) register accessor: Fast Start-up Polarity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmc_fspr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_fspr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_fspr`]
module"]
#[doc(alias = "PMC_FSPR")]
pub type PmcFspr = crate::Reg<pmc_fspr::PmcFsprSpec>;
#[doc = "Fast Start-up Polarity Register"]
pub mod pmc_fspr;
#[doc = "PMC_FOCR (w) register accessor: Fault Output Clear Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_focr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_focr`]
module"]
#[doc(alias = "PMC_FOCR")]
pub type PmcFocr = crate::Reg<pmc_focr::PmcFocrSpec>;
#[doc = "Fault Output Clear Register"]
pub mod pmc_focr;
#[doc = "PMC_WPMR (rw) register accessor: Write Protect Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmc_wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_wpmr`]
module"]
#[doc(alias = "PMC_WPMR")]
pub type PmcWpmr = crate::Reg<pmc_wpmr::PmcWpmrSpec>;
#[doc = "Write Protect Mode Register"]
pub mod pmc_wpmr;
#[doc = "PMC_WPSR (r) register accessor: Write Protect Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmc_wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmc_wpsr`]
module"]
#[doc(alias = "PMC_WPSR")]
pub type PmcWpsr = crate::Reg<pmc_wpsr::PmcWpsrSpec>;
#[doc = "Write Protect Status Register"]
pub mod pmc_wpsr;
