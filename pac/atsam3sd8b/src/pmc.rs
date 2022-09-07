#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Clock Enable Register"]
    pub pmc_scer: PMC_SCER,
    #[doc = "0x04 - System Clock Disable Register"]
    pub pmc_scdr: PMC_SCDR,
    #[doc = "0x08 - System Clock Status Register"]
    pub pmc_scsr: PMC_SCSR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Peripheral Clock Enable Register 0"]
    pub pmc_pcer0: PMC_PCER0,
    #[doc = "0x14 - Peripheral Clock Disable Register 0"]
    pub pmc_pcdr0: PMC_PCDR0,
    #[doc = "0x18 - Peripheral Clock Status Register 0"]
    pub pmc_pcsr0: PMC_PCSR0,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - Main Oscillator Register"]
    pub ckgr_mor: CKGR_MOR,
    #[doc = "0x24 - Main Clock Frequency Register"]
    pub ckgr_mcfr: CKGR_MCFR,
    #[doc = "0x28 - PLLA Register"]
    pub ckgr_pllar: CKGR_PLLAR,
    #[doc = "0x2c - PLLB Register"]
    pub ckgr_pllbr: CKGR_PLLBR,
    #[doc = "0x30 - Master Clock Register"]
    pub pmc_mckr: PMC_MCKR,
    _reserved11: [u8; 0x04],
    #[doc = "0x38 - USB Clock Register"]
    pub pmc_usb: PMC_USB,
    _reserved12: [u8; 0x04],
    #[doc = "0x40..0x4c - Programmable Clock 0 Register"]
    pub pmc_pck: [PMC_PCK; 3],
    _reserved13: [u8; 0x14],
    #[doc = "0x60 - Interrupt Enable Register"]
    pub pmc_ier: PMC_IER,
    #[doc = "0x64 - Interrupt Disable Register"]
    pub pmc_idr: PMC_IDR,
    #[doc = "0x68 - Status Register"]
    pub pmc_sr: PMC_SR,
    #[doc = "0x6c - Interrupt Mask Register"]
    pub pmc_imr: PMC_IMR,
    #[doc = "0x70 - Fast Startup Mode Register"]
    pub pmc_fsmr: PMC_FSMR,
    #[doc = "0x74 - Fast Startup Polarity Register"]
    pub pmc_fspr: PMC_FSPR,
    #[doc = "0x78 - Fault Output Clear Register"]
    pub pmc_focr: PMC_FOCR,
    _reserved20: [u8; 0x68],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub pmc_wpmr: PMC_WPMR,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub pmc_wpsr: PMC_WPSR,
    _reserved22: [u8; 0x14],
    #[doc = "0x100 - Peripheral Clock Enable Register 1"]
    pub pmc_pcer1: PMC_PCER1,
    #[doc = "0x104 - Peripheral Clock Disable Register 1"]
    pub pmc_pcdr1: PMC_PCDR1,
    #[doc = "0x108 - Peripheral Clock Status Register 1"]
    pub pmc_pcsr1: PMC_PCSR1,
    _reserved25: [u8; 0x04],
    #[doc = "0x110 - Oscillator Calibration Register"]
    pub pmc_ocr: PMC_OCR,
}
#[doc = "PMC_SCER (w) register accessor: an alias for `Reg<PMC_SCER_SPEC>`"]
pub type PMC_SCER = crate::Reg<pmc_scer::PMC_SCER_SPEC>;
#[doc = "System Clock Enable Register"]
pub mod pmc_scer;
#[doc = "PMC_SCDR (w) register accessor: an alias for `Reg<PMC_SCDR_SPEC>`"]
pub type PMC_SCDR = crate::Reg<pmc_scdr::PMC_SCDR_SPEC>;
#[doc = "System Clock Disable Register"]
pub mod pmc_scdr;
#[doc = "PMC_SCSR (r) register accessor: an alias for `Reg<PMC_SCSR_SPEC>`"]
pub type PMC_SCSR = crate::Reg<pmc_scsr::PMC_SCSR_SPEC>;
#[doc = "System Clock Status Register"]
pub mod pmc_scsr;
#[doc = "PMC_PCER0 (w) register accessor: an alias for `Reg<PMC_PCER0_SPEC>`"]
pub type PMC_PCER0 = crate::Reg<pmc_pcer0::PMC_PCER0_SPEC>;
#[doc = "Peripheral Clock Enable Register 0"]
pub mod pmc_pcer0;
#[doc = "PMC_PCDR0 (w) register accessor: an alias for `Reg<PMC_PCDR0_SPEC>`"]
pub type PMC_PCDR0 = crate::Reg<pmc_pcdr0::PMC_PCDR0_SPEC>;
#[doc = "Peripheral Clock Disable Register 0"]
pub mod pmc_pcdr0;
#[doc = "PMC_PCSR0 (r) register accessor: an alias for `Reg<PMC_PCSR0_SPEC>`"]
pub type PMC_PCSR0 = crate::Reg<pmc_pcsr0::PMC_PCSR0_SPEC>;
#[doc = "Peripheral Clock Status Register 0"]
pub mod pmc_pcsr0;
#[doc = "CKGR_MOR (rw) register accessor: an alias for `Reg<CKGR_MOR_SPEC>`"]
pub type CKGR_MOR = crate::Reg<ckgr_mor::CKGR_MOR_SPEC>;
#[doc = "Main Oscillator Register"]
pub mod ckgr_mor;
#[doc = "CKGR_MCFR (rw) register accessor: an alias for `Reg<CKGR_MCFR_SPEC>`"]
pub type CKGR_MCFR = crate::Reg<ckgr_mcfr::CKGR_MCFR_SPEC>;
#[doc = "Main Clock Frequency Register"]
pub mod ckgr_mcfr;
#[doc = "CKGR_PLLAR (rw) register accessor: an alias for `Reg<CKGR_PLLAR_SPEC>`"]
pub type CKGR_PLLAR = crate::Reg<ckgr_pllar::CKGR_PLLAR_SPEC>;
#[doc = "PLLA Register"]
pub mod ckgr_pllar;
#[doc = "CKGR_PLLBR (rw) register accessor: an alias for `Reg<CKGR_PLLBR_SPEC>`"]
pub type CKGR_PLLBR = crate::Reg<ckgr_pllbr::CKGR_PLLBR_SPEC>;
#[doc = "PLLB Register"]
pub mod ckgr_pllbr;
#[doc = "PMC_MCKR (rw) register accessor: an alias for `Reg<PMC_MCKR_SPEC>`"]
pub type PMC_MCKR = crate::Reg<pmc_mckr::PMC_MCKR_SPEC>;
#[doc = "Master Clock Register"]
pub mod pmc_mckr;
#[doc = "PMC_USB (rw) register accessor: an alias for `Reg<PMC_USB_SPEC>`"]
pub type PMC_USB = crate::Reg<pmc_usb::PMC_USB_SPEC>;
#[doc = "USB Clock Register"]
pub mod pmc_usb;
#[doc = "PMC_PCK (rw) register accessor: an alias for `Reg<PMC_PCK_SPEC>`"]
pub type PMC_PCK = crate::Reg<pmc_pck::PMC_PCK_SPEC>;
#[doc = "Programmable Clock 0 Register"]
pub mod pmc_pck;
#[doc = "PMC_IER (w) register accessor: an alias for `Reg<PMC_IER_SPEC>`"]
pub type PMC_IER = crate::Reg<pmc_ier::PMC_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod pmc_ier;
#[doc = "PMC_IDR (w) register accessor: an alias for `Reg<PMC_IDR_SPEC>`"]
pub type PMC_IDR = crate::Reg<pmc_idr::PMC_IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod pmc_idr;
#[doc = "PMC_SR (r) register accessor: an alias for `Reg<PMC_SR_SPEC>`"]
pub type PMC_SR = crate::Reg<pmc_sr::PMC_SR_SPEC>;
#[doc = "Status Register"]
pub mod pmc_sr;
#[doc = "PMC_IMR (r) register accessor: an alias for `Reg<PMC_IMR_SPEC>`"]
pub type PMC_IMR = crate::Reg<pmc_imr::PMC_IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod pmc_imr;
#[doc = "PMC_FSMR (rw) register accessor: an alias for `Reg<PMC_FSMR_SPEC>`"]
pub type PMC_FSMR = crate::Reg<pmc_fsmr::PMC_FSMR_SPEC>;
#[doc = "Fast Startup Mode Register"]
pub mod pmc_fsmr;
#[doc = "PMC_FSPR (rw) register accessor: an alias for `Reg<PMC_FSPR_SPEC>`"]
pub type PMC_FSPR = crate::Reg<pmc_fspr::PMC_FSPR_SPEC>;
#[doc = "Fast Startup Polarity Register"]
pub mod pmc_fspr;
#[doc = "PMC_FOCR (w) register accessor: an alias for `Reg<PMC_FOCR_SPEC>`"]
pub type PMC_FOCR = crate::Reg<pmc_focr::PMC_FOCR_SPEC>;
#[doc = "Fault Output Clear Register"]
pub mod pmc_focr;
#[doc = "PMC_WPMR (rw) register accessor: an alias for `Reg<PMC_WPMR_SPEC>`"]
pub type PMC_WPMR = crate::Reg<pmc_wpmr::PMC_WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod pmc_wpmr;
#[doc = "PMC_WPSR (r) register accessor: an alias for `Reg<PMC_WPSR_SPEC>`"]
pub type PMC_WPSR = crate::Reg<pmc_wpsr::PMC_WPSR_SPEC>;
#[doc = "Write Protect Status Register"]
pub mod pmc_wpsr;
#[doc = "PMC_PCER1 (w) register accessor: an alias for `Reg<PMC_PCER1_SPEC>`"]
pub type PMC_PCER1 = crate::Reg<pmc_pcer1::PMC_PCER1_SPEC>;
#[doc = "Peripheral Clock Enable Register 1"]
pub mod pmc_pcer1;
#[doc = "PMC_PCDR1 (w) register accessor: an alias for `Reg<PMC_PCDR1_SPEC>`"]
pub type PMC_PCDR1 = crate::Reg<pmc_pcdr1::PMC_PCDR1_SPEC>;
#[doc = "Peripheral Clock Disable Register 1"]
pub mod pmc_pcdr1;
#[doc = "PMC_PCSR1 (r) register accessor: an alias for `Reg<PMC_PCSR1_SPEC>`"]
pub type PMC_PCSR1 = crate::Reg<pmc_pcsr1::PMC_PCSR1_SPEC>;
#[doc = "Peripheral Clock Status Register 1"]
pub mod pmc_pcsr1;
#[doc = "PMC_OCR (rw) register accessor: an alias for `Reg<PMC_OCR_SPEC>`"]
pub type PMC_OCR = crate::Reg<pmc_ocr::PMC_OCR_SPEC>;
#[doc = "Oscillator Calibration Register"]
pub mod pmc_ocr;
