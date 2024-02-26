#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk: Clk,
    ena: Ena,
    dis: Dis,
    sr: Sr,
    ier1: Ier1,
    idr1: Idr1,
    imr1: Imr1,
    isr1: Isr1,
    scm: Scm,
    _reserved9: [u8; 0x04],
    scuc: Scuc,
    scup: Scup,
    scupupd: Scupupd,
    ier2: Ier2,
    idr2: Idr2,
    imr2: Imr2,
    isr2: Isr2,
    oov: Oov,
    os: Os,
    oss: Oss,
    osc: Osc,
    ossupd: Ossupd,
    oscupd: Oscupd,
    fmr: Fmr,
    fsr: Fsr,
    fcr: Fcr,
    fpv: Fpv,
    fpe: Fpe,
    _reserved27: [u8; 0x0c],
    elmr0: Elmr0,
    elmr1: Elmr1,
    _reserved29: [u8; 0x2c],
    smmr: Smmr,
    _reserved30: [u8; 0x30],
    wpcr: Wpcr,
    wpsr: Wpsr,
    _reserved32: [u8; 0x1c],
    tpr: Tpr,
    tcr: Tcr,
    _reserved34: [u8; 0x08],
    tnpr: Tnpr,
    tncr: Tncr,
    ptcr: Ptcr,
    ptsr: Ptsr,
    _reserved38: [u8; 0x08],
    cmpv0: Cmpv0,
    cmpvupd0: Cmpvupd0,
    cmpm0: Cmpm0,
    cmpmupd0: Cmpmupd0,
    cmpv1: Cmpv1,
    cmpvupd1: Cmpvupd1,
    cmpm1: Cmpm1,
    cmpmupd1: Cmpmupd1,
    cmpv2: Cmpv2,
    cmpvupd2: Cmpvupd2,
    cmpm2: Cmpm2,
    cmpmupd2: Cmpmupd2,
    cmpv3: Cmpv3,
    cmpvupd3: Cmpvupd3,
    cmpm3: Cmpm3,
    cmpmupd3: Cmpmupd3,
    cmpv4: Cmpv4,
    cmpvupd4: Cmpvupd4,
    cmpm4: Cmpm4,
    cmpmupd4: Cmpmupd4,
    cmpv5: Cmpv5,
    cmpvupd5: Cmpvupd5,
    cmpm5: Cmpm5,
    cmpmupd5: Cmpmupd5,
    cmpv6: Cmpv6,
    cmpvupd6: Cmpvupd6,
    cmpm6: Cmpm6,
    cmpmupd6: Cmpmupd6,
    cmpv7: Cmpv7,
    cmpvupd7: Cmpvupd7,
    cmpm7: Cmpm7,
    cmpmupd7: Cmpmupd7,
    _reserved70: [u8; 0x50],
    cmr0: Cmr0,
    cdty0: Cdty0,
    cdtyupd0: Cdtyupd0,
    cprd0: Cprd0,
    cprdupd0: Cprdupd0,
    ccnt0: Ccnt0,
    dt0: Dt0,
    dtupd0: Dtupd0,
    cmr1: Cmr1,
    cdty1: Cdty1,
    cdtyupd1: Cdtyupd1,
    cprd1: Cprd1,
    cprdupd1: Cprdupd1,
    ccnt1: Ccnt1,
    dt1: Dt1,
    dtupd1: Dtupd1,
    cmr2: Cmr2,
    cdty2: Cdty2,
    cdtyupd2: Cdtyupd2,
    cprd2: Cprd2,
    cprdupd2: Cprdupd2,
    ccnt2: Ccnt2,
    dt2: Dt2,
    dtupd2: Dtupd2,
    cmr3: Cmr3,
    cdty3: Cdty3,
    cdtyupd3: Cdtyupd3,
    cprd3: Cprd3,
    cprdupd3: Cprdupd3,
    ccnt3: Ccnt3,
    dt3: Dt3,
    dtupd3: Dtupd3,
}
impl RegisterBlock {
    #[doc = "0x00 - PWM Clock Register"]
    #[inline(always)]
    pub const fn clk(&self) -> &Clk {
        &self.clk
    }
    #[doc = "0x04 - PWM Enable Register"]
    #[inline(always)]
    pub const fn ena(&self) -> &Ena {
        &self.ena
    }
    #[doc = "0x08 - PWM Disable Register"]
    #[inline(always)]
    pub const fn dis(&self) -> &Dis {
        &self.dis
    }
    #[doc = "0x0c - PWM Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x10 - PWM Interrupt Enable Register 1"]
    #[inline(always)]
    pub const fn ier1(&self) -> &Ier1 {
        &self.ier1
    }
    #[doc = "0x14 - PWM Interrupt Disable Register 1"]
    #[inline(always)]
    pub const fn idr1(&self) -> &Idr1 {
        &self.idr1
    }
    #[doc = "0x18 - PWM Interrupt Mask Register 1"]
    #[inline(always)]
    pub const fn imr1(&self) -> &Imr1 {
        &self.imr1
    }
    #[doc = "0x1c - PWM Interrupt Status Register 1"]
    #[inline(always)]
    pub const fn isr1(&self) -> &Isr1 {
        &self.isr1
    }
    #[doc = "0x20 - PWM Sync Channels Mode Register"]
    #[inline(always)]
    pub const fn scm(&self) -> &Scm {
        &self.scm
    }
    #[doc = "0x28 - PWM Sync Channels Update Control Register"]
    #[inline(always)]
    pub const fn scuc(&self) -> &Scuc {
        &self.scuc
    }
    #[doc = "0x2c - PWM Sync Channels Update Period Register"]
    #[inline(always)]
    pub const fn scup(&self) -> &Scup {
        &self.scup
    }
    #[doc = "0x30 - PWM Sync Channels Update Period Update Register"]
    #[inline(always)]
    pub const fn scupupd(&self) -> &Scupupd {
        &self.scupupd
    }
    #[doc = "0x34 - PWM Interrupt Enable Register 2"]
    #[inline(always)]
    pub const fn ier2(&self) -> &Ier2 {
        &self.ier2
    }
    #[doc = "0x38 - PWM Interrupt Disable Register 2"]
    #[inline(always)]
    pub const fn idr2(&self) -> &Idr2 {
        &self.idr2
    }
    #[doc = "0x3c - PWM Interrupt Mask Register 2"]
    #[inline(always)]
    pub const fn imr2(&self) -> &Imr2 {
        &self.imr2
    }
    #[doc = "0x40 - PWM Interrupt Status Register 2"]
    #[inline(always)]
    pub const fn isr2(&self) -> &Isr2 {
        &self.isr2
    }
    #[doc = "0x44 - PWM Output Override Value Register"]
    #[inline(always)]
    pub const fn oov(&self) -> &Oov {
        &self.oov
    }
    #[doc = "0x48 - PWM Output Selection Register"]
    #[inline(always)]
    pub const fn os(&self) -> &Os {
        &self.os
    }
    #[doc = "0x4c - PWM Output Selection Set Register"]
    #[inline(always)]
    pub const fn oss(&self) -> &Oss {
        &self.oss
    }
    #[doc = "0x50 - PWM Output Selection Clear Register"]
    #[inline(always)]
    pub const fn osc(&self) -> &Osc {
        &self.osc
    }
    #[doc = "0x54 - PWM Output Selection Set Update Register"]
    #[inline(always)]
    pub const fn ossupd(&self) -> &Ossupd {
        &self.ossupd
    }
    #[doc = "0x58 - PWM Output Selection Clear Update Register"]
    #[inline(always)]
    pub const fn oscupd(&self) -> &Oscupd {
        &self.oscupd
    }
    #[doc = "0x5c - PWM Fault Mode Register"]
    #[inline(always)]
    pub const fn fmr(&self) -> &Fmr {
        &self.fmr
    }
    #[doc = "0x60 - PWM Fault Status Register"]
    #[inline(always)]
    pub const fn fsr(&self) -> &Fsr {
        &self.fsr
    }
    #[doc = "0x64 - PWM Fault Clear Register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &Fcr {
        &self.fcr
    }
    #[doc = "0x68 - PWM Fault Protection Value Register"]
    #[inline(always)]
    pub const fn fpv(&self) -> &Fpv {
        &self.fpv
    }
    #[doc = "0x6c - PWM Fault Protection Enable Register"]
    #[inline(always)]
    pub const fn fpe(&self) -> &Fpe {
        &self.fpe
    }
    #[doc = "0x7c - PWM Event Line 0 Mode Register 0"]
    #[inline(always)]
    pub const fn elmr0(&self) -> &Elmr0 {
        &self.elmr0
    }
    #[doc = "0x80 - PWM Event Line 0 Mode Register 1"]
    #[inline(always)]
    pub const fn elmr1(&self) -> &Elmr1 {
        &self.elmr1
    }
    #[doc = "0xb0 - PWM Stepper Motor Mode Register"]
    #[inline(always)]
    pub const fn smmr(&self) -> &Smmr {
        &self.smmr
    }
    #[doc = "0xe4 - PWM Write Protect Control Register"]
    #[inline(always)]
    pub const fn wpcr(&self) -> &Wpcr {
        &self.wpcr
    }
    #[doc = "0xe8 - PWM Write Protect Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &Wpsr {
        &self.wpsr
    }
    #[doc = "0x108 - Transmit Pointer Register"]
    #[inline(always)]
    pub const fn tpr(&self) -> &Tpr {
        &self.tpr
    }
    #[doc = "0x10c - Transmit Counter Register"]
    #[inline(always)]
    pub const fn tcr(&self) -> &Tcr {
        &self.tcr
    }
    #[doc = "0x118 - Transmit Next Pointer Register"]
    #[inline(always)]
    pub const fn tnpr(&self) -> &Tnpr {
        &self.tnpr
    }
    #[doc = "0x11c - Transmit Next Counter Register"]
    #[inline(always)]
    pub const fn tncr(&self) -> &Tncr {
        &self.tncr
    }
    #[doc = "0x120 - Transfer Control Register"]
    #[inline(always)]
    pub const fn ptcr(&self) -> &Ptcr {
        &self.ptcr
    }
    #[doc = "0x124 - Transfer Status Register"]
    #[inline(always)]
    pub const fn ptsr(&self) -> &Ptsr {
        &self.ptsr
    }
    #[doc = "0x130 - PWM Comparison 0 Value Register"]
    #[inline(always)]
    pub const fn cmpv0(&self) -> &Cmpv0 {
        &self.cmpv0
    }
    #[doc = "0x134 - PWM Comparison 0 Value Update Register"]
    #[inline(always)]
    pub const fn cmpvupd0(&self) -> &Cmpvupd0 {
        &self.cmpvupd0
    }
    #[doc = "0x138 - PWM Comparison 0 Mode Register"]
    #[inline(always)]
    pub const fn cmpm0(&self) -> &Cmpm0 {
        &self.cmpm0
    }
    #[doc = "0x13c - PWM Comparison 0 Mode Update Register"]
    #[inline(always)]
    pub const fn cmpmupd0(&self) -> &Cmpmupd0 {
        &self.cmpmupd0
    }
    #[doc = "0x140 - PWM Comparison 1 Value Register"]
    #[inline(always)]
    pub const fn cmpv1(&self) -> &Cmpv1 {
        &self.cmpv1
    }
    #[doc = "0x144 - PWM Comparison 1 Value Update Register"]
    #[inline(always)]
    pub const fn cmpvupd1(&self) -> &Cmpvupd1 {
        &self.cmpvupd1
    }
    #[doc = "0x148 - PWM Comparison 1 Mode Register"]
    #[inline(always)]
    pub const fn cmpm1(&self) -> &Cmpm1 {
        &self.cmpm1
    }
    #[doc = "0x14c - PWM Comparison 1 Mode Update Register"]
    #[inline(always)]
    pub const fn cmpmupd1(&self) -> &Cmpmupd1 {
        &self.cmpmupd1
    }
    #[doc = "0x150 - PWM Comparison 2 Value Register"]
    #[inline(always)]
    pub const fn cmpv2(&self) -> &Cmpv2 {
        &self.cmpv2
    }
    #[doc = "0x154 - PWM Comparison 2 Value Update Register"]
    #[inline(always)]
    pub const fn cmpvupd2(&self) -> &Cmpvupd2 {
        &self.cmpvupd2
    }
    #[doc = "0x158 - PWM Comparison 2 Mode Register"]
    #[inline(always)]
    pub const fn cmpm2(&self) -> &Cmpm2 {
        &self.cmpm2
    }
    #[doc = "0x15c - PWM Comparison 2 Mode Update Register"]
    #[inline(always)]
    pub const fn cmpmupd2(&self) -> &Cmpmupd2 {
        &self.cmpmupd2
    }
    #[doc = "0x160 - PWM Comparison 3 Value Register"]
    #[inline(always)]
    pub const fn cmpv3(&self) -> &Cmpv3 {
        &self.cmpv3
    }
    #[doc = "0x164 - PWM Comparison 3 Value Update Register"]
    #[inline(always)]
    pub const fn cmpvupd3(&self) -> &Cmpvupd3 {
        &self.cmpvupd3
    }
    #[doc = "0x168 - PWM Comparison 3 Mode Register"]
    #[inline(always)]
    pub const fn cmpm3(&self) -> &Cmpm3 {
        &self.cmpm3
    }
    #[doc = "0x16c - PWM Comparison 3 Mode Update Register"]
    #[inline(always)]
    pub const fn cmpmupd3(&self) -> &Cmpmupd3 {
        &self.cmpmupd3
    }
    #[doc = "0x170 - PWM Comparison 4 Value Register"]
    #[inline(always)]
    pub const fn cmpv4(&self) -> &Cmpv4 {
        &self.cmpv4
    }
    #[doc = "0x174 - PWM Comparison 4 Value Update Register"]
    #[inline(always)]
    pub const fn cmpvupd4(&self) -> &Cmpvupd4 {
        &self.cmpvupd4
    }
    #[doc = "0x178 - PWM Comparison 4 Mode Register"]
    #[inline(always)]
    pub const fn cmpm4(&self) -> &Cmpm4 {
        &self.cmpm4
    }
    #[doc = "0x17c - PWM Comparison 4 Mode Update Register"]
    #[inline(always)]
    pub const fn cmpmupd4(&self) -> &Cmpmupd4 {
        &self.cmpmupd4
    }
    #[doc = "0x180 - PWM Comparison 5 Value Register"]
    #[inline(always)]
    pub const fn cmpv5(&self) -> &Cmpv5 {
        &self.cmpv5
    }
    #[doc = "0x184 - PWM Comparison 5 Value Update Register"]
    #[inline(always)]
    pub const fn cmpvupd5(&self) -> &Cmpvupd5 {
        &self.cmpvupd5
    }
    #[doc = "0x188 - PWM Comparison 5 Mode Register"]
    #[inline(always)]
    pub const fn cmpm5(&self) -> &Cmpm5 {
        &self.cmpm5
    }
    #[doc = "0x18c - PWM Comparison 5 Mode Update Register"]
    #[inline(always)]
    pub const fn cmpmupd5(&self) -> &Cmpmupd5 {
        &self.cmpmupd5
    }
    #[doc = "0x190 - PWM Comparison 6 Value Register"]
    #[inline(always)]
    pub const fn cmpv6(&self) -> &Cmpv6 {
        &self.cmpv6
    }
    #[doc = "0x194 - PWM Comparison 6 Value Update Register"]
    #[inline(always)]
    pub const fn cmpvupd6(&self) -> &Cmpvupd6 {
        &self.cmpvupd6
    }
    #[doc = "0x198 - PWM Comparison 6 Mode Register"]
    #[inline(always)]
    pub const fn cmpm6(&self) -> &Cmpm6 {
        &self.cmpm6
    }
    #[doc = "0x19c - PWM Comparison 6 Mode Update Register"]
    #[inline(always)]
    pub const fn cmpmupd6(&self) -> &Cmpmupd6 {
        &self.cmpmupd6
    }
    #[doc = "0x1a0 - PWM Comparison 7 Value Register"]
    #[inline(always)]
    pub const fn cmpv7(&self) -> &Cmpv7 {
        &self.cmpv7
    }
    #[doc = "0x1a4 - PWM Comparison 7 Value Update Register"]
    #[inline(always)]
    pub const fn cmpvupd7(&self) -> &Cmpvupd7 {
        &self.cmpvupd7
    }
    #[doc = "0x1a8 - PWM Comparison 7 Mode Register"]
    #[inline(always)]
    pub const fn cmpm7(&self) -> &Cmpm7 {
        &self.cmpm7
    }
    #[doc = "0x1ac - PWM Comparison 7 Mode Update Register"]
    #[inline(always)]
    pub const fn cmpmupd7(&self) -> &Cmpmupd7 {
        &self.cmpmupd7
    }
    #[doc = "0x200 - PWM Channel Mode Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn cmr0(&self) -> &Cmr0 {
        &self.cmr0
    }
    #[doc = "0x204 - PWM Channel Duty Cycle Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn cdty0(&self) -> &Cdty0 {
        &self.cdty0
    }
    #[doc = "0x208 - PWM Channel Duty Cycle Update Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn cdtyupd0(&self) -> &Cdtyupd0 {
        &self.cdtyupd0
    }
    #[doc = "0x20c - PWM Channel Period Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn cprd0(&self) -> &Cprd0 {
        &self.cprd0
    }
    #[doc = "0x210 - PWM Channel Period Update Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn cprdupd0(&self) -> &Cprdupd0 {
        &self.cprdupd0
    }
    #[doc = "0x214 - PWM Channel Counter Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn ccnt0(&self) -> &Ccnt0 {
        &self.ccnt0
    }
    #[doc = "0x218 - PWM Channel Dead Time Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn dt0(&self) -> &Dt0 {
        &self.dt0
    }
    #[doc = "0x21c - PWM Channel Dead Time Update Register (ch_num = 0)"]
    #[inline(always)]
    pub const fn dtupd0(&self) -> &Dtupd0 {
        &self.dtupd0
    }
    #[doc = "0x220 - PWM Channel Mode Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn cmr1(&self) -> &Cmr1 {
        &self.cmr1
    }
    #[doc = "0x224 - PWM Channel Duty Cycle Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn cdty1(&self) -> &Cdty1 {
        &self.cdty1
    }
    #[doc = "0x228 - PWM Channel Duty Cycle Update Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn cdtyupd1(&self) -> &Cdtyupd1 {
        &self.cdtyupd1
    }
    #[doc = "0x22c - PWM Channel Period Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn cprd1(&self) -> &Cprd1 {
        &self.cprd1
    }
    #[doc = "0x230 - PWM Channel Period Update Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn cprdupd1(&self) -> &Cprdupd1 {
        &self.cprdupd1
    }
    #[doc = "0x234 - PWM Channel Counter Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn ccnt1(&self) -> &Ccnt1 {
        &self.ccnt1
    }
    #[doc = "0x238 - PWM Channel Dead Time Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn dt1(&self) -> &Dt1 {
        &self.dt1
    }
    #[doc = "0x23c - PWM Channel Dead Time Update Register (ch_num = 1)"]
    #[inline(always)]
    pub const fn dtupd1(&self) -> &Dtupd1 {
        &self.dtupd1
    }
    #[doc = "0x240 - PWM Channel Mode Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn cmr2(&self) -> &Cmr2 {
        &self.cmr2
    }
    #[doc = "0x244 - PWM Channel Duty Cycle Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn cdty2(&self) -> &Cdty2 {
        &self.cdty2
    }
    #[doc = "0x248 - PWM Channel Duty Cycle Update Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn cdtyupd2(&self) -> &Cdtyupd2 {
        &self.cdtyupd2
    }
    #[doc = "0x24c - PWM Channel Period Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn cprd2(&self) -> &Cprd2 {
        &self.cprd2
    }
    #[doc = "0x250 - PWM Channel Period Update Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn cprdupd2(&self) -> &Cprdupd2 {
        &self.cprdupd2
    }
    #[doc = "0x254 - PWM Channel Counter Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn ccnt2(&self) -> &Ccnt2 {
        &self.ccnt2
    }
    #[doc = "0x258 - PWM Channel Dead Time Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn dt2(&self) -> &Dt2 {
        &self.dt2
    }
    #[doc = "0x25c - PWM Channel Dead Time Update Register (ch_num = 2)"]
    #[inline(always)]
    pub const fn dtupd2(&self) -> &Dtupd2 {
        &self.dtupd2
    }
    #[doc = "0x260 - PWM Channel Mode Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn cmr3(&self) -> &Cmr3 {
        &self.cmr3
    }
    #[doc = "0x264 - PWM Channel Duty Cycle Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn cdty3(&self) -> &Cdty3 {
        &self.cdty3
    }
    #[doc = "0x268 - PWM Channel Duty Cycle Update Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn cdtyupd3(&self) -> &Cdtyupd3 {
        &self.cdtyupd3
    }
    #[doc = "0x26c - PWM Channel Period Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn cprd3(&self) -> &Cprd3 {
        &self.cprd3
    }
    #[doc = "0x270 - PWM Channel Period Update Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn cprdupd3(&self) -> &Cprdupd3 {
        &self.cprdupd3
    }
    #[doc = "0x274 - PWM Channel Counter Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn ccnt3(&self) -> &Ccnt3 {
        &self.ccnt3
    }
    #[doc = "0x278 - PWM Channel Dead Time Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn dt3(&self) -> &Dt3 {
        &self.dt3
    }
    #[doc = "0x27c - PWM Channel Dead Time Update Register (ch_num = 3)"]
    #[inline(always)]
    pub const fn dtupd3(&self) -> &Dtupd3 {
        &self.dtupd3
    }
}
#[doc = "CLK (rw) register accessor: PWM Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk`]
module"]
#[doc(alias = "CLK")]
pub type Clk = crate::Reg<clk::ClkSpec>;
#[doc = "PWM Clock Register"]
pub mod clk;
#[doc = "ENA (w) register accessor: PWM Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ena::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ena`]
module"]
#[doc(alias = "ENA")]
pub type Ena = crate::Reg<ena::EnaSpec>;
#[doc = "PWM Enable Register"]
pub mod ena;
#[doc = "DIS (w) register accessor: PWM Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dis::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dis`]
module"]
#[doc(alias = "DIS")]
pub type Dis = crate::Reg<dis::DisSpec>;
#[doc = "PWM Disable Register"]
pub mod dis;
#[doc = "SR (r) register accessor: PWM Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "PWM Status Register"]
pub mod sr;
#[doc = "IER1 (w) register accessor: PWM Interrupt Enable Register 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier1`]
module"]
#[doc(alias = "IER1")]
pub type Ier1 = crate::Reg<ier1::Ier1Spec>;
#[doc = "PWM Interrupt Enable Register 1"]
pub mod ier1;
#[doc = "IDR1 (w) register accessor: PWM Interrupt Disable Register 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr1`]
module"]
#[doc(alias = "IDR1")]
pub type Idr1 = crate::Reg<idr1::Idr1Spec>;
#[doc = "PWM Interrupt Disable Register 1"]
pub mod idr1;
#[doc = "IMR1 (r) register accessor: PWM Interrupt Mask Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr1`]
module"]
#[doc(alias = "IMR1")]
pub type Imr1 = crate::Reg<imr1::Imr1Spec>;
#[doc = "PWM Interrupt Mask Register 1"]
pub mod imr1;
#[doc = "ISR1 (r) register accessor: PWM Interrupt Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr1`]
module"]
#[doc(alias = "ISR1")]
pub type Isr1 = crate::Reg<isr1::Isr1Spec>;
#[doc = "PWM Interrupt Status Register 1"]
pub mod isr1;
#[doc = "SCM (rw) register accessor: PWM Sync Channels Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scm`]
module"]
#[doc(alias = "SCM")]
pub type Scm = crate::Reg<scm::ScmSpec>;
#[doc = "PWM Sync Channels Mode Register"]
pub mod scm;
#[doc = "SCUC (rw) register accessor: PWM Sync Channels Update Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scuc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scuc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuc`]
module"]
#[doc(alias = "SCUC")]
pub type Scuc = crate::Reg<scuc::ScucSpec>;
#[doc = "PWM Sync Channels Update Control Register"]
pub mod scuc;
#[doc = "SCUP (rw) register accessor: PWM Sync Channels Update Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scup`]
module"]
#[doc(alias = "SCUP")]
pub type Scup = crate::Reg<scup::ScupSpec>;
#[doc = "PWM Sync Channels Update Period Register"]
pub mod scup;
#[doc = "SCUPUPD (w) register accessor: PWM Sync Channels Update Period Update Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scupupd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scupupd`]
module"]
#[doc(alias = "SCUPUPD")]
pub type Scupupd = crate::Reg<scupupd::ScupupdSpec>;
#[doc = "PWM Sync Channels Update Period Update Register"]
pub mod scupupd;
#[doc = "IER2 (w) register accessor: PWM Interrupt Enable Register 2\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier2`]
module"]
#[doc(alias = "IER2")]
pub type Ier2 = crate::Reg<ier2::Ier2Spec>;
#[doc = "PWM Interrupt Enable Register 2"]
pub mod ier2;
#[doc = "IDR2 (w) register accessor: PWM Interrupt Disable Register 2\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr2`]
module"]
#[doc(alias = "IDR2")]
pub type Idr2 = crate::Reg<idr2::Idr2Spec>;
#[doc = "PWM Interrupt Disable Register 2"]
pub mod idr2;
#[doc = "IMR2 (r) register accessor: PWM Interrupt Mask Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr2`]
module"]
#[doc(alias = "IMR2")]
pub type Imr2 = crate::Reg<imr2::Imr2Spec>;
#[doc = "PWM Interrupt Mask Register 2"]
pub mod imr2;
#[doc = "ISR2 (r) register accessor: PWM Interrupt Status Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr2`]
module"]
#[doc(alias = "ISR2")]
pub type Isr2 = crate::Reg<isr2::Isr2Spec>;
#[doc = "PWM Interrupt Status Register 2"]
pub mod isr2;
#[doc = "OOV (rw) register accessor: PWM Output Override Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oov::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oov::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oov`]
module"]
#[doc(alias = "OOV")]
pub type Oov = crate::Reg<oov::OovSpec>;
#[doc = "PWM Output Override Value Register"]
pub mod oov;
#[doc = "OS (rw) register accessor: PWM Output Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`os::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`os::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@os`]
module"]
#[doc(alias = "OS")]
pub type Os = crate::Reg<os::OsSpec>;
#[doc = "PWM Output Selection Register"]
pub mod os;
#[doc = "OSS (w) register accessor: PWM Output Selection Set Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oss::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oss`]
module"]
#[doc(alias = "OSS")]
pub type Oss = crate::Reg<oss::OssSpec>;
#[doc = "PWM Output Selection Set Register"]
pub mod oss;
#[doc = "OSC (w) register accessor: PWM Output Selection Clear Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc`]
module"]
#[doc(alias = "OSC")]
pub type Osc = crate::Reg<osc::OscSpec>;
#[doc = "PWM Output Selection Clear Register"]
pub mod osc;
#[doc = "OSSUPD (w) register accessor: PWM Output Selection Set Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ossupd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ossupd`]
module"]
#[doc(alias = "OSSUPD")]
pub type Ossupd = crate::Reg<ossupd::OssupdSpec>;
#[doc = "PWM Output Selection Set Update Register"]
pub mod ossupd;
#[doc = "OSCUPD (w) register accessor: PWM Output Selection Clear Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscupd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oscupd`]
module"]
#[doc(alias = "OSCUPD")]
pub type Oscupd = crate::Reg<oscupd::OscupdSpec>;
#[doc = "PWM Output Selection Clear Update Register"]
pub mod oscupd;
#[doc = "FMR (rw) register accessor: PWM Fault Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmr`]
module"]
#[doc(alias = "FMR")]
pub type Fmr = crate::Reg<fmr::FmrSpec>;
#[doc = "PWM Fault Mode Register"]
pub mod fmr;
#[doc = "FSR (r) register accessor: PWM Fault Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsr`]
module"]
#[doc(alias = "FSR")]
pub type Fsr = crate::Reg<fsr::FsrSpec>;
#[doc = "PWM Fault Status Register"]
pub mod fsr;
#[doc = "FCR (w) register accessor: PWM Fault Clear Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
#[doc(alias = "FCR")]
pub type Fcr = crate::Reg<fcr::FcrSpec>;
#[doc = "PWM Fault Clear Register"]
pub mod fcr;
#[doc = "FPV (rw) register accessor: PWM Fault Protection Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpv`]
module"]
#[doc(alias = "FPV")]
pub type Fpv = crate::Reg<fpv::FpvSpec>;
#[doc = "PWM Fault Protection Value Register"]
pub mod fpv;
#[doc = "FPE (rw) register accessor: PWM Fault Protection Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpe`]
module"]
#[doc(alias = "FPE")]
pub type Fpe = crate::Reg<fpe::FpeSpec>;
#[doc = "PWM Fault Protection Enable Register"]
pub mod fpe;
#[doc = "ELMR0 (rw) register accessor: PWM Event Line 0 Mode Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`elmr0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`elmr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@elmr0`]
module"]
#[doc(alias = "ELMR0")]
pub type Elmr0 = crate::Reg<elmr0::Elmr0Spec>;
#[doc = "PWM Event Line 0 Mode Register 0"]
pub mod elmr0;
#[doc = "ELMR1 (rw) register accessor: PWM Event Line 0 Mode Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`elmr1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`elmr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@elmr1`]
module"]
#[doc(alias = "ELMR1")]
pub type Elmr1 = crate::Reg<elmr1::Elmr1Spec>;
#[doc = "PWM Event Line 0 Mode Register 1"]
pub mod elmr1;
#[doc = "SMMR (rw) register accessor: PWM Stepper Motor Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smmr`]
module"]
#[doc(alias = "SMMR")]
pub type Smmr = crate::Reg<smmr::SmmrSpec>;
#[doc = "PWM Stepper Motor Mode Register"]
pub mod smmr;
#[doc = "WPCR (w) register accessor: PWM Write Protect Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpcr`]
module"]
#[doc(alias = "WPCR")]
pub type Wpcr = crate::Reg<wpcr::WpcrSpec>;
#[doc = "PWM Write Protect Control Register"]
pub mod wpcr;
#[doc = "WPSR (r) register accessor: PWM Write Protect Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`]
module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "PWM Write Protect Status Register"]
pub mod wpsr;
#[doc = "CMPV0 (rw) register accessor: PWM Comparison 0 Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpv0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpv0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpv0`]
module"]
#[doc(alias = "CMPV0")]
pub type Cmpv0 = crate::Reg<cmpv0::Cmpv0Spec>;
#[doc = "PWM Comparison 0 Value Register"]
pub mod cmpv0;
#[doc = "CMPVUPD0 (w) register accessor: PWM Comparison 0 Value Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpvupd0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpvupd0`]
module"]
#[doc(alias = "CMPVUPD0")]
pub type Cmpvupd0 = crate::Reg<cmpvupd0::Cmpvupd0Spec>;
#[doc = "PWM Comparison 0 Value Update Register"]
pub mod cmpvupd0;
#[doc = "CMPM0 (rw) register accessor: PWM Comparison 0 Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpm0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpm0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpm0`]
module"]
#[doc(alias = "CMPM0")]
pub type Cmpm0 = crate::Reg<cmpm0::Cmpm0Spec>;
#[doc = "PWM Comparison 0 Mode Register"]
pub mod cmpm0;
#[doc = "CMPMUPD0 (w) register accessor: PWM Comparison 0 Mode Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpmupd0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpmupd0`]
module"]
#[doc(alias = "CMPMUPD0")]
pub type Cmpmupd0 = crate::Reg<cmpmupd0::Cmpmupd0Spec>;
#[doc = "PWM Comparison 0 Mode Update Register"]
pub mod cmpmupd0;
#[doc = "CMPV1 (rw) register accessor: PWM Comparison 1 Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpv1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpv1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpv1`]
module"]
#[doc(alias = "CMPV1")]
pub type Cmpv1 = crate::Reg<cmpv1::Cmpv1Spec>;
#[doc = "PWM Comparison 1 Value Register"]
pub mod cmpv1;
#[doc = "CMPVUPD1 (w) register accessor: PWM Comparison 1 Value Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpvupd1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpvupd1`]
module"]
#[doc(alias = "CMPVUPD1")]
pub type Cmpvupd1 = crate::Reg<cmpvupd1::Cmpvupd1Spec>;
#[doc = "PWM Comparison 1 Value Update Register"]
pub mod cmpvupd1;
#[doc = "CMPM1 (rw) register accessor: PWM Comparison 1 Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpm1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpm1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpm1`]
module"]
#[doc(alias = "CMPM1")]
pub type Cmpm1 = crate::Reg<cmpm1::Cmpm1Spec>;
#[doc = "PWM Comparison 1 Mode Register"]
pub mod cmpm1;
#[doc = "CMPMUPD1 (w) register accessor: PWM Comparison 1 Mode Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpmupd1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpmupd1`]
module"]
#[doc(alias = "CMPMUPD1")]
pub type Cmpmupd1 = crate::Reg<cmpmupd1::Cmpmupd1Spec>;
#[doc = "PWM Comparison 1 Mode Update Register"]
pub mod cmpmupd1;
#[doc = "CMPV2 (rw) register accessor: PWM Comparison 2 Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpv2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpv2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpv2`]
module"]
#[doc(alias = "CMPV2")]
pub type Cmpv2 = crate::Reg<cmpv2::Cmpv2Spec>;
#[doc = "PWM Comparison 2 Value Register"]
pub mod cmpv2;
#[doc = "CMPVUPD2 (w) register accessor: PWM Comparison 2 Value Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpvupd2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpvupd2`]
module"]
#[doc(alias = "CMPVUPD2")]
pub type Cmpvupd2 = crate::Reg<cmpvupd2::Cmpvupd2Spec>;
#[doc = "PWM Comparison 2 Value Update Register"]
pub mod cmpvupd2;
#[doc = "CMPM2 (rw) register accessor: PWM Comparison 2 Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpm2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpm2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpm2`]
module"]
#[doc(alias = "CMPM2")]
pub type Cmpm2 = crate::Reg<cmpm2::Cmpm2Spec>;
#[doc = "PWM Comparison 2 Mode Register"]
pub mod cmpm2;
#[doc = "CMPMUPD2 (w) register accessor: PWM Comparison 2 Mode Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpmupd2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpmupd2`]
module"]
#[doc(alias = "CMPMUPD2")]
pub type Cmpmupd2 = crate::Reg<cmpmupd2::Cmpmupd2Spec>;
#[doc = "PWM Comparison 2 Mode Update Register"]
pub mod cmpmupd2;
#[doc = "CMPV3 (rw) register accessor: PWM Comparison 3 Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpv3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpv3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpv3`]
module"]
#[doc(alias = "CMPV3")]
pub type Cmpv3 = crate::Reg<cmpv3::Cmpv3Spec>;
#[doc = "PWM Comparison 3 Value Register"]
pub mod cmpv3;
#[doc = "CMPVUPD3 (w) register accessor: PWM Comparison 3 Value Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpvupd3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpvupd3`]
module"]
#[doc(alias = "CMPVUPD3")]
pub type Cmpvupd3 = crate::Reg<cmpvupd3::Cmpvupd3Spec>;
#[doc = "PWM Comparison 3 Value Update Register"]
pub mod cmpvupd3;
#[doc = "CMPM3 (rw) register accessor: PWM Comparison 3 Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpm3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpm3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpm3`]
module"]
#[doc(alias = "CMPM3")]
pub type Cmpm3 = crate::Reg<cmpm3::Cmpm3Spec>;
#[doc = "PWM Comparison 3 Mode Register"]
pub mod cmpm3;
#[doc = "CMPMUPD3 (w) register accessor: PWM Comparison 3 Mode Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpmupd3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpmupd3`]
module"]
#[doc(alias = "CMPMUPD3")]
pub type Cmpmupd3 = crate::Reg<cmpmupd3::Cmpmupd3Spec>;
#[doc = "PWM Comparison 3 Mode Update Register"]
pub mod cmpmupd3;
#[doc = "CMPV4 (rw) register accessor: PWM Comparison 4 Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpv4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpv4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpv4`]
module"]
#[doc(alias = "CMPV4")]
pub type Cmpv4 = crate::Reg<cmpv4::Cmpv4Spec>;
#[doc = "PWM Comparison 4 Value Register"]
pub mod cmpv4;
#[doc = "CMPVUPD4 (w) register accessor: PWM Comparison 4 Value Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpvupd4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpvupd4`]
module"]
#[doc(alias = "CMPVUPD4")]
pub type Cmpvupd4 = crate::Reg<cmpvupd4::Cmpvupd4Spec>;
#[doc = "PWM Comparison 4 Value Update Register"]
pub mod cmpvupd4;
#[doc = "CMPM4 (rw) register accessor: PWM Comparison 4 Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpm4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpm4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpm4`]
module"]
#[doc(alias = "CMPM4")]
pub type Cmpm4 = crate::Reg<cmpm4::Cmpm4Spec>;
#[doc = "PWM Comparison 4 Mode Register"]
pub mod cmpm4;
#[doc = "CMPMUPD4 (w) register accessor: PWM Comparison 4 Mode Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpmupd4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpmupd4`]
module"]
#[doc(alias = "CMPMUPD4")]
pub type Cmpmupd4 = crate::Reg<cmpmupd4::Cmpmupd4Spec>;
#[doc = "PWM Comparison 4 Mode Update Register"]
pub mod cmpmupd4;
#[doc = "CMPV5 (rw) register accessor: PWM Comparison 5 Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpv5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpv5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpv5`]
module"]
#[doc(alias = "CMPV5")]
pub type Cmpv5 = crate::Reg<cmpv5::Cmpv5Spec>;
#[doc = "PWM Comparison 5 Value Register"]
pub mod cmpv5;
#[doc = "CMPVUPD5 (w) register accessor: PWM Comparison 5 Value Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpvupd5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpvupd5`]
module"]
#[doc(alias = "CMPVUPD5")]
pub type Cmpvupd5 = crate::Reg<cmpvupd5::Cmpvupd5Spec>;
#[doc = "PWM Comparison 5 Value Update Register"]
pub mod cmpvupd5;
#[doc = "CMPM5 (rw) register accessor: PWM Comparison 5 Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpm5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpm5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpm5`]
module"]
#[doc(alias = "CMPM5")]
pub type Cmpm5 = crate::Reg<cmpm5::Cmpm5Spec>;
#[doc = "PWM Comparison 5 Mode Register"]
pub mod cmpm5;
#[doc = "CMPMUPD5 (w) register accessor: PWM Comparison 5 Mode Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpmupd5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpmupd5`]
module"]
#[doc(alias = "CMPMUPD5")]
pub type Cmpmupd5 = crate::Reg<cmpmupd5::Cmpmupd5Spec>;
#[doc = "PWM Comparison 5 Mode Update Register"]
pub mod cmpmupd5;
#[doc = "CMPV6 (rw) register accessor: PWM Comparison 6 Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpv6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpv6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpv6`]
module"]
#[doc(alias = "CMPV6")]
pub type Cmpv6 = crate::Reg<cmpv6::Cmpv6Spec>;
#[doc = "PWM Comparison 6 Value Register"]
pub mod cmpv6;
#[doc = "CMPVUPD6 (w) register accessor: PWM Comparison 6 Value Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpvupd6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpvupd6`]
module"]
#[doc(alias = "CMPVUPD6")]
pub type Cmpvupd6 = crate::Reg<cmpvupd6::Cmpvupd6Spec>;
#[doc = "PWM Comparison 6 Value Update Register"]
pub mod cmpvupd6;
#[doc = "CMPM6 (rw) register accessor: PWM Comparison 6 Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpm6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpm6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpm6`]
module"]
#[doc(alias = "CMPM6")]
pub type Cmpm6 = crate::Reg<cmpm6::Cmpm6Spec>;
#[doc = "PWM Comparison 6 Mode Register"]
pub mod cmpm6;
#[doc = "CMPMUPD6 (w) register accessor: PWM Comparison 6 Mode Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpmupd6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpmupd6`]
module"]
#[doc(alias = "CMPMUPD6")]
pub type Cmpmupd6 = crate::Reg<cmpmupd6::Cmpmupd6Spec>;
#[doc = "PWM Comparison 6 Mode Update Register"]
pub mod cmpmupd6;
#[doc = "CMPV7 (rw) register accessor: PWM Comparison 7 Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpv7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpv7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpv7`]
module"]
#[doc(alias = "CMPV7")]
pub type Cmpv7 = crate::Reg<cmpv7::Cmpv7Spec>;
#[doc = "PWM Comparison 7 Value Register"]
pub mod cmpv7;
#[doc = "CMPVUPD7 (w) register accessor: PWM Comparison 7 Value Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpvupd7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpvupd7`]
module"]
#[doc(alias = "CMPVUPD7")]
pub type Cmpvupd7 = crate::Reg<cmpvupd7::Cmpvupd7Spec>;
#[doc = "PWM Comparison 7 Value Update Register"]
pub mod cmpvupd7;
#[doc = "CMPM7 (rw) register accessor: PWM Comparison 7 Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpm7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpm7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpm7`]
module"]
#[doc(alias = "CMPM7")]
pub type Cmpm7 = crate::Reg<cmpm7::Cmpm7Spec>;
#[doc = "PWM Comparison 7 Mode Register"]
pub mod cmpm7;
#[doc = "CMPMUPD7 (w) register accessor: PWM Comparison 7 Mode Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpmupd7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpmupd7`]
module"]
#[doc(alias = "CMPMUPD7")]
pub type Cmpmupd7 = crate::Reg<cmpmupd7::Cmpmupd7Spec>;
#[doc = "PWM Comparison 7 Mode Update Register"]
pub mod cmpmupd7;
#[doc = "CMR0 (rw) register accessor: PWM Channel Mode Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr0`]
module"]
#[doc(alias = "CMR0")]
pub type Cmr0 = crate::Reg<cmr0::Cmr0Spec>;
#[doc = "PWM Channel Mode Register (ch_num = 0)"]
pub mod cmr0;
#[doc = "CDTY0 (rw) register accessor: PWM Channel Duty Cycle Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdty0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdty0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdty0`]
module"]
#[doc(alias = "CDTY0")]
pub type Cdty0 = crate::Reg<cdty0::Cdty0Spec>;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 0)"]
pub mod cdty0;
#[doc = "CDTYUPD0 (w) register accessor: PWM Channel Duty Cycle Update Register (ch_num = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdtyupd0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdtyupd0`]
module"]
#[doc(alias = "CDTYUPD0")]
pub type Cdtyupd0 = crate::Reg<cdtyupd0::Cdtyupd0Spec>;
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 0)"]
pub mod cdtyupd0;
#[doc = "CPRD0 (rw) register accessor: PWM Channel Period Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cprd0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprd0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cprd0`]
module"]
#[doc(alias = "CPRD0")]
pub type Cprd0 = crate::Reg<cprd0::Cprd0Spec>;
#[doc = "PWM Channel Period Register (ch_num = 0)"]
pub mod cprd0;
#[doc = "CPRDUPD0 (w) register accessor: PWM Channel Period Update Register (ch_num = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprdupd0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cprdupd0`]
module"]
#[doc(alias = "CPRDUPD0")]
pub type Cprdupd0 = crate::Reg<cprdupd0::Cprdupd0Spec>;
#[doc = "PWM Channel Period Update Register (ch_num = 0)"]
pub mod cprdupd0;
#[doc = "CCNT0 (r) register accessor: PWM Channel Counter Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccnt0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccnt0`]
module"]
#[doc(alias = "CCNT0")]
pub type Ccnt0 = crate::Reg<ccnt0::Ccnt0Spec>;
#[doc = "PWM Channel Counter Register (ch_num = 0)"]
pub mod ccnt0;
#[doc = "DT0 (rw) register accessor: PWM Channel Dead Time Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt0`]
module"]
#[doc(alias = "DT0")]
pub type Dt0 = crate::Reg<dt0::Dt0Spec>;
#[doc = "PWM Channel Dead Time Register (ch_num = 0)"]
pub mod dt0;
#[doc = "DTUPD0 (w) register accessor: PWM Channel Dead Time Update Register (ch_num = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtupd0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtupd0`]
module"]
#[doc(alias = "DTUPD0")]
pub type Dtupd0 = crate::Reg<dtupd0::Dtupd0Spec>;
#[doc = "PWM Channel Dead Time Update Register (ch_num = 0)"]
pub mod dtupd0;
#[doc = "CMR1 (rw) register accessor: PWM Channel Mode Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr1`]
module"]
#[doc(alias = "CMR1")]
pub type Cmr1 = crate::Reg<cmr1::Cmr1Spec>;
#[doc = "PWM Channel Mode Register (ch_num = 1)"]
pub mod cmr1;
#[doc = "CDTY1 (rw) register accessor: PWM Channel Duty Cycle Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdty1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdty1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdty1`]
module"]
#[doc(alias = "CDTY1")]
pub type Cdty1 = crate::Reg<cdty1::Cdty1Spec>;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 1)"]
pub mod cdty1;
#[doc = "CDTYUPD1 (w) register accessor: PWM Channel Duty Cycle Update Register (ch_num = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdtyupd1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdtyupd1`]
module"]
#[doc(alias = "CDTYUPD1")]
pub type Cdtyupd1 = crate::Reg<cdtyupd1::Cdtyupd1Spec>;
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 1)"]
pub mod cdtyupd1;
#[doc = "CPRD1 (rw) register accessor: PWM Channel Period Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cprd1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprd1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cprd1`]
module"]
#[doc(alias = "CPRD1")]
pub type Cprd1 = crate::Reg<cprd1::Cprd1Spec>;
#[doc = "PWM Channel Period Register (ch_num = 1)"]
pub mod cprd1;
#[doc = "CPRDUPD1 (w) register accessor: PWM Channel Period Update Register (ch_num = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprdupd1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cprdupd1`]
module"]
#[doc(alias = "CPRDUPD1")]
pub type Cprdupd1 = crate::Reg<cprdupd1::Cprdupd1Spec>;
#[doc = "PWM Channel Period Update Register (ch_num = 1)"]
pub mod cprdupd1;
#[doc = "CCNT1 (r) register accessor: PWM Channel Counter Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccnt1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccnt1`]
module"]
#[doc(alias = "CCNT1")]
pub type Ccnt1 = crate::Reg<ccnt1::Ccnt1Spec>;
#[doc = "PWM Channel Counter Register (ch_num = 1)"]
pub mod ccnt1;
#[doc = "DT1 (rw) register accessor: PWM Channel Dead Time Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt1`]
module"]
#[doc(alias = "DT1")]
pub type Dt1 = crate::Reg<dt1::Dt1Spec>;
#[doc = "PWM Channel Dead Time Register (ch_num = 1)"]
pub mod dt1;
#[doc = "DTUPD1 (w) register accessor: PWM Channel Dead Time Update Register (ch_num = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtupd1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtupd1`]
module"]
#[doc(alias = "DTUPD1")]
pub type Dtupd1 = crate::Reg<dtupd1::Dtupd1Spec>;
#[doc = "PWM Channel Dead Time Update Register (ch_num = 1)"]
pub mod dtupd1;
#[doc = "CMR2 (rw) register accessor: PWM Channel Mode Register (ch_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr2`]
module"]
#[doc(alias = "CMR2")]
pub type Cmr2 = crate::Reg<cmr2::Cmr2Spec>;
#[doc = "PWM Channel Mode Register (ch_num = 2)"]
pub mod cmr2;
#[doc = "CDTY2 (rw) register accessor: PWM Channel Duty Cycle Register (ch_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdty2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdty2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdty2`]
module"]
#[doc(alias = "CDTY2")]
pub type Cdty2 = crate::Reg<cdty2::Cdty2Spec>;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 2)"]
pub mod cdty2;
#[doc = "CDTYUPD2 (w) register accessor: PWM Channel Duty Cycle Update Register (ch_num = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdtyupd2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdtyupd2`]
module"]
#[doc(alias = "CDTYUPD2")]
pub type Cdtyupd2 = crate::Reg<cdtyupd2::Cdtyupd2Spec>;
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 2)"]
pub mod cdtyupd2;
#[doc = "CPRD2 (rw) register accessor: PWM Channel Period Register (ch_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cprd2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprd2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cprd2`]
module"]
#[doc(alias = "CPRD2")]
pub type Cprd2 = crate::Reg<cprd2::Cprd2Spec>;
#[doc = "PWM Channel Period Register (ch_num = 2)"]
pub mod cprd2;
#[doc = "CPRDUPD2 (w) register accessor: PWM Channel Period Update Register (ch_num = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprdupd2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cprdupd2`]
module"]
#[doc(alias = "CPRDUPD2")]
pub type Cprdupd2 = crate::Reg<cprdupd2::Cprdupd2Spec>;
#[doc = "PWM Channel Period Update Register (ch_num = 2)"]
pub mod cprdupd2;
#[doc = "CCNT2 (r) register accessor: PWM Channel Counter Register (ch_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccnt2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccnt2`]
module"]
#[doc(alias = "CCNT2")]
pub type Ccnt2 = crate::Reg<ccnt2::Ccnt2Spec>;
#[doc = "PWM Channel Counter Register (ch_num = 2)"]
pub mod ccnt2;
#[doc = "DT2 (rw) register accessor: PWM Channel Dead Time Register (ch_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt2`]
module"]
#[doc(alias = "DT2")]
pub type Dt2 = crate::Reg<dt2::Dt2Spec>;
#[doc = "PWM Channel Dead Time Register (ch_num = 2)"]
pub mod dt2;
#[doc = "DTUPD2 (w) register accessor: PWM Channel Dead Time Update Register (ch_num = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtupd2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtupd2`]
module"]
#[doc(alias = "DTUPD2")]
pub type Dtupd2 = crate::Reg<dtupd2::Dtupd2Spec>;
#[doc = "PWM Channel Dead Time Update Register (ch_num = 2)"]
pub mod dtupd2;
#[doc = "CMR3 (rw) register accessor: PWM Channel Mode Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr3`]
module"]
#[doc(alias = "CMR3")]
pub type Cmr3 = crate::Reg<cmr3::Cmr3Spec>;
#[doc = "PWM Channel Mode Register (ch_num = 3)"]
pub mod cmr3;
#[doc = "CDTY3 (rw) register accessor: PWM Channel Duty Cycle Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdty3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdty3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdty3`]
module"]
#[doc(alias = "CDTY3")]
pub type Cdty3 = crate::Reg<cdty3::Cdty3Spec>;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 3)"]
pub mod cdty3;
#[doc = "CDTYUPD3 (w) register accessor: PWM Channel Duty Cycle Update Register (ch_num = 3)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdtyupd3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdtyupd3`]
module"]
#[doc(alias = "CDTYUPD3")]
pub type Cdtyupd3 = crate::Reg<cdtyupd3::Cdtyupd3Spec>;
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 3)"]
pub mod cdtyupd3;
#[doc = "CPRD3 (rw) register accessor: PWM Channel Period Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cprd3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprd3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cprd3`]
module"]
#[doc(alias = "CPRD3")]
pub type Cprd3 = crate::Reg<cprd3::Cprd3Spec>;
#[doc = "PWM Channel Period Register (ch_num = 3)"]
pub mod cprd3;
#[doc = "CPRDUPD3 (w) register accessor: PWM Channel Period Update Register (ch_num = 3)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprdupd3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cprdupd3`]
module"]
#[doc(alias = "CPRDUPD3")]
pub type Cprdupd3 = crate::Reg<cprdupd3::Cprdupd3Spec>;
#[doc = "PWM Channel Period Update Register (ch_num = 3)"]
pub mod cprdupd3;
#[doc = "CCNT3 (r) register accessor: PWM Channel Counter Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccnt3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccnt3`]
module"]
#[doc(alias = "CCNT3")]
pub type Ccnt3 = crate::Reg<ccnt3::Ccnt3Spec>;
#[doc = "PWM Channel Counter Register (ch_num = 3)"]
pub mod ccnt3;
#[doc = "DT3 (rw) register accessor: PWM Channel Dead Time Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt3`]
module"]
#[doc(alias = "DT3")]
pub type Dt3 = crate::Reg<dt3::Dt3Spec>;
#[doc = "PWM Channel Dead Time Register (ch_num = 3)"]
pub mod dt3;
#[doc = "DTUPD3 (w) register accessor: PWM Channel Dead Time Update Register (ch_num = 3)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtupd3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtupd3`]
module"]
#[doc(alias = "DTUPD3")]
pub type Dtupd3 = crate::Reg<dtupd3::Dtupd3Spec>;
#[doc = "PWM Channel Dead Time Update Register (ch_num = 3)"]
pub mod dtupd3;
#[doc = "TPR (rw) register accessor: Transmit Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpr`]
module"]
#[doc(alias = "TPR")]
pub type Tpr = crate::Reg<tpr::TprSpec>;
#[doc = "Transmit Pointer Register"]
pub mod tpr;
#[doc = "TCR (rw) register accessor: Transmit Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr`]
module"]
#[doc(alias = "TCR")]
pub type Tcr = crate::Reg<tcr::TcrSpec>;
#[doc = "Transmit Counter Register"]
pub mod tcr;
#[doc = "TNPR (rw) register accessor: Transmit Next Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tnpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tnpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tnpr`]
module"]
#[doc(alias = "TNPR")]
pub type Tnpr = crate::Reg<tnpr::TnprSpec>;
#[doc = "Transmit Next Pointer Register"]
pub mod tnpr;
#[doc = "TNCR (rw) register accessor: Transmit Next Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tncr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tncr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tncr`]
module"]
#[doc(alias = "TNCR")]
pub type Tncr = crate::Reg<tncr::TncrSpec>;
#[doc = "Transmit Next Counter Register"]
pub mod tncr;
#[doc = "PTCR (w) register accessor: Transfer Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptcr`]
module"]
#[doc(alias = "PTCR")]
pub type Ptcr = crate::Reg<ptcr::PtcrSpec>;
#[doc = "Transfer Control Register"]
pub mod ptcr;
#[doc = "PTSR (r) register accessor: Transfer Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptsr`]
module"]
#[doc(alias = "PTSR")]
pub type Ptsr = crate::Reg<ptsr::PtsrSpec>;
#[doc = "Transfer Status Register"]
pub mod ptsr;
