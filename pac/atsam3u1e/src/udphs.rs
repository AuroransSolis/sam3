#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    fnum: Fnum,
    _reserved2: [u8; 0x08],
    ien: Ien,
    intsta: Intsta,
    clrint: Clrint,
    eptrst: Eptrst,
    _reserved6: [u8; 0xc0],
    tst: Tst,
    _reserved7: [u8; 0x1c],
    eptcfg0: Eptcfg0,
    _reserved_8_eptctlenb0: [u8; 0x04],
    _reserved_9_eptctldis0: [u8; 0x04],
    _reserved_10_eptctl0: [u8; 0x04],
    _reserved11: [u8; 0x04],
    _reserved_11_eptsetsta0: [u8; 0x04],
    _reserved_12_eptclrsta0: [u8; 0x04],
    _reserved_13_eptsta0: [u8; 0x04],
    eptcfg1: Eptcfg1,
    _reserved_15_eptctlenb1: [u8; 0x04],
    _reserved_16_eptctldis1: [u8; 0x04],
    _reserved_17_eptctl1: [u8; 0x04],
    _reserved18: [u8; 0x04],
    _reserved_18_eptsetsta1: [u8; 0x04],
    _reserved_19_eptclrsta1: [u8; 0x04],
    _reserved_20_eptsta1: [u8; 0x04],
    eptcfg2: Eptcfg2,
    _reserved_22_eptctlenb2: [u8; 0x04],
    _reserved_23_eptctldis2: [u8; 0x04],
    _reserved_24_eptctl2: [u8; 0x04],
    _reserved25: [u8; 0x04],
    _reserved_25_eptsetsta2: [u8; 0x04],
    _reserved_26_eptclrsta2: [u8; 0x04],
    _reserved_27_eptsta2: [u8; 0x04],
    eptcfg3: Eptcfg3,
    _reserved_29_eptctlenb3: [u8; 0x04],
    _reserved_30_eptctldis3: [u8; 0x04],
    _reserved_31_eptctl3: [u8; 0x04],
    _reserved32: [u8; 0x04],
    _reserved_32_eptsetsta3: [u8; 0x04],
    _reserved_33_eptclrsta3: [u8; 0x04],
    _reserved_34_eptsta3: [u8; 0x04],
    eptcfg4: Eptcfg4,
    _reserved_36_eptctlenb4: [u8; 0x04],
    _reserved_37_eptctldis4: [u8; 0x04],
    _reserved_38_eptctl4: [u8; 0x04],
    _reserved39: [u8; 0x04],
    _reserved_39_eptsetsta4: [u8; 0x04],
    _reserved_40_eptclrsta4: [u8; 0x04],
    _reserved_41_eptsta4: [u8; 0x04],
    eptcfg5: Eptcfg5,
    _reserved_43_eptctlenb5: [u8; 0x04],
    _reserved_44_eptctldis5: [u8; 0x04],
    _reserved_45_eptctl5: [u8; 0x04],
    _reserved46: [u8; 0x04],
    _reserved_46_eptsetsta5: [u8; 0x04],
    _reserved_47_eptclrsta5: [u8; 0x04],
    _reserved_48_eptsta5: [u8; 0x04],
    eptcfg6: Eptcfg6,
    _reserved_50_eptctlenb6: [u8; 0x04],
    _reserved_51_eptctldis6: [u8; 0x04],
    _reserved_52_eptctl6: [u8; 0x04],
    _reserved53: [u8; 0x04],
    _reserved_53_eptsetsta6: [u8; 0x04],
    _reserved_54_eptclrsta6: [u8; 0x04],
    _reserved_55_eptsta6: [u8; 0x04],
    _reserved56: [u8; 0x0120],
    dmanxtdsc0: Dmanxtdsc0,
    dmaaddress0: Dmaaddress0,
    dmacontrol0: Dmacontrol0,
    dmastatus0: Dmastatus0,
    dmanxtdsc1: Dmanxtdsc1,
    dmaaddress1: Dmaaddress1,
    dmacontrol1: Dmacontrol1,
    dmastatus1: Dmastatus1,
    dmanxtdsc2: Dmanxtdsc2,
    dmaaddress2: Dmaaddress2,
    dmacontrol2: Dmacontrol2,
    dmastatus2: Dmastatus2,
    dmanxtdsc3: Dmanxtdsc3,
    dmaaddress3: Dmaaddress3,
    dmacontrol3: Dmacontrol3,
    dmastatus3: Dmastatus3,
    dmanxtdsc4: Dmanxtdsc4,
    dmaaddress4: Dmaaddress4,
    dmacontrol4: Dmacontrol4,
    dmastatus4: Dmastatus4,
    dmanxtdsc5: Dmanxtdsc5,
    dmaaddress5: Dmaaddress5,
    dmacontrol5: Dmacontrol5,
    dmastatus5: Dmastatus5,
}
impl RegisterBlock {
    #[doc = "0x00 - UDPHS Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - UDPHS Frame Number Register"]
    #[inline(always)]
    pub const fn fnum(&self) -> &Fnum {
        &self.fnum
    }
    #[doc = "0x10 - UDPHS Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x14 - UDPHS Interrupt Status Register"]
    #[inline(always)]
    pub const fn intsta(&self) -> &Intsta {
        &self.intsta
    }
    #[doc = "0x18 - UDPHS Clear Interrupt Register"]
    #[inline(always)]
    pub const fn clrint(&self) -> &Clrint {
        &self.clrint
    }
    #[doc = "0x1c - UDPHS Endpoints Reset Register"]
    #[inline(always)]
    pub const fn eptrst(&self) -> &Eptrst {
        &self.eptrst
    }
    #[doc = "0xe0 - UDPHS Test Register"]
    #[inline(always)]
    pub const fn tst(&self) -> &Tst {
        &self.tst
    }
    #[doc = "0x100 - UDPHS Endpoint Configuration Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn eptcfg0(&self) -> &Eptcfg0 {
        &self.eptcfg0
    }
    #[doc = "0x104 - UDPHS Endpoint Control Enable Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn isoendpt_eptctlenb0_isoendpt(&self) -> &IsoendptEptctlenb0Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(260).cast() }
    }
    #[doc = "0x104 - UDPHS Endpoint Control Enable Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn eptctlenb0(&self) -> &Eptctlenb0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(260).cast() }
    }
    #[doc = "0x108 - UDPHS Endpoint Control Disable Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn isoendpt_eptctldis0_isoendpt(&self) -> &IsoendptEptctldis0Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(264).cast() }
    }
    #[doc = "0x108 - UDPHS Endpoint Control Disable Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn eptctldis0(&self) -> &Eptctldis0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(264).cast() }
    }
    #[doc = "0x10c - UDPHS Endpoint Control Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn isoendpt_eptctl0_isoendpt(&self) -> &IsoendptEptctl0Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(268).cast() }
    }
    #[doc = "0x10c - UDPHS Endpoint Control Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn eptctl0(&self) -> &Eptctl0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(268).cast() }
    }
    #[doc = "0x114 - UDPHS Endpoint Set Status Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn isoendpt_eptsetsta0_isoendpt(&self) -> &IsoendptEptsetsta0Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(276).cast() }
    }
    #[doc = "0x114 - UDPHS Endpoint Set Status Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn eptsetsta0(&self) -> &Eptsetsta0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(276).cast() }
    }
    #[doc = "0x118 - UDPHS Endpoint Clear Status Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn isoendpt_eptclrsta0_isoendpt(&self) -> &IsoendptEptclrsta0Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(280).cast() }
    }
    #[doc = "0x118 - UDPHS Endpoint Clear Status Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn eptclrsta0(&self) -> &Eptclrsta0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(280).cast() }
    }
    #[doc = "0x11c - UDPHS Endpoint Status Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn isoendpt_eptsta0_isoendpt(&self) -> &IsoendptEptsta0Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(284).cast() }
    }
    #[doc = "0x11c - UDPHS Endpoint Status Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn eptsta0(&self) -> &Eptsta0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(284).cast() }
    }
    #[doc = "0x120 - UDPHS Endpoint Configuration Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn eptcfg1(&self) -> &Eptcfg1 {
        &self.eptcfg1
    }
    #[doc = "0x124 - UDPHS Endpoint Control Enable Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn isoendpt_eptctlenb1_isoendpt(&self) -> &IsoendptEptctlenb1Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(292).cast() }
    }
    #[doc = "0x124 - UDPHS Endpoint Control Enable Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn eptctlenb1(&self) -> &Eptctlenb1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(292).cast() }
    }
    #[doc = "0x128 - UDPHS Endpoint Control Disable Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn isoendpt_eptctldis1_isoendpt(&self) -> &IsoendptEptctldis1Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(296).cast() }
    }
    #[doc = "0x128 - UDPHS Endpoint Control Disable Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn eptctldis1(&self) -> &Eptctldis1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(296).cast() }
    }
    #[doc = "0x12c - UDPHS Endpoint Control Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn isoendpt_eptctl1_isoendpt(&self) -> &IsoendptEptctl1Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(300).cast() }
    }
    #[doc = "0x12c - UDPHS Endpoint Control Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn eptctl1(&self) -> &Eptctl1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(300).cast() }
    }
    #[doc = "0x134 - UDPHS Endpoint Set Status Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn isoendpt_eptsetsta1_isoendpt(&self) -> &IsoendptEptsetsta1Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(308).cast() }
    }
    #[doc = "0x134 - UDPHS Endpoint Set Status Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn eptsetsta1(&self) -> &Eptsetsta1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(308).cast() }
    }
    #[doc = "0x138 - UDPHS Endpoint Clear Status Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn isoendpt_eptclrsta1_isoendpt(&self) -> &IsoendptEptclrsta1Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(312).cast() }
    }
    #[doc = "0x138 - UDPHS Endpoint Clear Status Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn eptclrsta1(&self) -> &Eptclrsta1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(312).cast() }
    }
    #[doc = "0x13c - UDPHS Endpoint Status Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn isoendpt_eptsta1_isoendpt(&self) -> &IsoendptEptsta1Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(316).cast() }
    }
    #[doc = "0x13c - UDPHS Endpoint Status Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn eptsta1(&self) -> &Eptsta1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(316).cast() }
    }
    #[doc = "0x140 - UDPHS Endpoint Configuration Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn eptcfg2(&self) -> &Eptcfg2 {
        &self.eptcfg2
    }
    #[doc = "0x144 - UDPHS Endpoint Control Enable Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn isoendpt_eptctlenb2_isoendpt(&self) -> &IsoendptEptctlenb2Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(324).cast() }
    }
    #[doc = "0x144 - UDPHS Endpoint Control Enable Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn eptctlenb2(&self) -> &Eptctlenb2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(324).cast() }
    }
    #[doc = "0x148 - UDPHS Endpoint Control Disable Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn isoendpt_eptctldis2_isoendpt(&self) -> &IsoendptEptctldis2Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(328).cast() }
    }
    #[doc = "0x148 - UDPHS Endpoint Control Disable Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn eptctldis2(&self) -> &Eptctldis2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(328).cast() }
    }
    #[doc = "0x14c - UDPHS Endpoint Control Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn isoendpt_eptctl2_isoendpt(&self) -> &IsoendptEptctl2Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(332).cast() }
    }
    #[doc = "0x14c - UDPHS Endpoint Control Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn eptctl2(&self) -> &Eptctl2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(332).cast() }
    }
    #[doc = "0x154 - UDPHS Endpoint Set Status Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn isoendpt_eptsetsta2_isoendpt(&self) -> &IsoendptEptsetsta2Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(340).cast() }
    }
    #[doc = "0x154 - UDPHS Endpoint Set Status Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn eptsetsta2(&self) -> &Eptsetsta2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(340).cast() }
    }
    #[doc = "0x158 - UDPHS Endpoint Clear Status Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn isoendpt_eptclrsta2_isoendpt(&self) -> &IsoendptEptclrsta2Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(344).cast() }
    }
    #[doc = "0x158 - UDPHS Endpoint Clear Status Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn eptclrsta2(&self) -> &Eptclrsta2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(344).cast() }
    }
    #[doc = "0x15c - UDPHS Endpoint Status Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn isoendpt_eptsta2_isoendpt(&self) -> &IsoendptEptsta2Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(348).cast() }
    }
    #[doc = "0x15c - UDPHS Endpoint Status Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn eptsta2(&self) -> &Eptsta2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(348).cast() }
    }
    #[doc = "0x160 - UDPHS Endpoint Configuration Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn eptcfg3(&self) -> &Eptcfg3 {
        &self.eptcfg3
    }
    #[doc = "0x164 - UDPHS Endpoint Control Enable Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn isoendpt_eptctlenb3_isoendpt(&self) -> &IsoendptEptctlenb3Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(356).cast() }
    }
    #[doc = "0x164 - UDPHS Endpoint Control Enable Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn eptctlenb3(&self) -> &Eptctlenb3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(356).cast() }
    }
    #[doc = "0x168 - UDPHS Endpoint Control Disable Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn isoendpt_eptctldis3_isoendpt(&self) -> &IsoendptEptctldis3Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(360).cast() }
    }
    #[doc = "0x168 - UDPHS Endpoint Control Disable Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn eptctldis3(&self) -> &Eptctldis3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(360).cast() }
    }
    #[doc = "0x16c - UDPHS Endpoint Control Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn isoendpt_eptctl3_isoendpt(&self) -> &IsoendptEptctl3Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(364).cast() }
    }
    #[doc = "0x16c - UDPHS Endpoint Control Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn eptctl3(&self) -> &Eptctl3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(364).cast() }
    }
    #[doc = "0x174 - UDPHS Endpoint Set Status Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn isoendpt_eptsetsta3_isoendpt(&self) -> &IsoendptEptsetsta3Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(372).cast() }
    }
    #[doc = "0x174 - UDPHS Endpoint Set Status Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn eptsetsta3(&self) -> &Eptsetsta3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(372).cast() }
    }
    #[doc = "0x178 - UDPHS Endpoint Clear Status Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn isoendpt_eptclrsta3_isoendpt(&self) -> &IsoendptEptclrsta3Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(376).cast() }
    }
    #[doc = "0x178 - UDPHS Endpoint Clear Status Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn eptclrsta3(&self) -> &Eptclrsta3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(376).cast() }
    }
    #[doc = "0x17c - UDPHS Endpoint Status Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn isoendpt_eptsta3_isoendpt(&self) -> &IsoendptEptsta3Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(380).cast() }
    }
    #[doc = "0x17c - UDPHS Endpoint Status Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn eptsta3(&self) -> &Eptsta3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(380).cast() }
    }
    #[doc = "0x180 - UDPHS Endpoint Configuration Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn eptcfg4(&self) -> &Eptcfg4 {
        &self.eptcfg4
    }
    #[doc = "0x184 - UDPHS Endpoint Control Enable Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn isoendpt_eptctlenb4_isoendpt(&self) -> &IsoendptEptctlenb4Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(388).cast() }
    }
    #[doc = "0x184 - UDPHS Endpoint Control Enable Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn eptctlenb4(&self) -> &Eptctlenb4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(388).cast() }
    }
    #[doc = "0x188 - UDPHS Endpoint Control Disable Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn isoendpt_eptctldis4_isoendpt(&self) -> &IsoendptEptctldis4Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(392).cast() }
    }
    #[doc = "0x188 - UDPHS Endpoint Control Disable Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn eptctldis4(&self) -> &Eptctldis4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(392).cast() }
    }
    #[doc = "0x18c - UDPHS Endpoint Control Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn isoendpt_eptctl4_isoendpt(&self) -> &IsoendptEptctl4Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(396).cast() }
    }
    #[doc = "0x18c - UDPHS Endpoint Control Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn eptctl4(&self) -> &Eptctl4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(396).cast() }
    }
    #[doc = "0x194 - UDPHS Endpoint Set Status Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn isoendpt_eptsetsta4_isoendpt(&self) -> &IsoendptEptsetsta4Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(404).cast() }
    }
    #[doc = "0x194 - UDPHS Endpoint Set Status Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn eptsetsta4(&self) -> &Eptsetsta4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(404).cast() }
    }
    #[doc = "0x198 - UDPHS Endpoint Clear Status Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn isoendpt_eptclrsta4_isoendpt(&self) -> &IsoendptEptclrsta4Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(408).cast() }
    }
    #[doc = "0x198 - UDPHS Endpoint Clear Status Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn eptclrsta4(&self) -> &Eptclrsta4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(408).cast() }
    }
    #[doc = "0x19c - UDPHS Endpoint Status Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn isoendpt_eptsta4_isoendpt(&self) -> &IsoendptEptsta4Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(412).cast() }
    }
    #[doc = "0x19c - UDPHS Endpoint Status Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn eptsta4(&self) -> &Eptsta4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(412).cast() }
    }
    #[doc = "0x1a0 - UDPHS Endpoint Configuration Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn eptcfg5(&self) -> &Eptcfg5 {
        &self.eptcfg5
    }
    #[doc = "0x1a4 - UDPHS Endpoint Control Enable Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn isoendpt_eptctlenb5_isoendpt(&self) -> &IsoendptEptctlenb5Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(420).cast() }
    }
    #[doc = "0x1a4 - UDPHS Endpoint Control Enable Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn eptctlenb5(&self) -> &Eptctlenb5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(420).cast() }
    }
    #[doc = "0x1a8 - UDPHS Endpoint Control Disable Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn isoendpt_eptctldis5_isoendpt(&self) -> &IsoendptEptctldis5Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(424).cast() }
    }
    #[doc = "0x1a8 - UDPHS Endpoint Control Disable Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn eptctldis5(&self) -> &Eptctldis5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(424).cast() }
    }
    #[doc = "0x1ac - UDPHS Endpoint Control Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn isoendpt_eptctl5_isoendpt(&self) -> &IsoendptEptctl5Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(428).cast() }
    }
    #[doc = "0x1ac - UDPHS Endpoint Control Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn eptctl5(&self) -> &Eptctl5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(428).cast() }
    }
    #[doc = "0x1b4 - UDPHS Endpoint Set Status Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn isoendpt_eptsetsta5_isoendpt(&self) -> &IsoendptEptsetsta5Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(436).cast() }
    }
    #[doc = "0x1b4 - UDPHS Endpoint Set Status Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn eptsetsta5(&self) -> &Eptsetsta5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(436).cast() }
    }
    #[doc = "0x1b8 - UDPHS Endpoint Clear Status Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn isoendpt_eptclrsta5_isoendpt(&self) -> &IsoendptEptclrsta5Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(440).cast() }
    }
    #[doc = "0x1b8 - UDPHS Endpoint Clear Status Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn eptclrsta5(&self) -> &Eptclrsta5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(440).cast() }
    }
    #[doc = "0x1bc - UDPHS Endpoint Status Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn isoendpt_eptsta5_isoendpt(&self) -> &IsoendptEptsta5Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(444).cast() }
    }
    #[doc = "0x1bc - UDPHS Endpoint Status Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn eptsta5(&self) -> &Eptsta5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(444).cast() }
    }
    #[doc = "0x1c0 - UDPHS Endpoint Configuration Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn eptcfg6(&self) -> &Eptcfg6 {
        &self.eptcfg6
    }
    #[doc = "0x1c4 - UDPHS Endpoint Control Enable Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn isoendpt_eptctlenb6_isoendpt(&self) -> &IsoendptEptctlenb6Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(452).cast() }
    }
    #[doc = "0x1c4 - UDPHS Endpoint Control Enable Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn eptctlenb6(&self) -> &Eptctlenb6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(452).cast() }
    }
    #[doc = "0x1c8 - UDPHS Endpoint Control Disable Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn isoendpt_eptctldis6_isoendpt(&self) -> &IsoendptEptctldis6Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(456).cast() }
    }
    #[doc = "0x1c8 - UDPHS Endpoint Control Disable Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn eptctldis6(&self) -> &Eptctldis6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(456).cast() }
    }
    #[doc = "0x1cc - UDPHS Endpoint Control Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn isoendpt_eptctl6_isoendpt(&self) -> &IsoendptEptctl6Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(460).cast() }
    }
    #[doc = "0x1cc - UDPHS Endpoint Control Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn eptctl6(&self) -> &Eptctl6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(460).cast() }
    }
    #[doc = "0x1d4 - UDPHS Endpoint Set Status Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn isoendpt_eptsetsta6_isoendpt(&self) -> &IsoendptEptsetsta6Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(468).cast() }
    }
    #[doc = "0x1d4 - UDPHS Endpoint Set Status Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn eptsetsta6(&self) -> &Eptsetsta6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(468).cast() }
    }
    #[doc = "0x1d8 - UDPHS Endpoint Clear Status Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn isoendpt_eptclrsta6_isoendpt(&self) -> &IsoendptEptclrsta6Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(472).cast() }
    }
    #[doc = "0x1d8 - UDPHS Endpoint Clear Status Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn eptclrsta6(&self) -> &Eptclrsta6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(472).cast() }
    }
    #[doc = "0x1dc - UDPHS Endpoint Status Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn isoendpt_eptsta6_isoendpt(&self) -> &IsoendptEptsta6Isoendpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(476).cast() }
    }
    #[doc = "0x1dc - UDPHS Endpoint Status Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn eptsta6(&self) -> &Eptsta6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(476).cast() }
    }
    #[doc = "0x300 - UDPHS DMA Next Descriptor Address Register (channel = 0)"]
    #[inline(always)]
    pub const fn dmanxtdsc0(&self) -> &Dmanxtdsc0 {
        &self.dmanxtdsc0
    }
    #[doc = "0x304 - UDPHS DMA Channel Address Register (channel = 0)"]
    #[inline(always)]
    pub const fn dmaaddress0(&self) -> &Dmaaddress0 {
        &self.dmaaddress0
    }
    #[doc = "0x308 - UDPHS DMA Channel Control Register (channel = 0)"]
    #[inline(always)]
    pub const fn dmacontrol0(&self) -> &Dmacontrol0 {
        &self.dmacontrol0
    }
    #[doc = "0x30c - UDPHS DMA Channel Status Register (channel = 0)"]
    #[inline(always)]
    pub const fn dmastatus0(&self) -> &Dmastatus0 {
        &self.dmastatus0
    }
    #[doc = "0x310 - UDPHS DMA Next Descriptor Address Register (channel = 1)"]
    #[inline(always)]
    pub const fn dmanxtdsc1(&self) -> &Dmanxtdsc1 {
        &self.dmanxtdsc1
    }
    #[doc = "0x314 - UDPHS DMA Channel Address Register (channel = 1)"]
    #[inline(always)]
    pub const fn dmaaddress1(&self) -> &Dmaaddress1 {
        &self.dmaaddress1
    }
    #[doc = "0x318 - UDPHS DMA Channel Control Register (channel = 1)"]
    #[inline(always)]
    pub const fn dmacontrol1(&self) -> &Dmacontrol1 {
        &self.dmacontrol1
    }
    #[doc = "0x31c - UDPHS DMA Channel Status Register (channel = 1)"]
    #[inline(always)]
    pub const fn dmastatus1(&self) -> &Dmastatus1 {
        &self.dmastatus1
    }
    #[doc = "0x320 - UDPHS DMA Next Descriptor Address Register (channel = 2)"]
    #[inline(always)]
    pub const fn dmanxtdsc2(&self) -> &Dmanxtdsc2 {
        &self.dmanxtdsc2
    }
    #[doc = "0x324 - UDPHS DMA Channel Address Register (channel = 2)"]
    #[inline(always)]
    pub const fn dmaaddress2(&self) -> &Dmaaddress2 {
        &self.dmaaddress2
    }
    #[doc = "0x328 - UDPHS DMA Channel Control Register (channel = 2)"]
    #[inline(always)]
    pub const fn dmacontrol2(&self) -> &Dmacontrol2 {
        &self.dmacontrol2
    }
    #[doc = "0x32c - UDPHS DMA Channel Status Register (channel = 2)"]
    #[inline(always)]
    pub const fn dmastatus2(&self) -> &Dmastatus2 {
        &self.dmastatus2
    }
    #[doc = "0x330 - UDPHS DMA Next Descriptor Address Register (channel = 3)"]
    #[inline(always)]
    pub const fn dmanxtdsc3(&self) -> &Dmanxtdsc3 {
        &self.dmanxtdsc3
    }
    #[doc = "0x334 - UDPHS DMA Channel Address Register (channel = 3)"]
    #[inline(always)]
    pub const fn dmaaddress3(&self) -> &Dmaaddress3 {
        &self.dmaaddress3
    }
    #[doc = "0x338 - UDPHS DMA Channel Control Register (channel = 3)"]
    #[inline(always)]
    pub const fn dmacontrol3(&self) -> &Dmacontrol3 {
        &self.dmacontrol3
    }
    #[doc = "0x33c - UDPHS DMA Channel Status Register (channel = 3)"]
    #[inline(always)]
    pub const fn dmastatus3(&self) -> &Dmastatus3 {
        &self.dmastatus3
    }
    #[doc = "0x340 - UDPHS DMA Next Descriptor Address Register (channel = 4)"]
    #[inline(always)]
    pub const fn dmanxtdsc4(&self) -> &Dmanxtdsc4 {
        &self.dmanxtdsc4
    }
    #[doc = "0x344 - UDPHS DMA Channel Address Register (channel = 4)"]
    #[inline(always)]
    pub const fn dmaaddress4(&self) -> &Dmaaddress4 {
        &self.dmaaddress4
    }
    #[doc = "0x348 - UDPHS DMA Channel Control Register (channel = 4)"]
    #[inline(always)]
    pub const fn dmacontrol4(&self) -> &Dmacontrol4 {
        &self.dmacontrol4
    }
    #[doc = "0x34c - UDPHS DMA Channel Status Register (channel = 4)"]
    #[inline(always)]
    pub const fn dmastatus4(&self) -> &Dmastatus4 {
        &self.dmastatus4
    }
    #[doc = "0x350 - UDPHS DMA Next Descriptor Address Register (channel = 5)"]
    #[inline(always)]
    pub const fn dmanxtdsc5(&self) -> &Dmanxtdsc5 {
        &self.dmanxtdsc5
    }
    #[doc = "0x354 - UDPHS DMA Channel Address Register (channel = 5)"]
    #[inline(always)]
    pub const fn dmaaddress5(&self) -> &Dmaaddress5 {
        &self.dmaaddress5
    }
    #[doc = "0x358 - UDPHS DMA Channel Control Register (channel = 5)"]
    #[inline(always)]
    pub const fn dmacontrol5(&self) -> &Dmacontrol5 {
        &self.dmacontrol5
    }
    #[doc = "0x35c - UDPHS DMA Channel Status Register (channel = 5)"]
    #[inline(always)]
    pub const fn dmastatus5(&self) -> &Dmastatus5 {
        &self.dmastatus5
    }
}
#[doc = "CTRL (rw) register accessor: UDPHS Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "UDPHS Control Register"]
pub mod ctrl;
#[doc = "FNUM (r) register accessor: UDPHS Frame Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fnum::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fnum`]
module"]
#[doc(alias = "FNUM")]
pub type Fnum = crate::Reg<fnum::FnumSpec>;
#[doc = "UDPHS Frame Number Register"]
pub mod fnum;
#[doc = "IEN (rw) register accessor: UDPHS Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`]
module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IenSpec>;
#[doc = "UDPHS Interrupt Enable Register"]
pub mod ien;
#[doc = "INTSTA (r) register accessor: UDPHS Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intsta::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsta`]
module"]
#[doc(alias = "INTSTA")]
pub type Intsta = crate::Reg<intsta::IntstaSpec>;
#[doc = "UDPHS Interrupt Status Register"]
pub mod intsta;
#[doc = "CLRINT (w) register accessor: UDPHS Clear Interrupt Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clrint::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clrint`]
module"]
#[doc(alias = "CLRINT")]
pub type Clrint = crate::Reg<clrint::ClrintSpec>;
#[doc = "UDPHS Clear Interrupt Register"]
pub mod clrint;
#[doc = "EPTRST (w) register accessor: UDPHS Endpoints Reset Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptrst::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptrst`]
module"]
#[doc(alias = "EPTRST")]
pub type Eptrst = crate::Reg<eptrst::EptrstSpec>;
#[doc = "UDPHS Endpoints Reset Register"]
pub mod eptrst;
#[doc = "TST (rw) register accessor: UDPHS Test Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tst`]
module"]
#[doc(alias = "TST")]
pub type Tst = crate::Reg<tst::TstSpec>;
#[doc = "UDPHS Test Register"]
pub mod tst;
#[doc = "EPTCFG0 (rw) register accessor: UDPHS Endpoint Configuration Register (endpoint = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptcfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptcfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptcfg0`]
module"]
#[doc(alias = "EPTCFG0")]
pub type Eptcfg0 = crate::Reg<eptcfg0::Eptcfg0Spec>;
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 0)"]
pub mod eptcfg0;
#[doc = "EPTCTLENB0 (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptctlenb0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctlenb0`]
module"]
#[doc(alias = "EPTCTLENB0")]
pub type Eptctlenb0 = crate::Reg<eptctlenb0::Eptctlenb0Spec>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 0)"]
pub mod eptctlenb0;
#[doc = "ISOENDPT_EPTCTLENB0_ISOENDPT (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptctlenb0_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctlenb0_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCTLENB0_ISOENDPT")]
pub type IsoendptEptctlenb0Isoendpt =
    crate::Reg<isoendpt_eptctlenb0_isoendpt::IsoendptEptctlenb0IsoendptSpec>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 0)"]
pub mod isoendpt_eptctlenb0_isoendpt;
#[doc = "EPTCTLDIS0 (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptctldis0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctldis0`]
module"]
#[doc(alias = "EPTCTLDIS0")]
pub type Eptctldis0 = crate::Reg<eptctldis0::Eptctldis0Spec>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 0)"]
pub mod eptctldis0;
#[doc = "ISOENDPT_EPTCTLDIS0_ISOENDPT (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptctldis0_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctldis0_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCTLDIS0_ISOENDPT")]
pub type IsoendptEptctldis0Isoendpt =
    crate::Reg<isoendpt_eptctldis0_isoendpt::IsoendptEptctldis0IsoendptSpec>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 0)"]
pub mod isoendpt_eptctldis0_isoendpt;
#[doc = "EPTCTL0 (r) register accessor: UDPHS Endpoint Control Register (endpoint = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptctl0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctl0`]
module"]
#[doc(alias = "EPTCTL0")]
pub type Eptctl0 = crate::Reg<eptctl0::Eptctl0Spec>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 0)"]
pub mod eptctl0;
#[doc = "ISOENDPT_EPTCTL0_ISOENDPT (r) register accessor: UDPHS Endpoint Control Register (endpoint = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`isoendpt_eptctl0_isoendpt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctl0_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCTL0_ISOENDPT")]
pub type IsoendptEptctl0Isoendpt =
    crate::Reg<isoendpt_eptctl0_isoendpt::IsoendptEptctl0IsoendptSpec>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 0)"]
pub mod isoendpt_eptctl0_isoendpt;
#[doc = "EPTSETSTA0 (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptsetsta0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsetsta0`]
module"]
#[doc(alias = "EPTSETSTA0")]
pub type Eptsetsta0 = crate::Reg<eptsetsta0::Eptsetsta0Spec>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 0)"]
pub mod eptsetsta0;
#[doc = "ISOENDPT_EPTSETSTA0_ISOENDPT (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptsetsta0_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsetsta0_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTSETSTA0_ISOENDPT")]
pub type IsoendptEptsetsta0Isoendpt =
    crate::Reg<isoendpt_eptsetsta0_isoendpt::IsoendptEptsetsta0IsoendptSpec>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 0)"]
pub mod isoendpt_eptsetsta0_isoendpt;
#[doc = "EPTCLRSTA0 (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptclrsta0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptclrsta0`]
module"]
#[doc(alias = "EPTCLRSTA0")]
pub type Eptclrsta0 = crate::Reg<eptclrsta0::Eptclrsta0Spec>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 0)"]
pub mod eptclrsta0;
#[doc = "ISOENDPT_EPTCLRSTA0_ISOENDPT (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptclrsta0_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptclrsta0_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCLRSTA0_ISOENDPT")]
pub type IsoendptEptclrsta0Isoendpt =
    crate::Reg<isoendpt_eptclrsta0_isoendpt::IsoendptEptclrsta0IsoendptSpec>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 0)"]
pub mod isoendpt_eptclrsta0_isoendpt;
#[doc = "EPTSTA0 (r) register accessor: UDPHS Endpoint Status Register (endpoint = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptsta0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsta0`]
module"]
#[doc(alias = "EPTSTA0")]
pub type Eptsta0 = crate::Reg<eptsta0::Eptsta0Spec>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 0)"]
pub mod eptsta0;
#[doc = "ISOENDPT_EPTSTA0_ISOENDPT (r) register accessor: UDPHS Endpoint Status Register (endpoint = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`isoendpt_eptsta0_isoendpt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsta0_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTSTA0_ISOENDPT")]
pub type IsoendptEptsta0Isoendpt =
    crate::Reg<isoendpt_eptsta0_isoendpt::IsoendptEptsta0IsoendptSpec>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 0)"]
pub mod isoendpt_eptsta0_isoendpt;
#[doc = "EPTCFG1 (rw) register accessor: UDPHS Endpoint Configuration Register (endpoint = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptcfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptcfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptcfg1`]
module"]
#[doc(alias = "EPTCFG1")]
pub type Eptcfg1 = crate::Reg<eptcfg1::Eptcfg1Spec>;
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 1)"]
pub mod eptcfg1;
#[doc = "EPTCTLENB1 (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 1)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptctlenb1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctlenb1`]
module"]
#[doc(alias = "EPTCTLENB1")]
pub type Eptctlenb1 = crate::Reg<eptctlenb1::Eptctlenb1Spec>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 1)"]
pub mod eptctlenb1;
#[doc = "ISOENDPT_EPTCTLENB1_ISOENDPT (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 1)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptctlenb1_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctlenb1_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCTLENB1_ISOENDPT")]
pub type IsoendptEptctlenb1Isoendpt =
    crate::Reg<isoendpt_eptctlenb1_isoendpt::IsoendptEptctlenb1IsoendptSpec>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 1)"]
pub mod isoendpt_eptctlenb1_isoendpt;
#[doc = "EPTCTLDIS1 (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 1)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptctldis1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctldis1`]
module"]
#[doc(alias = "EPTCTLDIS1")]
pub type Eptctldis1 = crate::Reg<eptctldis1::Eptctldis1Spec>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 1)"]
pub mod eptctldis1;
#[doc = "ISOENDPT_EPTCTLDIS1_ISOENDPT (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 1)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptctldis1_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctldis1_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCTLDIS1_ISOENDPT")]
pub type IsoendptEptctldis1Isoendpt =
    crate::Reg<isoendpt_eptctldis1_isoendpt::IsoendptEptctldis1IsoendptSpec>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 1)"]
pub mod isoendpt_eptctldis1_isoendpt;
#[doc = "EPTCTL1 (r) register accessor: UDPHS Endpoint Control Register (endpoint = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptctl1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctl1`]
module"]
#[doc(alias = "EPTCTL1")]
pub type Eptctl1 = crate::Reg<eptctl1::Eptctl1Spec>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 1)"]
pub mod eptctl1;
#[doc = "ISOENDPT_EPTCTL1_ISOENDPT (r) register accessor: UDPHS Endpoint Control Register (endpoint = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`isoendpt_eptctl1_isoendpt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctl1_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCTL1_ISOENDPT")]
pub type IsoendptEptctl1Isoendpt =
    crate::Reg<isoendpt_eptctl1_isoendpt::IsoendptEptctl1IsoendptSpec>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 1)"]
pub mod isoendpt_eptctl1_isoendpt;
#[doc = "EPTSETSTA1 (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 1)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptsetsta1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsetsta1`]
module"]
#[doc(alias = "EPTSETSTA1")]
pub type Eptsetsta1 = crate::Reg<eptsetsta1::Eptsetsta1Spec>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 1)"]
pub mod eptsetsta1;
#[doc = "ISOENDPT_EPTSETSTA1_ISOENDPT (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 1)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptsetsta1_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsetsta1_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTSETSTA1_ISOENDPT")]
pub type IsoendptEptsetsta1Isoendpt =
    crate::Reg<isoendpt_eptsetsta1_isoendpt::IsoendptEptsetsta1IsoendptSpec>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 1)"]
pub mod isoendpt_eptsetsta1_isoendpt;
#[doc = "EPTCLRSTA1 (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 1)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptclrsta1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptclrsta1`]
module"]
#[doc(alias = "EPTCLRSTA1")]
pub type Eptclrsta1 = crate::Reg<eptclrsta1::Eptclrsta1Spec>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 1)"]
pub mod eptclrsta1;
#[doc = "ISOENDPT_EPTCLRSTA1_ISOENDPT (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 1)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptclrsta1_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptclrsta1_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCLRSTA1_ISOENDPT")]
pub type IsoendptEptclrsta1Isoendpt =
    crate::Reg<isoendpt_eptclrsta1_isoendpt::IsoendptEptclrsta1IsoendptSpec>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 1)"]
pub mod isoendpt_eptclrsta1_isoendpt;
#[doc = "EPTSTA1 (r) register accessor: UDPHS Endpoint Status Register (endpoint = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptsta1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsta1`]
module"]
#[doc(alias = "EPTSTA1")]
pub type Eptsta1 = crate::Reg<eptsta1::Eptsta1Spec>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 1)"]
pub mod eptsta1;
#[doc = "ISOENDPT_EPTSTA1_ISOENDPT (r) register accessor: UDPHS Endpoint Status Register (endpoint = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`isoendpt_eptsta1_isoendpt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsta1_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTSTA1_ISOENDPT")]
pub type IsoendptEptsta1Isoendpt =
    crate::Reg<isoendpt_eptsta1_isoendpt::IsoendptEptsta1IsoendptSpec>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 1)"]
pub mod isoendpt_eptsta1_isoendpt;
#[doc = "EPTCFG2 (rw) register accessor: UDPHS Endpoint Configuration Register (endpoint = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptcfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptcfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptcfg2`]
module"]
#[doc(alias = "EPTCFG2")]
pub type Eptcfg2 = crate::Reg<eptcfg2::Eptcfg2Spec>;
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 2)"]
pub mod eptcfg2;
#[doc = "EPTCTLENB2 (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 2)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptctlenb2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctlenb2`]
module"]
#[doc(alias = "EPTCTLENB2")]
pub type Eptctlenb2 = crate::Reg<eptctlenb2::Eptctlenb2Spec>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 2)"]
pub mod eptctlenb2;
#[doc = "ISOENDPT_EPTCTLENB2_ISOENDPT (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 2)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptctlenb2_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctlenb2_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCTLENB2_ISOENDPT")]
pub type IsoendptEptctlenb2Isoendpt =
    crate::Reg<isoendpt_eptctlenb2_isoendpt::IsoendptEptctlenb2IsoendptSpec>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 2)"]
pub mod isoendpt_eptctlenb2_isoendpt;
#[doc = "EPTCTLDIS2 (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 2)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptctldis2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctldis2`]
module"]
#[doc(alias = "EPTCTLDIS2")]
pub type Eptctldis2 = crate::Reg<eptctldis2::Eptctldis2Spec>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 2)"]
pub mod eptctldis2;
#[doc = "ISOENDPT_EPTCTLDIS2_ISOENDPT (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 2)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptctldis2_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctldis2_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCTLDIS2_ISOENDPT")]
pub type IsoendptEptctldis2Isoendpt =
    crate::Reg<isoendpt_eptctldis2_isoendpt::IsoendptEptctldis2IsoendptSpec>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 2)"]
pub mod isoendpt_eptctldis2_isoendpt;
#[doc = "EPTCTL2 (r) register accessor: UDPHS Endpoint Control Register (endpoint = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptctl2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctl2`]
module"]
#[doc(alias = "EPTCTL2")]
pub type Eptctl2 = crate::Reg<eptctl2::Eptctl2Spec>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 2)"]
pub mod eptctl2;
#[doc = "ISOENDPT_EPTCTL2_ISOENDPT (r) register accessor: UDPHS Endpoint Control Register (endpoint = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`isoendpt_eptctl2_isoendpt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctl2_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCTL2_ISOENDPT")]
pub type IsoendptEptctl2Isoendpt =
    crate::Reg<isoendpt_eptctl2_isoendpt::IsoendptEptctl2IsoendptSpec>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 2)"]
pub mod isoendpt_eptctl2_isoendpt;
#[doc = "EPTSETSTA2 (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 2)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptsetsta2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsetsta2`]
module"]
#[doc(alias = "EPTSETSTA2")]
pub type Eptsetsta2 = crate::Reg<eptsetsta2::Eptsetsta2Spec>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 2)"]
pub mod eptsetsta2;
#[doc = "ISOENDPT_EPTSETSTA2_ISOENDPT (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 2)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptsetsta2_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsetsta2_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTSETSTA2_ISOENDPT")]
pub type IsoendptEptsetsta2Isoendpt =
    crate::Reg<isoendpt_eptsetsta2_isoendpt::IsoendptEptsetsta2IsoendptSpec>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 2)"]
pub mod isoendpt_eptsetsta2_isoendpt;
#[doc = "EPTCLRSTA2 (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 2)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptclrsta2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptclrsta2`]
module"]
#[doc(alias = "EPTCLRSTA2")]
pub type Eptclrsta2 = crate::Reg<eptclrsta2::Eptclrsta2Spec>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 2)"]
pub mod eptclrsta2;
#[doc = "ISOENDPT_EPTCLRSTA2_ISOENDPT (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 2)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptclrsta2_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptclrsta2_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCLRSTA2_ISOENDPT")]
pub type IsoendptEptclrsta2Isoendpt =
    crate::Reg<isoendpt_eptclrsta2_isoendpt::IsoendptEptclrsta2IsoendptSpec>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 2)"]
pub mod isoendpt_eptclrsta2_isoendpt;
#[doc = "EPTSTA2 (r) register accessor: UDPHS Endpoint Status Register (endpoint = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptsta2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsta2`]
module"]
#[doc(alias = "EPTSTA2")]
pub type Eptsta2 = crate::Reg<eptsta2::Eptsta2Spec>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 2)"]
pub mod eptsta2;
#[doc = "ISOENDPT_EPTSTA2_ISOENDPT (r) register accessor: UDPHS Endpoint Status Register (endpoint = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`isoendpt_eptsta2_isoendpt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsta2_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTSTA2_ISOENDPT")]
pub type IsoendptEptsta2Isoendpt =
    crate::Reg<isoendpt_eptsta2_isoendpt::IsoendptEptsta2IsoendptSpec>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 2)"]
pub mod isoendpt_eptsta2_isoendpt;
#[doc = "EPTCFG3 (rw) register accessor: UDPHS Endpoint Configuration Register (endpoint = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptcfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptcfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptcfg3`]
module"]
#[doc(alias = "EPTCFG3")]
pub type Eptcfg3 = crate::Reg<eptcfg3::Eptcfg3Spec>;
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 3)"]
pub mod eptcfg3;
#[doc = "EPTCTLENB3 (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 3)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptctlenb3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctlenb3`]
module"]
#[doc(alias = "EPTCTLENB3")]
pub type Eptctlenb3 = crate::Reg<eptctlenb3::Eptctlenb3Spec>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 3)"]
pub mod eptctlenb3;
#[doc = "ISOENDPT_EPTCTLENB3_ISOENDPT (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 3)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptctlenb3_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctlenb3_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCTLENB3_ISOENDPT")]
pub type IsoendptEptctlenb3Isoendpt =
    crate::Reg<isoendpt_eptctlenb3_isoendpt::IsoendptEptctlenb3IsoendptSpec>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 3)"]
pub mod isoendpt_eptctlenb3_isoendpt;
#[doc = "EPTCTLDIS3 (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 3)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptctldis3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctldis3`]
module"]
#[doc(alias = "EPTCTLDIS3")]
pub type Eptctldis3 = crate::Reg<eptctldis3::Eptctldis3Spec>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 3)"]
pub mod eptctldis3;
#[doc = "ISOENDPT_EPTCTLDIS3_ISOENDPT (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 3)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptctldis3_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctldis3_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCTLDIS3_ISOENDPT")]
pub type IsoendptEptctldis3Isoendpt =
    crate::Reg<isoendpt_eptctldis3_isoendpt::IsoendptEptctldis3IsoendptSpec>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 3)"]
pub mod isoendpt_eptctldis3_isoendpt;
#[doc = "EPTCTL3 (r) register accessor: UDPHS Endpoint Control Register (endpoint = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptctl3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctl3`]
module"]
#[doc(alias = "EPTCTL3")]
pub type Eptctl3 = crate::Reg<eptctl3::Eptctl3Spec>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 3)"]
pub mod eptctl3;
#[doc = "ISOENDPT_EPTCTL3_ISOENDPT (r) register accessor: UDPHS Endpoint Control Register (endpoint = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`isoendpt_eptctl3_isoendpt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctl3_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCTL3_ISOENDPT")]
pub type IsoendptEptctl3Isoendpt =
    crate::Reg<isoendpt_eptctl3_isoendpt::IsoendptEptctl3IsoendptSpec>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 3)"]
pub mod isoendpt_eptctl3_isoendpt;
#[doc = "EPTSETSTA3 (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 3)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptsetsta3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsetsta3`]
module"]
#[doc(alias = "EPTSETSTA3")]
pub type Eptsetsta3 = crate::Reg<eptsetsta3::Eptsetsta3Spec>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 3)"]
pub mod eptsetsta3;
#[doc = "ISOENDPT_EPTSETSTA3_ISOENDPT (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 3)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptsetsta3_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsetsta3_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTSETSTA3_ISOENDPT")]
pub type IsoendptEptsetsta3Isoendpt =
    crate::Reg<isoendpt_eptsetsta3_isoendpt::IsoendptEptsetsta3IsoendptSpec>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 3)"]
pub mod isoendpt_eptsetsta3_isoendpt;
#[doc = "EPTCLRSTA3 (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 3)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptclrsta3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptclrsta3`]
module"]
#[doc(alias = "EPTCLRSTA3")]
pub type Eptclrsta3 = crate::Reg<eptclrsta3::Eptclrsta3Spec>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 3)"]
pub mod eptclrsta3;
#[doc = "ISOENDPT_EPTCLRSTA3_ISOENDPT (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 3)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptclrsta3_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptclrsta3_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCLRSTA3_ISOENDPT")]
pub type IsoendptEptclrsta3Isoendpt =
    crate::Reg<isoendpt_eptclrsta3_isoendpt::IsoendptEptclrsta3IsoendptSpec>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 3)"]
pub mod isoendpt_eptclrsta3_isoendpt;
#[doc = "EPTSTA3 (r) register accessor: UDPHS Endpoint Status Register (endpoint = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptsta3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsta3`]
module"]
#[doc(alias = "EPTSTA3")]
pub type Eptsta3 = crate::Reg<eptsta3::Eptsta3Spec>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 3)"]
pub mod eptsta3;
#[doc = "ISOENDPT_EPTSTA3_ISOENDPT (r) register accessor: UDPHS Endpoint Status Register (endpoint = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`isoendpt_eptsta3_isoendpt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsta3_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTSTA3_ISOENDPT")]
pub type IsoendptEptsta3Isoendpt =
    crate::Reg<isoendpt_eptsta3_isoendpt::IsoendptEptsta3IsoendptSpec>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 3)"]
pub mod isoendpt_eptsta3_isoendpt;
#[doc = "EPTCFG4 (rw) register accessor: UDPHS Endpoint Configuration Register (endpoint = 4)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptcfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptcfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptcfg4`]
module"]
#[doc(alias = "EPTCFG4")]
pub type Eptcfg4 = crate::Reg<eptcfg4::Eptcfg4Spec>;
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 4)"]
pub mod eptcfg4;
#[doc = "EPTCTLENB4 (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 4)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptctlenb4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctlenb4`]
module"]
#[doc(alias = "EPTCTLENB4")]
pub type Eptctlenb4 = crate::Reg<eptctlenb4::Eptctlenb4Spec>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 4)"]
pub mod eptctlenb4;
#[doc = "ISOENDPT_EPTCTLENB4_ISOENDPT (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 4)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptctlenb4_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctlenb4_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCTLENB4_ISOENDPT")]
pub type IsoendptEptctlenb4Isoendpt =
    crate::Reg<isoendpt_eptctlenb4_isoendpt::IsoendptEptctlenb4IsoendptSpec>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 4)"]
pub mod isoendpt_eptctlenb4_isoendpt;
#[doc = "EPTCTLDIS4 (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 4)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptctldis4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctldis4`]
module"]
#[doc(alias = "EPTCTLDIS4")]
pub type Eptctldis4 = crate::Reg<eptctldis4::Eptctldis4Spec>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 4)"]
pub mod eptctldis4;
#[doc = "ISOENDPT_EPTCTLDIS4_ISOENDPT (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 4)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptctldis4_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctldis4_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCTLDIS4_ISOENDPT")]
pub type IsoendptEptctldis4Isoendpt =
    crate::Reg<isoendpt_eptctldis4_isoendpt::IsoendptEptctldis4IsoendptSpec>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 4)"]
pub mod isoendpt_eptctldis4_isoendpt;
#[doc = "EPTCTL4 (r) register accessor: UDPHS Endpoint Control Register (endpoint = 4)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptctl4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctl4`]
module"]
#[doc(alias = "EPTCTL4")]
pub type Eptctl4 = crate::Reg<eptctl4::Eptctl4Spec>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 4)"]
pub mod eptctl4;
#[doc = "ISOENDPT_EPTCTL4_ISOENDPT (r) register accessor: UDPHS Endpoint Control Register (endpoint = 4)\n\nYou can [`read`](crate::Reg::read) this register and get [`isoendpt_eptctl4_isoendpt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctl4_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCTL4_ISOENDPT")]
pub type IsoendptEptctl4Isoendpt =
    crate::Reg<isoendpt_eptctl4_isoendpt::IsoendptEptctl4IsoendptSpec>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 4)"]
pub mod isoendpt_eptctl4_isoendpt;
#[doc = "EPTSETSTA4 (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 4)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptsetsta4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsetsta4`]
module"]
#[doc(alias = "EPTSETSTA4")]
pub type Eptsetsta4 = crate::Reg<eptsetsta4::Eptsetsta4Spec>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 4)"]
pub mod eptsetsta4;
#[doc = "ISOENDPT_EPTSETSTA4_ISOENDPT (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 4)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptsetsta4_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsetsta4_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTSETSTA4_ISOENDPT")]
pub type IsoendptEptsetsta4Isoendpt =
    crate::Reg<isoendpt_eptsetsta4_isoendpt::IsoendptEptsetsta4IsoendptSpec>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 4)"]
pub mod isoendpt_eptsetsta4_isoendpt;
#[doc = "EPTCLRSTA4 (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 4)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptclrsta4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptclrsta4`]
module"]
#[doc(alias = "EPTCLRSTA4")]
pub type Eptclrsta4 = crate::Reg<eptclrsta4::Eptclrsta4Spec>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 4)"]
pub mod eptclrsta4;
#[doc = "ISOENDPT_EPTCLRSTA4_ISOENDPT (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 4)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptclrsta4_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptclrsta4_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCLRSTA4_ISOENDPT")]
pub type IsoendptEptclrsta4Isoendpt =
    crate::Reg<isoendpt_eptclrsta4_isoendpt::IsoendptEptclrsta4IsoendptSpec>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 4)"]
pub mod isoendpt_eptclrsta4_isoendpt;
#[doc = "EPTSTA4 (r) register accessor: UDPHS Endpoint Status Register (endpoint = 4)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptsta4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsta4`]
module"]
#[doc(alias = "EPTSTA4")]
pub type Eptsta4 = crate::Reg<eptsta4::Eptsta4Spec>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 4)"]
pub mod eptsta4;
#[doc = "ISOENDPT_EPTSTA4_ISOENDPT (r) register accessor: UDPHS Endpoint Status Register (endpoint = 4)\n\nYou can [`read`](crate::Reg::read) this register and get [`isoendpt_eptsta4_isoendpt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsta4_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTSTA4_ISOENDPT")]
pub type IsoendptEptsta4Isoendpt =
    crate::Reg<isoendpt_eptsta4_isoendpt::IsoendptEptsta4IsoendptSpec>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 4)"]
pub mod isoendpt_eptsta4_isoendpt;
#[doc = "EPTCFG5 (rw) register accessor: UDPHS Endpoint Configuration Register (endpoint = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptcfg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptcfg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptcfg5`]
module"]
#[doc(alias = "EPTCFG5")]
pub type Eptcfg5 = crate::Reg<eptcfg5::Eptcfg5Spec>;
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 5)"]
pub mod eptcfg5;
#[doc = "EPTCTLENB5 (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 5)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptctlenb5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctlenb5`]
module"]
#[doc(alias = "EPTCTLENB5")]
pub type Eptctlenb5 = crate::Reg<eptctlenb5::Eptctlenb5Spec>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 5)"]
pub mod eptctlenb5;
#[doc = "ISOENDPT_EPTCTLENB5_ISOENDPT (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 5)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptctlenb5_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctlenb5_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCTLENB5_ISOENDPT")]
pub type IsoendptEptctlenb5Isoendpt =
    crate::Reg<isoendpt_eptctlenb5_isoendpt::IsoendptEptctlenb5IsoendptSpec>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 5)"]
pub mod isoendpt_eptctlenb5_isoendpt;
#[doc = "EPTCTLDIS5 (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 5)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptctldis5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctldis5`]
module"]
#[doc(alias = "EPTCTLDIS5")]
pub type Eptctldis5 = crate::Reg<eptctldis5::Eptctldis5Spec>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 5)"]
pub mod eptctldis5;
#[doc = "ISOENDPT_EPTCTLDIS5_ISOENDPT (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 5)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptctldis5_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctldis5_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCTLDIS5_ISOENDPT")]
pub type IsoendptEptctldis5Isoendpt =
    crate::Reg<isoendpt_eptctldis5_isoendpt::IsoendptEptctldis5IsoendptSpec>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 5)"]
pub mod isoendpt_eptctldis5_isoendpt;
#[doc = "EPTCTL5 (r) register accessor: UDPHS Endpoint Control Register (endpoint = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptctl5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctl5`]
module"]
#[doc(alias = "EPTCTL5")]
pub type Eptctl5 = crate::Reg<eptctl5::Eptctl5Spec>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 5)"]
pub mod eptctl5;
#[doc = "ISOENDPT_EPTCTL5_ISOENDPT (r) register accessor: UDPHS Endpoint Control Register (endpoint = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`isoendpt_eptctl5_isoendpt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctl5_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCTL5_ISOENDPT")]
pub type IsoendptEptctl5Isoendpt =
    crate::Reg<isoendpt_eptctl5_isoendpt::IsoendptEptctl5IsoendptSpec>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 5)"]
pub mod isoendpt_eptctl5_isoendpt;
#[doc = "EPTSETSTA5 (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 5)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptsetsta5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsetsta5`]
module"]
#[doc(alias = "EPTSETSTA5")]
pub type Eptsetsta5 = crate::Reg<eptsetsta5::Eptsetsta5Spec>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 5)"]
pub mod eptsetsta5;
#[doc = "ISOENDPT_EPTSETSTA5_ISOENDPT (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 5)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptsetsta5_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsetsta5_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTSETSTA5_ISOENDPT")]
pub type IsoendptEptsetsta5Isoendpt =
    crate::Reg<isoendpt_eptsetsta5_isoendpt::IsoendptEptsetsta5IsoendptSpec>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 5)"]
pub mod isoendpt_eptsetsta5_isoendpt;
#[doc = "EPTCLRSTA5 (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 5)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptclrsta5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptclrsta5`]
module"]
#[doc(alias = "EPTCLRSTA5")]
pub type Eptclrsta5 = crate::Reg<eptclrsta5::Eptclrsta5Spec>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 5)"]
pub mod eptclrsta5;
#[doc = "ISOENDPT_EPTCLRSTA5_ISOENDPT (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 5)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptclrsta5_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptclrsta5_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCLRSTA5_ISOENDPT")]
pub type IsoendptEptclrsta5Isoendpt =
    crate::Reg<isoendpt_eptclrsta5_isoendpt::IsoendptEptclrsta5IsoendptSpec>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 5)"]
pub mod isoendpt_eptclrsta5_isoendpt;
#[doc = "EPTSTA5 (r) register accessor: UDPHS Endpoint Status Register (endpoint = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptsta5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsta5`]
module"]
#[doc(alias = "EPTSTA5")]
pub type Eptsta5 = crate::Reg<eptsta5::Eptsta5Spec>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 5)"]
pub mod eptsta5;
#[doc = "ISOENDPT_EPTSTA5_ISOENDPT (r) register accessor: UDPHS Endpoint Status Register (endpoint = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`isoendpt_eptsta5_isoendpt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsta5_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTSTA5_ISOENDPT")]
pub type IsoendptEptsta5Isoendpt =
    crate::Reg<isoendpt_eptsta5_isoendpt::IsoendptEptsta5IsoendptSpec>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 5)"]
pub mod isoendpt_eptsta5_isoendpt;
#[doc = "EPTCFG6 (rw) register accessor: UDPHS Endpoint Configuration Register (endpoint = 6)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptcfg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptcfg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptcfg6`]
module"]
#[doc(alias = "EPTCFG6")]
pub type Eptcfg6 = crate::Reg<eptcfg6::Eptcfg6Spec>;
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 6)"]
pub mod eptcfg6;
#[doc = "EPTCTLENB6 (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 6)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptctlenb6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctlenb6`]
module"]
#[doc(alias = "EPTCTLENB6")]
pub type Eptctlenb6 = crate::Reg<eptctlenb6::Eptctlenb6Spec>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 6)"]
pub mod eptctlenb6;
#[doc = "ISOENDPT_EPTCTLENB6_ISOENDPT (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 6)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptctlenb6_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctlenb6_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCTLENB6_ISOENDPT")]
pub type IsoendptEptctlenb6Isoendpt =
    crate::Reg<isoendpt_eptctlenb6_isoendpt::IsoendptEptctlenb6IsoendptSpec>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 6)"]
pub mod isoendpt_eptctlenb6_isoendpt;
#[doc = "EPTCTLDIS6 (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 6)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptctldis6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctldis6`]
module"]
#[doc(alias = "EPTCTLDIS6")]
pub type Eptctldis6 = crate::Reg<eptctldis6::Eptctldis6Spec>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 6)"]
pub mod eptctldis6;
#[doc = "ISOENDPT_EPTCTLDIS6_ISOENDPT (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 6)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptctldis6_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctldis6_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCTLDIS6_ISOENDPT")]
pub type IsoendptEptctldis6Isoendpt =
    crate::Reg<isoendpt_eptctldis6_isoendpt::IsoendptEptctldis6IsoendptSpec>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 6)"]
pub mod isoendpt_eptctldis6_isoendpt;
#[doc = "EPTCTL6 (r) register accessor: UDPHS Endpoint Control Register (endpoint = 6)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptctl6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctl6`]
module"]
#[doc(alias = "EPTCTL6")]
pub type Eptctl6 = crate::Reg<eptctl6::Eptctl6Spec>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 6)"]
pub mod eptctl6;
#[doc = "ISOENDPT_EPTCTL6_ISOENDPT (r) register accessor: UDPHS Endpoint Control Register (endpoint = 6)\n\nYou can [`read`](crate::Reg::read) this register and get [`isoendpt_eptctl6_isoendpt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctl6_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCTL6_ISOENDPT")]
pub type IsoendptEptctl6Isoendpt =
    crate::Reg<isoendpt_eptctl6_isoendpt::IsoendptEptctl6IsoendptSpec>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 6)"]
pub mod isoendpt_eptctl6_isoendpt;
#[doc = "EPTSETSTA6 (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 6)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptsetsta6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsetsta6`]
module"]
#[doc(alias = "EPTSETSTA6")]
pub type Eptsetsta6 = crate::Reg<eptsetsta6::Eptsetsta6Spec>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 6)"]
pub mod eptsetsta6;
#[doc = "ISOENDPT_EPTSETSTA6_ISOENDPT (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 6)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptsetsta6_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsetsta6_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTSETSTA6_ISOENDPT")]
pub type IsoendptEptsetsta6Isoendpt =
    crate::Reg<isoendpt_eptsetsta6_isoendpt::IsoendptEptsetsta6IsoendptSpec>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 6)"]
pub mod isoendpt_eptsetsta6_isoendpt;
#[doc = "EPTCLRSTA6 (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 6)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptclrsta6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptclrsta6`]
module"]
#[doc(alias = "EPTCLRSTA6")]
pub type Eptclrsta6 = crate::Reg<eptclrsta6::Eptclrsta6Spec>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 6)"]
pub mod eptclrsta6;
#[doc = "ISOENDPT_EPTCLRSTA6_ISOENDPT (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 6)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoendpt_eptclrsta6_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptclrsta6_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTCLRSTA6_ISOENDPT")]
pub type IsoendptEptclrsta6Isoendpt =
    crate::Reg<isoendpt_eptclrsta6_isoendpt::IsoendptEptclrsta6IsoendptSpec>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 6)"]
pub mod isoendpt_eptclrsta6_isoendpt;
#[doc = "EPTSTA6 (r) register accessor: UDPHS Endpoint Status Register (endpoint = 6)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptsta6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsta6`]
module"]
#[doc(alias = "EPTSTA6")]
pub type Eptsta6 = crate::Reg<eptsta6::Eptsta6Spec>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 6)"]
pub mod eptsta6;
#[doc = "ISOENDPT_EPTSTA6_ISOENDPT (r) register accessor: UDPHS Endpoint Status Register (endpoint = 6)\n\nYou can [`read`](crate::Reg::read) this register and get [`isoendpt_eptsta6_isoendpt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsta6_isoendpt`]
module"]
#[doc(alias = "ISOENDPT_EPTSTA6_ISOENDPT")]
pub type IsoendptEptsta6Isoendpt =
    crate::Reg<isoendpt_eptsta6_isoendpt::IsoendptEptsta6IsoendptSpec>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 6)"]
pub mod isoendpt_eptsta6_isoendpt;
#[doc = "DMANXTDSC0 (rw) register accessor: UDPHS DMA Next Descriptor Address Register (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmanxtdsc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmanxtdsc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmanxtdsc0`]
module"]
#[doc(alias = "DMANXTDSC0")]
pub type Dmanxtdsc0 = crate::Reg<dmanxtdsc0::Dmanxtdsc0Spec>;
#[doc = "UDPHS DMA Next Descriptor Address Register (channel = 0)"]
pub mod dmanxtdsc0;
#[doc = "DMAADDRESS0 (rw) register accessor: UDPHS DMA Channel Address Register (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaaddress0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaaddress0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaaddress0`]
module"]
#[doc(alias = "DMAADDRESS0")]
pub type Dmaaddress0 = crate::Reg<dmaaddress0::Dmaaddress0Spec>;
#[doc = "UDPHS DMA Channel Address Register (channel = 0)"]
pub mod dmaaddress0;
#[doc = "DMACONTROL0 (rw) register accessor: UDPHS DMA Channel Control Register (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacontrol0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacontrol0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacontrol0`]
module"]
#[doc(alias = "DMACONTROL0")]
pub type Dmacontrol0 = crate::Reg<dmacontrol0::Dmacontrol0Spec>;
#[doc = "UDPHS DMA Channel Control Register (channel = 0)"]
pub mod dmacontrol0;
#[doc = "DMASTATUS0 (rw) register accessor: UDPHS DMA Channel Status Register (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmastatus0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmastatus0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmastatus0`]
module"]
#[doc(alias = "DMASTATUS0")]
pub type Dmastatus0 = crate::Reg<dmastatus0::Dmastatus0Spec>;
#[doc = "UDPHS DMA Channel Status Register (channel = 0)"]
pub mod dmastatus0;
#[doc = "DMANXTDSC1 (rw) register accessor: UDPHS DMA Next Descriptor Address Register (channel = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmanxtdsc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmanxtdsc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmanxtdsc1`]
module"]
#[doc(alias = "DMANXTDSC1")]
pub type Dmanxtdsc1 = crate::Reg<dmanxtdsc1::Dmanxtdsc1Spec>;
#[doc = "UDPHS DMA Next Descriptor Address Register (channel = 1)"]
pub mod dmanxtdsc1;
#[doc = "DMAADDRESS1 (rw) register accessor: UDPHS DMA Channel Address Register (channel = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaaddress1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaaddress1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaaddress1`]
module"]
#[doc(alias = "DMAADDRESS1")]
pub type Dmaaddress1 = crate::Reg<dmaaddress1::Dmaaddress1Spec>;
#[doc = "UDPHS DMA Channel Address Register (channel = 1)"]
pub mod dmaaddress1;
#[doc = "DMACONTROL1 (rw) register accessor: UDPHS DMA Channel Control Register (channel = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacontrol1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacontrol1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacontrol1`]
module"]
#[doc(alias = "DMACONTROL1")]
pub type Dmacontrol1 = crate::Reg<dmacontrol1::Dmacontrol1Spec>;
#[doc = "UDPHS DMA Channel Control Register (channel = 1)"]
pub mod dmacontrol1;
#[doc = "DMASTATUS1 (rw) register accessor: UDPHS DMA Channel Status Register (channel = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmastatus1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmastatus1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmastatus1`]
module"]
#[doc(alias = "DMASTATUS1")]
pub type Dmastatus1 = crate::Reg<dmastatus1::Dmastatus1Spec>;
#[doc = "UDPHS DMA Channel Status Register (channel = 1)"]
pub mod dmastatus1;
#[doc = "DMANXTDSC2 (rw) register accessor: UDPHS DMA Next Descriptor Address Register (channel = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmanxtdsc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmanxtdsc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmanxtdsc2`]
module"]
#[doc(alias = "DMANXTDSC2")]
pub type Dmanxtdsc2 = crate::Reg<dmanxtdsc2::Dmanxtdsc2Spec>;
#[doc = "UDPHS DMA Next Descriptor Address Register (channel = 2)"]
pub mod dmanxtdsc2;
#[doc = "DMAADDRESS2 (rw) register accessor: UDPHS DMA Channel Address Register (channel = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaaddress2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaaddress2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaaddress2`]
module"]
#[doc(alias = "DMAADDRESS2")]
pub type Dmaaddress2 = crate::Reg<dmaaddress2::Dmaaddress2Spec>;
#[doc = "UDPHS DMA Channel Address Register (channel = 2)"]
pub mod dmaaddress2;
#[doc = "DMACONTROL2 (rw) register accessor: UDPHS DMA Channel Control Register (channel = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacontrol2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacontrol2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacontrol2`]
module"]
#[doc(alias = "DMACONTROL2")]
pub type Dmacontrol2 = crate::Reg<dmacontrol2::Dmacontrol2Spec>;
#[doc = "UDPHS DMA Channel Control Register (channel = 2)"]
pub mod dmacontrol2;
#[doc = "DMASTATUS2 (rw) register accessor: UDPHS DMA Channel Status Register (channel = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmastatus2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmastatus2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmastatus2`]
module"]
#[doc(alias = "DMASTATUS2")]
pub type Dmastatus2 = crate::Reg<dmastatus2::Dmastatus2Spec>;
#[doc = "UDPHS DMA Channel Status Register (channel = 2)"]
pub mod dmastatus2;
#[doc = "DMANXTDSC3 (rw) register accessor: UDPHS DMA Next Descriptor Address Register (channel = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmanxtdsc3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmanxtdsc3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmanxtdsc3`]
module"]
#[doc(alias = "DMANXTDSC3")]
pub type Dmanxtdsc3 = crate::Reg<dmanxtdsc3::Dmanxtdsc3Spec>;
#[doc = "UDPHS DMA Next Descriptor Address Register (channel = 3)"]
pub mod dmanxtdsc3;
#[doc = "DMAADDRESS3 (rw) register accessor: UDPHS DMA Channel Address Register (channel = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaaddress3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaaddress3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaaddress3`]
module"]
#[doc(alias = "DMAADDRESS3")]
pub type Dmaaddress3 = crate::Reg<dmaaddress3::Dmaaddress3Spec>;
#[doc = "UDPHS DMA Channel Address Register (channel = 3)"]
pub mod dmaaddress3;
#[doc = "DMACONTROL3 (rw) register accessor: UDPHS DMA Channel Control Register (channel = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacontrol3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacontrol3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacontrol3`]
module"]
#[doc(alias = "DMACONTROL3")]
pub type Dmacontrol3 = crate::Reg<dmacontrol3::Dmacontrol3Spec>;
#[doc = "UDPHS DMA Channel Control Register (channel = 3)"]
pub mod dmacontrol3;
#[doc = "DMASTATUS3 (rw) register accessor: UDPHS DMA Channel Status Register (channel = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmastatus3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmastatus3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmastatus3`]
module"]
#[doc(alias = "DMASTATUS3")]
pub type Dmastatus3 = crate::Reg<dmastatus3::Dmastatus3Spec>;
#[doc = "UDPHS DMA Channel Status Register (channel = 3)"]
pub mod dmastatus3;
#[doc = "DMANXTDSC4 (rw) register accessor: UDPHS DMA Next Descriptor Address Register (channel = 4)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmanxtdsc4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmanxtdsc4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmanxtdsc4`]
module"]
#[doc(alias = "DMANXTDSC4")]
pub type Dmanxtdsc4 = crate::Reg<dmanxtdsc4::Dmanxtdsc4Spec>;
#[doc = "UDPHS DMA Next Descriptor Address Register (channel = 4)"]
pub mod dmanxtdsc4;
#[doc = "DMAADDRESS4 (rw) register accessor: UDPHS DMA Channel Address Register (channel = 4)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaaddress4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaaddress4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaaddress4`]
module"]
#[doc(alias = "DMAADDRESS4")]
pub type Dmaaddress4 = crate::Reg<dmaaddress4::Dmaaddress4Spec>;
#[doc = "UDPHS DMA Channel Address Register (channel = 4)"]
pub mod dmaaddress4;
#[doc = "DMACONTROL4 (rw) register accessor: UDPHS DMA Channel Control Register (channel = 4)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacontrol4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacontrol4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacontrol4`]
module"]
#[doc(alias = "DMACONTROL4")]
pub type Dmacontrol4 = crate::Reg<dmacontrol4::Dmacontrol4Spec>;
#[doc = "UDPHS DMA Channel Control Register (channel = 4)"]
pub mod dmacontrol4;
#[doc = "DMASTATUS4 (rw) register accessor: UDPHS DMA Channel Status Register (channel = 4)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmastatus4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmastatus4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmastatus4`]
module"]
#[doc(alias = "DMASTATUS4")]
pub type Dmastatus4 = crate::Reg<dmastatus4::Dmastatus4Spec>;
#[doc = "UDPHS DMA Channel Status Register (channel = 4)"]
pub mod dmastatus4;
#[doc = "DMANXTDSC5 (rw) register accessor: UDPHS DMA Next Descriptor Address Register (channel = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmanxtdsc5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmanxtdsc5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmanxtdsc5`]
module"]
#[doc(alias = "DMANXTDSC5")]
pub type Dmanxtdsc5 = crate::Reg<dmanxtdsc5::Dmanxtdsc5Spec>;
#[doc = "UDPHS DMA Next Descriptor Address Register (channel = 5)"]
pub mod dmanxtdsc5;
#[doc = "DMAADDRESS5 (rw) register accessor: UDPHS DMA Channel Address Register (channel = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaaddress5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaaddress5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaaddress5`]
module"]
#[doc(alias = "DMAADDRESS5")]
pub type Dmaaddress5 = crate::Reg<dmaaddress5::Dmaaddress5Spec>;
#[doc = "UDPHS DMA Channel Address Register (channel = 5)"]
pub mod dmaaddress5;
#[doc = "DMACONTROL5 (rw) register accessor: UDPHS DMA Channel Control Register (channel = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacontrol5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacontrol5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacontrol5`]
module"]
#[doc(alias = "DMACONTROL5")]
pub type Dmacontrol5 = crate::Reg<dmacontrol5::Dmacontrol5Spec>;
#[doc = "UDPHS DMA Channel Control Register (channel = 5)"]
pub mod dmacontrol5;
#[doc = "DMASTATUS5 (rw) register accessor: UDPHS DMA Channel Status Register (channel = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmastatus5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmastatus5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmastatus5`]
module"]
#[doc(alias = "DMASTATUS5")]
pub type Dmastatus5 = crate::Reg<dmastatus5::Dmastatus5Spec>;
#[doc = "UDPHS DMA Channel Status Register (channel = 5)"]
pub mod dmastatus5;
