#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMAC Global Configuration Register"]
    pub gcfg: crate::Reg<gcfg::GCFG_SPEC>,
    #[doc = "0x04 - DMAC Enable Register"]
    pub en: crate::Reg<en::EN_SPEC>,
    #[doc = "0x08 - DMAC Software Single Request Register"]
    pub sreq: crate::Reg<sreq::SREQ_SPEC>,
    #[doc = "0x0c - DMAC Software Chunk Transfer Request Register"]
    pub creq: crate::Reg<creq::CREQ_SPEC>,
    #[doc = "0x10 - DMAC Software Last Transfer Flag Register"]
    pub last: crate::Reg<last::LAST_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Enable register."]
    pub ebcier: crate::Reg<ebcier::EBCIER_SPEC>,
    #[doc = "0x1c - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Disable register."]
    pub ebcidr: crate::Reg<ebcidr::EBCIDR_SPEC>,
    #[doc = "0x20 - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Mask Register."]
    pub ebcimr: crate::Reg<ebcimr::EBCIMR_SPEC>,
    #[doc = "0x24 - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Status Register."]
    pub ebcisr: crate::Reg<ebcisr::EBCISR_SPEC>,
    #[doc = "0x28 - DMAC Channel Handler Enable Register"]
    pub cher: crate::Reg<cher::CHER_SPEC>,
    #[doc = "0x2c - DMAC Channel Handler Disable Register"]
    pub chdr: crate::Reg<chdr::CHDR_SPEC>,
    #[doc = "0x30 - DMAC Channel Handler Status Register"]
    pub chsr: crate::Reg<chsr::CHSR_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x3c - DMAC Channel Source Address Register (ch_num = 0)"]
    pub saddr0: crate::Reg<saddr0::SADDR0_SPEC>,
    #[doc = "0x40 - DMAC Channel Destination Address Register (ch_num = 0)"]
    pub daddr0: crate::Reg<daddr0::DADDR0_SPEC>,
    #[doc = "0x44 - DMAC Channel Descriptor Address Register (ch_num = 0)"]
    pub dscr0: crate::Reg<dscr0::DSCR0_SPEC>,
    #[doc = "0x48 - DMAC Channel Control A Register (ch_num = 0)"]
    pub ctrla0: crate::Reg<ctrla0::CTRLA0_SPEC>,
    #[doc = "0x4c - DMAC Channel Control B Register (ch_num = 0)"]
    pub ctrlb0: crate::Reg<ctrlb0::CTRLB0_SPEC>,
    #[doc = "0x50 - DMAC Channel Configuration Register (ch_num = 0)"]
    pub cfg0: crate::Reg<cfg0::CFG0_SPEC>,
    _reserved18: [u8; 0x10],
    #[doc = "0x64 - DMAC Channel Source Address Register (ch_num = 1)"]
    pub saddr1: crate::Reg<saddr1::SADDR1_SPEC>,
    #[doc = "0x68 - DMAC Channel Destination Address Register (ch_num = 1)"]
    pub daddr1: crate::Reg<daddr1::DADDR1_SPEC>,
    #[doc = "0x6c - DMAC Channel Descriptor Address Register (ch_num = 1)"]
    pub dscr1: crate::Reg<dscr1::DSCR1_SPEC>,
    #[doc = "0x70 - DMAC Channel Control A Register (ch_num = 1)"]
    pub ctrla1: crate::Reg<ctrla1::CTRLA1_SPEC>,
    #[doc = "0x74 - DMAC Channel Control B Register (ch_num = 1)"]
    pub ctrlb1: crate::Reg<ctrlb1::CTRLB1_SPEC>,
    #[doc = "0x78 - DMAC Channel Configuration Register (ch_num = 1)"]
    pub cfg1: crate::Reg<cfg1::CFG1_SPEC>,
    _reserved24: [u8; 0x10],
    #[doc = "0x8c - DMAC Channel Source Address Register (ch_num = 2)"]
    pub saddr2: crate::Reg<saddr2::SADDR2_SPEC>,
    #[doc = "0x90 - DMAC Channel Destination Address Register (ch_num = 2)"]
    pub daddr2: crate::Reg<daddr2::DADDR2_SPEC>,
    #[doc = "0x94 - DMAC Channel Descriptor Address Register (ch_num = 2)"]
    pub dscr2: crate::Reg<dscr2::DSCR2_SPEC>,
    #[doc = "0x98 - DMAC Channel Control A Register (ch_num = 2)"]
    pub ctrla2: crate::Reg<ctrla2::CTRLA2_SPEC>,
    #[doc = "0x9c - DMAC Channel Control B Register (ch_num = 2)"]
    pub ctrlb2: crate::Reg<ctrlb2::CTRLB2_SPEC>,
    #[doc = "0xa0 - DMAC Channel Configuration Register (ch_num = 2)"]
    pub cfg2: crate::Reg<cfg2::CFG2_SPEC>,
    _reserved30: [u8; 0x10],
    #[doc = "0xb4 - DMAC Channel Source Address Register (ch_num = 3)"]
    pub saddr3: crate::Reg<saddr3::SADDR3_SPEC>,
    #[doc = "0xb8 - DMAC Channel Destination Address Register (ch_num = 3)"]
    pub daddr3: crate::Reg<daddr3::DADDR3_SPEC>,
    #[doc = "0xbc - DMAC Channel Descriptor Address Register (ch_num = 3)"]
    pub dscr3: crate::Reg<dscr3::DSCR3_SPEC>,
    #[doc = "0xc0 - DMAC Channel Control A Register (ch_num = 3)"]
    pub ctrla3: crate::Reg<ctrla3::CTRLA3_SPEC>,
    #[doc = "0xc4 - DMAC Channel Control B Register (ch_num = 3)"]
    pub ctrlb3: crate::Reg<ctrlb3::CTRLB3_SPEC>,
    #[doc = "0xc8 - DMAC Channel Configuration Register (ch_num = 3)"]
    pub cfg3: crate::Reg<cfg3::CFG3_SPEC>,
    _reserved36: [u8; 0x0118],
    #[doc = "0x1e4 - DMAC Write Protect Mode Register"]
    pub wpmr: crate::Reg<wpmr::WPMR_SPEC>,
    #[doc = "0x1e8 - DMAC Write Protect Status Register"]
    pub wpsr: crate::Reg<wpsr::WPSR_SPEC>,
}
#[doc = "GCFG register accessor: an alias for `Reg<GCFG_SPEC>`"]
pub type GCFG = crate::Reg<gcfg::GCFG_SPEC>;
#[doc = "DMAC Global Configuration Register"]
pub mod gcfg;
#[doc = "EN register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "DMAC Enable Register"]
pub mod en;
#[doc = "SREQ register accessor: an alias for `Reg<SREQ_SPEC>`"]
pub type SREQ = crate::Reg<sreq::SREQ_SPEC>;
#[doc = "DMAC Software Single Request Register"]
pub mod sreq;
#[doc = "CREQ register accessor: an alias for `Reg<CREQ_SPEC>`"]
pub type CREQ = crate::Reg<creq::CREQ_SPEC>;
#[doc = "DMAC Software Chunk Transfer Request Register"]
pub mod creq;
#[doc = "LAST register accessor: an alias for `Reg<LAST_SPEC>`"]
pub type LAST = crate::Reg<last::LAST_SPEC>;
#[doc = "DMAC Software Last Transfer Flag Register"]
pub mod last;
#[doc = "EBCIER register accessor: an alias for `Reg<EBCIER_SPEC>`"]
pub type EBCIER = crate::Reg<ebcier::EBCIER_SPEC>;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Enable register."]
pub mod ebcier;
#[doc = "EBCIDR register accessor: an alias for `Reg<EBCIDR_SPEC>`"]
pub type EBCIDR = crate::Reg<ebcidr::EBCIDR_SPEC>;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Disable register."]
pub mod ebcidr;
#[doc = "EBCIMR register accessor: an alias for `Reg<EBCIMR_SPEC>`"]
pub type EBCIMR = crate::Reg<ebcimr::EBCIMR_SPEC>;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Mask Register."]
pub mod ebcimr;
#[doc = "EBCISR register accessor: an alias for `Reg<EBCISR_SPEC>`"]
pub type EBCISR = crate::Reg<ebcisr::EBCISR_SPEC>;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Status Register."]
pub mod ebcisr;
#[doc = "CHER register accessor: an alias for `Reg<CHER_SPEC>`"]
pub type CHER = crate::Reg<cher::CHER_SPEC>;
#[doc = "DMAC Channel Handler Enable Register"]
pub mod cher;
#[doc = "CHDR register accessor: an alias for `Reg<CHDR_SPEC>`"]
pub type CHDR = crate::Reg<chdr::CHDR_SPEC>;
#[doc = "DMAC Channel Handler Disable Register"]
pub mod chdr;
#[doc = "CHSR register accessor: an alias for `Reg<CHSR_SPEC>`"]
pub type CHSR = crate::Reg<chsr::CHSR_SPEC>;
#[doc = "DMAC Channel Handler Status Register"]
pub mod chsr;
#[doc = "SADDR0 register accessor: an alias for `Reg<SADDR0_SPEC>`"]
pub type SADDR0 = crate::Reg<saddr0::SADDR0_SPEC>;
#[doc = "DMAC Channel Source Address Register (ch_num = 0)"]
pub mod saddr0;
#[doc = "DADDR0 register accessor: an alias for `Reg<DADDR0_SPEC>`"]
pub type DADDR0 = crate::Reg<daddr0::DADDR0_SPEC>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 0)"]
pub mod daddr0;
#[doc = "DSCR0 register accessor: an alias for `Reg<DSCR0_SPEC>`"]
pub type DSCR0 = crate::Reg<dscr0::DSCR0_SPEC>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 0)"]
pub mod dscr0;
#[doc = "CTRLA0 register accessor: an alias for `Reg<CTRLA0_SPEC>`"]
pub type CTRLA0 = crate::Reg<ctrla0::CTRLA0_SPEC>;
#[doc = "DMAC Channel Control A Register (ch_num = 0)"]
pub mod ctrla0;
#[doc = "CTRLB0 register accessor: an alias for `Reg<CTRLB0_SPEC>`"]
pub type CTRLB0 = crate::Reg<ctrlb0::CTRLB0_SPEC>;
#[doc = "DMAC Channel Control B Register (ch_num = 0)"]
pub mod ctrlb0;
#[doc = "CFG0 register accessor: an alias for `Reg<CFG0_SPEC>`"]
pub type CFG0 = crate::Reg<cfg0::CFG0_SPEC>;
#[doc = "DMAC Channel Configuration Register (ch_num = 0)"]
pub mod cfg0;
#[doc = "SADDR1 register accessor: an alias for `Reg<SADDR1_SPEC>`"]
pub type SADDR1 = crate::Reg<saddr1::SADDR1_SPEC>;
#[doc = "DMAC Channel Source Address Register (ch_num = 1)"]
pub mod saddr1;
#[doc = "DADDR1 register accessor: an alias for `Reg<DADDR1_SPEC>`"]
pub type DADDR1 = crate::Reg<daddr1::DADDR1_SPEC>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 1)"]
pub mod daddr1;
#[doc = "DSCR1 register accessor: an alias for `Reg<DSCR1_SPEC>`"]
pub type DSCR1 = crate::Reg<dscr1::DSCR1_SPEC>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 1)"]
pub mod dscr1;
#[doc = "CTRLA1 register accessor: an alias for `Reg<CTRLA1_SPEC>`"]
pub type CTRLA1 = crate::Reg<ctrla1::CTRLA1_SPEC>;
#[doc = "DMAC Channel Control A Register (ch_num = 1)"]
pub mod ctrla1;
#[doc = "CTRLB1 register accessor: an alias for `Reg<CTRLB1_SPEC>`"]
pub type CTRLB1 = crate::Reg<ctrlb1::CTRLB1_SPEC>;
#[doc = "DMAC Channel Control B Register (ch_num = 1)"]
pub mod ctrlb1;
#[doc = "CFG1 register accessor: an alias for `Reg<CFG1_SPEC>`"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "DMAC Channel Configuration Register (ch_num = 1)"]
pub mod cfg1;
#[doc = "SADDR2 register accessor: an alias for `Reg<SADDR2_SPEC>`"]
pub type SADDR2 = crate::Reg<saddr2::SADDR2_SPEC>;
#[doc = "DMAC Channel Source Address Register (ch_num = 2)"]
pub mod saddr2;
#[doc = "DADDR2 register accessor: an alias for `Reg<DADDR2_SPEC>`"]
pub type DADDR2 = crate::Reg<daddr2::DADDR2_SPEC>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 2)"]
pub mod daddr2;
#[doc = "DSCR2 register accessor: an alias for `Reg<DSCR2_SPEC>`"]
pub type DSCR2 = crate::Reg<dscr2::DSCR2_SPEC>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 2)"]
pub mod dscr2;
#[doc = "CTRLA2 register accessor: an alias for `Reg<CTRLA2_SPEC>`"]
pub type CTRLA2 = crate::Reg<ctrla2::CTRLA2_SPEC>;
#[doc = "DMAC Channel Control A Register (ch_num = 2)"]
pub mod ctrla2;
#[doc = "CTRLB2 register accessor: an alias for `Reg<CTRLB2_SPEC>`"]
pub type CTRLB2 = crate::Reg<ctrlb2::CTRLB2_SPEC>;
#[doc = "DMAC Channel Control B Register (ch_num = 2)"]
pub mod ctrlb2;
#[doc = "CFG2 register accessor: an alias for `Reg<CFG2_SPEC>`"]
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
#[doc = "DMAC Channel Configuration Register (ch_num = 2)"]
pub mod cfg2;
#[doc = "SADDR3 register accessor: an alias for `Reg<SADDR3_SPEC>`"]
pub type SADDR3 = crate::Reg<saddr3::SADDR3_SPEC>;
#[doc = "DMAC Channel Source Address Register (ch_num = 3)"]
pub mod saddr3;
#[doc = "DADDR3 register accessor: an alias for `Reg<DADDR3_SPEC>`"]
pub type DADDR3 = crate::Reg<daddr3::DADDR3_SPEC>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 3)"]
pub mod daddr3;
#[doc = "DSCR3 register accessor: an alias for `Reg<DSCR3_SPEC>`"]
pub type DSCR3 = crate::Reg<dscr3::DSCR3_SPEC>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 3)"]
pub mod dscr3;
#[doc = "CTRLA3 register accessor: an alias for `Reg<CTRLA3_SPEC>`"]
pub type CTRLA3 = crate::Reg<ctrla3::CTRLA3_SPEC>;
#[doc = "DMAC Channel Control A Register (ch_num = 3)"]
pub mod ctrla3;
#[doc = "CTRLB3 register accessor: an alias for `Reg<CTRLB3_SPEC>`"]
pub type CTRLB3 = crate::Reg<ctrlb3::CTRLB3_SPEC>;
#[doc = "DMAC Channel Control B Register (ch_num = 3)"]
pub mod ctrlb3;
#[doc = "CFG3 register accessor: an alias for `Reg<CFG3_SPEC>`"]
pub type CFG3 = crate::Reg<cfg3::CFG3_SPEC>;
#[doc = "DMAC Channel Configuration Register (ch_num = 3)"]
pub mod cfg3;
#[doc = "WPMR register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "DMAC Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "DMAC Write Protect Status Register"]
pub mod wpsr;
