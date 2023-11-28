#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    fnum: FNUM,
    _reserved2: [u8; 0x08],
    ien: IEN,
    intsta: INTSTA,
    clrint: CLRINT,
    eptrst: EPTRST,
    _reserved6: [u8; 0xc0],
    tst: TST,
    _reserved7: [u8; 0x1c],
    eptcfg0: EPTCFG0,
    _reserved_8_eptctlenb0: [u8; 0x04],
    _reserved_9_eptctldis0: [u8; 0x04],
    _reserved_10_eptctl0: [u8; 0x04],
    _reserved11: [u8; 0x04],
    _reserved_11_eptsetsta0: [u8; 0x04],
    _reserved_12_eptclrsta0: [u8; 0x04],
    _reserved_13_eptsta0: [u8; 0x04],
    eptcfg1: EPTCFG1,
    _reserved_15_eptctlenb1: [u8; 0x04],
    _reserved_16_eptctldis1: [u8; 0x04],
    _reserved_17_eptctl1: [u8; 0x04],
    _reserved18: [u8; 0x04],
    _reserved_18_eptsetsta1: [u8; 0x04],
    _reserved_19_eptclrsta1: [u8; 0x04],
    _reserved_20_eptsta1: [u8; 0x04],
    eptcfg2: EPTCFG2,
    _reserved_22_eptctlenb2: [u8; 0x04],
    _reserved_23_eptctldis2: [u8; 0x04],
    _reserved_24_eptctl2: [u8; 0x04],
    _reserved25: [u8; 0x04],
    _reserved_25_eptsetsta2: [u8; 0x04],
    _reserved_26_eptclrsta2: [u8; 0x04],
    _reserved_27_eptsta2: [u8; 0x04],
    eptcfg3: EPTCFG3,
    _reserved_29_eptctlenb3: [u8; 0x04],
    _reserved_30_eptctldis3: [u8; 0x04],
    _reserved_31_eptctl3: [u8; 0x04],
    _reserved32: [u8; 0x04],
    _reserved_32_eptsetsta3: [u8; 0x04],
    _reserved_33_eptclrsta3: [u8; 0x04],
    _reserved_34_eptsta3: [u8; 0x04],
    eptcfg4: EPTCFG4,
    _reserved_36_eptctlenb4: [u8; 0x04],
    _reserved_37_eptctldis4: [u8; 0x04],
    _reserved_38_eptctl4: [u8; 0x04],
    _reserved39: [u8; 0x04],
    _reserved_39_eptsetsta4: [u8; 0x04],
    _reserved_40_eptclrsta4: [u8; 0x04],
    _reserved_41_eptsta4: [u8; 0x04],
    eptcfg5: EPTCFG5,
    _reserved_43_eptctlenb5: [u8; 0x04],
    _reserved_44_eptctldis5: [u8; 0x04],
    _reserved_45_eptctl5: [u8; 0x04],
    _reserved46: [u8; 0x04],
    _reserved_46_eptsetsta5: [u8; 0x04],
    _reserved_47_eptclrsta5: [u8; 0x04],
    _reserved_48_eptsta5: [u8; 0x04],
    eptcfg6: EPTCFG6,
    _reserved_50_eptctlenb6: [u8; 0x04],
    _reserved_51_eptctldis6: [u8; 0x04],
    _reserved_52_eptctl6: [u8; 0x04],
    _reserved53: [u8; 0x04],
    _reserved_53_eptsetsta6: [u8; 0x04],
    _reserved_54_eptclrsta6: [u8; 0x04],
    _reserved_55_eptsta6: [u8; 0x04],
    _reserved56: [u8; 0x0120],
    dmanxtdsc0: DMANXTDSC0,
    dmaaddress0: DMAADDRESS0,
    dmacontrol0: DMACONTROL0,
    dmastatus0: DMASTATUS0,
    dmanxtdsc1: DMANXTDSC1,
    dmaaddress1: DMAADDRESS1,
    dmacontrol1: DMACONTROL1,
    dmastatus1: DMASTATUS1,
    dmanxtdsc2: DMANXTDSC2,
    dmaaddress2: DMAADDRESS2,
    dmacontrol2: DMACONTROL2,
    dmastatus2: DMASTATUS2,
    dmanxtdsc3: DMANXTDSC3,
    dmaaddress3: DMAADDRESS3,
    dmacontrol3: DMACONTROL3,
    dmastatus3: DMASTATUS3,
    dmanxtdsc4: DMANXTDSC4,
    dmaaddress4: DMAADDRESS4,
    dmacontrol4: DMACONTROL4,
    dmastatus4: DMASTATUS4,
    dmanxtdsc5: DMANXTDSC5,
    dmaaddress5: DMAADDRESS5,
    dmacontrol5: DMACONTROL5,
    dmastatus5: DMASTATUS5,
}
impl RegisterBlock {
    #[doc = "0x00 - UDPHS Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - UDPHS Frame Number Register"]
    #[inline(always)]
    pub const fn fnum(&self) -> &FNUM {
        &self.fnum
    }
    #[doc = "0x10 - UDPHS Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &IEN {
        &self.ien
    }
    #[doc = "0x14 - UDPHS Interrupt Status Register"]
    #[inline(always)]
    pub const fn intsta(&self) -> &INTSTA {
        &self.intsta
    }
    #[doc = "0x18 - UDPHS Clear Interrupt Register"]
    #[inline(always)]
    pub const fn clrint(&self) -> &CLRINT {
        &self.clrint
    }
    #[doc = "0x1c - UDPHS Endpoints Reset Register"]
    #[inline(always)]
    pub const fn eptrst(&self) -> &EPTRST {
        &self.eptrst
    }
    #[doc = "0xe0 - UDPHS Test Register"]
    #[inline(always)]
    pub const fn tst(&self) -> &TST {
        &self.tst
    }
    #[doc = "0x100 - UDPHS Endpoint Configuration Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn eptcfg0(&self) -> &EPTCFG0 {
        &self.eptcfg0
    }
    #[doc = "0x104 - UDPHS Endpoint Control Enable Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn isoendpt_eptctlenb0_isoendpt(&self) -> &ISOENDPT_EPTCTLENB0_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(260).cast() }
    }
    #[doc = "0x104 - UDPHS Endpoint Control Enable Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn eptctlenb0(&self) -> &EPTCTLENB0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(260).cast() }
    }
    #[doc = "0x108 - UDPHS Endpoint Control Disable Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn isoendpt_eptctldis0_isoendpt(&self) -> &ISOENDPT_EPTCTLDIS0_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(264).cast() }
    }
    #[doc = "0x108 - UDPHS Endpoint Control Disable Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn eptctldis0(&self) -> &EPTCTLDIS0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(264).cast() }
    }
    #[doc = "0x10c - UDPHS Endpoint Control Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn isoendpt_eptctl0_isoendpt(&self) -> &ISOENDPT_EPTCTL0_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(268).cast() }
    }
    #[doc = "0x10c - UDPHS Endpoint Control Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn eptctl0(&self) -> &EPTCTL0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(268).cast() }
    }
    #[doc = "0x114 - UDPHS Endpoint Set Status Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn isoendpt_eptsetsta0_isoendpt(&self) -> &ISOENDPT_EPTSETSTA0_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(276).cast() }
    }
    #[doc = "0x114 - UDPHS Endpoint Set Status Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn eptsetsta0(&self) -> &EPTSETSTA0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(276).cast() }
    }
    #[doc = "0x118 - UDPHS Endpoint Clear Status Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn isoendpt_eptclrsta0_isoendpt(&self) -> &ISOENDPT_EPTCLRSTA0_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(280).cast() }
    }
    #[doc = "0x118 - UDPHS Endpoint Clear Status Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn eptclrsta0(&self) -> &EPTCLRSTA0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(280).cast() }
    }
    #[doc = "0x11c - UDPHS Endpoint Status Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn isoendpt_eptsta0_isoendpt(&self) -> &ISOENDPT_EPTSTA0_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(284).cast() }
    }
    #[doc = "0x11c - UDPHS Endpoint Status Register (endpoint = 0)"]
    #[inline(always)]
    pub const fn eptsta0(&self) -> &EPTSTA0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(284).cast() }
    }
    #[doc = "0x120 - UDPHS Endpoint Configuration Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn eptcfg1(&self) -> &EPTCFG1 {
        &self.eptcfg1
    }
    #[doc = "0x124 - UDPHS Endpoint Control Enable Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn isoendpt_eptctlenb1_isoendpt(&self) -> &ISOENDPT_EPTCTLENB1_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(292).cast() }
    }
    #[doc = "0x124 - UDPHS Endpoint Control Enable Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn eptctlenb1(&self) -> &EPTCTLENB1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(292).cast() }
    }
    #[doc = "0x128 - UDPHS Endpoint Control Disable Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn isoendpt_eptctldis1_isoendpt(&self) -> &ISOENDPT_EPTCTLDIS1_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(296).cast() }
    }
    #[doc = "0x128 - UDPHS Endpoint Control Disable Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn eptctldis1(&self) -> &EPTCTLDIS1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(296).cast() }
    }
    #[doc = "0x12c - UDPHS Endpoint Control Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn isoendpt_eptctl1_isoendpt(&self) -> &ISOENDPT_EPTCTL1_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(300).cast() }
    }
    #[doc = "0x12c - UDPHS Endpoint Control Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn eptctl1(&self) -> &EPTCTL1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(300).cast() }
    }
    #[doc = "0x134 - UDPHS Endpoint Set Status Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn isoendpt_eptsetsta1_isoendpt(&self) -> &ISOENDPT_EPTSETSTA1_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(308).cast() }
    }
    #[doc = "0x134 - UDPHS Endpoint Set Status Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn eptsetsta1(&self) -> &EPTSETSTA1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(308).cast() }
    }
    #[doc = "0x138 - UDPHS Endpoint Clear Status Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn isoendpt_eptclrsta1_isoendpt(&self) -> &ISOENDPT_EPTCLRSTA1_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(312).cast() }
    }
    #[doc = "0x138 - UDPHS Endpoint Clear Status Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn eptclrsta1(&self) -> &EPTCLRSTA1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(312).cast() }
    }
    #[doc = "0x13c - UDPHS Endpoint Status Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn isoendpt_eptsta1_isoendpt(&self) -> &ISOENDPT_EPTSTA1_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(316).cast() }
    }
    #[doc = "0x13c - UDPHS Endpoint Status Register (endpoint = 1)"]
    #[inline(always)]
    pub const fn eptsta1(&self) -> &EPTSTA1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(316).cast() }
    }
    #[doc = "0x140 - UDPHS Endpoint Configuration Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn eptcfg2(&self) -> &EPTCFG2 {
        &self.eptcfg2
    }
    #[doc = "0x144 - UDPHS Endpoint Control Enable Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn isoendpt_eptctlenb2_isoendpt(&self) -> &ISOENDPT_EPTCTLENB2_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(324).cast() }
    }
    #[doc = "0x144 - UDPHS Endpoint Control Enable Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn eptctlenb2(&self) -> &EPTCTLENB2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(324).cast() }
    }
    #[doc = "0x148 - UDPHS Endpoint Control Disable Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn isoendpt_eptctldis2_isoendpt(&self) -> &ISOENDPT_EPTCTLDIS2_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(328).cast() }
    }
    #[doc = "0x148 - UDPHS Endpoint Control Disable Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn eptctldis2(&self) -> &EPTCTLDIS2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(328).cast() }
    }
    #[doc = "0x14c - UDPHS Endpoint Control Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn isoendpt_eptctl2_isoendpt(&self) -> &ISOENDPT_EPTCTL2_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(332).cast() }
    }
    #[doc = "0x14c - UDPHS Endpoint Control Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn eptctl2(&self) -> &EPTCTL2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(332).cast() }
    }
    #[doc = "0x154 - UDPHS Endpoint Set Status Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn isoendpt_eptsetsta2_isoendpt(&self) -> &ISOENDPT_EPTSETSTA2_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(340).cast() }
    }
    #[doc = "0x154 - UDPHS Endpoint Set Status Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn eptsetsta2(&self) -> &EPTSETSTA2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(340).cast() }
    }
    #[doc = "0x158 - UDPHS Endpoint Clear Status Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn isoendpt_eptclrsta2_isoendpt(&self) -> &ISOENDPT_EPTCLRSTA2_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(344).cast() }
    }
    #[doc = "0x158 - UDPHS Endpoint Clear Status Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn eptclrsta2(&self) -> &EPTCLRSTA2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(344).cast() }
    }
    #[doc = "0x15c - UDPHS Endpoint Status Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn isoendpt_eptsta2_isoendpt(&self) -> &ISOENDPT_EPTSTA2_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(348).cast() }
    }
    #[doc = "0x15c - UDPHS Endpoint Status Register (endpoint = 2)"]
    #[inline(always)]
    pub const fn eptsta2(&self) -> &EPTSTA2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(348).cast() }
    }
    #[doc = "0x160 - UDPHS Endpoint Configuration Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn eptcfg3(&self) -> &EPTCFG3 {
        &self.eptcfg3
    }
    #[doc = "0x164 - UDPHS Endpoint Control Enable Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn isoendpt_eptctlenb3_isoendpt(&self) -> &ISOENDPT_EPTCTLENB3_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(356).cast() }
    }
    #[doc = "0x164 - UDPHS Endpoint Control Enable Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn eptctlenb3(&self) -> &EPTCTLENB3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(356).cast() }
    }
    #[doc = "0x168 - UDPHS Endpoint Control Disable Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn isoendpt_eptctldis3_isoendpt(&self) -> &ISOENDPT_EPTCTLDIS3_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(360).cast() }
    }
    #[doc = "0x168 - UDPHS Endpoint Control Disable Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn eptctldis3(&self) -> &EPTCTLDIS3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(360).cast() }
    }
    #[doc = "0x16c - UDPHS Endpoint Control Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn isoendpt_eptctl3_isoendpt(&self) -> &ISOENDPT_EPTCTL3_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(364).cast() }
    }
    #[doc = "0x16c - UDPHS Endpoint Control Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn eptctl3(&self) -> &EPTCTL3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(364).cast() }
    }
    #[doc = "0x174 - UDPHS Endpoint Set Status Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn isoendpt_eptsetsta3_isoendpt(&self) -> &ISOENDPT_EPTSETSTA3_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(372).cast() }
    }
    #[doc = "0x174 - UDPHS Endpoint Set Status Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn eptsetsta3(&self) -> &EPTSETSTA3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(372).cast() }
    }
    #[doc = "0x178 - UDPHS Endpoint Clear Status Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn isoendpt_eptclrsta3_isoendpt(&self) -> &ISOENDPT_EPTCLRSTA3_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(376).cast() }
    }
    #[doc = "0x178 - UDPHS Endpoint Clear Status Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn eptclrsta3(&self) -> &EPTCLRSTA3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(376).cast() }
    }
    #[doc = "0x17c - UDPHS Endpoint Status Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn isoendpt_eptsta3_isoendpt(&self) -> &ISOENDPT_EPTSTA3_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(380).cast() }
    }
    #[doc = "0x17c - UDPHS Endpoint Status Register (endpoint = 3)"]
    #[inline(always)]
    pub const fn eptsta3(&self) -> &EPTSTA3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(380).cast() }
    }
    #[doc = "0x180 - UDPHS Endpoint Configuration Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn eptcfg4(&self) -> &EPTCFG4 {
        &self.eptcfg4
    }
    #[doc = "0x184 - UDPHS Endpoint Control Enable Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn isoendpt_eptctlenb4_isoendpt(&self) -> &ISOENDPT_EPTCTLENB4_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(388).cast() }
    }
    #[doc = "0x184 - UDPHS Endpoint Control Enable Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn eptctlenb4(&self) -> &EPTCTLENB4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(388).cast() }
    }
    #[doc = "0x188 - UDPHS Endpoint Control Disable Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn isoendpt_eptctldis4_isoendpt(&self) -> &ISOENDPT_EPTCTLDIS4_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(392).cast() }
    }
    #[doc = "0x188 - UDPHS Endpoint Control Disable Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn eptctldis4(&self) -> &EPTCTLDIS4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(392).cast() }
    }
    #[doc = "0x18c - UDPHS Endpoint Control Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn isoendpt_eptctl4_isoendpt(&self) -> &ISOENDPT_EPTCTL4_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(396).cast() }
    }
    #[doc = "0x18c - UDPHS Endpoint Control Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn eptctl4(&self) -> &EPTCTL4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(396).cast() }
    }
    #[doc = "0x194 - UDPHS Endpoint Set Status Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn isoendpt_eptsetsta4_isoendpt(&self) -> &ISOENDPT_EPTSETSTA4_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(404).cast() }
    }
    #[doc = "0x194 - UDPHS Endpoint Set Status Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn eptsetsta4(&self) -> &EPTSETSTA4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(404).cast() }
    }
    #[doc = "0x198 - UDPHS Endpoint Clear Status Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn isoendpt_eptclrsta4_isoendpt(&self) -> &ISOENDPT_EPTCLRSTA4_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(408).cast() }
    }
    #[doc = "0x198 - UDPHS Endpoint Clear Status Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn eptclrsta4(&self) -> &EPTCLRSTA4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(408).cast() }
    }
    #[doc = "0x19c - UDPHS Endpoint Status Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn isoendpt_eptsta4_isoendpt(&self) -> &ISOENDPT_EPTSTA4_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(412).cast() }
    }
    #[doc = "0x19c - UDPHS Endpoint Status Register (endpoint = 4)"]
    #[inline(always)]
    pub const fn eptsta4(&self) -> &EPTSTA4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(412).cast() }
    }
    #[doc = "0x1a0 - UDPHS Endpoint Configuration Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn eptcfg5(&self) -> &EPTCFG5 {
        &self.eptcfg5
    }
    #[doc = "0x1a4 - UDPHS Endpoint Control Enable Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn isoendpt_eptctlenb5_isoendpt(&self) -> &ISOENDPT_EPTCTLENB5_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(420).cast() }
    }
    #[doc = "0x1a4 - UDPHS Endpoint Control Enable Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn eptctlenb5(&self) -> &EPTCTLENB5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(420).cast() }
    }
    #[doc = "0x1a8 - UDPHS Endpoint Control Disable Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn isoendpt_eptctldis5_isoendpt(&self) -> &ISOENDPT_EPTCTLDIS5_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(424).cast() }
    }
    #[doc = "0x1a8 - UDPHS Endpoint Control Disable Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn eptctldis5(&self) -> &EPTCTLDIS5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(424).cast() }
    }
    #[doc = "0x1ac - UDPHS Endpoint Control Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn isoendpt_eptctl5_isoendpt(&self) -> &ISOENDPT_EPTCTL5_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(428).cast() }
    }
    #[doc = "0x1ac - UDPHS Endpoint Control Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn eptctl5(&self) -> &EPTCTL5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(428).cast() }
    }
    #[doc = "0x1b4 - UDPHS Endpoint Set Status Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn isoendpt_eptsetsta5_isoendpt(&self) -> &ISOENDPT_EPTSETSTA5_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(436).cast() }
    }
    #[doc = "0x1b4 - UDPHS Endpoint Set Status Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn eptsetsta5(&self) -> &EPTSETSTA5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(436).cast() }
    }
    #[doc = "0x1b8 - UDPHS Endpoint Clear Status Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn isoendpt_eptclrsta5_isoendpt(&self) -> &ISOENDPT_EPTCLRSTA5_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(440).cast() }
    }
    #[doc = "0x1b8 - UDPHS Endpoint Clear Status Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn eptclrsta5(&self) -> &EPTCLRSTA5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(440).cast() }
    }
    #[doc = "0x1bc - UDPHS Endpoint Status Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn isoendpt_eptsta5_isoendpt(&self) -> &ISOENDPT_EPTSTA5_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(444).cast() }
    }
    #[doc = "0x1bc - UDPHS Endpoint Status Register (endpoint = 5)"]
    #[inline(always)]
    pub const fn eptsta5(&self) -> &EPTSTA5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(444).cast() }
    }
    #[doc = "0x1c0 - UDPHS Endpoint Configuration Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn eptcfg6(&self) -> &EPTCFG6 {
        &self.eptcfg6
    }
    #[doc = "0x1c4 - UDPHS Endpoint Control Enable Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn isoendpt_eptctlenb6_isoendpt(&self) -> &ISOENDPT_EPTCTLENB6_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(452).cast() }
    }
    #[doc = "0x1c4 - UDPHS Endpoint Control Enable Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn eptctlenb6(&self) -> &EPTCTLENB6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(452).cast() }
    }
    #[doc = "0x1c8 - UDPHS Endpoint Control Disable Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn isoendpt_eptctldis6_isoendpt(&self) -> &ISOENDPT_EPTCTLDIS6_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(456).cast() }
    }
    #[doc = "0x1c8 - UDPHS Endpoint Control Disable Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn eptctldis6(&self) -> &EPTCTLDIS6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(456).cast() }
    }
    #[doc = "0x1cc - UDPHS Endpoint Control Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn isoendpt_eptctl6_isoendpt(&self) -> &ISOENDPT_EPTCTL6_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(460).cast() }
    }
    #[doc = "0x1cc - UDPHS Endpoint Control Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn eptctl6(&self) -> &EPTCTL6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(460).cast() }
    }
    #[doc = "0x1d4 - UDPHS Endpoint Set Status Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn isoendpt_eptsetsta6_isoendpt(&self) -> &ISOENDPT_EPTSETSTA6_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(468).cast() }
    }
    #[doc = "0x1d4 - UDPHS Endpoint Set Status Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn eptsetsta6(&self) -> &EPTSETSTA6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(468).cast() }
    }
    #[doc = "0x1d8 - UDPHS Endpoint Clear Status Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn isoendpt_eptclrsta6_isoendpt(&self) -> &ISOENDPT_EPTCLRSTA6_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(472).cast() }
    }
    #[doc = "0x1d8 - UDPHS Endpoint Clear Status Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn eptclrsta6(&self) -> &EPTCLRSTA6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(472).cast() }
    }
    #[doc = "0x1dc - UDPHS Endpoint Status Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn isoendpt_eptsta6_isoendpt(&self) -> &ISOENDPT_EPTSTA6_ISOENDPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(476).cast() }
    }
    #[doc = "0x1dc - UDPHS Endpoint Status Register (endpoint = 6)"]
    #[inline(always)]
    pub const fn eptsta6(&self) -> &EPTSTA6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(476).cast() }
    }
    #[doc = "0x300 - UDPHS DMA Next Descriptor Address Register (channel = 0)"]
    #[inline(always)]
    pub const fn dmanxtdsc0(&self) -> &DMANXTDSC0 {
        &self.dmanxtdsc0
    }
    #[doc = "0x304 - UDPHS DMA Channel Address Register (channel = 0)"]
    #[inline(always)]
    pub const fn dmaaddress0(&self) -> &DMAADDRESS0 {
        &self.dmaaddress0
    }
    #[doc = "0x308 - UDPHS DMA Channel Control Register (channel = 0)"]
    #[inline(always)]
    pub const fn dmacontrol0(&self) -> &DMACONTROL0 {
        &self.dmacontrol0
    }
    #[doc = "0x30c - UDPHS DMA Channel Status Register (channel = 0)"]
    #[inline(always)]
    pub const fn dmastatus0(&self) -> &DMASTATUS0 {
        &self.dmastatus0
    }
    #[doc = "0x310 - UDPHS DMA Next Descriptor Address Register (channel = 1)"]
    #[inline(always)]
    pub const fn dmanxtdsc1(&self) -> &DMANXTDSC1 {
        &self.dmanxtdsc1
    }
    #[doc = "0x314 - UDPHS DMA Channel Address Register (channel = 1)"]
    #[inline(always)]
    pub const fn dmaaddress1(&self) -> &DMAADDRESS1 {
        &self.dmaaddress1
    }
    #[doc = "0x318 - UDPHS DMA Channel Control Register (channel = 1)"]
    #[inline(always)]
    pub const fn dmacontrol1(&self) -> &DMACONTROL1 {
        &self.dmacontrol1
    }
    #[doc = "0x31c - UDPHS DMA Channel Status Register (channel = 1)"]
    #[inline(always)]
    pub const fn dmastatus1(&self) -> &DMASTATUS1 {
        &self.dmastatus1
    }
    #[doc = "0x320 - UDPHS DMA Next Descriptor Address Register (channel = 2)"]
    #[inline(always)]
    pub const fn dmanxtdsc2(&self) -> &DMANXTDSC2 {
        &self.dmanxtdsc2
    }
    #[doc = "0x324 - UDPHS DMA Channel Address Register (channel = 2)"]
    #[inline(always)]
    pub const fn dmaaddress2(&self) -> &DMAADDRESS2 {
        &self.dmaaddress2
    }
    #[doc = "0x328 - UDPHS DMA Channel Control Register (channel = 2)"]
    #[inline(always)]
    pub const fn dmacontrol2(&self) -> &DMACONTROL2 {
        &self.dmacontrol2
    }
    #[doc = "0x32c - UDPHS DMA Channel Status Register (channel = 2)"]
    #[inline(always)]
    pub const fn dmastatus2(&self) -> &DMASTATUS2 {
        &self.dmastatus2
    }
    #[doc = "0x330 - UDPHS DMA Next Descriptor Address Register (channel = 3)"]
    #[inline(always)]
    pub const fn dmanxtdsc3(&self) -> &DMANXTDSC3 {
        &self.dmanxtdsc3
    }
    #[doc = "0x334 - UDPHS DMA Channel Address Register (channel = 3)"]
    #[inline(always)]
    pub const fn dmaaddress3(&self) -> &DMAADDRESS3 {
        &self.dmaaddress3
    }
    #[doc = "0x338 - UDPHS DMA Channel Control Register (channel = 3)"]
    #[inline(always)]
    pub const fn dmacontrol3(&self) -> &DMACONTROL3 {
        &self.dmacontrol3
    }
    #[doc = "0x33c - UDPHS DMA Channel Status Register (channel = 3)"]
    #[inline(always)]
    pub const fn dmastatus3(&self) -> &DMASTATUS3 {
        &self.dmastatus3
    }
    #[doc = "0x340 - UDPHS DMA Next Descriptor Address Register (channel = 4)"]
    #[inline(always)]
    pub const fn dmanxtdsc4(&self) -> &DMANXTDSC4 {
        &self.dmanxtdsc4
    }
    #[doc = "0x344 - UDPHS DMA Channel Address Register (channel = 4)"]
    #[inline(always)]
    pub const fn dmaaddress4(&self) -> &DMAADDRESS4 {
        &self.dmaaddress4
    }
    #[doc = "0x348 - UDPHS DMA Channel Control Register (channel = 4)"]
    #[inline(always)]
    pub const fn dmacontrol4(&self) -> &DMACONTROL4 {
        &self.dmacontrol4
    }
    #[doc = "0x34c - UDPHS DMA Channel Status Register (channel = 4)"]
    #[inline(always)]
    pub const fn dmastatus4(&self) -> &DMASTATUS4 {
        &self.dmastatus4
    }
    #[doc = "0x350 - UDPHS DMA Next Descriptor Address Register (channel = 5)"]
    #[inline(always)]
    pub const fn dmanxtdsc5(&self) -> &DMANXTDSC5 {
        &self.dmanxtdsc5
    }
    #[doc = "0x354 - UDPHS DMA Channel Address Register (channel = 5)"]
    #[inline(always)]
    pub const fn dmaaddress5(&self) -> &DMAADDRESS5 {
        &self.dmaaddress5
    }
    #[doc = "0x358 - UDPHS DMA Channel Control Register (channel = 5)"]
    #[inline(always)]
    pub const fn dmacontrol5(&self) -> &DMACONTROL5 {
        &self.dmacontrol5
    }
    #[doc = "0x35c - UDPHS DMA Channel Status Register (channel = 5)"]
    #[inline(always)]
    pub const fn dmastatus5(&self) -> &DMASTATUS5 {
        &self.dmastatus5
    }
}
#[doc = "CTRL (rw) register accessor: UDPHS Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "UDPHS Control Register"]
pub mod ctrl;
#[doc = "FNUM (r) register accessor: UDPHS Frame Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fnum::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fnum`]
module"]
pub type FNUM = crate::Reg<fnum::FNUM_SPEC>;
#[doc = "UDPHS Frame Number Register"]
pub mod fnum;
#[doc = "IEN (rw) register accessor: UDPHS Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`]
module"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "UDPHS Interrupt Enable Register"]
pub mod ien;
#[doc = "INTSTA (r) register accessor: UDPHS Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsta::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsta`]
module"]
pub type INTSTA = crate::Reg<intsta::INTSTA_SPEC>;
#[doc = "UDPHS Interrupt Status Register"]
pub mod intsta;
#[doc = "CLRINT (w) register accessor: UDPHS Clear Interrupt Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clrint::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clrint`]
module"]
pub type CLRINT = crate::Reg<clrint::CLRINT_SPEC>;
#[doc = "UDPHS Clear Interrupt Register"]
pub mod clrint;
#[doc = "EPTRST (w) register accessor: UDPHS Endpoints Reset Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptrst::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptrst`]
module"]
pub type EPTRST = crate::Reg<eptrst::EPTRST_SPEC>;
#[doc = "UDPHS Endpoints Reset Register"]
pub mod eptrst;
#[doc = "TST (rw) register accessor: UDPHS Test Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tst`]
module"]
pub type TST = crate::Reg<tst::TST_SPEC>;
#[doc = "UDPHS Test Register"]
pub mod tst;
#[doc = "EPTCFG0 (rw) register accessor: UDPHS Endpoint Configuration Register (endpoint = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptcfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptcfg0`]
module"]
pub type EPTCFG0 = crate::Reg<eptcfg0::EPTCFG0_SPEC>;
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 0)"]
pub mod eptcfg0;
#[doc = "EPTCTLENB0 (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptctlenb0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctlenb0`]
module"]
pub type EPTCTLENB0 = crate::Reg<eptctlenb0::EPTCTLENB0_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 0)"]
pub mod eptctlenb0;
#[doc = "ISOENDPT_EPTCTLENB0_ISOENDPT (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptctlenb0_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctlenb0_isoendpt`]
module"]
pub type ISOENDPT_EPTCTLENB0_ISOENDPT =
    crate::Reg<isoendpt_eptctlenb0_isoendpt::ISOENDPT_EPTCTLENB0_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 0)"]
pub mod isoendpt_eptctlenb0_isoendpt;
#[doc = "EPTCTLDIS0 (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptctldis0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctldis0`]
module"]
pub type EPTCTLDIS0 = crate::Reg<eptctldis0::EPTCTLDIS0_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 0)"]
pub mod eptctldis0;
#[doc = "ISOENDPT_EPTCTLDIS0_ISOENDPT (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptctldis0_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctldis0_isoendpt`]
module"]
pub type ISOENDPT_EPTCTLDIS0_ISOENDPT =
    crate::Reg<isoendpt_eptctldis0_isoendpt::ISOENDPT_EPTCTLDIS0_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 0)"]
pub mod isoendpt_eptctldis0_isoendpt;
#[doc = "EPTCTL0 (r) register accessor: UDPHS Endpoint Control Register (endpoint = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptctl0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctl0`]
module"]
pub type EPTCTL0 = crate::Reg<eptctl0::EPTCTL0_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 0)"]
pub mod eptctl0;
#[doc = "ISOENDPT_EPTCTL0_ISOENDPT (r) register accessor: UDPHS Endpoint Control Register (endpoint = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoendpt_eptctl0_isoendpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctl0_isoendpt`]
module"]
pub type ISOENDPT_EPTCTL0_ISOENDPT =
    crate::Reg<isoendpt_eptctl0_isoendpt::ISOENDPT_EPTCTL0_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 0)"]
pub mod isoendpt_eptctl0_isoendpt;
#[doc = "EPTSETSTA0 (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptsetsta0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsetsta0`]
module"]
pub type EPTSETSTA0 = crate::Reg<eptsetsta0::EPTSETSTA0_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 0)"]
pub mod eptsetsta0;
#[doc = "ISOENDPT_EPTSETSTA0_ISOENDPT (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptsetsta0_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsetsta0_isoendpt`]
module"]
pub type ISOENDPT_EPTSETSTA0_ISOENDPT =
    crate::Reg<isoendpt_eptsetsta0_isoendpt::ISOENDPT_EPTSETSTA0_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 0)"]
pub mod isoendpt_eptsetsta0_isoendpt;
#[doc = "EPTCLRSTA0 (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptclrsta0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptclrsta0`]
module"]
pub type EPTCLRSTA0 = crate::Reg<eptclrsta0::EPTCLRSTA0_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 0)"]
pub mod eptclrsta0;
#[doc = "ISOENDPT_EPTCLRSTA0_ISOENDPT (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptclrsta0_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptclrsta0_isoendpt`]
module"]
pub type ISOENDPT_EPTCLRSTA0_ISOENDPT =
    crate::Reg<isoendpt_eptclrsta0_isoendpt::ISOENDPT_EPTCLRSTA0_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 0)"]
pub mod isoendpt_eptclrsta0_isoendpt;
#[doc = "EPTSTA0 (r) register accessor: UDPHS Endpoint Status Register (endpoint = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptsta0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsta0`]
module"]
pub type EPTSTA0 = crate::Reg<eptsta0::EPTSTA0_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 0)"]
pub mod eptsta0;
#[doc = "ISOENDPT_EPTSTA0_ISOENDPT (r) register accessor: UDPHS Endpoint Status Register (endpoint = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoendpt_eptsta0_isoendpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsta0_isoendpt`]
module"]
pub type ISOENDPT_EPTSTA0_ISOENDPT =
    crate::Reg<isoendpt_eptsta0_isoendpt::ISOENDPT_EPTSTA0_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 0)"]
pub mod isoendpt_eptsta0_isoendpt;
#[doc = "EPTCFG1 (rw) register accessor: UDPHS Endpoint Configuration Register (endpoint = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptcfg1`]
module"]
pub type EPTCFG1 = crate::Reg<eptcfg1::EPTCFG1_SPEC>;
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 1)"]
pub mod eptcfg1;
#[doc = "EPTCTLENB1 (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptctlenb1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctlenb1`]
module"]
pub type EPTCTLENB1 = crate::Reg<eptctlenb1::EPTCTLENB1_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 1)"]
pub mod eptctlenb1;
#[doc = "ISOENDPT_EPTCTLENB1_ISOENDPT (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptctlenb1_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctlenb1_isoendpt`]
module"]
pub type ISOENDPT_EPTCTLENB1_ISOENDPT =
    crate::Reg<isoendpt_eptctlenb1_isoendpt::ISOENDPT_EPTCTLENB1_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 1)"]
pub mod isoendpt_eptctlenb1_isoendpt;
#[doc = "EPTCTLDIS1 (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptctldis1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctldis1`]
module"]
pub type EPTCTLDIS1 = crate::Reg<eptctldis1::EPTCTLDIS1_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 1)"]
pub mod eptctldis1;
#[doc = "ISOENDPT_EPTCTLDIS1_ISOENDPT (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptctldis1_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctldis1_isoendpt`]
module"]
pub type ISOENDPT_EPTCTLDIS1_ISOENDPT =
    crate::Reg<isoendpt_eptctldis1_isoendpt::ISOENDPT_EPTCTLDIS1_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 1)"]
pub mod isoendpt_eptctldis1_isoendpt;
#[doc = "EPTCTL1 (r) register accessor: UDPHS Endpoint Control Register (endpoint = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptctl1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctl1`]
module"]
pub type EPTCTL1 = crate::Reg<eptctl1::EPTCTL1_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 1)"]
pub mod eptctl1;
#[doc = "ISOENDPT_EPTCTL1_ISOENDPT (r) register accessor: UDPHS Endpoint Control Register (endpoint = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoendpt_eptctl1_isoendpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctl1_isoendpt`]
module"]
pub type ISOENDPT_EPTCTL1_ISOENDPT =
    crate::Reg<isoendpt_eptctl1_isoendpt::ISOENDPT_EPTCTL1_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 1)"]
pub mod isoendpt_eptctl1_isoendpt;
#[doc = "EPTSETSTA1 (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptsetsta1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsetsta1`]
module"]
pub type EPTSETSTA1 = crate::Reg<eptsetsta1::EPTSETSTA1_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 1)"]
pub mod eptsetsta1;
#[doc = "ISOENDPT_EPTSETSTA1_ISOENDPT (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptsetsta1_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsetsta1_isoendpt`]
module"]
pub type ISOENDPT_EPTSETSTA1_ISOENDPT =
    crate::Reg<isoendpt_eptsetsta1_isoendpt::ISOENDPT_EPTSETSTA1_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 1)"]
pub mod isoendpt_eptsetsta1_isoendpt;
#[doc = "EPTCLRSTA1 (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptclrsta1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptclrsta1`]
module"]
pub type EPTCLRSTA1 = crate::Reg<eptclrsta1::EPTCLRSTA1_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 1)"]
pub mod eptclrsta1;
#[doc = "ISOENDPT_EPTCLRSTA1_ISOENDPT (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 1)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptclrsta1_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptclrsta1_isoendpt`]
module"]
pub type ISOENDPT_EPTCLRSTA1_ISOENDPT =
    crate::Reg<isoendpt_eptclrsta1_isoendpt::ISOENDPT_EPTCLRSTA1_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 1)"]
pub mod isoendpt_eptclrsta1_isoendpt;
#[doc = "EPTSTA1 (r) register accessor: UDPHS Endpoint Status Register (endpoint = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptsta1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsta1`]
module"]
pub type EPTSTA1 = crate::Reg<eptsta1::EPTSTA1_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 1)"]
pub mod eptsta1;
#[doc = "ISOENDPT_EPTSTA1_ISOENDPT (r) register accessor: UDPHS Endpoint Status Register (endpoint = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoendpt_eptsta1_isoendpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsta1_isoendpt`]
module"]
pub type ISOENDPT_EPTSTA1_ISOENDPT =
    crate::Reg<isoendpt_eptsta1_isoendpt::ISOENDPT_EPTSTA1_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 1)"]
pub mod isoendpt_eptsta1_isoendpt;
#[doc = "EPTCFG2 (rw) register accessor: UDPHS Endpoint Configuration Register (endpoint = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptcfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptcfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptcfg2`]
module"]
pub type EPTCFG2 = crate::Reg<eptcfg2::EPTCFG2_SPEC>;
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 2)"]
pub mod eptcfg2;
#[doc = "EPTCTLENB2 (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptctlenb2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctlenb2`]
module"]
pub type EPTCTLENB2 = crate::Reg<eptctlenb2::EPTCTLENB2_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 2)"]
pub mod eptctlenb2;
#[doc = "ISOENDPT_EPTCTLENB2_ISOENDPT (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptctlenb2_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctlenb2_isoendpt`]
module"]
pub type ISOENDPT_EPTCTLENB2_ISOENDPT =
    crate::Reg<isoendpt_eptctlenb2_isoendpt::ISOENDPT_EPTCTLENB2_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 2)"]
pub mod isoendpt_eptctlenb2_isoendpt;
#[doc = "EPTCTLDIS2 (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptctldis2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctldis2`]
module"]
pub type EPTCTLDIS2 = crate::Reg<eptctldis2::EPTCTLDIS2_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 2)"]
pub mod eptctldis2;
#[doc = "ISOENDPT_EPTCTLDIS2_ISOENDPT (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptctldis2_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctldis2_isoendpt`]
module"]
pub type ISOENDPT_EPTCTLDIS2_ISOENDPT =
    crate::Reg<isoendpt_eptctldis2_isoendpt::ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 2)"]
pub mod isoendpt_eptctldis2_isoendpt;
#[doc = "EPTCTL2 (r) register accessor: UDPHS Endpoint Control Register (endpoint = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptctl2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctl2`]
module"]
pub type EPTCTL2 = crate::Reg<eptctl2::EPTCTL2_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 2)"]
pub mod eptctl2;
#[doc = "ISOENDPT_EPTCTL2_ISOENDPT (r) register accessor: UDPHS Endpoint Control Register (endpoint = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoendpt_eptctl2_isoendpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctl2_isoendpt`]
module"]
pub type ISOENDPT_EPTCTL2_ISOENDPT =
    crate::Reg<isoendpt_eptctl2_isoendpt::ISOENDPT_EPTCTL2_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 2)"]
pub mod isoendpt_eptctl2_isoendpt;
#[doc = "EPTSETSTA2 (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptsetsta2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsetsta2`]
module"]
pub type EPTSETSTA2 = crate::Reg<eptsetsta2::EPTSETSTA2_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 2)"]
pub mod eptsetsta2;
#[doc = "ISOENDPT_EPTSETSTA2_ISOENDPT (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptsetsta2_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsetsta2_isoendpt`]
module"]
pub type ISOENDPT_EPTSETSTA2_ISOENDPT =
    crate::Reg<isoendpt_eptsetsta2_isoendpt::ISOENDPT_EPTSETSTA2_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 2)"]
pub mod isoendpt_eptsetsta2_isoendpt;
#[doc = "EPTCLRSTA2 (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptclrsta2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptclrsta2`]
module"]
pub type EPTCLRSTA2 = crate::Reg<eptclrsta2::EPTCLRSTA2_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 2)"]
pub mod eptclrsta2;
#[doc = "ISOENDPT_EPTCLRSTA2_ISOENDPT (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptclrsta2_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptclrsta2_isoendpt`]
module"]
pub type ISOENDPT_EPTCLRSTA2_ISOENDPT =
    crate::Reg<isoendpt_eptclrsta2_isoendpt::ISOENDPT_EPTCLRSTA2_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 2)"]
pub mod isoendpt_eptclrsta2_isoendpt;
#[doc = "EPTSTA2 (r) register accessor: UDPHS Endpoint Status Register (endpoint = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptsta2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsta2`]
module"]
pub type EPTSTA2 = crate::Reg<eptsta2::EPTSTA2_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 2)"]
pub mod eptsta2;
#[doc = "ISOENDPT_EPTSTA2_ISOENDPT (r) register accessor: UDPHS Endpoint Status Register (endpoint = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoendpt_eptsta2_isoendpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsta2_isoendpt`]
module"]
pub type ISOENDPT_EPTSTA2_ISOENDPT =
    crate::Reg<isoendpt_eptsta2_isoendpt::ISOENDPT_EPTSTA2_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 2)"]
pub mod isoendpt_eptsta2_isoendpt;
#[doc = "EPTCFG3 (rw) register accessor: UDPHS Endpoint Configuration Register (endpoint = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptcfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptcfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptcfg3`]
module"]
pub type EPTCFG3 = crate::Reg<eptcfg3::EPTCFG3_SPEC>;
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 3)"]
pub mod eptcfg3;
#[doc = "EPTCTLENB3 (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 3)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptctlenb3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctlenb3`]
module"]
pub type EPTCTLENB3 = crate::Reg<eptctlenb3::EPTCTLENB3_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 3)"]
pub mod eptctlenb3;
#[doc = "ISOENDPT_EPTCTLENB3_ISOENDPT (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 3)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptctlenb3_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctlenb3_isoendpt`]
module"]
pub type ISOENDPT_EPTCTLENB3_ISOENDPT =
    crate::Reg<isoendpt_eptctlenb3_isoendpt::ISOENDPT_EPTCTLENB3_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 3)"]
pub mod isoendpt_eptctlenb3_isoendpt;
#[doc = "EPTCTLDIS3 (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 3)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptctldis3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctldis3`]
module"]
pub type EPTCTLDIS3 = crate::Reg<eptctldis3::EPTCTLDIS3_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 3)"]
pub mod eptctldis3;
#[doc = "ISOENDPT_EPTCTLDIS3_ISOENDPT (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 3)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptctldis3_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctldis3_isoendpt`]
module"]
pub type ISOENDPT_EPTCTLDIS3_ISOENDPT =
    crate::Reg<isoendpt_eptctldis3_isoendpt::ISOENDPT_EPTCTLDIS3_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 3)"]
pub mod isoendpt_eptctldis3_isoendpt;
#[doc = "EPTCTL3 (r) register accessor: UDPHS Endpoint Control Register (endpoint = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptctl3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctl3`]
module"]
pub type EPTCTL3 = crate::Reg<eptctl3::EPTCTL3_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 3)"]
pub mod eptctl3;
#[doc = "ISOENDPT_EPTCTL3_ISOENDPT (r) register accessor: UDPHS Endpoint Control Register (endpoint = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoendpt_eptctl3_isoendpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctl3_isoendpt`]
module"]
pub type ISOENDPT_EPTCTL3_ISOENDPT =
    crate::Reg<isoendpt_eptctl3_isoendpt::ISOENDPT_EPTCTL3_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 3)"]
pub mod isoendpt_eptctl3_isoendpt;
#[doc = "EPTSETSTA3 (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 3)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptsetsta3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsetsta3`]
module"]
pub type EPTSETSTA3 = crate::Reg<eptsetsta3::EPTSETSTA3_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 3)"]
pub mod eptsetsta3;
#[doc = "ISOENDPT_EPTSETSTA3_ISOENDPT (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 3)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptsetsta3_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsetsta3_isoendpt`]
module"]
pub type ISOENDPT_EPTSETSTA3_ISOENDPT =
    crate::Reg<isoendpt_eptsetsta3_isoendpt::ISOENDPT_EPTSETSTA3_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 3)"]
pub mod isoendpt_eptsetsta3_isoendpt;
#[doc = "EPTCLRSTA3 (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 3)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptclrsta3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptclrsta3`]
module"]
pub type EPTCLRSTA3 = crate::Reg<eptclrsta3::EPTCLRSTA3_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 3)"]
pub mod eptclrsta3;
#[doc = "ISOENDPT_EPTCLRSTA3_ISOENDPT (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 3)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptclrsta3_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptclrsta3_isoendpt`]
module"]
pub type ISOENDPT_EPTCLRSTA3_ISOENDPT =
    crate::Reg<isoendpt_eptclrsta3_isoendpt::ISOENDPT_EPTCLRSTA3_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 3)"]
pub mod isoendpt_eptclrsta3_isoendpt;
#[doc = "EPTSTA3 (r) register accessor: UDPHS Endpoint Status Register (endpoint = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptsta3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsta3`]
module"]
pub type EPTSTA3 = crate::Reg<eptsta3::EPTSTA3_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 3)"]
pub mod eptsta3;
#[doc = "ISOENDPT_EPTSTA3_ISOENDPT (r) register accessor: UDPHS Endpoint Status Register (endpoint = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoendpt_eptsta3_isoendpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsta3_isoendpt`]
module"]
pub type ISOENDPT_EPTSTA3_ISOENDPT =
    crate::Reg<isoendpt_eptsta3_isoendpt::ISOENDPT_EPTSTA3_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 3)"]
pub mod isoendpt_eptsta3_isoendpt;
#[doc = "EPTCFG4 (rw) register accessor: UDPHS Endpoint Configuration Register (endpoint = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptcfg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptcfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptcfg4`]
module"]
pub type EPTCFG4 = crate::Reg<eptcfg4::EPTCFG4_SPEC>;
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 4)"]
pub mod eptcfg4;
#[doc = "EPTCTLENB4 (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 4)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptctlenb4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctlenb4`]
module"]
pub type EPTCTLENB4 = crate::Reg<eptctlenb4::EPTCTLENB4_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 4)"]
pub mod eptctlenb4;
#[doc = "ISOENDPT_EPTCTLENB4_ISOENDPT (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 4)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptctlenb4_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctlenb4_isoendpt`]
module"]
pub type ISOENDPT_EPTCTLENB4_ISOENDPT =
    crate::Reg<isoendpt_eptctlenb4_isoendpt::ISOENDPT_EPTCTLENB4_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 4)"]
pub mod isoendpt_eptctlenb4_isoendpt;
#[doc = "EPTCTLDIS4 (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 4)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptctldis4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctldis4`]
module"]
pub type EPTCTLDIS4 = crate::Reg<eptctldis4::EPTCTLDIS4_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 4)"]
pub mod eptctldis4;
#[doc = "ISOENDPT_EPTCTLDIS4_ISOENDPT (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 4)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptctldis4_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctldis4_isoendpt`]
module"]
pub type ISOENDPT_EPTCTLDIS4_ISOENDPT =
    crate::Reg<isoendpt_eptctldis4_isoendpt::ISOENDPT_EPTCTLDIS4_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 4)"]
pub mod isoendpt_eptctldis4_isoendpt;
#[doc = "EPTCTL4 (r) register accessor: UDPHS Endpoint Control Register (endpoint = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptctl4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctl4`]
module"]
pub type EPTCTL4 = crate::Reg<eptctl4::EPTCTL4_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 4)"]
pub mod eptctl4;
#[doc = "ISOENDPT_EPTCTL4_ISOENDPT (r) register accessor: UDPHS Endpoint Control Register (endpoint = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoendpt_eptctl4_isoendpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctl4_isoendpt`]
module"]
pub type ISOENDPT_EPTCTL4_ISOENDPT =
    crate::Reg<isoendpt_eptctl4_isoendpt::ISOENDPT_EPTCTL4_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 4)"]
pub mod isoendpt_eptctl4_isoendpt;
#[doc = "EPTSETSTA4 (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 4)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptsetsta4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsetsta4`]
module"]
pub type EPTSETSTA4 = crate::Reg<eptsetsta4::EPTSETSTA4_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 4)"]
pub mod eptsetsta4;
#[doc = "ISOENDPT_EPTSETSTA4_ISOENDPT (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 4)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptsetsta4_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsetsta4_isoendpt`]
module"]
pub type ISOENDPT_EPTSETSTA4_ISOENDPT =
    crate::Reg<isoendpt_eptsetsta4_isoendpt::ISOENDPT_EPTSETSTA4_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 4)"]
pub mod isoendpt_eptsetsta4_isoendpt;
#[doc = "EPTCLRSTA4 (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 4)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptclrsta4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptclrsta4`]
module"]
pub type EPTCLRSTA4 = crate::Reg<eptclrsta4::EPTCLRSTA4_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 4)"]
pub mod eptclrsta4;
#[doc = "ISOENDPT_EPTCLRSTA4_ISOENDPT (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 4)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptclrsta4_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptclrsta4_isoendpt`]
module"]
pub type ISOENDPT_EPTCLRSTA4_ISOENDPT =
    crate::Reg<isoendpt_eptclrsta4_isoendpt::ISOENDPT_EPTCLRSTA4_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 4)"]
pub mod isoendpt_eptclrsta4_isoendpt;
#[doc = "EPTSTA4 (r) register accessor: UDPHS Endpoint Status Register (endpoint = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptsta4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsta4`]
module"]
pub type EPTSTA4 = crate::Reg<eptsta4::EPTSTA4_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 4)"]
pub mod eptsta4;
#[doc = "ISOENDPT_EPTSTA4_ISOENDPT (r) register accessor: UDPHS Endpoint Status Register (endpoint = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoendpt_eptsta4_isoendpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsta4_isoendpt`]
module"]
pub type ISOENDPT_EPTSTA4_ISOENDPT =
    crate::Reg<isoendpt_eptsta4_isoendpt::ISOENDPT_EPTSTA4_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 4)"]
pub mod isoendpt_eptsta4_isoendpt;
#[doc = "EPTCFG5 (rw) register accessor: UDPHS Endpoint Configuration Register (endpoint = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptcfg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptcfg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptcfg5`]
module"]
pub type EPTCFG5 = crate::Reg<eptcfg5::EPTCFG5_SPEC>;
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 5)"]
pub mod eptcfg5;
#[doc = "EPTCTLENB5 (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 5)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptctlenb5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctlenb5`]
module"]
pub type EPTCTLENB5 = crate::Reg<eptctlenb5::EPTCTLENB5_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 5)"]
pub mod eptctlenb5;
#[doc = "ISOENDPT_EPTCTLENB5_ISOENDPT (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 5)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptctlenb5_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctlenb5_isoendpt`]
module"]
pub type ISOENDPT_EPTCTLENB5_ISOENDPT =
    crate::Reg<isoendpt_eptctlenb5_isoendpt::ISOENDPT_EPTCTLENB5_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 5)"]
pub mod isoendpt_eptctlenb5_isoendpt;
#[doc = "EPTCTLDIS5 (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 5)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptctldis5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctldis5`]
module"]
pub type EPTCTLDIS5 = crate::Reg<eptctldis5::EPTCTLDIS5_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 5)"]
pub mod eptctldis5;
#[doc = "ISOENDPT_EPTCTLDIS5_ISOENDPT (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 5)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptctldis5_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctldis5_isoendpt`]
module"]
pub type ISOENDPT_EPTCTLDIS5_ISOENDPT =
    crate::Reg<isoendpt_eptctldis5_isoendpt::ISOENDPT_EPTCTLDIS5_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 5)"]
pub mod isoendpt_eptctldis5_isoendpt;
#[doc = "EPTCTL5 (r) register accessor: UDPHS Endpoint Control Register (endpoint = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptctl5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctl5`]
module"]
pub type EPTCTL5 = crate::Reg<eptctl5::EPTCTL5_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 5)"]
pub mod eptctl5;
#[doc = "ISOENDPT_EPTCTL5_ISOENDPT (r) register accessor: UDPHS Endpoint Control Register (endpoint = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoendpt_eptctl5_isoendpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctl5_isoendpt`]
module"]
pub type ISOENDPT_EPTCTL5_ISOENDPT =
    crate::Reg<isoendpt_eptctl5_isoendpt::ISOENDPT_EPTCTL5_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 5)"]
pub mod isoendpt_eptctl5_isoendpt;
#[doc = "EPTSETSTA5 (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 5)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptsetsta5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsetsta5`]
module"]
pub type EPTSETSTA5 = crate::Reg<eptsetsta5::EPTSETSTA5_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 5)"]
pub mod eptsetsta5;
#[doc = "ISOENDPT_EPTSETSTA5_ISOENDPT (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 5)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptsetsta5_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsetsta5_isoendpt`]
module"]
pub type ISOENDPT_EPTSETSTA5_ISOENDPT =
    crate::Reg<isoendpt_eptsetsta5_isoendpt::ISOENDPT_EPTSETSTA5_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 5)"]
pub mod isoendpt_eptsetsta5_isoendpt;
#[doc = "EPTCLRSTA5 (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 5)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptclrsta5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptclrsta5`]
module"]
pub type EPTCLRSTA5 = crate::Reg<eptclrsta5::EPTCLRSTA5_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 5)"]
pub mod eptclrsta5;
#[doc = "ISOENDPT_EPTCLRSTA5_ISOENDPT (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 5)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptclrsta5_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptclrsta5_isoendpt`]
module"]
pub type ISOENDPT_EPTCLRSTA5_ISOENDPT =
    crate::Reg<isoendpt_eptclrsta5_isoendpt::ISOENDPT_EPTCLRSTA5_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 5)"]
pub mod isoendpt_eptclrsta5_isoendpt;
#[doc = "EPTSTA5 (r) register accessor: UDPHS Endpoint Status Register (endpoint = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptsta5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsta5`]
module"]
pub type EPTSTA5 = crate::Reg<eptsta5::EPTSTA5_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 5)"]
pub mod eptsta5;
#[doc = "ISOENDPT_EPTSTA5_ISOENDPT (r) register accessor: UDPHS Endpoint Status Register (endpoint = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoendpt_eptsta5_isoendpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsta5_isoendpt`]
module"]
pub type ISOENDPT_EPTSTA5_ISOENDPT =
    crate::Reg<isoendpt_eptsta5_isoendpt::ISOENDPT_EPTSTA5_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 5)"]
pub mod isoendpt_eptsta5_isoendpt;
#[doc = "EPTCFG6 (rw) register accessor: UDPHS Endpoint Configuration Register (endpoint = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptcfg6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptcfg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptcfg6`]
module"]
pub type EPTCFG6 = crate::Reg<eptcfg6::EPTCFG6_SPEC>;
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 6)"]
pub mod eptcfg6;
#[doc = "EPTCTLENB6 (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 6)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptctlenb6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctlenb6`]
module"]
pub type EPTCTLENB6 = crate::Reg<eptctlenb6::EPTCTLENB6_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 6)"]
pub mod eptctlenb6;
#[doc = "ISOENDPT_EPTCTLENB6_ISOENDPT (w) register accessor: UDPHS Endpoint Control Enable Register (endpoint = 6)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptctlenb6_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctlenb6_isoendpt`]
module"]
pub type ISOENDPT_EPTCTLENB6_ISOENDPT =
    crate::Reg<isoendpt_eptctlenb6_isoendpt::ISOENDPT_EPTCTLENB6_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 6)"]
pub mod isoendpt_eptctlenb6_isoendpt;
#[doc = "EPTCTLDIS6 (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 6)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptctldis6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctldis6`]
module"]
pub type EPTCTLDIS6 = crate::Reg<eptctldis6::EPTCTLDIS6_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 6)"]
pub mod eptctldis6;
#[doc = "ISOENDPT_EPTCTLDIS6_ISOENDPT (w) register accessor: UDPHS Endpoint Control Disable Register (endpoint = 6)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptctldis6_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctldis6_isoendpt`]
module"]
pub type ISOENDPT_EPTCTLDIS6_ISOENDPT =
    crate::Reg<isoendpt_eptctldis6_isoendpt::ISOENDPT_EPTCTLDIS6_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 6)"]
pub mod isoendpt_eptctldis6_isoendpt;
#[doc = "EPTCTL6 (r) register accessor: UDPHS Endpoint Control Register (endpoint = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptctl6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptctl6`]
module"]
pub type EPTCTL6 = crate::Reg<eptctl6::EPTCTL6_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 6)"]
pub mod eptctl6;
#[doc = "ISOENDPT_EPTCTL6_ISOENDPT (r) register accessor: UDPHS Endpoint Control Register (endpoint = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoendpt_eptctl6_isoendpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptctl6_isoendpt`]
module"]
pub type ISOENDPT_EPTCTL6_ISOENDPT =
    crate::Reg<isoendpt_eptctl6_isoendpt::ISOENDPT_EPTCTL6_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 6)"]
pub mod isoendpt_eptctl6_isoendpt;
#[doc = "EPTSETSTA6 (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 6)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptsetsta6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsetsta6`]
module"]
pub type EPTSETSTA6 = crate::Reg<eptsetsta6::EPTSETSTA6_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 6)"]
pub mod eptsetsta6;
#[doc = "ISOENDPT_EPTSETSTA6_ISOENDPT (w) register accessor: UDPHS Endpoint Set Status Register (endpoint = 6)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptsetsta6_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsetsta6_isoendpt`]
module"]
pub type ISOENDPT_EPTSETSTA6_ISOENDPT =
    crate::Reg<isoendpt_eptsetsta6_isoendpt::ISOENDPT_EPTSETSTA6_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 6)"]
pub mod isoendpt_eptsetsta6_isoendpt;
#[doc = "EPTCLRSTA6 (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 6)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptclrsta6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptclrsta6`]
module"]
pub type EPTCLRSTA6 = crate::Reg<eptclrsta6::EPTCLRSTA6_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 6)"]
pub mod eptclrsta6;
#[doc = "ISOENDPT_EPTCLRSTA6_ISOENDPT (w) register accessor: UDPHS Endpoint Clear Status Register (endpoint = 6)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptclrsta6_isoendpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptclrsta6_isoendpt`]
module"]
pub type ISOENDPT_EPTCLRSTA6_ISOENDPT =
    crate::Reg<isoendpt_eptclrsta6_isoendpt::ISOENDPT_EPTCLRSTA6_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 6)"]
pub mod isoendpt_eptclrsta6_isoendpt;
#[doc = "EPTSTA6 (r) register accessor: UDPHS Endpoint Status Register (endpoint = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptsta6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eptsta6`]
module"]
pub type EPTSTA6 = crate::Reg<eptsta6::EPTSTA6_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 6)"]
pub mod eptsta6;
#[doc = "ISOENDPT_EPTSTA6_ISOENDPT (r) register accessor: UDPHS Endpoint Status Register (endpoint = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoendpt_eptsta6_isoendpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoendpt_eptsta6_isoendpt`]
module"]
pub type ISOENDPT_EPTSTA6_ISOENDPT =
    crate::Reg<isoendpt_eptsta6_isoendpt::ISOENDPT_EPTSTA6_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 6)"]
pub mod isoendpt_eptsta6_isoendpt;
#[doc = "DMANXTDSC0 (rw) register accessor: UDPHS DMA Next Descriptor Address Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmanxtdsc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmanxtdsc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmanxtdsc0`]
module"]
pub type DMANXTDSC0 = crate::Reg<dmanxtdsc0::DMANXTDSC0_SPEC>;
#[doc = "UDPHS DMA Next Descriptor Address Register (channel = 0)"]
pub mod dmanxtdsc0;
#[doc = "DMAADDRESS0 (rw) register accessor: UDPHS DMA Channel Address Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaaddress0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaaddress0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaaddress0`]
module"]
pub type DMAADDRESS0 = crate::Reg<dmaaddress0::DMAADDRESS0_SPEC>;
#[doc = "UDPHS DMA Channel Address Register (channel = 0)"]
pub mod dmaaddress0;
#[doc = "DMACONTROL0 (rw) register accessor: UDPHS DMA Channel Control Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacontrol0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacontrol0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacontrol0`]
module"]
pub type DMACONTROL0 = crate::Reg<dmacontrol0::DMACONTROL0_SPEC>;
#[doc = "UDPHS DMA Channel Control Register (channel = 0)"]
pub mod dmacontrol0;
#[doc = "DMASTATUS0 (rw) register accessor: UDPHS DMA Channel Status Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmastatus0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmastatus0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmastatus0`]
module"]
pub type DMASTATUS0 = crate::Reg<dmastatus0::DMASTATUS0_SPEC>;
#[doc = "UDPHS DMA Channel Status Register (channel = 0)"]
pub mod dmastatus0;
#[doc = "DMANXTDSC1 (rw) register accessor: UDPHS DMA Next Descriptor Address Register (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmanxtdsc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmanxtdsc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmanxtdsc1`]
module"]
pub type DMANXTDSC1 = crate::Reg<dmanxtdsc1::DMANXTDSC1_SPEC>;
#[doc = "UDPHS DMA Next Descriptor Address Register (channel = 1)"]
pub mod dmanxtdsc1;
#[doc = "DMAADDRESS1 (rw) register accessor: UDPHS DMA Channel Address Register (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaaddress1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaaddress1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaaddress1`]
module"]
pub type DMAADDRESS1 = crate::Reg<dmaaddress1::DMAADDRESS1_SPEC>;
#[doc = "UDPHS DMA Channel Address Register (channel = 1)"]
pub mod dmaaddress1;
#[doc = "DMACONTROL1 (rw) register accessor: UDPHS DMA Channel Control Register (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacontrol1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacontrol1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacontrol1`]
module"]
pub type DMACONTROL1 = crate::Reg<dmacontrol1::DMACONTROL1_SPEC>;
#[doc = "UDPHS DMA Channel Control Register (channel = 1)"]
pub mod dmacontrol1;
#[doc = "DMASTATUS1 (rw) register accessor: UDPHS DMA Channel Status Register (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmastatus1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmastatus1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmastatus1`]
module"]
pub type DMASTATUS1 = crate::Reg<dmastatus1::DMASTATUS1_SPEC>;
#[doc = "UDPHS DMA Channel Status Register (channel = 1)"]
pub mod dmastatus1;
#[doc = "DMANXTDSC2 (rw) register accessor: UDPHS DMA Next Descriptor Address Register (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmanxtdsc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmanxtdsc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmanxtdsc2`]
module"]
pub type DMANXTDSC2 = crate::Reg<dmanxtdsc2::DMANXTDSC2_SPEC>;
#[doc = "UDPHS DMA Next Descriptor Address Register (channel = 2)"]
pub mod dmanxtdsc2;
#[doc = "DMAADDRESS2 (rw) register accessor: UDPHS DMA Channel Address Register (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaaddress2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaaddress2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaaddress2`]
module"]
pub type DMAADDRESS2 = crate::Reg<dmaaddress2::DMAADDRESS2_SPEC>;
#[doc = "UDPHS DMA Channel Address Register (channel = 2)"]
pub mod dmaaddress2;
#[doc = "DMACONTROL2 (rw) register accessor: UDPHS DMA Channel Control Register (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacontrol2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacontrol2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacontrol2`]
module"]
pub type DMACONTROL2 = crate::Reg<dmacontrol2::DMACONTROL2_SPEC>;
#[doc = "UDPHS DMA Channel Control Register (channel = 2)"]
pub mod dmacontrol2;
#[doc = "DMASTATUS2 (rw) register accessor: UDPHS DMA Channel Status Register (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmastatus2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmastatus2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmastatus2`]
module"]
pub type DMASTATUS2 = crate::Reg<dmastatus2::DMASTATUS2_SPEC>;
#[doc = "UDPHS DMA Channel Status Register (channel = 2)"]
pub mod dmastatus2;
#[doc = "DMANXTDSC3 (rw) register accessor: UDPHS DMA Next Descriptor Address Register (channel = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmanxtdsc3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmanxtdsc3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmanxtdsc3`]
module"]
pub type DMANXTDSC3 = crate::Reg<dmanxtdsc3::DMANXTDSC3_SPEC>;
#[doc = "UDPHS DMA Next Descriptor Address Register (channel = 3)"]
pub mod dmanxtdsc3;
#[doc = "DMAADDRESS3 (rw) register accessor: UDPHS DMA Channel Address Register (channel = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaaddress3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaaddress3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaaddress3`]
module"]
pub type DMAADDRESS3 = crate::Reg<dmaaddress3::DMAADDRESS3_SPEC>;
#[doc = "UDPHS DMA Channel Address Register (channel = 3)"]
pub mod dmaaddress3;
#[doc = "DMACONTROL3 (rw) register accessor: UDPHS DMA Channel Control Register (channel = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacontrol3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacontrol3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacontrol3`]
module"]
pub type DMACONTROL3 = crate::Reg<dmacontrol3::DMACONTROL3_SPEC>;
#[doc = "UDPHS DMA Channel Control Register (channel = 3)"]
pub mod dmacontrol3;
#[doc = "DMASTATUS3 (rw) register accessor: UDPHS DMA Channel Status Register (channel = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmastatus3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmastatus3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmastatus3`]
module"]
pub type DMASTATUS3 = crate::Reg<dmastatus3::DMASTATUS3_SPEC>;
#[doc = "UDPHS DMA Channel Status Register (channel = 3)"]
pub mod dmastatus3;
#[doc = "DMANXTDSC4 (rw) register accessor: UDPHS DMA Next Descriptor Address Register (channel = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmanxtdsc4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmanxtdsc4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmanxtdsc4`]
module"]
pub type DMANXTDSC4 = crate::Reg<dmanxtdsc4::DMANXTDSC4_SPEC>;
#[doc = "UDPHS DMA Next Descriptor Address Register (channel = 4)"]
pub mod dmanxtdsc4;
#[doc = "DMAADDRESS4 (rw) register accessor: UDPHS DMA Channel Address Register (channel = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaaddress4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaaddress4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaaddress4`]
module"]
pub type DMAADDRESS4 = crate::Reg<dmaaddress4::DMAADDRESS4_SPEC>;
#[doc = "UDPHS DMA Channel Address Register (channel = 4)"]
pub mod dmaaddress4;
#[doc = "DMACONTROL4 (rw) register accessor: UDPHS DMA Channel Control Register (channel = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacontrol4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacontrol4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacontrol4`]
module"]
pub type DMACONTROL4 = crate::Reg<dmacontrol4::DMACONTROL4_SPEC>;
#[doc = "UDPHS DMA Channel Control Register (channel = 4)"]
pub mod dmacontrol4;
#[doc = "DMASTATUS4 (rw) register accessor: UDPHS DMA Channel Status Register (channel = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmastatus4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmastatus4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmastatus4`]
module"]
pub type DMASTATUS4 = crate::Reg<dmastatus4::DMASTATUS4_SPEC>;
#[doc = "UDPHS DMA Channel Status Register (channel = 4)"]
pub mod dmastatus4;
#[doc = "DMANXTDSC5 (rw) register accessor: UDPHS DMA Next Descriptor Address Register (channel = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmanxtdsc5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmanxtdsc5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmanxtdsc5`]
module"]
pub type DMANXTDSC5 = crate::Reg<dmanxtdsc5::DMANXTDSC5_SPEC>;
#[doc = "UDPHS DMA Next Descriptor Address Register (channel = 5)"]
pub mod dmanxtdsc5;
#[doc = "DMAADDRESS5 (rw) register accessor: UDPHS DMA Channel Address Register (channel = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaaddress5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaaddress5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaaddress5`]
module"]
pub type DMAADDRESS5 = crate::Reg<dmaaddress5::DMAADDRESS5_SPEC>;
#[doc = "UDPHS DMA Channel Address Register (channel = 5)"]
pub mod dmaaddress5;
#[doc = "DMACONTROL5 (rw) register accessor: UDPHS DMA Channel Control Register (channel = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacontrol5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacontrol5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacontrol5`]
module"]
pub type DMACONTROL5 = crate::Reg<dmacontrol5::DMACONTROL5_SPEC>;
#[doc = "UDPHS DMA Channel Control Register (channel = 5)"]
pub mod dmacontrol5;
#[doc = "DMASTATUS5 (rw) register accessor: UDPHS DMA Channel Status Register (channel = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmastatus5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmastatus5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmastatus5`]
module"]
pub type DMASTATUS5 = crate::Reg<dmastatus5::DMASTATUS5_SPEC>;
#[doc = "UDPHS DMA Channel Status Register (channel = 5)"]
pub mod dmastatus5;
