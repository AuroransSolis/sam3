#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ncr: Ncr,
    ncfgr: Ncfgr,
    nsr: Nsr,
    _reserved3: [u8; 0x08],
    tsr: Tsr,
    rbqp: Rbqp,
    tbqp: Tbqp,
    rsr: Rsr,
    isr: Isr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    man: Man,
    ptr: Ptr,
    pfr: Pfr,
    fto: Fto,
    scf: Scf,
    mcf: Mcf,
    fro: Fro,
    fcse: Fcse,
    ale: Ale,
    dtf: Dtf,
    lcol: Lcol,
    ecol: Ecol,
    tund: Tund,
    cse: Cse,
    rre: Rre,
    rov: Rov,
    rse: Rse,
    ele: Ele,
    rja: Rja,
    usf: Usf,
    ste: Ste,
    rle: Rle,
    _reserved33: [u8; 0x04],
    hrb: Hrb,
    hrt: Hrt,
    sa1b: Sa1b,
    sa1t: Sa1t,
    sa2b: Sa2b,
    sa2t: Sa2t,
    sa3b: Sa3b,
    sa3t: Sa3t,
    sa4b: Sa4b,
    sa4t: Sa4t,
    tid: Tid,
    _reserved44: [u8; 0x04],
    usrio: Usrio,
}
impl RegisterBlock {
    #[doc = "0x00 - Network Control Register"]
    #[inline(always)]
    pub const fn ncr(&self) -> &Ncr {
        &self.ncr
    }
    #[doc = "0x04 - Network Configuration Register"]
    #[inline(always)]
    pub const fn ncfgr(&self) -> &Ncfgr {
        &self.ncfgr
    }
    #[doc = "0x08 - Network Status Register"]
    #[inline(always)]
    pub const fn nsr(&self) -> &Nsr {
        &self.nsr
    }
    #[doc = "0x14 - Transmit Status Register"]
    #[inline(always)]
    pub const fn tsr(&self) -> &Tsr {
        &self.tsr
    }
    #[doc = "0x18 - Receive Buffer Queue Pointer Register"]
    #[inline(always)]
    pub const fn rbqp(&self) -> &Rbqp {
        &self.rbqp
    }
    #[doc = "0x1c - Transmit Buffer Queue Pointer Register"]
    #[inline(always)]
    pub const fn tbqp(&self) -> &Tbqp {
        &self.tbqp
    }
    #[doc = "0x20 - Receive Status Register"]
    #[inline(always)]
    pub const fn rsr(&self) -> &Rsr {
        &self.rsr
    }
    #[doc = "0x24 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x28 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x2c - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x30 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x34 - Phy Maintenance Register"]
    #[inline(always)]
    pub const fn man(&self) -> &Man {
        &self.man
    }
    #[doc = "0x38 - Pause Time Register"]
    #[inline(always)]
    pub const fn ptr(&self) -> &Ptr {
        &self.ptr
    }
    #[doc = "0x3c - Pause Frames Received Register"]
    #[inline(always)]
    pub const fn pfr(&self) -> &Pfr {
        &self.pfr
    }
    #[doc = "0x40 - Frames Transmitted Ok Register"]
    #[inline(always)]
    pub const fn fto(&self) -> &Fto {
        &self.fto
    }
    #[doc = "0x44 - Single Collision Frames Register"]
    #[inline(always)]
    pub const fn scf(&self) -> &Scf {
        &self.scf
    }
    #[doc = "0x48 - Multiple Collision Frames Register"]
    #[inline(always)]
    pub const fn mcf(&self) -> &Mcf {
        &self.mcf
    }
    #[doc = "0x4c - Frames Received Ok Register"]
    #[inline(always)]
    pub const fn fro(&self) -> &Fro {
        &self.fro
    }
    #[doc = "0x50 - Frame Check Sequence Errors Register"]
    #[inline(always)]
    pub const fn fcse(&self) -> &Fcse {
        &self.fcse
    }
    #[doc = "0x54 - Alignment Errors Register"]
    #[inline(always)]
    pub const fn ale(&self) -> &Ale {
        &self.ale
    }
    #[doc = "0x58 - Deferred Transmission Frames Register"]
    #[inline(always)]
    pub const fn dtf(&self) -> &Dtf {
        &self.dtf
    }
    #[doc = "0x5c - Late Collisions Register"]
    #[inline(always)]
    pub const fn lcol(&self) -> &Lcol {
        &self.lcol
    }
    #[doc = "0x60 - Excessive Collisions Register"]
    #[inline(always)]
    pub const fn ecol(&self) -> &Ecol {
        &self.ecol
    }
    #[doc = "0x64 - Transmit Underrun Errors Register"]
    #[inline(always)]
    pub const fn tund(&self) -> &Tund {
        &self.tund
    }
    #[doc = "0x68 - Carrier Sense Errors Register"]
    #[inline(always)]
    pub const fn cse(&self) -> &Cse {
        &self.cse
    }
    #[doc = "0x6c - Receive Resource Errors Register"]
    #[inline(always)]
    pub const fn rre(&self) -> &Rre {
        &self.rre
    }
    #[doc = "0x70 - Receive Overrun Errors Register"]
    #[inline(always)]
    pub const fn rov(&self) -> &Rov {
        &self.rov
    }
    #[doc = "0x74 - Receive Symbol Errors Register"]
    #[inline(always)]
    pub const fn rse(&self) -> &Rse {
        &self.rse
    }
    #[doc = "0x78 - Excessive Length Errors Register"]
    #[inline(always)]
    pub const fn ele(&self) -> &Ele {
        &self.ele
    }
    #[doc = "0x7c - Receive Jabbers Register"]
    #[inline(always)]
    pub const fn rja(&self) -> &Rja {
        &self.rja
    }
    #[doc = "0x80 - Undersize Frames Register"]
    #[inline(always)]
    pub const fn usf(&self) -> &Usf {
        &self.usf
    }
    #[doc = "0x84 - SQE Test Errors Register"]
    #[inline(always)]
    pub const fn ste(&self) -> &Ste {
        &self.ste
    }
    #[doc = "0x88 - Received Length Field Mismatch Register"]
    #[inline(always)]
    pub const fn rle(&self) -> &Rle {
        &self.rle
    }
    #[doc = "0x90 - Hash Register Bottom \\[31:0\\]
Register"]
    #[inline(always)]
    pub const fn hrb(&self) -> &Hrb {
        &self.hrb
    }
    #[doc = "0x94 - Hash Register Top \\[63:32\\]
Register"]
    #[inline(always)]
    pub const fn hrt(&self) -> &Hrt {
        &self.hrt
    }
    #[doc = "0x98 - Specific Address 1 Bottom Register"]
    #[inline(always)]
    pub const fn sa1b(&self) -> &Sa1b {
        &self.sa1b
    }
    #[doc = "0x9c - Specific Address 1 Top Register"]
    #[inline(always)]
    pub const fn sa1t(&self) -> &Sa1t {
        &self.sa1t
    }
    #[doc = "0xa0 - Specific Address 2 Bottom Register"]
    #[inline(always)]
    pub const fn sa2b(&self) -> &Sa2b {
        &self.sa2b
    }
    #[doc = "0xa4 - Specific Address 2 Top Register"]
    #[inline(always)]
    pub const fn sa2t(&self) -> &Sa2t {
        &self.sa2t
    }
    #[doc = "0xa8 - Specific Address 3 Bottom Register"]
    #[inline(always)]
    pub const fn sa3b(&self) -> &Sa3b {
        &self.sa3b
    }
    #[doc = "0xac - Specific Address 3 Top Register"]
    #[inline(always)]
    pub const fn sa3t(&self) -> &Sa3t {
        &self.sa3t
    }
    #[doc = "0xb0 - Specific Address 4 Bottom Register"]
    #[inline(always)]
    pub const fn sa4b(&self) -> &Sa4b {
        &self.sa4b
    }
    #[doc = "0xb4 - Specific Address 4 Top Register"]
    #[inline(always)]
    pub const fn sa4t(&self) -> &Sa4t {
        &self.sa4t
    }
    #[doc = "0xb8 - Type ID Checking Register"]
    #[inline(always)]
    pub const fn tid(&self) -> &Tid {
        &self.tid
    }
    #[doc = "0xc0 - User Input/Output Register"]
    #[inline(always)]
    pub const fn usrio(&self) -> &Usrio {
        &self.usrio
    }
}
#[doc = "NCR (rw) register accessor: Network Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ncr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ncr`]
module"]
#[doc(alias = "NCR")]
pub type Ncr = crate::Reg<ncr::NcrSpec>;
#[doc = "Network Control Register"]
pub mod ncr;
#[doc = "NCFGR (rw) register accessor: Network Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ncfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ncfgr`]
module"]
#[doc(alias = "NCFGR")]
pub type Ncfgr = crate::Reg<ncfgr::NcfgrSpec>;
#[doc = "Network Configuration Register"]
pub mod ncfgr;
#[doc = "NSR (r) register accessor: Network Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsr`]
module"]
#[doc(alias = "NSR")]
pub type Nsr = crate::Reg<nsr::NsrSpec>;
#[doc = "Network Status Register"]
pub mod nsr;
#[doc = "TSR (rw) register accessor: Transmit Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsr`]
module"]
#[doc(alias = "TSR")]
pub type Tsr = crate::Reg<tsr::TsrSpec>;
#[doc = "Transmit Status Register"]
pub mod tsr;
#[doc = "RBQP (rw) register accessor: Receive Buffer Queue Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rbqp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbqp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbqp`]
module"]
#[doc(alias = "RBQP")]
pub type Rbqp = crate::Reg<rbqp::RbqpSpec>;
#[doc = "Receive Buffer Queue Pointer Register"]
pub mod rbqp;
#[doc = "TBQP (rw) register accessor: Transmit Buffer Queue Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbqp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbqp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbqp`]
module"]
#[doc(alias = "TBQP")]
pub type Tbqp = crate::Reg<tbqp::TbqpSpec>;
#[doc = "Transmit Buffer Queue Pointer Register"]
pub mod tbqp;
#[doc = "RSR (rw) register accessor: Receive Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsr`]
module"]
#[doc(alias = "RSR")]
pub type Rsr = crate::Reg<rsr::RsrSpec>;
#[doc = "Receive Status Register"]
pub mod rsr;
#[doc = "ISR (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt Status Register"]
pub mod isr;
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
#[doc = "MAN (rw) register accessor: Phy Maintenance Register\n\nYou can [`read`](crate::Reg::read) this register and get [`man::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`man::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@man`]
module"]
#[doc(alias = "MAN")]
pub type Man = crate::Reg<man::ManSpec>;
#[doc = "Phy Maintenance Register"]
pub mod man;
#[doc = "PTR (rw) register accessor: Pause Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptr`]
module"]
#[doc(alias = "PTR")]
pub type Ptr = crate::Reg<ptr::PtrSpec>;
#[doc = "Pause Time Register"]
pub mod ptr;
#[doc = "PFR (rw) register accessor: Pause Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfr`]
module"]
#[doc(alias = "PFR")]
pub type Pfr = crate::Reg<pfr::PfrSpec>;
#[doc = "Pause Frames Received Register"]
pub mod pfr;
#[doc = "FTO (rw) register accessor: Frames Transmitted Ok Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fto::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fto::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fto`]
module"]
#[doc(alias = "FTO")]
pub type Fto = crate::Reg<fto::FtoSpec>;
#[doc = "Frames Transmitted Ok Register"]
pub mod fto;
#[doc = "SCF (rw) register accessor: Single Collision Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scf`]
module"]
#[doc(alias = "SCF")]
pub type Scf = crate::Reg<scf::ScfSpec>;
#[doc = "Single Collision Frames Register"]
pub mod scf;
#[doc = "MCF (rw) register accessor: Multiple Collision Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcf`]
module"]
#[doc(alias = "MCF")]
pub type Mcf = crate::Reg<mcf::McfSpec>;
#[doc = "Multiple Collision Frames Register"]
pub mod mcf;
#[doc = "FRO (rw) register accessor: Frames Received Ok Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fro::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fro::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fro`]
module"]
#[doc(alias = "FRO")]
pub type Fro = crate::Reg<fro::FroSpec>;
#[doc = "Frames Received Ok Register"]
pub mod fro;
#[doc = "FCSE (rw) register accessor: Frame Check Sequence Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcse`]
module"]
#[doc(alias = "FCSE")]
pub type Fcse = crate::Reg<fcse::FcseSpec>;
#[doc = "Frame Check Sequence Errors Register"]
pub mod fcse;
#[doc = "ALE (rw) register accessor: Alignment Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ale::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ale`]
module"]
#[doc(alias = "ALE")]
pub type Ale = crate::Reg<ale::AleSpec>;
#[doc = "Alignment Errors Register"]
pub mod ale;
#[doc = "DTF (rw) register accessor: Deferred Transmission Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtf`]
module"]
#[doc(alias = "DTF")]
pub type Dtf = crate::Reg<dtf::DtfSpec>;
#[doc = "Deferred Transmission Frames Register"]
pub mod dtf;
#[doc = "LCOL (rw) register accessor: Late Collisions Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcol`]
module"]
#[doc(alias = "LCOL")]
pub type Lcol = crate::Reg<lcol::LcolSpec>;
#[doc = "Late Collisions Register"]
pub mod lcol;
#[doc = "ECOL (rw) register accessor: Excessive Collisions Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecol`]
module"]
#[doc(alias = "ECOL")]
pub type Ecol = crate::Reg<ecol::EcolSpec>;
#[doc = "Excessive Collisions Register"]
pub mod ecol;
#[doc = "TUND (rw) register accessor: Transmit Underrun Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tund::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tund::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tund`]
module"]
#[doc(alias = "TUND")]
pub type Tund = crate::Reg<tund::TundSpec>;
#[doc = "Transmit Underrun Errors Register"]
pub mod tund;
#[doc = "CSE (rw) register accessor: Carrier Sense Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cse`]
module"]
#[doc(alias = "CSE")]
pub type Cse = crate::Reg<cse::CseSpec>;
#[doc = "Carrier Sense Errors Register"]
pub mod cse;
#[doc = "RRE (rw) register accessor: Receive Resource Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rre`]
module"]
#[doc(alias = "RRE")]
pub type Rre = crate::Reg<rre::RreSpec>;
#[doc = "Receive Resource Errors Register"]
pub mod rre;
#[doc = "ROV (rw) register accessor: Receive Overrun Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rov::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rov::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rov`]
module"]
#[doc(alias = "ROV")]
pub type Rov = crate::Reg<rov::RovSpec>;
#[doc = "Receive Overrun Errors Register"]
pub mod rov;
#[doc = "RSE (rw) register accessor: Receive Symbol Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rse`]
module"]
#[doc(alias = "RSE")]
pub type Rse = crate::Reg<rse::RseSpec>;
#[doc = "Receive Symbol Errors Register"]
pub mod rse;
#[doc = "ELE (rw) register accessor: Excessive Length Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ele::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ele::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ele`]
module"]
#[doc(alias = "ELE")]
pub type Ele = crate::Reg<ele::EleSpec>;
#[doc = "Excessive Length Errors Register"]
pub mod ele;
#[doc = "RJA (rw) register accessor: Receive Jabbers Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rja::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rja::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rja`]
module"]
#[doc(alias = "RJA")]
pub type Rja = crate::Reg<rja::RjaSpec>;
#[doc = "Receive Jabbers Register"]
pub mod rja;
#[doc = "USF (rw) register accessor: Undersize Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usf`]
module"]
#[doc(alias = "USF")]
pub type Usf = crate::Reg<usf::UsfSpec>;
#[doc = "Undersize Frames Register"]
pub mod usf;
#[doc = "STE (rw) register accessor: SQE Test Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ste::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ste::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ste`]
module"]
#[doc(alias = "STE")]
pub type Ste = crate::Reg<ste::SteSpec>;
#[doc = "SQE Test Errors Register"]
pub mod ste;
#[doc = "RLE (rw) register accessor: Received Length Field Mismatch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rle`]
module"]
#[doc(alias = "RLE")]
pub type Rle = crate::Reg<rle::RleSpec>;
#[doc = "Received Length Field Mismatch Register"]
pub mod rle;
#[doc = "HRB (rw) register accessor: Hash Register Bottom \\[31:0\\]
Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrb`]
module"]
#[doc(alias = "HRB")]
pub type Hrb = crate::Reg<hrb::HrbSpec>;
#[doc = "Hash Register Bottom \\[31:0\\]
Register"]
pub mod hrb;
#[doc = "HRT (rw) register accessor: Hash Register Top \\[63:32\\]
Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hrt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrt`]
module"]
#[doc(alias = "HRT")]
pub type Hrt = crate::Reg<hrt::HrtSpec>;
#[doc = "Hash Register Top \\[63:32\\]
Register"]
pub mod hrt;
#[doc = "SA1B (rw) register accessor: Specific Address 1 Bottom Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sa1b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa1b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa1b`]
module"]
#[doc(alias = "SA1B")]
pub type Sa1b = crate::Reg<sa1b::Sa1bSpec>;
#[doc = "Specific Address 1 Bottom Register"]
pub mod sa1b;
#[doc = "SA1T (rw) register accessor: Specific Address 1 Top Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sa1t::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa1t::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa1t`]
module"]
#[doc(alias = "SA1T")]
pub type Sa1t = crate::Reg<sa1t::Sa1tSpec>;
#[doc = "Specific Address 1 Top Register"]
pub mod sa1t;
#[doc = "SA2B (rw) register accessor: Specific Address 2 Bottom Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sa2b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa2b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa2b`]
module"]
#[doc(alias = "SA2B")]
pub type Sa2b = crate::Reg<sa2b::Sa2bSpec>;
#[doc = "Specific Address 2 Bottom Register"]
pub mod sa2b;
#[doc = "SA2T (rw) register accessor: Specific Address 2 Top Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sa2t::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa2t::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa2t`]
module"]
#[doc(alias = "SA2T")]
pub type Sa2t = crate::Reg<sa2t::Sa2tSpec>;
#[doc = "Specific Address 2 Top Register"]
pub mod sa2t;
#[doc = "SA3B (rw) register accessor: Specific Address 3 Bottom Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sa3b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa3b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa3b`]
module"]
#[doc(alias = "SA3B")]
pub type Sa3b = crate::Reg<sa3b::Sa3bSpec>;
#[doc = "Specific Address 3 Bottom Register"]
pub mod sa3b;
#[doc = "SA3T (rw) register accessor: Specific Address 3 Top Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sa3t::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa3t::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa3t`]
module"]
#[doc(alias = "SA3T")]
pub type Sa3t = crate::Reg<sa3t::Sa3tSpec>;
#[doc = "Specific Address 3 Top Register"]
pub mod sa3t;
#[doc = "SA4B (rw) register accessor: Specific Address 4 Bottom Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sa4b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa4b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa4b`]
module"]
#[doc(alias = "SA4B")]
pub type Sa4b = crate::Reg<sa4b::Sa4bSpec>;
#[doc = "Specific Address 4 Bottom Register"]
pub mod sa4b;
#[doc = "SA4T (rw) register accessor: Specific Address 4 Top Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sa4t::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa4t::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa4t`]
module"]
#[doc(alias = "SA4T")]
pub type Sa4t = crate::Reg<sa4t::Sa4tSpec>;
#[doc = "Specific Address 4 Top Register"]
pub mod sa4t;
#[doc = "TID (rw) register accessor: Type ID Checking Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tid`]
module"]
#[doc(alias = "TID")]
pub type Tid = crate::Reg<tid::TidSpec>;
#[doc = "Type ID Checking Register"]
pub mod tid;
#[doc = "USRIO (rw) register accessor: User Input/Output Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usrio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usrio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usrio`]
module"]
#[doc(alias = "USRIO")]
pub type Usrio = crate::Reg<usrio::UsrioSpec>;
#[doc = "User Input/Output Register"]
pub mod usrio;
