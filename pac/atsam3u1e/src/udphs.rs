#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UDPHS Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - UDPHS Frame Number Register"]
    pub fnum: FNUM,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - UDPHS Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x14 - UDPHS Interrupt Status Register"]
    pub intsta: INTSTA,
    #[doc = "0x18 - UDPHS Clear Interrupt Register"]
    pub clrint: CLRINT,
    #[doc = "0x1c - UDPHS Endpoints Reset Register"]
    pub eptrst: EPTRST,
    _reserved6: [u8; 0xc0],
    #[doc = "0xe0 - UDPHS Test Register"]
    pub tst: TST,
    _reserved7: [u8; 0x1c],
    #[doc = "0x100 - UDPHS Endpoint Configuration Register (endpoint = 0)"]
    pub eptcfg0: EPTCFG0,
    _reserved_8_eptctlenb0: [u8; 0x04],
    _reserved_9_eptctldis0: [u8; 0x04],
    _reserved_10_eptctl0: [u8; 0x04],
    _reserved11: [u8; 0x04],
    _reserved_11_eptsetsta0: [u8; 0x04],
    _reserved_12_eptclrsta0: [u8; 0x04],
    _reserved_13_eptsta0: [u8; 0x04],
    #[doc = "0x120 - UDPHS Endpoint Configuration Register (endpoint = 1)"]
    pub eptcfg1: EPTCFG1,
    _reserved_15_eptctlenb1: [u8; 0x04],
    _reserved_16_eptctldis1: [u8; 0x04],
    _reserved_17_eptctl1: [u8; 0x04],
    _reserved18: [u8; 0x04],
    _reserved_18_eptsetsta1: [u8; 0x04],
    _reserved_19_eptclrsta1: [u8; 0x04],
    _reserved_20_eptsta1: [u8; 0x04],
    #[doc = "0x140 - UDPHS Endpoint Configuration Register (endpoint = 2)"]
    pub eptcfg2: EPTCFG2,
    _reserved_22_eptctlenb2: [u8; 0x04],
    _reserved_23_eptctldis2: [u8; 0x04],
    _reserved_24_eptctl2: [u8; 0x04],
    _reserved25: [u8; 0x04],
    _reserved_25_eptsetsta2: [u8; 0x04],
    _reserved_26_eptclrsta2: [u8; 0x04],
    _reserved_27_eptsta2: [u8; 0x04],
    #[doc = "0x160 - UDPHS Endpoint Configuration Register (endpoint = 3)"]
    pub eptcfg3: EPTCFG3,
    _reserved_29_eptctlenb3: [u8; 0x04],
    _reserved_30_eptctldis3: [u8; 0x04],
    _reserved_31_eptctl3: [u8; 0x04],
    _reserved32: [u8; 0x04],
    _reserved_32_eptsetsta3: [u8; 0x04],
    _reserved_33_eptclrsta3: [u8; 0x04],
    _reserved_34_eptsta3: [u8; 0x04],
    #[doc = "0x180 - UDPHS Endpoint Configuration Register (endpoint = 4)"]
    pub eptcfg4: EPTCFG4,
    _reserved_36_eptctlenb4: [u8; 0x04],
    _reserved_37_eptctldis4: [u8; 0x04],
    _reserved_38_eptctl4: [u8; 0x04],
    _reserved39: [u8; 0x04],
    _reserved_39_eptsetsta4: [u8; 0x04],
    _reserved_40_eptclrsta4: [u8; 0x04],
    _reserved_41_eptsta4: [u8; 0x04],
    #[doc = "0x1a0 - UDPHS Endpoint Configuration Register (endpoint = 5)"]
    pub eptcfg5: EPTCFG5,
    _reserved_43_eptctlenb5: [u8; 0x04],
    _reserved_44_eptctldis5: [u8; 0x04],
    _reserved_45_eptctl5: [u8; 0x04],
    _reserved46: [u8; 0x04],
    _reserved_46_eptsetsta5: [u8; 0x04],
    _reserved_47_eptclrsta5: [u8; 0x04],
    _reserved_48_eptsta5: [u8; 0x04],
    #[doc = "0x1c0 - UDPHS Endpoint Configuration Register (endpoint = 6)"]
    pub eptcfg6: EPTCFG6,
    _reserved_50_eptctlenb6: [u8; 0x04],
    _reserved_51_eptctldis6: [u8; 0x04],
    _reserved_52_eptctl6: [u8; 0x04],
    _reserved53: [u8; 0x04],
    _reserved_53_eptsetsta6: [u8; 0x04],
    _reserved_54_eptclrsta6: [u8; 0x04],
    _reserved_55_eptsta6: [u8; 0x04],
    _reserved56: [u8; 0x0120],
    #[doc = "0x300 - UDPHS DMA Next Descriptor Address Register (channel = 0)"]
    pub dmanxtdsc0: DMANXTDSC0,
    #[doc = "0x304 - UDPHS DMA Channel Address Register (channel = 0)"]
    pub dmaaddress0: DMAADDRESS0,
    #[doc = "0x308 - UDPHS DMA Channel Control Register (channel = 0)"]
    pub dmacontrol0: DMACONTROL0,
    #[doc = "0x30c - UDPHS DMA Channel Status Register (channel = 0)"]
    pub dmastatus0: DMASTATUS0,
    #[doc = "0x310 - UDPHS DMA Next Descriptor Address Register (channel = 1)"]
    pub dmanxtdsc1: DMANXTDSC1,
    #[doc = "0x314 - UDPHS DMA Channel Address Register (channel = 1)"]
    pub dmaaddress1: DMAADDRESS1,
    #[doc = "0x318 - UDPHS DMA Channel Control Register (channel = 1)"]
    pub dmacontrol1: DMACONTROL1,
    #[doc = "0x31c - UDPHS DMA Channel Status Register (channel = 1)"]
    pub dmastatus1: DMASTATUS1,
    #[doc = "0x320 - UDPHS DMA Next Descriptor Address Register (channel = 2)"]
    pub dmanxtdsc2: DMANXTDSC2,
    #[doc = "0x324 - UDPHS DMA Channel Address Register (channel = 2)"]
    pub dmaaddress2: DMAADDRESS2,
    #[doc = "0x328 - UDPHS DMA Channel Control Register (channel = 2)"]
    pub dmacontrol2: DMACONTROL2,
    #[doc = "0x32c - UDPHS DMA Channel Status Register (channel = 2)"]
    pub dmastatus2: DMASTATUS2,
    #[doc = "0x330 - UDPHS DMA Next Descriptor Address Register (channel = 3)"]
    pub dmanxtdsc3: DMANXTDSC3,
    #[doc = "0x334 - UDPHS DMA Channel Address Register (channel = 3)"]
    pub dmaaddress3: DMAADDRESS3,
    #[doc = "0x338 - UDPHS DMA Channel Control Register (channel = 3)"]
    pub dmacontrol3: DMACONTROL3,
    #[doc = "0x33c - UDPHS DMA Channel Status Register (channel = 3)"]
    pub dmastatus3: DMASTATUS3,
    #[doc = "0x340 - UDPHS DMA Next Descriptor Address Register (channel = 4)"]
    pub dmanxtdsc4: DMANXTDSC4,
    #[doc = "0x344 - UDPHS DMA Channel Address Register (channel = 4)"]
    pub dmaaddress4: DMAADDRESS4,
    #[doc = "0x348 - UDPHS DMA Channel Control Register (channel = 4)"]
    pub dmacontrol4: DMACONTROL4,
    #[doc = "0x34c - UDPHS DMA Channel Status Register (channel = 4)"]
    pub dmastatus4: DMASTATUS4,
    #[doc = "0x350 - UDPHS DMA Next Descriptor Address Register (channel = 5)"]
    pub dmanxtdsc5: DMANXTDSC5,
    #[doc = "0x354 - UDPHS DMA Channel Address Register (channel = 5)"]
    pub dmaaddress5: DMAADDRESS5,
    #[doc = "0x358 - UDPHS DMA Channel Control Register (channel = 5)"]
    pub dmacontrol5: DMACONTROL5,
    #[doc = "0x35c - UDPHS DMA Channel Status Register (channel = 5)"]
    pub dmastatus5: DMASTATUS5,
}
impl RegisterBlock {
    #[doc = "0x104 - UDPHS Endpoint Control Enable Register (endpoint = 0)"]
    #[inline(always)]
    pub fn isoendpt_eptctlenb0_isoendpt(&self) -> &ISOENDPT_EPTCTLENB0_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(260usize)
                as *const ISOENDPT_EPTCTLENB0_ISOENDPT)
        }
    }
    #[doc = "0x104 - UDPHS Endpoint Control Enable Register (endpoint = 0)"]
    #[inline(always)]
    pub fn eptctlenb0(&self) -> &EPTCTLENB0 {
        unsafe { &*(((self as *const Self) as *const u8).add(260usize) as *const EPTCTLENB0) }
    }
    #[doc = "0x108 - UDPHS Endpoint Control Disable Register (endpoint = 0)"]
    #[inline(always)]
    pub fn isoendpt_eptctldis0_isoendpt(&self) -> &ISOENDPT_EPTCTLDIS0_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(264usize)
                as *const ISOENDPT_EPTCTLDIS0_ISOENDPT)
        }
    }
    #[doc = "0x108 - UDPHS Endpoint Control Disable Register (endpoint = 0)"]
    #[inline(always)]
    pub fn eptctldis0(&self) -> &EPTCTLDIS0 {
        unsafe { &*(((self as *const Self) as *const u8).add(264usize) as *const EPTCTLDIS0) }
    }
    #[doc = "0x10c - UDPHS Endpoint Control Register (endpoint = 0)"]
    #[inline(always)]
    pub fn isoendpt_eptctl0_isoendpt(&self) -> &ISOENDPT_EPTCTL0_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(268usize)
                as *const ISOENDPT_EPTCTL0_ISOENDPT)
        }
    }
    #[doc = "0x10c - UDPHS Endpoint Control Register (endpoint = 0)"]
    #[inline(always)]
    pub fn eptctl0(&self) -> &EPTCTL0 {
        unsafe { &*(((self as *const Self) as *const u8).add(268usize) as *const EPTCTL0) }
    }
    #[doc = "0x114 - UDPHS Endpoint Set Status Register (endpoint = 0)"]
    #[inline(always)]
    pub fn isoendpt_eptsetsta0_isoendpt(&self) -> &ISOENDPT_EPTSETSTA0_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(276usize)
                as *const ISOENDPT_EPTSETSTA0_ISOENDPT)
        }
    }
    #[doc = "0x114 - UDPHS Endpoint Set Status Register (endpoint = 0)"]
    #[inline(always)]
    pub fn eptsetsta0(&self) -> &EPTSETSTA0 {
        unsafe { &*(((self as *const Self) as *const u8).add(276usize) as *const EPTSETSTA0) }
    }
    #[doc = "0x118 - UDPHS Endpoint Clear Status Register (endpoint = 0)"]
    #[inline(always)]
    pub fn isoendpt_eptclrsta0_isoendpt(&self) -> &ISOENDPT_EPTCLRSTA0_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(280usize)
                as *const ISOENDPT_EPTCLRSTA0_ISOENDPT)
        }
    }
    #[doc = "0x118 - UDPHS Endpoint Clear Status Register (endpoint = 0)"]
    #[inline(always)]
    pub fn eptclrsta0(&self) -> &EPTCLRSTA0 {
        unsafe { &*(((self as *const Self) as *const u8).add(280usize) as *const EPTCLRSTA0) }
    }
    #[doc = "0x11c - UDPHS Endpoint Status Register (endpoint = 0)"]
    #[inline(always)]
    pub fn isoendpt_eptsta0_isoendpt(&self) -> &ISOENDPT_EPTSTA0_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(284usize)
                as *const ISOENDPT_EPTSTA0_ISOENDPT)
        }
    }
    #[doc = "0x11c - UDPHS Endpoint Status Register (endpoint = 0)"]
    #[inline(always)]
    pub fn eptsta0(&self) -> &EPTSTA0 {
        unsafe { &*(((self as *const Self) as *const u8).add(284usize) as *const EPTSTA0) }
    }
    #[doc = "0x124 - UDPHS Endpoint Control Enable Register (endpoint = 1)"]
    #[inline(always)]
    pub fn isoendpt_eptctlenb1_isoendpt(&self) -> &ISOENDPT_EPTCTLENB1_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(292usize)
                as *const ISOENDPT_EPTCTLENB1_ISOENDPT)
        }
    }
    #[doc = "0x124 - UDPHS Endpoint Control Enable Register (endpoint = 1)"]
    #[inline(always)]
    pub fn eptctlenb1(&self) -> &EPTCTLENB1 {
        unsafe { &*(((self as *const Self) as *const u8).add(292usize) as *const EPTCTLENB1) }
    }
    #[doc = "0x128 - UDPHS Endpoint Control Disable Register (endpoint = 1)"]
    #[inline(always)]
    pub fn isoendpt_eptctldis1_isoendpt(&self) -> &ISOENDPT_EPTCTLDIS1_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(296usize)
                as *const ISOENDPT_EPTCTLDIS1_ISOENDPT)
        }
    }
    #[doc = "0x128 - UDPHS Endpoint Control Disable Register (endpoint = 1)"]
    #[inline(always)]
    pub fn eptctldis1(&self) -> &EPTCTLDIS1 {
        unsafe { &*(((self as *const Self) as *const u8).add(296usize) as *const EPTCTLDIS1) }
    }
    #[doc = "0x12c - UDPHS Endpoint Control Register (endpoint = 1)"]
    #[inline(always)]
    pub fn isoendpt_eptctl1_isoendpt(&self) -> &ISOENDPT_EPTCTL1_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(300usize)
                as *const ISOENDPT_EPTCTL1_ISOENDPT)
        }
    }
    #[doc = "0x12c - UDPHS Endpoint Control Register (endpoint = 1)"]
    #[inline(always)]
    pub fn eptctl1(&self) -> &EPTCTL1 {
        unsafe { &*(((self as *const Self) as *const u8).add(300usize) as *const EPTCTL1) }
    }
    #[doc = "0x134 - UDPHS Endpoint Set Status Register (endpoint = 1)"]
    #[inline(always)]
    pub fn isoendpt_eptsetsta1_isoendpt(&self) -> &ISOENDPT_EPTSETSTA1_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(308usize)
                as *const ISOENDPT_EPTSETSTA1_ISOENDPT)
        }
    }
    #[doc = "0x134 - UDPHS Endpoint Set Status Register (endpoint = 1)"]
    #[inline(always)]
    pub fn eptsetsta1(&self) -> &EPTSETSTA1 {
        unsafe { &*(((self as *const Self) as *const u8).add(308usize) as *const EPTSETSTA1) }
    }
    #[doc = "0x138 - UDPHS Endpoint Clear Status Register (endpoint = 1)"]
    #[inline(always)]
    pub fn isoendpt_eptclrsta1_isoendpt(&self) -> &ISOENDPT_EPTCLRSTA1_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(312usize)
                as *const ISOENDPT_EPTCLRSTA1_ISOENDPT)
        }
    }
    #[doc = "0x138 - UDPHS Endpoint Clear Status Register (endpoint = 1)"]
    #[inline(always)]
    pub fn eptclrsta1(&self) -> &EPTCLRSTA1 {
        unsafe { &*(((self as *const Self) as *const u8).add(312usize) as *const EPTCLRSTA1) }
    }
    #[doc = "0x13c - UDPHS Endpoint Status Register (endpoint = 1)"]
    #[inline(always)]
    pub fn isoendpt_eptsta1_isoendpt(&self) -> &ISOENDPT_EPTSTA1_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(316usize)
                as *const ISOENDPT_EPTSTA1_ISOENDPT)
        }
    }
    #[doc = "0x13c - UDPHS Endpoint Status Register (endpoint = 1)"]
    #[inline(always)]
    pub fn eptsta1(&self) -> &EPTSTA1 {
        unsafe { &*(((self as *const Self) as *const u8).add(316usize) as *const EPTSTA1) }
    }
    #[doc = "0x144 - UDPHS Endpoint Control Enable Register (endpoint = 2)"]
    #[inline(always)]
    pub fn isoendpt_eptctlenb2_isoendpt(&self) -> &ISOENDPT_EPTCTLENB2_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(324usize)
                as *const ISOENDPT_EPTCTLENB2_ISOENDPT)
        }
    }
    #[doc = "0x144 - UDPHS Endpoint Control Enable Register (endpoint = 2)"]
    #[inline(always)]
    pub fn eptctlenb2(&self) -> &EPTCTLENB2 {
        unsafe { &*(((self as *const Self) as *const u8).add(324usize) as *const EPTCTLENB2) }
    }
    #[doc = "0x148 - UDPHS Endpoint Control Disable Register (endpoint = 2)"]
    #[inline(always)]
    pub fn isoendpt_eptctldis2_isoendpt(&self) -> &ISOENDPT_EPTCTLDIS2_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(328usize)
                as *const ISOENDPT_EPTCTLDIS2_ISOENDPT)
        }
    }
    #[doc = "0x148 - UDPHS Endpoint Control Disable Register (endpoint = 2)"]
    #[inline(always)]
    pub fn eptctldis2(&self) -> &EPTCTLDIS2 {
        unsafe { &*(((self as *const Self) as *const u8).add(328usize) as *const EPTCTLDIS2) }
    }
    #[doc = "0x14c - UDPHS Endpoint Control Register (endpoint = 2)"]
    #[inline(always)]
    pub fn isoendpt_eptctl2_isoendpt(&self) -> &ISOENDPT_EPTCTL2_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(332usize)
                as *const ISOENDPT_EPTCTL2_ISOENDPT)
        }
    }
    #[doc = "0x14c - UDPHS Endpoint Control Register (endpoint = 2)"]
    #[inline(always)]
    pub fn eptctl2(&self) -> &EPTCTL2 {
        unsafe { &*(((self as *const Self) as *const u8).add(332usize) as *const EPTCTL2) }
    }
    #[doc = "0x154 - UDPHS Endpoint Set Status Register (endpoint = 2)"]
    #[inline(always)]
    pub fn isoendpt_eptsetsta2_isoendpt(&self) -> &ISOENDPT_EPTSETSTA2_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(340usize)
                as *const ISOENDPT_EPTSETSTA2_ISOENDPT)
        }
    }
    #[doc = "0x154 - UDPHS Endpoint Set Status Register (endpoint = 2)"]
    #[inline(always)]
    pub fn eptsetsta2(&self) -> &EPTSETSTA2 {
        unsafe { &*(((self as *const Self) as *const u8).add(340usize) as *const EPTSETSTA2) }
    }
    #[doc = "0x158 - UDPHS Endpoint Clear Status Register (endpoint = 2)"]
    #[inline(always)]
    pub fn isoendpt_eptclrsta2_isoendpt(&self) -> &ISOENDPT_EPTCLRSTA2_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(344usize)
                as *const ISOENDPT_EPTCLRSTA2_ISOENDPT)
        }
    }
    #[doc = "0x158 - UDPHS Endpoint Clear Status Register (endpoint = 2)"]
    #[inline(always)]
    pub fn eptclrsta2(&self) -> &EPTCLRSTA2 {
        unsafe { &*(((self as *const Self) as *const u8).add(344usize) as *const EPTCLRSTA2) }
    }
    #[doc = "0x15c - UDPHS Endpoint Status Register (endpoint = 2)"]
    #[inline(always)]
    pub fn isoendpt_eptsta2_isoendpt(&self) -> &ISOENDPT_EPTSTA2_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(348usize)
                as *const ISOENDPT_EPTSTA2_ISOENDPT)
        }
    }
    #[doc = "0x15c - UDPHS Endpoint Status Register (endpoint = 2)"]
    #[inline(always)]
    pub fn eptsta2(&self) -> &EPTSTA2 {
        unsafe { &*(((self as *const Self) as *const u8).add(348usize) as *const EPTSTA2) }
    }
    #[doc = "0x164 - UDPHS Endpoint Control Enable Register (endpoint = 3)"]
    #[inline(always)]
    pub fn isoendpt_eptctlenb3_isoendpt(&self) -> &ISOENDPT_EPTCTLENB3_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(356usize)
                as *const ISOENDPT_EPTCTLENB3_ISOENDPT)
        }
    }
    #[doc = "0x164 - UDPHS Endpoint Control Enable Register (endpoint = 3)"]
    #[inline(always)]
    pub fn eptctlenb3(&self) -> &EPTCTLENB3 {
        unsafe { &*(((self as *const Self) as *const u8).add(356usize) as *const EPTCTLENB3) }
    }
    #[doc = "0x168 - UDPHS Endpoint Control Disable Register (endpoint = 3)"]
    #[inline(always)]
    pub fn isoendpt_eptctldis3_isoendpt(&self) -> &ISOENDPT_EPTCTLDIS3_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(360usize)
                as *const ISOENDPT_EPTCTLDIS3_ISOENDPT)
        }
    }
    #[doc = "0x168 - UDPHS Endpoint Control Disable Register (endpoint = 3)"]
    #[inline(always)]
    pub fn eptctldis3(&self) -> &EPTCTLDIS3 {
        unsafe { &*(((self as *const Self) as *const u8).add(360usize) as *const EPTCTLDIS3) }
    }
    #[doc = "0x16c - UDPHS Endpoint Control Register (endpoint = 3)"]
    #[inline(always)]
    pub fn isoendpt_eptctl3_isoendpt(&self) -> &ISOENDPT_EPTCTL3_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(364usize)
                as *const ISOENDPT_EPTCTL3_ISOENDPT)
        }
    }
    #[doc = "0x16c - UDPHS Endpoint Control Register (endpoint = 3)"]
    #[inline(always)]
    pub fn eptctl3(&self) -> &EPTCTL3 {
        unsafe { &*(((self as *const Self) as *const u8).add(364usize) as *const EPTCTL3) }
    }
    #[doc = "0x174 - UDPHS Endpoint Set Status Register (endpoint = 3)"]
    #[inline(always)]
    pub fn isoendpt_eptsetsta3_isoendpt(&self) -> &ISOENDPT_EPTSETSTA3_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(372usize)
                as *const ISOENDPT_EPTSETSTA3_ISOENDPT)
        }
    }
    #[doc = "0x174 - UDPHS Endpoint Set Status Register (endpoint = 3)"]
    #[inline(always)]
    pub fn eptsetsta3(&self) -> &EPTSETSTA3 {
        unsafe { &*(((self as *const Self) as *const u8).add(372usize) as *const EPTSETSTA3) }
    }
    #[doc = "0x178 - UDPHS Endpoint Clear Status Register (endpoint = 3)"]
    #[inline(always)]
    pub fn isoendpt_eptclrsta3_isoendpt(&self) -> &ISOENDPT_EPTCLRSTA3_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(376usize)
                as *const ISOENDPT_EPTCLRSTA3_ISOENDPT)
        }
    }
    #[doc = "0x178 - UDPHS Endpoint Clear Status Register (endpoint = 3)"]
    #[inline(always)]
    pub fn eptclrsta3(&self) -> &EPTCLRSTA3 {
        unsafe { &*(((self as *const Self) as *const u8).add(376usize) as *const EPTCLRSTA3) }
    }
    #[doc = "0x17c - UDPHS Endpoint Status Register (endpoint = 3)"]
    #[inline(always)]
    pub fn isoendpt_eptsta3_isoendpt(&self) -> &ISOENDPT_EPTSTA3_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(380usize)
                as *const ISOENDPT_EPTSTA3_ISOENDPT)
        }
    }
    #[doc = "0x17c - UDPHS Endpoint Status Register (endpoint = 3)"]
    #[inline(always)]
    pub fn eptsta3(&self) -> &EPTSTA3 {
        unsafe { &*(((self as *const Self) as *const u8).add(380usize) as *const EPTSTA3) }
    }
    #[doc = "0x184 - UDPHS Endpoint Control Enable Register (endpoint = 4)"]
    #[inline(always)]
    pub fn isoendpt_eptctlenb4_isoendpt(&self) -> &ISOENDPT_EPTCTLENB4_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(388usize)
                as *const ISOENDPT_EPTCTLENB4_ISOENDPT)
        }
    }
    #[doc = "0x184 - UDPHS Endpoint Control Enable Register (endpoint = 4)"]
    #[inline(always)]
    pub fn eptctlenb4(&self) -> &EPTCTLENB4 {
        unsafe { &*(((self as *const Self) as *const u8).add(388usize) as *const EPTCTLENB4) }
    }
    #[doc = "0x188 - UDPHS Endpoint Control Disable Register (endpoint = 4)"]
    #[inline(always)]
    pub fn isoendpt_eptctldis4_isoendpt(&self) -> &ISOENDPT_EPTCTLDIS4_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(392usize)
                as *const ISOENDPT_EPTCTLDIS4_ISOENDPT)
        }
    }
    #[doc = "0x188 - UDPHS Endpoint Control Disable Register (endpoint = 4)"]
    #[inline(always)]
    pub fn eptctldis4(&self) -> &EPTCTLDIS4 {
        unsafe { &*(((self as *const Self) as *const u8).add(392usize) as *const EPTCTLDIS4) }
    }
    #[doc = "0x18c - UDPHS Endpoint Control Register (endpoint = 4)"]
    #[inline(always)]
    pub fn isoendpt_eptctl4_isoendpt(&self) -> &ISOENDPT_EPTCTL4_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(396usize)
                as *const ISOENDPT_EPTCTL4_ISOENDPT)
        }
    }
    #[doc = "0x18c - UDPHS Endpoint Control Register (endpoint = 4)"]
    #[inline(always)]
    pub fn eptctl4(&self) -> &EPTCTL4 {
        unsafe { &*(((self as *const Self) as *const u8).add(396usize) as *const EPTCTL4) }
    }
    #[doc = "0x194 - UDPHS Endpoint Set Status Register (endpoint = 4)"]
    #[inline(always)]
    pub fn isoendpt_eptsetsta4_isoendpt(&self) -> &ISOENDPT_EPTSETSTA4_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(404usize)
                as *const ISOENDPT_EPTSETSTA4_ISOENDPT)
        }
    }
    #[doc = "0x194 - UDPHS Endpoint Set Status Register (endpoint = 4)"]
    #[inline(always)]
    pub fn eptsetsta4(&self) -> &EPTSETSTA4 {
        unsafe { &*(((self as *const Self) as *const u8).add(404usize) as *const EPTSETSTA4) }
    }
    #[doc = "0x198 - UDPHS Endpoint Clear Status Register (endpoint = 4)"]
    #[inline(always)]
    pub fn isoendpt_eptclrsta4_isoendpt(&self) -> &ISOENDPT_EPTCLRSTA4_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(408usize)
                as *const ISOENDPT_EPTCLRSTA4_ISOENDPT)
        }
    }
    #[doc = "0x198 - UDPHS Endpoint Clear Status Register (endpoint = 4)"]
    #[inline(always)]
    pub fn eptclrsta4(&self) -> &EPTCLRSTA4 {
        unsafe { &*(((self as *const Self) as *const u8).add(408usize) as *const EPTCLRSTA4) }
    }
    #[doc = "0x19c - UDPHS Endpoint Status Register (endpoint = 4)"]
    #[inline(always)]
    pub fn isoendpt_eptsta4_isoendpt(&self) -> &ISOENDPT_EPTSTA4_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(412usize)
                as *const ISOENDPT_EPTSTA4_ISOENDPT)
        }
    }
    #[doc = "0x19c - UDPHS Endpoint Status Register (endpoint = 4)"]
    #[inline(always)]
    pub fn eptsta4(&self) -> &EPTSTA4 {
        unsafe { &*(((self as *const Self) as *const u8).add(412usize) as *const EPTSTA4) }
    }
    #[doc = "0x1a4 - UDPHS Endpoint Control Enable Register (endpoint = 5)"]
    #[inline(always)]
    pub fn isoendpt_eptctlenb5_isoendpt(&self) -> &ISOENDPT_EPTCTLENB5_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(420usize)
                as *const ISOENDPT_EPTCTLENB5_ISOENDPT)
        }
    }
    #[doc = "0x1a4 - UDPHS Endpoint Control Enable Register (endpoint = 5)"]
    #[inline(always)]
    pub fn eptctlenb5(&self) -> &EPTCTLENB5 {
        unsafe { &*(((self as *const Self) as *const u8).add(420usize) as *const EPTCTLENB5) }
    }
    #[doc = "0x1a8 - UDPHS Endpoint Control Disable Register (endpoint = 5)"]
    #[inline(always)]
    pub fn isoendpt_eptctldis5_isoendpt(&self) -> &ISOENDPT_EPTCTLDIS5_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(424usize)
                as *const ISOENDPT_EPTCTLDIS5_ISOENDPT)
        }
    }
    #[doc = "0x1a8 - UDPHS Endpoint Control Disable Register (endpoint = 5)"]
    #[inline(always)]
    pub fn eptctldis5(&self) -> &EPTCTLDIS5 {
        unsafe { &*(((self as *const Self) as *const u8).add(424usize) as *const EPTCTLDIS5) }
    }
    #[doc = "0x1ac - UDPHS Endpoint Control Register (endpoint = 5)"]
    #[inline(always)]
    pub fn isoendpt_eptctl5_isoendpt(&self) -> &ISOENDPT_EPTCTL5_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(428usize)
                as *const ISOENDPT_EPTCTL5_ISOENDPT)
        }
    }
    #[doc = "0x1ac - UDPHS Endpoint Control Register (endpoint = 5)"]
    #[inline(always)]
    pub fn eptctl5(&self) -> &EPTCTL5 {
        unsafe { &*(((self as *const Self) as *const u8).add(428usize) as *const EPTCTL5) }
    }
    #[doc = "0x1b4 - UDPHS Endpoint Set Status Register (endpoint = 5)"]
    #[inline(always)]
    pub fn isoendpt_eptsetsta5_isoendpt(&self) -> &ISOENDPT_EPTSETSTA5_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(436usize)
                as *const ISOENDPT_EPTSETSTA5_ISOENDPT)
        }
    }
    #[doc = "0x1b4 - UDPHS Endpoint Set Status Register (endpoint = 5)"]
    #[inline(always)]
    pub fn eptsetsta5(&self) -> &EPTSETSTA5 {
        unsafe { &*(((self as *const Self) as *const u8).add(436usize) as *const EPTSETSTA5) }
    }
    #[doc = "0x1b8 - UDPHS Endpoint Clear Status Register (endpoint = 5)"]
    #[inline(always)]
    pub fn isoendpt_eptclrsta5_isoendpt(&self) -> &ISOENDPT_EPTCLRSTA5_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(440usize)
                as *const ISOENDPT_EPTCLRSTA5_ISOENDPT)
        }
    }
    #[doc = "0x1b8 - UDPHS Endpoint Clear Status Register (endpoint = 5)"]
    #[inline(always)]
    pub fn eptclrsta5(&self) -> &EPTCLRSTA5 {
        unsafe { &*(((self as *const Self) as *const u8).add(440usize) as *const EPTCLRSTA5) }
    }
    #[doc = "0x1bc - UDPHS Endpoint Status Register (endpoint = 5)"]
    #[inline(always)]
    pub fn isoendpt_eptsta5_isoendpt(&self) -> &ISOENDPT_EPTSTA5_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(444usize)
                as *const ISOENDPT_EPTSTA5_ISOENDPT)
        }
    }
    #[doc = "0x1bc - UDPHS Endpoint Status Register (endpoint = 5)"]
    #[inline(always)]
    pub fn eptsta5(&self) -> &EPTSTA5 {
        unsafe { &*(((self as *const Self) as *const u8).add(444usize) as *const EPTSTA5) }
    }
    #[doc = "0x1c4 - UDPHS Endpoint Control Enable Register (endpoint = 6)"]
    #[inline(always)]
    pub fn isoendpt_eptctlenb6_isoendpt(&self) -> &ISOENDPT_EPTCTLENB6_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(452usize)
                as *const ISOENDPT_EPTCTLENB6_ISOENDPT)
        }
    }
    #[doc = "0x1c4 - UDPHS Endpoint Control Enable Register (endpoint = 6)"]
    #[inline(always)]
    pub fn eptctlenb6(&self) -> &EPTCTLENB6 {
        unsafe { &*(((self as *const Self) as *const u8).add(452usize) as *const EPTCTLENB6) }
    }
    #[doc = "0x1c8 - UDPHS Endpoint Control Disable Register (endpoint = 6)"]
    #[inline(always)]
    pub fn isoendpt_eptctldis6_isoendpt(&self) -> &ISOENDPT_EPTCTLDIS6_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(456usize)
                as *const ISOENDPT_EPTCTLDIS6_ISOENDPT)
        }
    }
    #[doc = "0x1c8 - UDPHS Endpoint Control Disable Register (endpoint = 6)"]
    #[inline(always)]
    pub fn eptctldis6(&self) -> &EPTCTLDIS6 {
        unsafe { &*(((self as *const Self) as *const u8).add(456usize) as *const EPTCTLDIS6) }
    }
    #[doc = "0x1cc - UDPHS Endpoint Control Register (endpoint = 6)"]
    #[inline(always)]
    pub fn isoendpt_eptctl6_isoendpt(&self) -> &ISOENDPT_EPTCTL6_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(460usize)
                as *const ISOENDPT_EPTCTL6_ISOENDPT)
        }
    }
    #[doc = "0x1cc - UDPHS Endpoint Control Register (endpoint = 6)"]
    #[inline(always)]
    pub fn eptctl6(&self) -> &EPTCTL6 {
        unsafe { &*(((self as *const Self) as *const u8).add(460usize) as *const EPTCTL6) }
    }
    #[doc = "0x1d4 - UDPHS Endpoint Set Status Register (endpoint = 6)"]
    #[inline(always)]
    pub fn isoendpt_eptsetsta6_isoendpt(&self) -> &ISOENDPT_EPTSETSTA6_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(468usize)
                as *const ISOENDPT_EPTSETSTA6_ISOENDPT)
        }
    }
    #[doc = "0x1d4 - UDPHS Endpoint Set Status Register (endpoint = 6)"]
    #[inline(always)]
    pub fn eptsetsta6(&self) -> &EPTSETSTA6 {
        unsafe { &*(((self as *const Self) as *const u8).add(468usize) as *const EPTSETSTA6) }
    }
    #[doc = "0x1d8 - UDPHS Endpoint Clear Status Register (endpoint = 6)"]
    #[inline(always)]
    pub fn isoendpt_eptclrsta6_isoendpt(&self) -> &ISOENDPT_EPTCLRSTA6_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(472usize)
                as *const ISOENDPT_EPTCLRSTA6_ISOENDPT)
        }
    }
    #[doc = "0x1d8 - UDPHS Endpoint Clear Status Register (endpoint = 6)"]
    #[inline(always)]
    pub fn eptclrsta6(&self) -> &EPTCLRSTA6 {
        unsafe { &*(((self as *const Self) as *const u8).add(472usize) as *const EPTCLRSTA6) }
    }
    #[doc = "0x1dc - UDPHS Endpoint Status Register (endpoint = 6)"]
    #[inline(always)]
    pub fn isoendpt_eptsta6_isoendpt(&self) -> &ISOENDPT_EPTSTA6_ISOENDPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(476usize)
                as *const ISOENDPT_EPTSTA6_ISOENDPT)
        }
    }
    #[doc = "0x1dc - UDPHS Endpoint Status Register (endpoint = 6)"]
    #[inline(always)]
    pub fn eptsta6(&self) -> &EPTSTA6 {
        unsafe { &*(((self as *const Self) as *const u8).add(476usize) as *const EPTSTA6) }
    }
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "UDPHS Control Register"]
pub mod ctrl;
#[doc = "FNUM (r) register accessor: an alias for `Reg<FNUM_SPEC>`"]
pub type FNUM = crate::Reg<fnum::FNUM_SPEC>;
#[doc = "UDPHS Frame Number Register"]
pub mod fnum;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "UDPHS Interrupt Enable Register"]
pub mod ien;
#[doc = "INTSTA (r) register accessor: an alias for `Reg<INTSTA_SPEC>`"]
pub type INTSTA = crate::Reg<intsta::INTSTA_SPEC>;
#[doc = "UDPHS Interrupt Status Register"]
pub mod intsta;
#[doc = "CLRINT (w) register accessor: an alias for `Reg<CLRINT_SPEC>`"]
pub type CLRINT = crate::Reg<clrint::CLRINT_SPEC>;
#[doc = "UDPHS Clear Interrupt Register"]
pub mod clrint;
#[doc = "EPTRST (w) register accessor: an alias for `Reg<EPTRST_SPEC>`"]
pub type EPTRST = crate::Reg<eptrst::EPTRST_SPEC>;
#[doc = "UDPHS Endpoints Reset Register"]
pub mod eptrst;
#[doc = "TST (rw) register accessor: an alias for `Reg<TST_SPEC>`"]
pub type TST = crate::Reg<tst::TST_SPEC>;
#[doc = "UDPHS Test Register"]
pub mod tst;
#[doc = "EPTCFG0 (rw) register accessor: an alias for `Reg<EPTCFG0_SPEC>`"]
pub type EPTCFG0 = crate::Reg<eptcfg0::EPTCFG0_SPEC>;
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 0)"]
pub mod eptcfg0;
#[doc = "EPTCTLENB0 (w) register accessor: an alias for `Reg<EPTCTLENB0_SPEC>`"]
pub type EPTCTLENB0 = crate::Reg<eptctlenb0::EPTCTLENB0_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 0)"]
pub mod eptctlenb0;
#[doc = "ISOENDPT_EPTCTLENB0_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTCTLENB0_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCTLENB0_ISOENDPT =
    crate::Reg<isoendpt_eptctlenb0_isoendpt::ISOENDPT_EPTCTLENB0_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 0)"]
pub mod isoendpt_eptctlenb0_isoendpt;
#[doc = "EPTCTLDIS0 (w) register accessor: an alias for `Reg<EPTCTLDIS0_SPEC>`"]
pub type EPTCTLDIS0 = crate::Reg<eptctldis0::EPTCTLDIS0_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 0)"]
pub mod eptctldis0;
#[doc = "ISOENDPT_EPTCTLDIS0_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTCTLDIS0_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCTLDIS0_ISOENDPT =
    crate::Reg<isoendpt_eptctldis0_isoendpt::ISOENDPT_EPTCTLDIS0_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 0)"]
pub mod isoendpt_eptctldis0_isoendpt;
#[doc = "EPTCTL0 (r) register accessor: an alias for `Reg<EPTCTL0_SPEC>`"]
pub type EPTCTL0 = crate::Reg<eptctl0::EPTCTL0_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 0)"]
pub mod eptctl0;
#[doc = "ISOENDPT_EPTCTL0_ISOENDPT (r) register accessor: an alias for `Reg<ISOENDPT_EPTCTL0_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCTL0_ISOENDPT =
    crate::Reg<isoendpt_eptctl0_isoendpt::ISOENDPT_EPTCTL0_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 0)"]
pub mod isoendpt_eptctl0_isoendpt;
#[doc = "EPTSETSTA0 (w) register accessor: an alias for `Reg<EPTSETSTA0_SPEC>`"]
pub type EPTSETSTA0 = crate::Reg<eptsetsta0::EPTSETSTA0_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 0)"]
pub mod eptsetsta0;
#[doc = "ISOENDPT_EPTSETSTA0_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTSETSTA0_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTSETSTA0_ISOENDPT =
    crate::Reg<isoendpt_eptsetsta0_isoendpt::ISOENDPT_EPTSETSTA0_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 0)"]
pub mod isoendpt_eptsetsta0_isoendpt;
#[doc = "EPTCLRSTA0 (w) register accessor: an alias for `Reg<EPTCLRSTA0_SPEC>`"]
pub type EPTCLRSTA0 = crate::Reg<eptclrsta0::EPTCLRSTA0_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 0)"]
pub mod eptclrsta0;
#[doc = "ISOENDPT_EPTCLRSTA0_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTCLRSTA0_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCLRSTA0_ISOENDPT =
    crate::Reg<isoendpt_eptclrsta0_isoendpt::ISOENDPT_EPTCLRSTA0_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 0)"]
pub mod isoendpt_eptclrsta0_isoendpt;
#[doc = "EPTSTA0 (r) register accessor: an alias for `Reg<EPTSTA0_SPEC>`"]
pub type EPTSTA0 = crate::Reg<eptsta0::EPTSTA0_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 0)"]
pub mod eptsta0;
#[doc = "ISOENDPT_EPTSTA0_ISOENDPT (r) register accessor: an alias for `Reg<ISOENDPT_EPTSTA0_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTSTA0_ISOENDPT =
    crate::Reg<isoendpt_eptsta0_isoendpt::ISOENDPT_EPTSTA0_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 0)"]
pub mod isoendpt_eptsta0_isoendpt;
#[doc = "EPTCFG1 (rw) register accessor: an alias for `Reg<EPTCFG1_SPEC>`"]
pub type EPTCFG1 = crate::Reg<eptcfg1::EPTCFG1_SPEC>;
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 1)"]
pub mod eptcfg1;
#[doc = "EPTCTLENB1 (w) register accessor: an alias for `Reg<EPTCTLENB1_SPEC>`"]
pub type EPTCTLENB1 = crate::Reg<eptctlenb1::EPTCTLENB1_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 1)"]
pub mod eptctlenb1;
#[doc = "ISOENDPT_EPTCTLENB1_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTCTLENB1_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCTLENB1_ISOENDPT =
    crate::Reg<isoendpt_eptctlenb1_isoendpt::ISOENDPT_EPTCTLENB1_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 1)"]
pub mod isoendpt_eptctlenb1_isoendpt;
#[doc = "EPTCTLDIS1 (w) register accessor: an alias for `Reg<EPTCTLDIS1_SPEC>`"]
pub type EPTCTLDIS1 = crate::Reg<eptctldis1::EPTCTLDIS1_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 1)"]
pub mod eptctldis1;
#[doc = "ISOENDPT_EPTCTLDIS1_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTCTLDIS1_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCTLDIS1_ISOENDPT =
    crate::Reg<isoendpt_eptctldis1_isoendpt::ISOENDPT_EPTCTLDIS1_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 1)"]
pub mod isoendpt_eptctldis1_isoendpt;
#[doc = "EPTCTL1 (r) register accessor: an alias for `Reg<EPTCTL1_SPEC>`"]
pub type EPTCTL1 = crate::Reg<eptctl1::EPTCTL1_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 1)"]
pub mod eptctl1;
#[doc = "ISOENDPT_EPTCTL1_ISOENDPT (r) register accessor: an alias for `Reg<ISOENDPT_EPTCTL1_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCTL1_ISOENDPT =
    crate::Reg<isoendpt_eptctl1_isoendpt::ISOENDPT_EPTCTL1_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 1)"]
pub mod isoendpt_eptctl1_isoendpt;
#[doc = "EPTSETSTA1 (w) register accessor: an alias for `Reg<EPTSETSTA1_SPEC>`"]
pub type EPTSETSTA1 = crate::Reg<eptsetsta1::EPTSETSTA1_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 1)"]
pub mod eptsetsta1;
#[doc = "ISOENDPT_EPTSETSTA1_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTSETSTA1_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTSETSTA1_ISOENDPT =
    crate::Reg<isoendpt_eptsetsta1_isoendpt::ISOENDPT_EPTSETSTA1_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 1)"]
pub mod isoendpt_eptsetsta1_isoendpt;
#[doc = "EPTCLRSTA1 (w) register accessor: an alias for `Reg<EPTCLRSTA1_SPEC>`"]
pub type EPTCLRSTA1 = crate::Reg<eptclrsta1::EPTCLRSTA1_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 1)"]
pub mod eptclrsta1;
#[doc = "ISOENDPT_EPTCLRSTA1_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTCLRSTA1_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCLRSTA1_ISOENDPT =
    crate::Reg<isoendpt_eptclrsta1_isoendpt::ISOENDPT_EPTCLRSTA1_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 1)"]
pub mod isoendpt_eptclrsta1_isoendpt;
#[doc = "EPTSTA1 (r) register accessor: an alias for `Reg<EPTSTA1_SPEC>`"]
pub type EPTSTA1 = crate::Reg<eptsta1::EPTSTA1_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 1)"]
pub mod eptsta1;
#[doc = "ISOENDPT_EPTSTA1_ISOENDPT (r) register accessor: an alias for `Reg<ISOENDPT_EPTSTA1_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTSTA1_ISOENDPT =
    crate::Reg<isoendpt_eptsta1_isoendpt::ISOENDPT_EPTSTA1_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 1)"]
pub mod isoendpt_eptsta1_isoendpt;
#[doc = "EPTCFG2 (rw) register accessor: an alias for `Reg<EPTCFG2_SPEC>`"]
pub type EPTCFG2 = crate::Reg<eptcfg2::EPTCFG2_SPEC>;
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 2)"]
pub mod eptcfg2;
#[doc = "EPTCTLENB2 (w) register accessor: an alias for `Reg<EPTCTLENB2_SPEC>`"]
pub type EPTCTLENB2 = crate::Reg<eptctlenb2::EPTCTLENB2_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 2)"]
pub mod eptctlenb2;
#[doc = "ISOENDPT_EPTCTLENB2_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTCTLENB2_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCTLENB2_ISOENDPT =
    crate::Reg<isoendpt_eptctlenb2_isoendpt::ISOENDPT_EPTCTLENB2_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 2)"]
pub mod isoendpt_eptctlenb2_isoendpt;
#[doc = "EPTCTLDIS2 (w) register accessor: an alias for `Reg<EPTCTLDIS2_SPEC>`"]
pub type EPTCTLDIS2 = crate::Reg<eptctldis2::EPTCTLDIS2_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 2)"]
pub mod eptctldis2;
#[doc = "ISOENDPT_EPTCTLDIS2_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCTLDIS2_ISOENDPT =
    crate::Reg<isoendpt_eptctldis2_isoendpt::ISOENDPT_EPTCTLDIS2_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 2)"]
pub mod isoendpt_eptctldis2_isoendpt;
#[doc = "EPTCTL2 (r) register accessor: an alias for `Reg<EPTCTL2_SPEC>`"]
pub type EPTCTL2 = crate::Reg<eptctl2::EPTCTL2_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 2)"]
pub mod eptctl2;
#[doc = "ISOENDPT_EPTCTL2_ISOENDPT (r) register accessor: an alias for `Reg<ISOENDPT_EPTCTL2_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCTL2_ISOENDPT =
    crate::Reg<isoendpt_eptctl2_isoendpt::ISOENDPT_EPTCTL2_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 2)"]
pub mod isoendpt_eptctl2_isoendpt;
#[doc = "EPTSETSTA2 (w) register accessor: an alias for `Reg<EPTSETSTA2_SPEC>`"]
pub type EPTSETSTA2 = crate::Reg<eptsetsta2::EPTSETSTA2_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 2)"]
pub mod eptsetsta2;
#[doc = "ISOENDPT_EPTSETSTA2_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTSETSTA2_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTSETSTA2_ISOENDPT =
    crate::Reg<isoendpt_eptsetsta2_isoendpt::ISOENDPT_EPTSETSTA2_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 2)"]
pub mod isoendpt_eptsetsta2_isoendpt;
#[doc = "EPTCLRSTA2 (w) register accessor: an alias for `Reg<EPTCLRSTA2_SPEC>`"]
pub type EPTCLRSTA2 = crate::Reg<eptclrsta2::EPTCLRSTA2_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 2)"]
pub mod eptclrsta2;
#[doc = "ISOENDPT_EPTCLRSTA2_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTCLRSTA2_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCLRSTA2_ISOENDPT =
    crate::Reg<isoendpt_eptclrsta2_isoendpt::ISOENDPT_EPTCLRSTA2_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 2)"]
pub mod isoendpt_eptclrsta2_isoendpt;
#[doc = "EPTSTA2 (r) register accessor: an alias for `Reg<EPTSTA2_SPEC>`"]
pub type EPTSTA2 = crate::Reg<eptsta2::EPTSTA2_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 2)"]
pub mod eptsta2;
#[doc = "ISOENDPT_EPTSTA2_ISOENDPT (r) register accessor: an alias for `Reg<ISOENDPT_EPTSTA2_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTSTA2_ISOENDPT =
    crate::Reg<isoendpt_eptsta2_isoendpt::ISOENDPT_EPTSTA2_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 2)"]
pub mod isoendpt_eptsta2_isoendpt;
#[doc = "EPTCFG3 (rw) register accessor: an alias for `Reg<EPTCFG3_SPEC>`"]
pub type EPTCFG3 = crate::Reg<eptcfg3::EPTCFG3_SPEC>;
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 3)"]
pub mod eptcfg3;
#[doc = "EPTCTLENB3 (w) register accessor: an alias for `Reg<EPTCTLENB3_SPEC>`"]
pub type EPTCTLENB3 = crate::Reg<eptctlenb3::EPTCTLENB3_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 3)"]
pub mod eptctlenb3;
#[doc = "ISOENDPT_EPTCTLENB3_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTCTLENB3_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCTLENB3_ISOENDPT =
    crate::Reg<isoendpt_eptctlenb3_isoendpt::ISOENDPT_EPTCTLENB3_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 3)"]
pub mod isoendpt_eptctlenb3_isoendpt;
#[doc = "EPTCTLDIS3 (w) register accessor: an alias for `Reg<EPTCTLDIS3_SPEC>`"]
pub type EPTCTLDIS3 = crate::Reg<eptctldis3::EPTCTLDIS3_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 3)"]
pub mod eptctldis3;
#[doc = "ISOENDPT_EPTCTLDIS3_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTCTLDIS3_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCTLDIS3_ISOENDPT =
    crate::Reg<isoendpt_eptctldis3_isoendpt::ISOENDPT_EPTCTLDIS3_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 3)"]
pub mod isoendpt_eptctldis3_isoendpt;
#[doc = "EPTCTL3 (r) register accessor: an alias for `Reg<EPTCTL3_SPEC>`"]
pub type EPTCTL3 = crate::Reg<eptctl3::EPTCTL3_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 3)"]
pub mod eptctl3;
#[doc = "ISOENDPT_EPTCTL3_ISOENDPT (r) register accessor: an alias for `Reg<ISOENDPT_EPTCTL3_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCTL3_ISOENDPT =
    crate::Reg<isoendpt_eptctl3_isoendpt::ISOENDPT_EPTCTL3_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 3)"]
pub mod isoendpt_eptctl3_isoendpt;
#[doc = "EPTSETSTA3 (w) register accessor: an alias for `Reg<EPTSETSTA3_SPEC>`"]
pub type EPTSETSTA3 = crate::Reg<eptsetsta3::EPTSETSTA3_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 3)"]
pub mod eptsetsta3;
#[doc = "ISOENDPT_EPTSETSTA3_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTSETSTA3_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTSETSTA3_ISOENDPT =
    crate::Reg<isoendpt_eptsetsta3_isoendpt::ISOENDPT_EPTSETSTA3_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 3)"]
pub mod isoendpt_eptsetsta3_isoendpt;
#[doc = "EPTCLRSTA3 (w) register accessor: an alias for `Reg<EPTCLRSTA3_SPEC>`"]
pub type EPTCLRSTA3 = crate::Reg<eptclrsta3::EPTCLRSTA3_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 3)"]
pub mod eptclrsta3;
#[doc = "ISOENDPT_EPTCLRSTA3_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTCLRSTA3_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCLRSTA3_ISOENDPT =
    crate::Reg<isoendpt_eptclrsta3_isoendpt::ISOENDPT_EPTCLRSTA3_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 3)"]
pub mod isoendpt_eptclrsta3_isoendpt;
#[doc = "EPTSTA3 (r) register accessor: an alias for `Reg<EPTSTA3_SPEC>`"]
pub type EPTSTA3 = crate::Reg<eptsta3::EPTSTA3_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 3)"]
pub mod eptsta3;
#[doc = "ISOENDPT_EPTSTA3_ISOENDPT (r) register accessor: an alias for `Reg<ISOENDPT_EPTSTA3_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTSTA3_ISOENDPT =
    crate::Reg<isoendpt_eptsta3_isoendpt::ISOENDPT_EPTSTA3_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 3)"]
pub mod isoendpt_eptsta3_isoendpt;
#[doc = "EPTCFG4 (rw) register accessor: an alias for `Reg<EPTCFG4_SPEC>`"]
pub type EPTCFG4 = crate::Reg<eptcfg4::EPTCFG4_SPEC>;
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 4)"]
pub mod eptcfg4;
#[doc = "EPTCTLENB4 (w) register accessor: an alias for `Reg<EPTCTLENB4_SPEC>`"]
pub type EPTCTLENB4 = crate::Reg<eptctlenb4::EPTCTLENB4_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 4)"]
pub mod eptctlenb4;
#[doc = "ISOENDPT_EPTCTLENB4_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTCTLENB4_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCTLENB4_ISOENDPT =
    crate::Reg<isoendpt_eptctlenb4_isoendpt::ISOENDPT_EPTCTLENB4_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 4)"]
pub mod isoendpt_eptctlenb4_isoendpt;
#[doc = "EPTCTLDIS4 (w) register accessor: an alias for `Reg<EPTCTLDIS4_SPEC>`"]
pub type EPTCTLDIS4 = crate::Reg<eptctldis4::EPTCTLDIS4_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 4)"]
pub mod eptctldis4;
#[doc = "ISOENDPT_EPTCTLDIS4_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTCTLDIS4_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCTLDIS4_ISOENDPT =
    crate::Reg<isoendpt_eptctldis4_isoendpt::ISOENDPT_EPTCTLDIS4_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 4)"]
pub mod isoendpt_eptctldis4_isoendpt;
#[doc = "EPTCTL4 (r) register accessor: an alias for `Reg<EPTCTL4_SPEC>`"]
pub type EPTCTL4 = crate::Reg<eptctl4::EPTCTL4_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 4)"]
pub mod eptctl4;
#[doc = "ISOENDPT_EPTCTL4_ISOENDPT (r) register accessor: an alias for `Reg<ISOENDPT_EPTCTL4_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCTL4_ISOENDPT =
    crate::Reg<isoendpt_eptctl4_isoendpt::ISOENDPT_EPTCTL4_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 4)"]
pub mod isoendpt_eptctl4_isoendpt;
#[doc = "EPTSETSTA4 (w) register accessor: an alias for `Reg<EPTSETSTA4_SPEC>`"]
pub type EPTSETSTA4 = crate::Reg<eptsetsta4::EPTSETSTA4_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 4)"]
pub mod eptsetsta4;
#[doc = "ISOENDPT_EPTSETSTA4_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTSETSTA4_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTSETSTA4_ISOENDPT =
    crate::Reg<isoendpt_eptsetsta4_isoendpt::ISOENDPT_EPTSETSTA4_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 4)"]
pub mod isoendpt_eptsetsta4_isoendpt;
#[doc = "EPTCLRSTA4 (w) register accessor: an alias for `Reg<EPTCLRSTA4_SPEC>`"]
pub type EPTCLRSTA4 = crate::Reg<eptclrsta4::EPTCLRSTA4_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 4)"]
pub mod eptclrsta4;
#[doc = "ISOENDPT_EPTCLRSTA4_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTCLRSTA4_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCLRSTA4_ISOENDPT =
    crate::Reg<isoendpt_eptclrsta4_isoendpt::ISOENDPT_EPTCLRSTA4_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 4)"]
pub mod isoendpt_eptclrsta4_isoendpt;
#[doc = "EPTSTA4 (r) register accessor: an alias for `Reg<EPTSTA4_SPEC>`"]
pub type EPTSTA4 = crate::Reg<eptsta4::EPTSTA4_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 4)"]
pub mod eptsta4;
#[doc = "ISOENDPT_EPTSTA4_ISOENDPT (r) register accessor: an alias for `Reg<ISOENDPT_EPTSTA4_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTSTA4_ISOENDPT =
    crate::Reg<isoendpt_eptsta4_isoendpt::ISOENDPT_EPTSTA4_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 4)"]
pub mod isoendpt_eptsta4_isoendpt;
#[doc = "EPTCFG5 (rw) register accessor: an alias for `Reg<EPTCFG5_SPEC>`"]
pub type EPTCFG5 = crate::Reg<eptcfg5::EPTCFG5_SPEC>;
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 5)"]
pub mod eptcfg5;
#[doc = "EPTCTLENB5 (w) register accessor: an alias for `Reg<EPTCTLENB5_SPEC>`"]
pub type EPTCTLENB5 = crate::Reg<eptctlenb5::EPTCTLENB5_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 5)"]
pub mod eptctlenb5;
#[doc = "ISOENDPT_EPTCTLENB5_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTCTLENB5_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCTLENB5_ISOENDPT =
    crate::Reg<isoendpt_eptctlenb5_isoendpt::ISOENDPT_EPTCTLENB5_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 5)"]
pub mod isoendpt_eptctlenb5_isoendpt;
#[doc = "EPTCTLDIS5 (w) register accessor: an alias for `Reg<EPTCTLDIS5_SPEC>`"]
pub type EPTCTLDIS5 = crate::Reg<eptctldis5::EPTCTLDIS5_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 5)"]
pub mod eptctldis5;
#[doc = "ISOENDPT_EPTCTLDIS5_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTCTLDIS5_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCTLDIS5_ISOENDPT =
    crate::Reg<isoendpt_eptctldis5_isoendpt::ISOENDPT_EPTCTLDIS5_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 5)"]
pub mod isoendpt_eptctldis5_isoendpt;
#[doc = "EPTCTL5 (r) register accessor: an alias for `Reg<EPTCTL5_SPEC>`"]
pub type EPTCTL5 = crate::Reg<eptctl5::EPTCTL5_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 5)"]
pub mod eptctl5;
#[doc = "ISOENDPT_EPTCTL5_ISOENDPT (r) register accessor: an alias for `Reg<ISOENDPT_EPTCTL5_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCTL5_ISOENDPT =
    crate::Reg<isoendpt_eptctl5_isoendpt::ISOENDPT_EPTCTL5_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 5)"]
pub mod isoendpt_eptctl5_isoendpt;
#[doc = "EPTSETSTA5 (w) register accessor: an alias for `Reg<EPTSETSTA5_SPEC>`"]
pub type EPTSETSTA5 = crate::Reg<eptsetsta5::EPTSETSTA5_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 5)"]
pub mod eptsetsta5;
#[doc = "ISOENDPT_EPTSETSTA5_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTSETSTA5_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTSETSTA5_ISOENDPT =
    crate::Reg<isoendpt_eptsetsta5_isoendpt::ISOENDPT_EPTSETSTA5_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 5)"]
pub mod isoendpt_eptsetsta5_isoendpt;
#[doc = "EPTCLRSTA5 (w) register accessor: an alias for `Reg<EPTCLRSTA5_SPEC>`"]
pub type EPTCLRSTA5 = crate::Reg<eptclrsta5::EPTCLRSTA5_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 5)"]
pub mod eptclrsta5;
#[doc = "ISOENDPT_EPTCLRSTA5_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTCLRSTA5_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCLRSTA5_ISOENDPT =
    crate::Reg<isoendpt_eptclrsta5_isoendpt::ISOENDPT_EPTCLRSTA5_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 5)"]
pub mod isoendpt_eptclrsta5_isoendpt;
#[doc = "EPTSTA5 (r) register accessor: an alias for `Reg<EPTSTA5_SPEC>`"]
pub type EPTSTA5 = crate::Reg<eptsta5::EPTSTA5_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 5)"]
pub mod eptsta5;
#[doc = "ISOENDPT_EPTSTA5_ISOENDPT (r) register accessor: an alias for `Reg<ISOENDPT_EPTSTA5_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTSTA5_ISOENDPT =
    crate::Reg<isoendpt_eptsta5_isoendpt::ISOENDPT_EPTSTA5_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 5)"]
pub mod isoendpt_eptsta5_isoendpt;
#[doc = "EPTCFG6 (rw) register accessor: an alias for `Reg<EPTCFG6_SPEC>`"]
pub type EPTCFG6 = crate::Reg<eptcfg6::EPTCFG6_SPEC>;
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 6)"]
pub mod eptcfg6;
#[doc = "EPTCTLENB6 (w) register accessor: an alias for `Reg<EPTCTLENB6_SPEC>`"]
pub type EPTCTLENB6 = crate::Reg<eptctlenb6::EPTCTLENB6_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 6)"]
pub mod eptctlenb6;
#[doc = "ISOENDPT_EPTCTLENB6_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTCTLENB6_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCTLENB6_ISOENDPT =
    crate::Reg<isoendpt_eptctlenb6_isoendpt::ISOENDPT_EPTCTLENB6_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 6)"]
pub mod isoendpt_eptctlenb6_isoendpt;
#[doc = "EPTCTLDIS6 (w) register accessor: an alias for `Reg<EPTCTLDIS6_SPEC>`"]
pub type EPTCTLDIS6 = crate::Reg<eptctldis6::EPTCTLDIS6_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 6)"]
pub mod eptctldis6;
#[doc = "ISOENDPT_EPTCTLDIS6_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTCTLDIS6_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCTLDIS6_ISOENDPT =
    crate::Reg<isoendpt_eptctldis6_isoendpt::ISOENDPT_EPTCTLDIS6_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 6)"]
pub mod isoendpt_eptctldis6_isoendpt;
#[doc = "EPTCTL6 (r) register accessor: an alias for `Reg<EPTCTL6_SPEC>`"]
pub type EPTCTL6 = crate::Reg<eptctl6::EPTCTL6_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 6)"]
pub mod eptctl6;
#[doc = "ISOENDPT_EPTCTL6_ISOENDPT (r) register accessor: an alias for `Reg<ISOENDPT_EPTCTL6_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCTL6_ISOENDPT =
    crate::Reg<isoendpt_eptctl6_isoendpt::ISOENDPT_EPTCTL6_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Control Register (endpoint = 6)"]
pub mod isoendpt_eptctl6_isoendpt;
#[doc = "EPTSETSTA6 (w) register accessor: an alias for `Reg<EPTSETSTA6_SPEC>`"]
pub type EPTSETSTA6 = crate::Reg<eptsetsta6::EPTSETSTA6_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 6)"]
pub mod eptsetsta6;
#[doc = "ISOENDPT_EPTSETSTA6_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTSETSTA6_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTSETSTA6_ISOENDPT =
    crate::Reg<isoendpt_eptsetsta6_isoendpt::ISOENDPT_EPTSETSTA6_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Set Status Register (endpoint = 6)"]
pub mod isoendpt_eptsetsta6_isoendpt;
#[doc = "EPTCLRSTA6 (w) register accessor: an alias for `Reg<EPTCLRSTA6_SPEC>`"]
pub type EPTCLRSTA6 = crate::Reg<eptclrsta6::EPTCLRSTA6_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 6)"]
pub mod eptclrsta6;
#[doc = "ISOENDPT_EPTCLRSTA6_ISOENDPT (w) register accessor: an alias for `Reg<ISOENDPT_EPTCLRSTA6_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTCLRSTA6_ISOENDPT =
    crate::Reg<isoendpt_eptclrsta6_isoendpt::ISOENDPT_EPTCLRSTA6_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 6)"]
pub mod isoendpt_eptclrsta6_isoendpt;
#[doc = "EPTSTA6 (r) register accessor: an alias for `Reg<EPTSTA6_SPEC>`"]
pub type EPTSTA6 = crate::Reg<eptsta6::EPTSTA6_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 6)"]
pub mod eptsta6;
#[doc = "ISOENDPT_EPTSTA6_ISOENDPT (r) register accessor: an alias for `Reg<ISOENDPT_EPTSTA6_ISOENDPT_SPEC>`"]
pub type ISOENDPT_EPTSTA6_ISOENDPT =
    crate::Reg<isoendpt_eptsta6_isoendpt::ISOENDPT_EPTSTA6_ISOENDPT_SPEC>;
#[doc = "UDPHS Endpoint Status Register (endpoint = 6)"]
pub mod isoendpt_eptsta6_isoendpt;
#[doc = "DMANXTDSC0 (rw) register accessor: an alias for `Reg<DMANXTDSC0_SPEC>`"]
pub type DMANXTDSC0 = crate::Reg<dmanxtdsc0::DMANXTDSC0_SPEC>;
#[doc = "UDPHS DMA Next Descriptor Address Register (channel = 0)"]
pub mod dmanxtdsc0;
#[doc = "DMAADDRESS0 (rw) register accessor: an alias for `Reg<DMAADDRESS0_SPEC>`"]
pub type DMAADDRESS0 = crate::Reg<dmaaddress0::DMAADDRESS0_SPEC>;
#[doc = "UDPHS DMA Channel Address Register (channel = 0)"]
pub mod dmaaddress0;
#[doc = "DMACONTROL0 (rw) register accessor: an alias for `Reg<DMACONTROL0_SPEC>`"]
pub type DMACONTROL0 = crate::Reg<dmacontrol0::DMACONTROL0_SPEC>;
#[doc = "UDPHS DMA Channel Control Register (channel = 0)"]
pub mod dmacontrol0;
#[doc = "DMASTATUS0 (rw) register accessor: an alias for `Reg<DMASTATUS0_SPEC>`"]
pub type DMASTATUS0 = crate::Reg<dmastatus0::DMASTATUS0_SPEC>;
#[doc = "UDPHS DMA Channel Status Register (channel = 0)"]
pub mod dmastatus0;
#[doc = "DMANXTDSC1 (rw) register accessor: an alias for `Reg<DMANXTDSC1_SPEC>`"]
pub type DMANXTDSC1 = crate::Reg<dmanxtdsc1::DMANXTDSC1_SPEC>;
#[doc = "UDPHS DMA Next Descriptor Address Register (channel = 1)"]
pub mod dmanxtdsc1;
#[doc = "DMAADDRESS1 (rw) register accessor: an alias for `Reg<DMAADDRESS1_SPEC>`"]
pub type DMAADDRESS1 = crate::Reg<dmaaddress1::DMAADDRESS1_SPEC>;
#[doc = "UDPHS DMA Channel Address Register (channel = 1)"]
pub mod dmaaddress1;
#[doc = "DMACONTROL1 (rw) register accessor: an alias for `Reg<DMACONTROL1_SPEC>`"]
pub type DMACONTROL1 = crate::Reg<dmacontrol1::DMACONTROL1_SPEC>;
#[doc = "UDPHS DMA Channel Control Register (channel = 1)"]
pub mod dmacontrol1;
#[doc = "DMASTATUS1 (rw) register accessor: an alias for `Reg<DMASTATUS1_SPEC>`"]
pub type DMASTATUS1 = crate::Reg<dmastatus1::DMASTATUS1_SPEC>;
#[doc = "UDPHS DMA Channel Status Register (channel = 1)"]
pub mod dmastatus1;
#[doc = "DMANXTDSC2 (rw) register accessor: an alias for `Reg<DMANXTDSC2_SPEC>`"]
pub type DMANXTDSC2 = crate::Reg<dmanxtdsc2::DMANXTDSC2_SPEC>;
#[doc = "UDPHS DMA Next Descriptor Address Register (channel = 2)"]
pub mod dmanxtdsc2;
#[doc = "DMAADDRESS2 (rw) register accessor: an alias for `Reg<DMAADDRESS2_SPEC>`"]
pub type DMAADDRESS2 = crate::Reg<dmaaddress2::DMAADDRESS2_SPEC>;
#[doc = "UDPHS DMA Channel Address Register (channel = 2)"]
pub mod dmaaddress2;
#[doc = "DMACONTROL2 (rw) register accessor: an alias for `Reg<DMACONTROL2_SPEC>`"]
pub type DMACONTROL2 = crate::Reg<dmacontrol2::DMACONTROL2_SPEC>;
#[doc = "UDPHS DMA Channel Control Register (channel = 2)"]
pub mod dmacontrol2;
#[doc = "DMASTATUS2 (rw) register accessor: an alias for `Reg<DMASTATUS2_SPEC>`"]
pub type DMASTATUS2 = crate::Reg<dmastatus2::DMASTATUS2_SPEC>;
#[doc = "UDPHS DMA Channel Status Register (channel = 2)"]
pub mod dmastatus2;
#[doc = "DMANXTDSC3 (rw) register accessor: an alias for `Reg<DMANXTDSC3_SPEC>`"]
pub type DMANXTDSC3 = crate::Reg<dmanxtdsc3::DMANXTDSC3_SPEC>;
#[doc = "UDPHS DMA Next Descriptor Address Register (channel = 3)"]
pub mod dmanxtdsc3;
#[doc = "DMAADDRESS3 (rw) register accessor: an alias for `Reg<DMAADDRESS3_SPEC>`"]
pub type DMAADDRESS3 = crate::Reg<dmaaddress3::DMAADDRESS3_SPEC>;
#[doc = "UDPHS DMA Channel Address Register (channel = 3)"]
pub mod dmaaddress3;
#[doc = "DMACONTROL3 (rw) register accessor: an alias for `Reg<DMACONTROL3_SPEC>`"]
pub type DMACONTROL3 = crate::Reg<dmacontrol3::DMACONTROL3_SPEC>;
#[doc = "UDPHS DMA Channel Control Register (channel = 3)"]
pub mod dmacontrol3;
#[doc = "DMASTATUS3 (rw) register accessor: an alias for `Reg<DMASTATUS3_SPEC>`"]
pub type DMASTATUS3 = crate::Reg<dmastatus3::DMASTATUS3_SPEC>;
#[doc = "UDPHS DMA Channel Status Register (channel = 3)"]
pub mod dmastatus3;
#[doc = "DMANXTDSC4 (rw) register accessor: an alias for `Reg<DMANXTDSC4_SPEC>`"]
pub type DMANXTDSC4 = crate::Reg<dmanxtdsc4::DMANXTDSC4_SPEC>;
#[doc = "UDPHS DMA Next Descriptor Address Register (channel = 4)"]
pub mod dmanxtdsc4;
#[doc = "DMAADDRESS4 (rw) register accessor: an alias for `Reg<DMAADDRESS4_SPEC>`"]
pub type DMAADDRESS4 = crate::Reg<dmaaddress4::DMAADDRESS4_SPEC>;
#[doc = "UDPHS DMA Channel Address Register (channel = 4)"]
pub mod dmaaddress4;
#[doc = "DMACONTROL4 (rw) register accessor: an alias for `Reg<DMACONTROL4_SPEC>`"]
pub type DMACONTROL4 = crate::Reg<dmacontrol4::DMACONTROL4_SPEC>;
#[doc = "UDPHS DMA Channel Control Register (channel = 4)"]
pub mod dmacontrol4;
#[doc = "DMASTATUS4 (rw) register accessor: an alias for `Reg<DMASTATUS4_SPEC>`"]
pub type DMASTATUS4 = crate::Reg<dmastatus4::DMASTATUS4_SPEC>;
#[doc = "UDPHS DMA Channel Status Register (channel = 4)"]
pub mod dmastatus4;
#[doc = "DMANXTDSC5 (rw) register accessor: an alias for `Reg<DMANXTDSC5_SPEC>`"]
pub type DMANXTDSC5 = crate::Reg<dmanxtdsc5::DMANXTDSC5_SPEC>;
#[doc = "UDPHS DMA Next Descriptor Address Register (channel = 5)"]
pub mod dmanxtdsc5;
#[doc = "DMAADDRESS5 (rw) register accessor: an alias for `Reg<DMAADDRESS5_SPEC>`"]
pub type DMAADDRESS5 = crate::Reg<dmaaddress5::DMAADDRESS5_SPEC>;
#[doc = "UDPHS DMA Channel Address Register (channel = 5)"]
pub mod dmaaddress5;
#[doc = "DMACONTROL5 (rw) register accessor: an alias for `Reg<DMACONTROL5_SPEC>`"]
pub type DMACONTROL5 = crate::Reg<dmacontrol5::DMACONTROL5_SPEC>;
#[doc = "UDPHS DMA Channel Control Register (channel = 5)"]
pub mod dmacontrol5;
#[doc = "DMASTATUS5 (rw) register accessor: an alias for `Reg<DMASTATUS5_SPEC>`"]
pub type DMASTATUS5 = crate::Reg<dmastatus5::DMASTATUS5_SPEC>;
#[doc = "UDPHS DMA Channel Status Register (channel = 5)"]
pub mod dmastatus5;
