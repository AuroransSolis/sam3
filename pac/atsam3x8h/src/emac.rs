#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Network Control Register"]
    pub ncr: crate::Reg<ncr::NCR_SPEC>,
    #[doc = "0x04 - Network Configuration Register"]
    pub ncfgr: crate::Reg<ncfgr::NCFGR_SPEC>,
    #[doc = "0x08 - Network Status Register"]
    pub nsr: crate::Reg<nsr::NSR_SPEC>,
    _reserved3: [u8; 0x08],
    #[doc = "0x14 - Transmit Status Register"]
    pub tsr: crate::Reg<tsr::TSR_SPEC>,
    #[doc = "0x18 - Receive Buffer Queue Pointer Register"]
    pub rbqp: crate::Reg<rbqp::RBQP_SPEC>,
    #[doc = "0x1c - Transmit Buffer Queue Pointer Register"]
    pub tbqp: crate::Reg<tbqp::TBQP_SPEC>,
    #[doc = "0x20 - Receive Status Register"]
    pub rsr: crate::Reg<rsr::RSR_SPEC>,
    #[doc = "0x24 - Interrupt Status Register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x28 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x2c - Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x30 - Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x34 - Phy Maintenance Register"]
    pub man: crate::Reg<man::MAN_SPEC>,
    #[doc = "0x38 - Pause Time Register"]
    pub ptr: crate::Reg<ptr::PTR_SPEC>,
    #[doc = "0x3c - Pause Frames Received Register"]
    pub pfr: crate::Reg<pfr::PFR_SPEC>,
    #[doc = "0x40 - Frames Transmitted Ok Register"]
    pub fto: crate::Reg<fto::FTO_SPEC>,
    #[doc = "0x44 - Single Collision Frames Register"]
    pub scf: crate::Reg<scf::SCF_SPEC>,
    #[doc = "0x48 - Multiple Collision Frames Register"]
    pub mcf: crate::Reg<mcf::MCF_SPEC>,
    #[doc = "0x4c - Frames Received Ok Register"]
    pub fro: crate::Reg<fro::FRO_SPEC>,
    #[doc = "0x50 - Frame Check Sequence Errors Register"]
    pub fcse: crate::Reg<fcse::FCSE_SPEC>,
    #[doc = "0x54 - Alignment Errors Register"]
    pub ale: crate::Reg<ale::ALE_SPEC>,
    #[doc = "0x58 - Deferred Transmission Frames Register"]
    pub dtf: crate::Reg<dtf::DTF_SPEC>,
    #[doc = "0x5c - Late Collisions Register"]
    pub lcol: crate::Reg<lcol::LCOL_SPEC>,
    #[doc = "0x60 - Excessive Collisions Register"]
    pub ecol: crate::Reg<ecol::ECOL_SPEC>,
    #[doc = "0x64 - Transmit Underrun Errors Register"]
    pub tund: crate::Reg<tund::TUND_SPEC>,
    #[doc = "0x68 - Carrier Sense Errors Register"]
    pub cse: crate::Reg<cse::CSE_SPEC>,
    #[doc = "0x6c - Receive Resource Errors Register"]
    pub rre: crate::Reg<rre::RRE_SPEC>,
    #[doc = "0x70 - Receive Overrun Errors Register"]
    pub rov: crate::Reg<rov::ROV_SPEC>,
    #[doc = "0x74 - Receive Symbol Errors Register"]
    pub rse: crate::Reg<rse::RSE_SPEC>,
    #[doc = "0x78 - Excessive Length Errors Register"]
    pub ele: crate::Reg<ele::ELE_SPEC>,
    #[doc = "0x7c - Receive Jabbers Register"]
    pub rja: crate::Reg<rja::RJA_SPEC>,
    #[doc = "0x80 - Undersize Frames Register"]
    pub usf: crate::Reg<usf::USF_SPEC>,
    #[doc = "0x84 - SQE Test Errors Register"]
    pub ste: crate::Reg<ste::STE_SPEC>,
    #[doc = "0x88 - Received Length Field Mismatch Register"]
    pub rle: crate::Reg<rle::RLE_SPEC>,
    _reserved33: [u8; 0x04],
    #[doc = "0x90 - Hash Register Bottom \\[31:0\\]
Register"]
    pub hrb: crate::Reg<hrb::HRB_SPEC>,
    #[doc = "0x94 - Hash Register Top \\[63:32\\]
Register"]
    pub hrt: crate::Reg<hrt::HRT_SPEC>,
    #[doc = "0x98 - Specific Address 1 Bottom Register"]
    pub sa1b: crate::Reg<sa1b::SA1B_SPEC>,
    #[doc = "0x9c - Specific Address 1 Top Register"]
    pub sa1t: crate::Reg<sa1t::SA1T_SPEC>,
    #[doc = "0xa0 - Specific Address 2 Bottom Register"]
    pub sa2b: crate::Reg<sa2b::SA2B_SPEC>,
    #[doc = "0xa4 - Specific Address 2 Top Register"]
    pub sa2t: crate::Reg<sa2t::SA2T_SPEC>,
    #[doc = "0xa8 - Specific Address 3 Bottom Register"]
    pub sa3b: crate::Reg<sa3b::SA3B_SPEC>,
    #[doc = "0xac - Specific Address 3 Top Register"]
    pub sa3t: crate::Reg<sa3t::SA3T_SPEC>,
    #[doc = "0xb0 - Specific Address 4 Bottom Register"]
    pub sa4b: crate::Reg<sa4b::SA4B_SPEC>,
    #[doc = "0xb4 - Specific Address 4 Top Register"]
    pub sa4t: crate::Reg<sa4t::SA4T_SPEC>,
    #[doc = "0xb8 - Type ID Checking Register"]
    pub tid: crate::Reg<tid::TID_SPEC>,
    _reserved44: [u8; 0x04],
    #[doc = "0xc0 - User Input/Output Register"]
    pub usrio: crate::Reg<usrio::USRIO_SPEC>,
}
#[doc = "NCR register accessor: an alias for `Reg<NCR_SPEC>`"]
pub type NCR = crate::Reg<ncr::NCR_SPEC>;
#[doc = "Network Control Register"]
pub mod ncr;
#[doc = "NCFGR register accessor: an alias for `Reg<NCFGR_SPEC>`"]
pub type NCFGR = crate::Reg<ncfgr::NCFGR_SPEC>;
#[doc = "Network Configuration Register"]
pub mod ncfgr;
#[doc = "NSR register accessor: an alias for `Reg<NSR_SPEC>`"]
pub type NSR = crate::Reg<nsr::NSR_SPEC>;
#[doc = "Network Status Register"]
pub mod nsr;
#[doc = "TSR register accessor: an alias for `Reg<TSR_SPEC>`"]
pub type TSR = crate::Reg<tsr::TSR_SPEC>;
#[doc = "Transmit Status Register"]
pub mod tsr;
#[doc = "RBQP register accessor: an alias for `Reg<RBQP_SPEC>`"]
pub type RBQP = crate::Reg<rbqp::RBQP_SPEC>;
#[doc = "Receive Buffer Queue Pointer Register"]
pub mod rbqp;
#[doc = "TBQP register accessor: an alias for `Reg<TBQP_SPEC>`"]
pub type TBQP = crate::Reg<tbqp::TBQP_SPEC>;
#[doc = "Transmit Buffer Queue Pointer Register"]
pub mod tbqp;
#[doc = "RSR register accessor: an alias for `Reg<RSR_SPEC>`"]
pub type RSR = crate::Reg<rsr::RSR_SPEC>;
#[doc = "Receive Status Register"]
pub mod rsr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "MAN register accessor: an alias for `Reg<MAN_SPEC>`"]
pub type MAN = crate::Reg<man::MAN_SPEC>;
#[doc = "Phy Maintenance Register"]
pub mod man;
#[doc = "PTR register accessor: an alias for `Reg<PTR_SPEC>`"]
pub type PTR = crate::Reg<ptr::PTR_SPEC>;
#[doc = "Pause Time Register"]
pub mod ptr;
#[doc = "PFR register accessor: an alias for `Reg<PFR_SPEC>`"]
pub type PFR = crate::Reg<pfr::PFR_SPEC>;
#[doc = "Pause Frames Received Register"]
pub mod pfr;
#[doc = "FTO register accessor: an alias for `Reg<FTO_SPEC>`"]
pub type FTO = crate::Reg<fto::FTO_SPEC>;
#[doc = "Frames Transmitted Ok Register"]
pub mod fto;
#[doc = "SCF register accessor: an alias for `Reg<SCF_SPEC>`"]
pub type SCF = crate::Reg<scf::SCF_SPEC>;
#[doc = "Single Collision Frames Register"]
pub mod scf;
#[doc = "MCF register accessor: an alias for `Reg<MCF_SPEC>`"]
pub type MCF = crate::Reg<mcf::MCF_SPEC>;
#[doc = "Multiple Collision Frames Register"]
pub mod mcf;
#[doc = "FRO register accessor: an alias for `Reg<FRO_SPEC>`"]
pub type FRO = crate::Reg<fro::FRO_SPEC>;
#[doc = "Frames Received Ok Register"]
pub mod fro;
#[doc = "FCSE register accessor: an alias for `Reg<FCSE_SPEC>`"]
pub type FCSE = crate::Reg<fcse::FCSE_SPEC>;
#[doc = "Frame Check Sequence Errors Register"]
pub mod fcse;
#[doc = "ALE register accessor: an alias for `Reg<ALE_SPEC>`"]
pub type ALE = crate::Reg<ale::ALE_SPEC>;
#[doc = "Alignment Errors Register"]
pub mod ale;
#[doc = "DTF register accessor: an alias for `Reg<DTF_SPEC>`"]
pub type DTF = crate::Reg<dtf::DTF_SPEC>;
#[doc = "Deferred Transmission Frames Register"]
pub mod dtf;
#[doc = "LCOL register accessor: an alias for `Reg<LCOL_SPEC>`"]
pub type LCOL = crate::Reg<lcol::LCOL_SPEC>;
#[doc = "Late Collisions Register"]
pub mod lcol;
#[doc = "ECOL register accessor: an alias for `Reg<ECOL_SPEC>`"]
pub type ECOL = crate::Reg<ecol::ECOL_SPEC>;
#[doc = "Excessive Collisions Register"]
pub mod ecol;
#[doc = "TUND register accessor: an alias for `Reg<TUND_SPEC>`"]
pub type TUND = crate::Reg<tund::TUND_SPEC>;
#[doc = "Transmit Underrun Errors Register"]
pub mod tund;
#[doc = "CSE register accessor: an alias for `Reg<CSE_SPEC>`"]
pub type CSE = crate::Reg<cse::CSE_SPEC>;
#[doc = "Carrier Sense Errors Register"]
pub mod cse;
#[doc = "RRE register accessor: an alias for `Reg<RRE_SPEC>`"]
pub type RRE = crate::Reg<rre::RRE_SPEC>;
#[doc = "Receive Resource Errors Register"]
pub mod rre;
#[doc = "ROV register accessor: an alias for `Reg<ROV_SPEC>`"]
pub type ROV = crate::Reg<rov::ROV_SPEC>;
#[doc = "Receive Overrun Errors Register"]
pub mod rov;
#[doc = "RSE register accessor: an alias for `Reg<RSE_SPEC>`"]
pub type RSE = crate::Reg<rse::RSE_SPEC>;
#[doc = "Receive Symbol Errors Register"]
pub mod rse;
#[doc = "ELE register accessor: an alias for `Reg<ELE_SPEC>`"]
pub type ELE = crate::Reg<ele::ELE_SPEC>;
#[doc = "Excessive Length Errors Register"]
pub mod ele;
#[doc = "RJA register accessor: an alias for `Reg<RJA_SPEC>`"]
pub type RJA = crate::Reg<rja::RJA_SPEC>;
#[doc = "Receive Jabbers Register"]
pub mod rja;
#[doc = "USF register accessor: an alias for `Reg<USF_SPEC>`"]
pub type USF = crate::Reg<usf::USF_SPEC>;
#[doc = "Undersize Frames Register"]
pub mod usf;
#[doc = "STE register accessor: an alias for `Reg<STE_SPEC>`"]
pub type STE = crate::Reg<ste::STE_SPEC>;
#[doc = "SQE Test Errors Register"]
pub mod ste;
#[doc = "RLE register accessor: an alias for `Reg<RLE_SPEC>`"]
pub type RLE = crate::Reg<rle::RLE_SPEC>;
#[doc = "Received Length Field Mismatch Register"]
pub mod rle;
#[doc = "HRB register accessor: an alias for `Reg<HRB_SPEC>`"]
pub type HRB = crate::Reg<hrb::HRB_SPEC>;
#[doc = "Hash Register Bottom \\[31:0\\]
Register"]
pub mod hrb;
#[doc = "HRT register accessor: an alias for `Reg<HRT_SPEC>`"]
pub type HRT = crate::Reg<hrt::HRT_SPEC>;
#[doc = "Hash Register Top \\[63:32\\]
Register"]
pub mod hrt;
#[doc = "SA1B register accessor: an alias for `Reg<SA1B_SPEC>`"]
pub type SA1B = crate::Reg<sa1b::SA1B_SPEC>;
#[doc = "Specific Address 1 Bottom Register"]
pub mod sa1b;
#[doc = "SA1T register accessor: an alias for `Reg<SA1T_SPEC>`"]
pub type SA1T = crate::Reg<sa1t::SA1T_SPEC>;
#[doc = "Specific Address 1 Top Register"]
pub mod sa1t;
#[doc = "SA2B register accessor: an alias for `Reg<SA2B_SPEC>`"]
pub type SA2B = crate::Reg<sa2b::SA2B_SPEC>;
#[doc = "Specific Address 2 Bottom Register"]
pub mod sa2b;
#[doc = "SA2T register accessor: an alias for `Reg<SA2T_SPEC>`"]
pub type SA2T = crate::Reg<sa2t::SA2T_SPEC>;
#[doc = "Specific Address 2 Top Register"]
pub mod sa2t;
#[doc = "SA3B register accessor: an alias for `Reg<SA3B_SPEC>`"]
pub type SA3B = crate::Reg<sa3b::SA3B_SPEC>;
#[doc = "Specific Address 3 Bottom Register"]
pub mod sa3b;
#[doc = "SA3T register accessor: an alias for `Reg<SA3T_SPEC>`"]
pub type SA3T = crate::Reg<sa3t::SA3T_SPEC>;
#[doc = "Specific Address 3 Top Register"]
pub mod sa3t;
#[doc = "SA4B register accessor: an alias for `Reg<SA4B_SPEC>`"]
pub type SA4B = crate::Reg<sa4b::SA4B_SPEC>;
#[doc = "Specific Address 4 Bottom Register"]
pub mod sa4b;
#[doc = "SA4T register accessor: an alias for `Reg<SA4T_SPEC>`"]
pub type SA4T = crate::Reg<sa4t::SA4T_SPEC>;
#[doc = "Specific Address 4 Top Register"]
pub mod sa4t;
#[doc = "TID register accessor: an alias for `Reg<TID_SPEC>`"]
pub type TID = crate::Reg<tid::TID_SPEC>;
#[doc = "Type ID Checking Register"]
pub mod tid;
#[doc = "USRIO register accessor: an alias for `Reg<USRIO_SPEC>`"]
pub type USRIO = crate::Reg<usrio::USRIO_SPEC>;
#[doc = "User Input/Output Register"]
pub mod usrio;
