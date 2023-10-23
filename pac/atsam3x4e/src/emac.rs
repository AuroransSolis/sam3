#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Network Control Register"]
    pub ncr: NCR,
    #[doc = "0x04 - Network Configuration Register"]
    pub ncfgr: NCFGR,
    #[doc = "0x08 - Network Status Register"]
    pub nsr: NSR,
    _reserved3: [u8; 0x08],
    #[doc = "0x14 - Transmit Status Register"]
    pub tsr: TSR,
    #[doc = "0x18 - Receive Buffer Queue Pointer Register"]
    pub rbqp: RBQP,
    #[doc = "0x1c - Transmit Buffer Queue Pointer Register"]
    pub tbqp: TBQP,
    #[doc = "0x20 - Receive Status Register"]
    pub rsr: RSR,
    #[doc = "0x24 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x28 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x2c - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x30 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x34 - Phy Maintenance Register"]
    pub man: MAN,
    #[doc = "0x38 - Pause Time Register"]
    pub ptr: PTR,
    #[doc = "0x3c - Pause Frames Received Register"]
    pub pfr: PFR,
    #[doc = "0x40 - Frames Transmitted Ok Register"]
    pub fto: FTO,
    #[doc = "0x44 - Single Collision Frames Register"]
    pub scf: SCF,
    #[doc = "0x48 - Multiple Collision Frames Register"]
    pub mcf: MCF,
    #[doc = "0x4c - Frames Received Ok Register"]
    pub fro: FRO,
    #[doc = "0x50 - Frame Check Sequence Errors Register"]
    pub fcse: FCSE,
    #[doc = "0x54 - Alignment Errors Register"]
    pub ale: ALE,
    #[doc = "0x58 - Deferred Transmission Frames Register"]
    pub dtf: DTF,
    #[doc = "0x5c - Late Collisions Register"]
    pub lcol: LCOL,
    #[doc = "0x60 - Excessive Collisions Register"]
    pub ecol: ECOL,
    #[doc = "0x64 - Transmit Underrun Errors Register"]
    pub tund: TUND,
    #[doc = "0x68 - Carrier Sense Errors Register"]
    pub cse: CSE,
    #[doc = "0x6c - Receive Resource Errors Register"]
    pub rre: RRE,
    #[doc = "0x70 - Receive Overrun Errors Register"]
    pub rov: ROV,
    #[doc = "0x74 - Receive Symbol Errors Register"]
    pub rse: RSE,
    #[doc = "0x78 - Excessive Length Errors Register"]
    pub ele: ELE,
    #[doc = "0x7c - Receive Jabbers Register"]
    pub rja: RJA,
    #[doc = "0x80 - Undersize Frames Register"]
    pub usf: USF,
    #[doc = "0x84 - SQE Test Errors Register"]
    pub ste: STE,
    #[doc = "0x88 - Received Length Field Mismatch Register"]
    pub rle: RLE,
    _reserved33: [u8; 0x04],
    #[doc = "0x90 - Hash Register Bottom \\[31:0\\]
Register"]
    pub hrb: HRB,
    #[doc = "0x94 - Hash Register Top \\[63:32\\]
Register"]
    pub hrt: HRT,
    #[doc = "0x98 - Specific Address 1 Bottom Register"]
    pub sa1b: SA1B,
    #[doc = "0x9c - Specific Address 1 Top Register"]
    pub sa1t: SA1T,
    #[doc = "0xa0 - Specific Address 2 Bottom Register"]
    pub sa2b: SA2B,
    #[doc = "0xa4 - Specific Address 2 Top Register"]
    pub sa2t: SA2T,
    #[doc = "0xa8 - Specific Address 3 Bottom Register"]
    pub sa3b: SA3B,
    #[doc = "0xac - Specific Address 3 Top Register"]
    pub sa3t: SA3T,
    #[doc = "0xb0 - Specific Address 4 Bottom Register"]
    pub sa4b: SA4B,
    #[doc = "0xb4 - Specific Address 4 Top Register"]
    pub sa4t: SA4T,
    #[doc = "0xb8 - Type ID Checking Register"]
    pub tid: TID,
    _reserved44: [u8; 0x04],
    #[doc = "0xc0 - User Input/Output Register"]
    pub usrio: USRIO,
}
#[doc = "NCR (rw) register accessor: Network Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ncr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ncr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ncr`]
module"]
pub type NCR = crate::Reg<ncr::NCR_SPEC>;
#[doc = "Network Control Register"]
pub mod ncr;
#[doc = "NCFGR (rw) register accessor: Network Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ncfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ncfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ncfgr`]
module"]
pub type NCFGR = crate::Reg<ncfgr::NCFGR_SPEC>;
#[doc = "Network Configuration Register"]
pub mod ncfgr;
#[doc = "NSR (r) register accessor: Network Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsr`]
module"]
pub type NSR = crate::Reg<nsr::NSR_SPEC>;
#[doc = "Network Status Register"]
pub mod nsr;
#[doc = "TSR (rw) register accessor: Transmit Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsr`]
module"]
pub type TSR = crate::Reg<tsr::TSR_SPEC>;
#[doc = "Transmit Status Register"]
pub mod tsr;
#[doc = "RBQP (rw) register accessor: Receive Buffer Queue Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbqp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbqp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbqp`]
module"]
pub type RBQP = crate::Reg<rbqp::RBQP_SPEC>;
#[doc = "Receive Buffer Queue Pointer Register"]
pub mod rbqp;
#[doc = "TBQP (rw) register accessor: Transmit Buffer Queue Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbqp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbqp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbqp`]
module"]
pub type TBQP = crate::Reg<tbqp::TBQP_SPEC>;
#[doc = "Transmit Buffer Queue Pointer Register"]
pub mod tbqp;
#[doc = "RSR (rw) register accessor: Receive Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsr`]
module"]
pub type RSR = crate::Reg<rsr::RSR_SPEC>;
#[doc = "Receive Status Register"]
pub mod rsr;
#[doc = "ISR (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
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
#[doc = "MAN (rw) register accessor: Phy Maintenance Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`man::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`man::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@man`]
module"]
pub type MAN = crate::Reg<man::MAN_SPEC>;
#[doc = "Phy Maintenance Register"]
pub mod man;
#[doc = "PTR (rw) register accessor: Pause Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptr`]
module"]
pub type PTR = crate::Reg<ptr::PTR_SPEC>;
#[doc = "Pause Time Register"]
pub mod ptr;
#[doc = "PFR (rw) register accessor: Pause Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfr`]
module"]
pub type PFR = crate::Reg<pfr::PFR_SPEC>;
#[doc = "Pause Frames Received Register"]
pub mod pfr;
#[doc = "FTO (rw) register accessor: Frames Transmitted Ok Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fto::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fto::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fto`]
module"]
pub type FTO = crate::Reg<fto::FTO_SPEC>;
#[doc = "Frames Transmitted Ok Register"]
pub mod fto;
#[doc = "SCF (rw) register accessor: Single Collision Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scf`]
module"]
pub type SCF = crate::Reg<scf::SCF_SPEC>;
#[doc = "Single Collision Frames Register"]
pub mod scf;
#[doc = "MCF (rw) register accessor: Multiple Collision Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcf`]
module"]
pub type MCF = crate::Reg<mcf::MCF_SPEC>;
#[doc = "Multiple Collision Frames Register"]
pub mod mcf;
#[doc = "FRO (rw) register accessor: Frames Received Ok Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fro::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fro::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fro`]
module"]
pub type FRO = crate::Reg<fro::FRO_SPEC>;
#[doc = "Frames Received Ok Register"]
pub mod fro;
#[doc = "FCSE (rw) register accessor: Frame Check Sequence Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcse`]
module"]
pub type FCSE = crate::Reg<fcse::FCSE_SPEC>;
#[doc = "Frame Check Sequence Errors Register"]
pub mod fcse;
#[doc = "ALE (rw) register accessor: Alignment Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ale::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ale::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale`]
module"]
pub type ALE = crate::Reg<ale::ALE_SPEC>;
#[doc = "Alignment Errors Register"]
pub mod ale;
#[doc = "DTF (rw) register accessor: Deferred Transmission Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtf`]
module"]
pub type DTF = crate::Reg<dtf::DTF_SPEC>;
#[doc = "Deferred Transmission Frames Register"]
pub mod dtf;
#[doc = "LCOL (rw) register accessor: Late Collisions Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcol`]
module"]
pub type LCOL = crate::Reg<lcol::LCOL_SPEC>;
#[doc = "Late Collisions Register"]
pub mod lcol;
#[doc = "ECOL (rw) register accessor: Excessive Collisions Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecol`]
module"]
pub type ECOL = crate::Reg<ecol::ECOL_SPEC>;
#[doc = "Excessive Collisions Register"]
pub mod ecol;
#[doc = "TUND (rw) register accessor: Transmit Underrun Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tund::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tund::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tund`]
module"]
pub type TUND = crate::Reg<tund::TUND_SPEC>;
#[doc = "Transmit Underrun Errors Register"]
pub mod tund;
#[doc = "CSE (rw) register accessor: Carrier Sense Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cse`]
module"]
pub type CSE = crate::Reg<cse::CSE_SPEC>;
#[doc = "Carrier Sense Errors Register"]
pub mod cse;
#[doc = "RRE (rw) register accessor: Receive Resource Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rre::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rre::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rre`]
module"]
pub type RRE = crate::Reg<rre::RRE_SPEC>;
#[doc = "Receive Resource Errors Register"]
pub mod rre;
#[doc = "ROV (rw) register accessor: Receive Overrun Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rov::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rov::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rov`]
module"]
pub type ROV = crate::Reg<rov::ROV_SPEC>;
#[doc = "Receive Overrun Errors Register"]
pub mod rov;
#[doc = "RSE (rw) register accessor: Receive Symbol Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rse`]
module"]
pub type RSE = crate::Reg<rse::RSE_SPEC>;
#[doc = "Receive Symbol Errors Register"]
pub mod rse;
#[doc = "ELE (rw) register accessor: Excessive Length Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ele::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ele::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ele`]
module"]
pub type ELE = crate::Reg<ele::ELE_SPEC>;
#[doc = "Excessive Length Errors Register"]
pub mod ele;
#[doc = "RJA (rw) register accessor: Receive Jabbers Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rja::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rja::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rja`]
module"]
pub type RJA = crate::Reg<rja::RJA_SPEC>;
#[doc = "Receive Jabbers Register"]
pub mod rja;
#[doc = "USF (rw) register accessor: Undersize Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usf`]
module"]
pub type USF = crate::Reg<usf::USF_SPEC>;
#[doc = "Undersize Frames Register"]
pub mod usf;
#[doc = "STE (rw) register accessor: SQE Test Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ste::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ste::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ste`]
module"]
pub type STE = crate::Reg<ste::STE_SPEC>;
#[doc = "SQE Test Errors Register"]
pub mod ste;
#[doc = "RLE (rw) register accessor: Received Length Field Mismatch Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rle::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rle::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rle`]
module"]
pub type RLE = crate::Reg<rle::RLE_SPEC>;
#[doc = "Received Length Field Mismatch Register"]
pub mod rle;
#[doc = "HRB (rw) register accessor: Hash Register Bottom \\[31:0\\]
Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrb`]
module"]
pub type HRB = crate::Reg<hrb::HRB_SPEC>;
#[doc = "Hash Register Bottom \\[31:0\\]
Register"]
pub mod hrb;
#[doc = "HRT (rw) register accessor: Hash Register Top \\[63:32\\]
Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrt`]
module"]
pub type HRT = crate::Reg<hrt::HRT_SPEC>;
#[doc = "Hash Register Top \\[63:32\\]
Register"]
pub mod hrt;
#[doc = "SA1B (rw) register accessor: Specific Address 1 Bottom Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sa1b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sa1b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa1b`]
module"]
pub type SA1B = crate::Reg<sa1b::SA1B_SPEC>;
#[doc = "Specific Address 1 Bottom Register"]
pub mod sa1b;
#[doc = "SA1T (rw) register accessor: Specific Address 1 Top Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sa1t::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sa1t::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa1t`]
module"]
pub type SA1T = crate::Reg<sa1t::SA1T_SPEC>;
#[doc = "Specific Address 1 Top Register"]
pub mod sa1t;
#[doc = "SA2B (rw) register accessor: Specific Address 2 Bottom Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sa2b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sa2b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa2b`]
module"]
pub type SA2B = crate::Reg<sa2b::SA2B_SPEC>;
#[doc = "Specific Address 2 Bottom Register"]
pub mod sa2b;
#[doc = "SA2T (rw) register accessor: Specific Address 2 Top Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sa2t::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sa2t::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa2t`]
module"]
pub type SA2T = crate::Reg<sa2t::SA2T_SPEC>;
#[doc = "Specific Address 2 Top Register"]
pub mod sa2t;
#[doc = "SA3B (rw) register accessor: Specific Address 3 Bottom Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sa3b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sa3b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa3b`]
module"]
pub type SA3B = crate::Reg<sa3b::SA3B_SPEC>;
#[doc = "Specific Address 3 Bottom Register"]
pub mod sa3b;
#[doc = "SA3T (rw) register accessor: Specific Address 3 Top Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sa3t::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sa3t::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa3t`]
module"]
pub type SA3T = crate::Reg<sa3t::SA3T_SPEC>;
#[doc = "Specific Address 3 Top Register"]
pub mod sa3t;
#[doc = "SA4B (rw) register accessor: Specific Address 4 Bottom Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sa4b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sa4b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa4b`]
module"]
pub type SA4B = crate::Reg<sa4b::SA4B_SPEC>;
#[doc = "Specific Address 4 Bottom Register"]
pub mod sa4b;
#[doc = "SA4T (rw) register accessor: Specific Address 4 Top Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sa4t::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sa4t::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa4t`]
module"]
pub type SA4T = crate::Reg<sa4t::SA4T_SPEC>;
#[doc = "Specific Address 4 Top Register"]
pub mod sa4t;
#[doc = "TID (rw) register accessor: Type ID Checking Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tid`]
module"]
pub type TID = crate::Reg<tid::TID_SPEC>;
#[doc = "Type ID Checking Register"]
pub mod tid;
#[doc = "USRIO (rw) register accessor: User Input/Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usrio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usrio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usrio`]
module"]
pub type USRIO = crate::Reg<usrio::USRIO_SPEC>;
#[doc = "User Input/Output Register"]
pub mod usrio;
