#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel Control Register (channel = 0)"]
    pub ccr0: CCR0,
    _reserved_1_cmr0: [u8; 0x04],
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Counter Value (channel = 0)"]
    pub cv0: CV0,
    #[doc = "0x14 - Register A (channel = 0)"]
    pub ra0: RA0,
    #[doc = "0x18 - Register B (channel = 0)"]
    pub rb0: RB0,
    #[doc = "0x1c - Register C (channel = 0)"]
    pub rc0: RC0,
    #[doc = "0x20 - Status Register (channel = 0)"]
    pub sr0: SR0,
    #[doc = "0x24 - Interrupt Enable Register (channel = 0)"]
    pub ier0: IER0,
    #[doc = "0x28 - Interrupt Disable Register (channel = 0)"]
    pub idr0: IDR0,
    #[doc = "0x2c - Interrupt Mask Register (channel = 0)"]
    pub imr0: IMR0,
    _reserved10: [u8; 0x10],
    #[doc = "0x40 - Channel Control Register (channel = 1)"]
    pub ccr1: CCR1,
    _reserved_11_cmr1: [u8; 0x04],
    _reserved12: [u8; 0x08],
    #[doc = "0x50 - Counter Value (channel = 1)"]
    pub cv1: CV1,
    #[doc = "0x54 - Register A (channel = 1)"]
    pub ra1: RA1,
    #[doc = "0x58 - Register B (channel = 1)"]
    pub rb1: RB1,
    #[doc = "0x5c - Register C (channel = 1)"]
    pub rc1: RC1,
    #[doc = "0x60 - Status Register (channel = 1)"]
    pub sr1: SR1,
    #[doc = "0x64 - Interrupt Enable Register (channel = 1)"]
    pub ier1: IER1,
    #[doc = "0x68 - Interrupt Disable Register (channel = 1)"]
    pub idr1: IDR1,
    #[doc = "0x6c - Interrupt Mask Register (channel = 1)"]
    pub imr1: IMR1,
    _reserved20: [u8; 0x10],
    #[doc = "0x80 - Channel Control Register (channel = 2)"]
    pub ccr2: CCR2,
    _reserved_21_cmr2: [u8; 0x04],
    _reserved22: [u8; 0x08],
    #[doc = "0x90 - Counter Value (channel = 2)"]
    pub cv2: CV2,
    #[doc = "0x94 - Register A (channel = 2)"]
    pub ra2: RA2,
    #[doc = "0x98 - Register B (channel = 2)"]
    pub rb2: RB2,
    #[doc = "0x9c - Register C (channel = 2)"]
    pub rc2: RC2,
    #[doc = "0xa0 - Status Register (channel = 2)"]
    pub sr2: SR2,
    #[doc = "0xa4 - Interrupt Enable Register (channel = 2)"]
    pub ier2: IER2,
    #[doc = "0xa8 - Interrupt Disable Register (channel = 2)"]
    pub idr2: IDR2,
    #[doc = "0xac - Interrupt Mask Register (channel = 2)"]
    pub imr2: IMR2,
    _reserved30: [u8; 0x10],
    #[doc = "0xc0 - Block Control Register"]
    pub bcr: BCR,
    #[doc = "0xc4 - Block Mode Register"]
    pub bmr: BMR,
    #[doc = "0xc8 - QDEC Interrupt Enable Register"]
    pub qier: QIER,
    #[doc = "0xcc - QDEC Interrupt Disable Register"]
    pub qidr: QIDR,
    #[doc = "0xd0 - QDEC Interrupt Mask Register"]
    pub qimr: QIMR,
    #[doc = "0xd4 - QDEC Interrupt Status Register"]
    pub qisr: QISR,
}
impl RegisterBlock {
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub fn waveform_mode_cmr0_waveform_mode(&self) -> &WAVEFORM_MODE_CMR0_WAVEFORM_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const WAVEFORM_MODE_CMR0_WAVEFORM_MODE)
        }
    }
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub fn cmr0(&self) -> &CMR0 {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const CMR0) }
    }
    #[doc = "0x44 - Channel Mode Register (channel = 1)"]
    #[inline(always)]
    pub fn waveform_mode_cmr1_waveform_mode(&self) -> &WAVEFORM_MODE_CMR1_WAVEFORM_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize)
                as *const WAVEFORM_MODE_CMR1_WAVEFORM_MODE)
        }
    }
    #[doc = "0x44 - Channel Mode Register (channel = 1)"]
    #[inline(always)]
    pub fn cmr1(&self) -> &CMR1 {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const CMR1) }
    }
    #[doc = "0x84 - Channel Mode Register (channel = 2)"]
    #[inline(always)]
    pub fn waveform_mode_cmr2_waveform_mode(&self) -> &WAVEFORM_MODE_CMR2_WAVEFORM_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(132usize)
                as *const WAVEFORM_MODE_CMR2_WAVEFORM_MODE)
        }
    }
    #[doc = "0x84 - Channel Mode Register (channel = 2)"]
    #[inline(always)]
    pub fn cmr2(&self) -> &CMR2 {
        unsafe { &*(((self as *const Self) as *const u8).add(132usize) as *const CMR2) }
    }
}
#[doc = "CCR0 (w) register accessor: an alias for `Reg<CCR0_SPEC>`"]
pub type CCR0 = crate::Reg<ccr0::CCR0_SPEC>;
#[doc = "Channel Control Register (channel = 0)"]
pub mod ccr0;
#[doc = "CMR0 (rw) register accessor: an alias for `Reg<CMR0_SPEC>`"]
pub type CMR0 = crate::Reg<cmr0::CMR0_SPEC>;
#[doc = "Channel Mode Register (channel = 0)"]
pub mod cmr0;
#[doc = "WAVEFORM_MODE_CMR0_WAVEFORM_MODE (rw) register accessor: an alias for `Reg<WAVEFORM_MODE_CMR0_WAVEFORM_MODE_SPEC>`"]
pub type WAVEFORM_MODE_CMR0_WAVEFORM_MODE =
    crate::Reg<waveform_mode_cmr0_waveform_mode::WAVEFORM_MODE_CMR0_WAVEFORM_MODE_SPEC>;
#[doc = "Channel Mode Register (channel = 0)"]
pub mod waveform_mode_cmr0_waveform_mode;
#[doc = "CV0 (r) register accessor: an alias for `Reg<CV0_SPEC>`"]
pub type CV0 = crate::Reg<cv0::CV0_SPEC>;
#[doc = "Counter Value (channel = 0)"]
pub mod cv0;
#[doc = "RA0 (rw) register accessor: an alias for `Reg<RA0_SPEC>`"]
pub type RA0 = crate::Reg<ra0::RA0_SPEC>;
#[doc = "Register A (channel = 0)"]
pub mod ra0;
#[doc = "RB0 (rw) register accessor: an alias for `Reg<RB0_SPEC>`"]
pub type RB0 = crate::Reg<rb0::RB0_SPEC>;
#[doc = "Register B (channel = 0)"]
pub mod rb0;
#[doc = "RC0 (rw) register accessor: an alias for `Reg<RC0_SPEC>`"]
pub type RC0 = crate::Reg<rc0::RC0_SPEC>;
#[doc = "Register C (channel = 0)"]
pub mod rc0;
#[doc = "SR0 (r) register accessor: an alias for `Reg<SR0_SPEC>`"]
pub type SR0 = crate::Reg<sr0::SR0_SPEC>;
#[doc = "Status Register (channel = 0)"]
pub mod sr0;
#[doc = "IER0 (w) register accessor: an alias for `Reg<IER0_SPEC>`"]
pub type IER0 = crate::Reg<ier0::IER0_SPEC>;
#[doc = "Interrupt Enable Register (channel = 0)"]
pub mod ier0;
#[doc = "IDR0 (w) register accessor: an alias for `Reg<IDR0_SPEC>`"]
pub type IDR0 = crate::Reg<idr0::IDR0_SPEC>;
#[doc = "Interrupt Disable Register (channel = 0)"]
pub mod idr0;
#[doc = "IMR0 (r) register accessor: an alias for `Reg<IMR0_SPEC>`"]
pub type IMR0 = crate::Reg<imr0::IMR0_SPEC>;
#[doc = "Interrupt Mask Register (channel = 0)"]
pub mod imr0;
#[doc = "CCR1 (w) register accessor: an alias for `Reg<CCR1_SPEC>`"]
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
#[doc = "Channel Control Register (channel = 1)"]
pub mod ccr1;
#[doc = "CMR1 (rw) register accessor: an alias for `Reg<CMR1_SPEC>`"]
pub type CMR1 = crate::Reg<cmr1::CMR1_SPEC>;
#[doc = "Channel Mode Register (channel = 1)"]
pub mod cmr1;
#[doc = "WAVEFORM_MODE_CMR1_WAVEFORM_MODE (rw) register accessor: an alias for `Reg<WAVEFORM_MODE_CMR1_WAVEFORM_MODE_SPEC>`"]
pub type WAVEFORM_MODE_CMR1_WAVEFORM_MODE =
    crate::Reg<waveform_mode_cmr1_waveform_mode::WAVEFORM_MODE_CMR1_WAVEFORM_MODE_SPEC>;
#[doc = "Channel Mode Register (channel = 1)"]
pub mod waveform_mode_cmr1_waveform_mode;
#[doc = "CV1 (r) register accessor: an alias for `Reg<CV1_SPEC>`"]
pub type CV1 = crate::Reg<cv1::CV1_SPEC>;
#[doc = "Counter Value (channel = 1)"]
pub mod cv1;
#[doc = "RA1 (rw) register accessor: an alias for `Reg<RA1_SPEC>`"]
pub type RA1 = crate::Reg<ra1::RA1_SPEC>;
#[doc = "Register A (channel = 1)"]
pub mod ra1;
#[doc = "RB1 (rw) register accessor: an alias for `Reg<RB1_SPEC>`"]
pub type RB1 = crate::Reg<rb1::RB1_SPEC>;
#[doc = "Register B (channel = 1)"]
pub mod rb1;
#[doc = "RC1 (rw) register accessor: an alias for `Reg<RC1_SPEC>`"]
pub type RC1 = crate::Reg<rc1::RC1_SPEC>;
#[doc = "Register C (channel = 1)"]
pub mod rc1;
#[doc = "SR1 (r) register accessor: an alias for `Reg<SR1_SPEC>`"]
pub type SR1 = crate::Reg<sr1::SR1_SPEC>;
#[doc = "Status Register (channel = 1)"]
pub mod sr1;
#[doc = "IER1 (w) register accessor: an alias for `Reg<IER1_SPEC>`"]
pub type IER1 = crate::Reg<ier1::IER1_SPEC>;
#[doc = "Interrupt Enable Register (channel = 1)"]
pub mod ier1;
#[doc = "IDR1 (w) register accessor: an alias for `Reg<IDR1_SPEC>`"]
pub type IDR1 = crate::Reg<idr1::IDR1_SPEC>;
#[doc = "Interrupt Disable Register (channel = 1)"]
pub mod idr1;
#[doc = "IMR1 (r) register accessor: an alias for `Reg<IMR1_SPEC>`"]
pub type IMR1 = crate::Reg<imr1::IMR1_SPEC>;
#[doc = "Interrupt Mask Register (channel = 1)"]
pub mod imr1;
#[doc = "CCR2 (w) register accessor: an alias for `Reg<CCR2_SPEC>`"]
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
#[doc = "Channel Control Register (channel = 2)"]
pub mod ccr2;
#[doc = "CMR2 (rw) register accessor: an alias for `Reg<CMR2_SPEC>`"]
pub type CMR2 = crate::Reg<cmr2::CMR2_SPEC>;
#[doc = "Channel Mode Register (channel = 2)"]
pub mod cmr2;
#[doc = "WAVEFORM_MODE_CMR2_WAVEFORM_MODE (rw) register accessor: an alias for `Reg<WAVEFORM_MODE_CMR2_WAVEFORM_MODE_SPEC>`"]
pub type WAVEFORM_MODE_CMR2_WAVEFORM_MODE =
    crate::Reg<waveform_mode_cmr2_waveform_mode::WAVEFORM_MODE_CMR2_WAVEFORM_MODE_SPEC>;
#[doc = "Channel Mode Register (channel = 2)"]
pub mod waveform_mode_cmr2_waveform_mode;
#[doc = "CV2 (r) register accessor: an alias for `Reg<CV2_SPEC>`"]
pub type CV2 = crate::Reg<cv2::CV2_SPEC>;
#[doc = "Counter Value (channel = 2)"]
pub mod cv2;
#[doc = "RA2 (rw) register accessor: an alias for `Reg<RA2_SPEC>`"]
pub type RA2 = crate::Reg<ra2::RA2_SPEC>;
#[doc = "Register A (channel = 2)"]
pub mod ra2;
#[doc = "RB2 (rw) register accessor: an alias for `Reg<RB2_SPEC>`"]
pub type RB2 = crate::Reg<rb2::RB2_SPEC>;
#[doc = "Register B (channel = 2)"]
pub mod rb2;
#[doc = "RC2 (rw) register accessor: an alias for `Reg<RC2_SPEC>`"]
pub type RC2 = crate::Reg<rc2::RC2_SPEC>;
#[doc = "Register C (channel = 2)"]
pub mod rc2;
#[doc = "SR2 (r) register accessor: an alias for `Reg<SR2_SPEC>`"]
pub type SR2 = crate::Reg<sr2::SR2_SPEC>;
#[doc = "Status Register (channel = 2)"]
pub mod sr2;
#[doc = "IER2 (w) register accessor: an alias for `Reg<IER2_SPEC>`"]
pub type IER2 = crate::Reg<ier2::IER2_SPEC>;
#[doc = "Interrupt Enable Register (channel = 2)"]
pub mod ier2;
#[doc = "IDR2 (w) register accessor: an alias for `Reg<IDR2_SPEC>`"]
pub type IDR2 = crate::Reg<idr2::IDR2_SPEC>;
#[doc = "Interrupt Disable Register (channel = 2)"]
pub mod idr2;
#[doc = "IMR2 (r) register accessor: an alias for `Reg<IMR2_SPEC>`"]
pub type IMR2 = crate::Reg<imr2::IMR2_SPEC>;
#[doc = "Interrupt Mask Register (channel = 2)"]
pub mod imr2;
#[doc = "BCR (w) register accessor: an alias for `Reg<BCR_SPEC>`"]
pub type BCR = crate::Reg<bcr::BCR_SPEC>;
#[doc = "Block Control Register"]
pub mod bcr;
#[doc = "BMR (rw) register accessor: an alias for `Reg<BMR_SPEC>`"]
pub type BMR = crate::Reg<bmr::BMR_SPEC>;
#[doc = "Block Mode Register"]
pub mod bmr;
#[doc = "QIER (w) register accessor: an alias for `Reg<QIER_SPEC>`"]
pub type QIER = crate::Reg<qier::QIER_SPEC>;
#[doc = "QDEC Interrupt Enable Register"]
pub mod qier;
#[doc = "QIDR (w) register accessor: an alias for `Reg<QIDR_SPEC>`"]
pub type QIDR = crate::Reg<qidr::QIDR_SPEC>;
#[doc = "QDEC Interrupt Disable Register"]
pub mod qidr;
#[doc = "QIMR (r) register accessor: an alias for `Reg<QIMR_SPEC>`"]
pub type QIMR = crate::Reg<qimr::QIMR_SPEC>;
#[doc = "QDEC Interrupt Mask Register"]
pub mod qimr;
#[doc = "QISR (r) register accessor: an alias for `Reg<QISR_SPEC>`"]
pub type QISR = crate::Reg<qisr::QISR_SPEC>;
#[doc = "QDEC Interrupt Status Register"]
pub mod qisr;
