#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device General Control Register"]
    pub devctrl: crate::Reg<devctrl::DEVCTRL_SPEC>,
    #[doc = "0x04 - Device Global Interrupt Status Register"]
    pub devisr: crate::Reg<devisr::DEVISR_SPEC>,
    #[doc = "0x08 - Device Global Interrupt Clear Register"]
    pub devicr: crate::Reg<devicr::DEVICR_SPEC>,
    #[doc = "0x0c - Device Global Interrupt Set Register"]
    pub devifr: crate::Reg<devifr::DEVIFR_SPEC>,
    #[doc = "0x10 - Device Global Interrupt Mask Register"]
    pub devimr: crate::Reg<devimr::DEVIMR_SPEC>,
    #[doc = "0x14 - Device Global Interrupt Disable Register"]
    pub devidr: crate::Reg<devidr::DEVIDR_SPEC>,
    #[doc = "0x18 - Device Global Interrupt Enable Register"]
    pub devier: crate::Reg<devier::DEVIER_SPEC>,
    #[doc = "0x1c - Device Endpoint Register"]
    pub devept: crate::Reg<devept::DEVEPT_SPEC>,
    #[doc = "0x20 - Device Frame Number Register"]
    pub devfnum: crate::Reg<devfnum::DEVFNUM_SPEC>,
    _reserved9: [u8; 0xdc],
    #[doc = "0x100..0x128 - Device Endpoint Configuration Register (n = 0)"]
    pub deveptcfg: [crate::Reg<deveptcfg::DEVEPTCFG_SPEC>; 10],
    _reserved10: [u8; 0x08],
    _reserved_10_deveptisr: [u8; 0x28],
    _reserved11: [u8; 0x08],
    _reserved_11_devepticr: [u8; 0x28],
    _reserved12: [u8; 0x08],
    _reserved_12_deveptifr: [u8; 0x28],
    _reserved13: [u8; 0x08],
    _reserved_13_deveptimr: [u8; 0x28],
    _reserved14: [u8; 0x08],
    _reserved_14_deveptier: [u8; 0x28],
    _reserved15: [u8; 0x08],
    _reserved_15_deveptidr: [u8; 0x28],
    _reserved16: [u8; 0xc8],
    #[doc = "0x310 - Device DMA Channel Next Descriptor Address Register (n = 1)"]
    pub devdmanxtdsc1: crate::Reg<devdmanxtdsc1::DEVDMANXTDSC1_SPEC>,
    #[doc = "0x314 - Device DMA Channel Address Register (n = 1)"]
    pub devdmaaddress1: crate::Reg<devdmaaddress1::DEVDMAADDRESS1_SPEC>,
    #[doc = "0x318 - Device DMA Channel Control Register (n = 1)"]
    pub devdmacontrol1: crate::Reg<devdmacontrol1::DEVDMACONTROL1_SPEC>,
    #[doc = "0x31c - Device DMA Channel Status Register (n = 1)"]
    pub devdmastatus1: crate::Reg<devdmastatus1::DEVDMASTATUS1_SPEC>,
    #[doc = "0x320 - Device DMA Channel Next Descriptor Address Register (n = 2)"]
    pub devdmanxtdsc2: crate::Reg<devdmanxtdsc2::DEVDMANXTDSC2_SPEC>,
    #[doc = "0x324 - Device DMA Channel Address Register (n = 2)"]
    pub devdmaaddress2: crate::Reg<devdmaaddress2::DEVDMAADDRESS2_SPEC>,
    #[doc = "0x328 - Device DMA Channel Control Register (n = 2)"]
    pub devdmacontrol2: crate::Reg<devdmacontrol2::DEVDMACONTROL2_SPEC>,
    #[doc = "0x32c - Device DMA Channel Status Register (n = 2)"]
    pub devdmastatus2: crate::Reg<devdmastatus2::DEVDMASTATUS2_SPEC>,
    #[doc = "0x330 - Device DMA Channel Next Descriptor Address Register (n = 3)"]
    pub devdmanxtdsc3: crate::Reg<devdmanxtdsc3::DEVDMANXTDSC3_SPEC>,
    #[doc = "0x334 - Device DMA Channel Address Register (n = 3)"]
    pub devdmaaddress3: crate::Reg<devdmaaddress3::DEVDMAADDRESS3_SPEC>,
    #[doc = "0x338 - Device DMA Channel Control Register (n = 3)"]
    pub devdmacontrol3: crate::Reg<devdmacontrol3::DEVDMACONTROL3_SPEC>,
    #[doc = "0x33c - Device DMA Channel Status Register (n = 3)"]
    pub devdmastatus3: crate::Reg<devdmastatus3::DEVDMASTATUS3_SPEC>,
    #[doc = "0x340 - Device DMA Channel Next Descriptor Address Register (n = 4)"]
    pub devdmanxtdsc4: crate::Reg<devdmanxtdsc4::DEVDMANXTDSC4_SPEC>,
    #[doc = "0x344 - Device DMA Channel Address Register (n = 4)"]
    pub devdmaaddress4: crate::Reg<devdmaaddress4::DEVDMAADDRESS4_SPEC>,
    #[doc = "0x348 - Device DMA Channel Control Register (n = 4)"]
    pub devdmacontrol4: crate::Reg<devdmacontrol4::DEVDMACONTROL4_SPEC>,
    #[doc = "0x34c - Device DMA Channel Status Register (n = 4)"]
    pub devdmastatus4: crate::Reg<devdmastatus4::DEVDMASTATUS4_SPEC>,
    #[doc = "0x350 - Device DMA Channel Next Descriptor Address Register (n = 5)"]
    pub devdmanxtdsc5: crate::Reg<devdmanxtdsc5::DEVDMANXTDSC5_SPEC>,
    #[doc = "0x354 - Device DMA Channel Address Register (n = 5)"]
    pub devdmaaddress5: crate::Reg<devdmaaddress5::DEVDMAADDRESS5_SPEC>,
    #[doc = "0x358 - Device DMA Channel Control Register (n = 5)"]
    pub devdmacontrol5: crate::Reg<devdmacontrol5::DEVDMACONTROL5_SPEC>,
    #[doc = "0x35c - Device DMA Channel Status Register (n = 5)"]
    pub devdmastatus5: crate::Reg<devdmastatus5::DEVDMASTATUS5_SPEC>,
    #[doc = "0x360 - Device DMA Channel Next Descriptor Address Register (n = 6)"]
    pub devdmanxtdsc6: crate::Reg<devdmanxtdsc6::DEVDMANXTDSC6_SPEC>,
    #[doc = "0x364 - Device DMA Channel Address Register (n = 6)"]
    pub devdmaaddress6: crate::Reg<devdmaaddress6::DEVDMAADDRESS6_SPEC>,
    #[doc = "0x368 - Device DMA Channel Control Register (n = 6)"]
    pub devdmacontrol6: crate::Reg<devdmacontrol6::DEVDMACONTROL6_SPEC>,
    #[doc = "0x36c - Device DMA Channel Status Register (n = 6)"]
    pub devdmastatus6: crate::Reg<devdmastatus6::DEVDMASTATUS6_SPEC>,
    #[doc = "0x370 - Device DMA Channel Next Descriptor Address Register (n = 7)"]
    pub devdmanxtdsc7: crate::Reg<devdmanxtdsc7::DEVDMANXTDSC7_SPEC>,
    #[doc = "0x374 - Device DMA Channel Address Register (n = 7)"]
    pub devdmaaddress7: crate::Reg<devdmaaddress7::DEVDMAADDRESS7_SPEC>,
    #[doc = "0x378 - Device DMA Channel Control Register (n = 7)"]
    pub devdmacontrol7: crate::Reg<devdmacontrol7::DEVDMACONTROL7_SPEC>,
    #[doc = "0x37c - Device DMA Channel Status Register (n = 7)"]
    pub devdmastatus7: crate::Reg<devdmastatus7::DEVDMASTATUS7_SPEC>,
    _reserved44: [u8; 0x80],
    #[doc = "0x400 - Host General Control Register"]
    pub hstctrl: crate::Reg<hstctrl::HSTCTRL_SPEC>,
    #[doc = "0x404 - Host Global Interrupt Status Register"]
    pub hstisr: crate::Reg<hstisr::HSTISR_SPEC>,
    #[doc = "0x408 - Host Global Interrupt Clear Register"]
    pub hsticr: crate::Reg<hsticr::HSTICR_SPEC>,
    #[doc = "0x40c - Host Global Interrupt Set Register"]
    pub hstifr: crate::Reg<hstifr::HSTIFR_SPEC>,
    #[doc = "0x410 - Host Global Interrupt Mask Register"]
    pub hstimr: crate::Reg<hstimr::HSTIMR_SPEC>,
    #[doc = "0x414 - Host Global Interrupt Disable Register"]
    pub hstidr: crate::Reg<hstidr::HSTIDR_SPEC>,
    #[doc = "0x418 - Host Global Interrupt Enable Register"]
    pub hstier: crate::Reg<hstier::HSTIER_SPEC>,
    #[doc = "0x41c - Host Pipe Register"]
    pub hstpip: crate::Reg<hstpip::HSTPIP_SPEC>,
    #[doc = "0x420 - Host Frame Number Register"]
    pub hstfnum: crate::Reg<hstfnum::HSTFNUM_SPEC>,
    #[doc = "0x424 - Host Address 1 Register"]
    pub hstaddr1: crate::Reg<hstaddr1::HSTADDR1_SPEC>,
    #[doc = "0x428 - Host Address 2 Register"]
    pub hstaddr2: crate::Reg<hstaddr2::HSTADDR2_SPEC>,
    #[doc = "0x42c - Host Address 3 Register"]
    pub hstaddr3: crate::Reg<hstaddr3::HSTADDR3_SPEC>,
    _reserved56: [u8; 0xd0],
    _reserved_56_hstpipcfg: [u8; 0x28],
    _reserved57: [u8; 0x08],
    _reserved_57_hstpipisr: [u8; 0x28],
    _reserved58: [u8; 0x08],
    _reserved_58_hstpipicr: [u8; 0x28],
    _reserved59: [u8; 0x08],
    _reserved_59_hstpipifr: [u8; 0x28],
    _reserved60: [u8; 0x08],
    _reserved_60_hstpipimr: [u8; 0x28],
    _reserved61: [u8; 0x08],
    _reserved_61_hstpipier: [u8; 0x28],
    _reserved62: [u8; 0x08],
    _reserved_62_hstpipidr: [u8; 0x28],
    _reserved63: [u8; 0x08],
    #[doc = "0x650..0x678 - Host Pipe IN Request Register (n = 0)"]
    pub hstpipinrq: [crate::Reg<hstpipinrq::HSTPIPINRQ_SPEC>; 10],
    _reserved64: [u8; 0x08],
    #[doc = "0x680..0x6a8 - Host Pipe Error Register (n = 0)"]
    pub hstpiperr: [crate::Reg<hstpiperr::HSTPIPERR_SPEC>; 10],
    _reserved65: [u8; 0x68],
    #[doc = "0x710 - Host DMA Channel Next Descriptor Address Register (n = 1)"]
    pub hstdmanxtdsc1: crate::Reg<hstdmanxtdsc1::HSTDMANXTDSC1_SPEC>,
    #[doc = "0x714 - Host DMA Channel Address Register (n = 1)"]
    pub hstdmaaddress1: crate::Reg<hstdmaaddress1::HSTDMAADDRESS1_SPEC>,
    #[doc = "0x718 - Host DMA Channel Control Register (n = 1)"]
    pub hstdmacontrol1: crate::Reg<hstdmacontrol1::HSTDMACONTROL1_SPEC>,
    #[doc = "0x71c - Host DMA Channel Status Register (n = 1)"]
    pub hstdmastatus1: crate::Reg<hstdmastatus1::HSTDMASTATUS1_SPEC>,
    #[doc = "0x720 - Host DMA Channel Next Descriptor Address Register (n = 2)"]
    pub hstdmanxtdsc2: crate::Reg<hstdmanxtdsc2::HSTDMANXTDSC2_SPEC>,
    #[doc = "0x724 - Host DMA Channel Address Register (n = 2)"]
    pub hstdmaaddress2: crate::Reg<hstdmaaddress2::HSTDMAADDRESS2_SPEC>,
    #[doc = "0x728 - Host DMA Channel Control Register (n = 2)"]
    pub hstdmacontrol2: crate::Reg<hstdmacontrol2::HSTDMACONTROL2_SPEC>,
    #[doc = "0x72c - Host DMA Channel Status Register (n = 2)"]
    pub hstdmastatus2: crate::Reg<hstdmastatus2::HSTDMASTATUS2_SPEC>,
    #[doc = "0x730 - Host DMA Channel Next Descriptor Address Register (n = 3)"]
    pub hstdmanxtdsc3: crate::Reg<hstdmanxtdsc3::HSTDMANXTDSC3_SPEC>,
    #[doc = "0x734 - Host DMA Channel Address Register (n = 3)"]
    pub hstdmaaddress3: crate::Reg<hstdmaaddress3::HSTDMAADDRESS3_SPEC>,
    #[doc = "0x738 - Host DMA Channel Control Register (n = 3)"]
    pub hstdmacontrol3: crate::Reg<hstdmacontrol3::HSTDMACONTROL3_SPEC>,
    #[doc = "0x73c - Host DMA Channel Status Register (n = 3)"]
    pub hstdmastatus3: crate::Reg<hstdmastatus3::HSTDMASTATUS3_SPEC>,
    #[doc = "0x740 - Host DMA Channel Next Descriptor Address Register (n = 4)"]
    pub hstdmanxtdsc4: crate::Reg<hstdmanxtdsc4::HSTDMANXTDSC4_SPEC>,
    #[doc = "0x744 - Host DMA Channel Address Register (n = 4)"]
    pub hstdmaaddress4: crate::Reg<hstdmaaddress4::HSTDMAADDRESS4_SPEC>,
    #[doc = "0x748 - Host DMA Channel Control Register (n = 4)"]
    pub hstdmacontrol4: crate::Reg<hstdmacontrol4::HSTDMACONTROL4_SPEC>,
    #[doc = "0x74c - Host DMA Channel Status Register (n = 4)"]
    pub hstdmastatus4: crate::Reg<hstdmastatus4::HSTDMASTATUS4_SPEC>,
    #[doc = "0x750 - Host DMA Channel Next Descriptor Address Register (n = 5)"]
    pub hstdmanxtdsc5: crate::Reg<hstdmanxtdsc5::HSTDMANXTDSC5_SPEC>,
    #[doc = "0x754 - Host DMA Channel Address Register (n = 5)"]
    pub hstdmaaddress5: crate::Reg<hstdmaaddress5::HSTDMAADDRESS5_SPEC>,
    #[doc = "0x758 - Host DMA Channel Control Register (n = 5)"]
    pub hstdmacontrol5: crate::Reg<hstdmacontrol5::HSTDMACONTROL5_SPEC>,
    #[doc = "0x75c - Host DMA Channel Status Register (n = 5)"]
    pub hstdmastatus5: crate::Reg<hstdmastatus5::HSTDMASTATUS5_SPEC>,
    #[doc = "0x760 - Host DMA Channel Next Descriptor Address Register (n = 6)"]
    pub hstdmanxtdsc6: crate::Reg<hstdmanxtdsc6::HSTDMANXTDSC6_SPEC>,
    #[doc = "0x764 - Host DMA Channel Address Register (n = 6)"]
    pub hstdmaaddress6: crate::Reg<hstdmaaddress6::HSTDMAADDRESS6_SPEC>,
    #[doc = "0x768 - Host DMA Channel Control Register (n = 6)"]
    pub hstdmacontrol6: crate::Reg<hstdmacontrol6::HSTDMACONTROL6_SPEC>,
    #[doc = "0x76c - Host DMA Channel Status Register (n = 6)"]
    pub hstdmastatus6: crate::Reg<hstdmastatus6::HSTDMASTATUS6_SPEC>,
    #[doc = "0x770 - Host DMA Channel Next Descriptor Address Register (n = 7)"]
    pub hstdmanxtdsc7: crate::Reg<hstdmanxtdsc7::HSTDMANXTDSC7_SPEC>,
    #[doc = "0x774 - Host DMA Channel Address Register (n = 7)"]
    pub hstdmaaddress7: crate::Reg<hstdmaaddress7::HSTDMAADDRESS7_SPEC>,
    #[doc = "0x778 - Host DMA Channel Control Register (n = 7)"]
    pub hstdmacontrol7: crate::Reg<hstdmacontrol7::HSTDMACONTROL7_SPEC>,
    #[doc = "0x77c - Host DMA Channel Status Register (n = 7)"]
    pub hstdmastatus7: crate::Reg<hstdmastatus7::HSTDMASTATUS7_SPEC>,
    _reserved93: [u8; 0x80],
    #[doc = "0x800 - General Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x804 - General Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x808 - General Status Clear Register"]
    pub scr: crate::Reg<scr::SCR_SPEC>,
    #[doc = "0x80c - General Status Set Register"]
    pub sfr: crate::Reg<sfr::SFR_SPEC>,
    _reserved97: [u8; 0x1c],
    #[doc = "0x82c - General Finite State Machine Register"]
    pub fsm: crate::Reg<fsm::FSM_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x130 - Device Endpoint Status Register (n = 0)"]
    #[inline(always)]
    pub fn isoenpt_deveptisr0_isoenpt(
        &self,
    ) -> &crate::Reg<isoenpt_deveptisr0_isoenpt::ISOENPT_DEVEPTISR0_ISOENPT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(304usize)
                as *const crate::Reg<isoenpt_deveptisr0_isoenpt::ISOENPT_DEVEPTISR0_ISOENPT_SPEC>)
        }
    }
    #[doc = "0x130..0x158 - Device Endpoint Status Register (n = 0)"]
    #[inline(always)]
    pub fn deveptisr(&self) -> &[crate::Reg<deveptisr::DEVEPTISR_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(304usize)
                as *const [crate::Reg<deveptisr::DEVEPTISR_SPEC>; 10])
        }
    }
    #[doc = "0x160 - Device Endpoint Clear Register (n = 0)"]
    #[inline(always)]
    pub fn isoenpt_devepticr0_isoenpt(
        &self,
    ) -> &crate::Reg<isoenpt_devepticr0_isoenpt::ISOENPT_DEVEPTICR0_ISOENPT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(352usize)
                as *const crate::Reg<isoenpt_devepticr0_isoenpt::ISOENPT_DEVEPTICR0_ISOENPT_SPEC>)
        }
    }
    #[doc = "0x160..0x188 - Device Endpoint Clear Register (n = 0)"]
    #[inline(always)]
    pub fn devepticr(&self) -> &[crate::Reg<devepticr::DEVEPTICR_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(352usize)
                as *const [crate::Reg<devepticr::DEVEPTICR_SPEC>; 10])
        }
    }
    #[doc = "0x190 - Device Endpoint Set Register (n = 0)"]
    #[inline(always)]
    pub fn isoenpt_deveptifr0_isoenpt(
        &self,
    ) -> &crate::Reg<isoenpt_deveptifr0_isoenpt::ISOENPT_DEVEPTIFR0_ISOENPT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(400usize)
                as *const crate::Reg<isoenpt_deveptifr0_isoenpt::ISOENPT_DEVEPTIFR0_ISOENPT_SPEC>)
        }
    }
    #[doc = "0x190..0x1b8 - Device Endpoint Set Register (n = 0)"]
    #[inline(always)]
    pub fn deveptifr(&self) -> &[crate::Reg<deveptifr::DEVEPTIFR_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(400usize)
                as *const [crate::Reg<deveptifr::DEVEPTIFR_SPEC>; 10])
        }
    }
    #[doc = "0x1c0 - Device Endpoint Mask Register (n = 0)"]
    #[inline(always)]
    pub fn isoenpt_deveptimr0_isoenpt(
        &self,
    ) -> &crate::Reg<isoenpt_deveptimr0_isoenpt::ISOENPT_DEVEPTIMR0_ISOENPT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(448usize)
                as *const crate::Reg<isoenpt_deveptimr0_isoenpt::ISOENPT_DEVEPTIMR0_ISOENPT_SPEC>)
        }
    }
    #[doc = "0x1c0..0x1e8 - Device Endpoint Mask Register (n = 0)"]
    #[inline(always)]
    pub fn deveptimr(&self) -> &[crate::Reg<deveptimr::DEVEPTIMR_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(448usize)
                as *const [crate::Reg<deveptimr::DEVEPTIMR_SPEC>; 10])
        }
    }
    #[doc = "0x1f0 - Device Endpoint Enable Register (n = 0)"]
    #[inline(always)]
    pub fn isoenpt_deveptier0_isoenpt(
        &self,
    ) -> &crate::Reg<isoenpt_deveptier0_isoenpt::ISOENPT_DEVEPTIER0_ISOENPT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(496usize)
                as *const crate::Reg<isoenpt_deveptier0_isoenpt::ISOENPT_DEVEPTIER0_ISOENPT_SPEC>)
        }
    }
    #[doc = "0x1f0..0x218 - Device Endpoint Enable Register (n = 0)"]
    #[inline(always)]
    pub fn deveptier(&self) -> &[crate::Reg<deveptier::DEVEPTIER_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(496usize)
                as *const [crate::Reg<deveptier::DEVEPTIER_SPEC>; 10])
        }
    }
    #[doc = "0x220 - Device Endpoint Disable Register (n = 0)"]
    #[inline(always)]
    pub fn isoenpt_deveptidr0_isoenpt(
        &self,
    ) -> &crate::Reg<isoenpt_deveptidr0_isoenpt::ISOENPT_DEVEPTIDR0_ISOENPT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(544usize)
                as *const crate::Reg<isoenpt_deveptidr0_isoenpt::ISOENPT_DEVEPTIDR0_ISOENPT_SPEC>)
        }
    }
    #[doc = "0x220..0x248 - Device Endpoint Disable Register (n = 0)"]
    #[inline(always)]
    pub fn deveptidr(&self) -> &[crate::Reg<deveptidr::DEVEPTIDR_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(544usize)
                as *const [crate::Reg<deveptidr::DEVEPTIDR_SPEC>; 10])
        }
    }
    #[doc = "0x500 - Host Pipe Configuration Register (n = 0)"]
    #[inline(always)]
    pub fn hsbohscp_hstpipcfg0_hsbohscp(
        &self,
    ) -> &crate::Reg<hsbohscp_hstpipcfg0_hsbohscp::HSBOHSCP_HSTPIPCFG0_HSBOHSCP_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1280usize)
                as *const crate::Reg<
                    hsbohscp_hstpipcfg0_hsbohscp::HSBOHSCP_HSTPIPCFG0_HSBOHSCP_SPEC,
                >)
        }
    }
    #[doc = "0x500..0x528 - Host Pipe Configuration Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipcfg(&self) -> &[crate::Reg<hstpipcfg::HSTPIPCFG_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1280usize)
                as *const [crate::Reg<hstpipcfg::HSTPIPCFG_SPEC>; 10])
        }
    }
    #[doc = "0x530 - Host Pipe Status Register (n = 0)"]
    #[inline(always)]
    pub fn isopipes_hstpipisr0_isopipes(
        &self,
    ) -> &crate::Reg<isopipes_hstpipisr0_isopipes::ISOPIPES_HSTPIPISR0_ISOPIPES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1328usize)
                as *const crate::Reg<
                    isopipes_hstpipisr0_isopipes::ISOPIPES_HSTPIPISR0_ISOPIPES_SPEC,
                >)
        }
    }
    #[doc = "0x530 - Host Pipe Status Register (n = 0)"]
    #[inline(always)]
    pub fn intpipes_hstpipisr0_intpipes(
        &self,
    ) -> &crate::Reg<intpipes_hstpipisr0_intpipes::INTPIPES_HSTPIPISR0_INTPIPES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1328usize)
                as *const crate::Reg<
                    intpipes_hstpipisr0_intpipes::INTPIPES_HSTPIPISR0_INTPIPES_SPEC,
                >)
        }
    }
    #[doc = "0x530..0x558 - Host Pipe Status Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipisr(&self) -> &[crate::Reg<hstpipisr::HSTPIPISR_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1328usize)
                as *const [crate::Reg<hstpipisr::HSTPIPISR_SPEC>; 10])
        }
    }
    #[doc = "0x560 - Host Pipe Clear Register (n = 0)"]
    #[inline(always)]
    pub fn isopipes_hstpipicr0_isopipes(
        &self,
    ) -> &crate::Reg<isopipes_hstpipicr0_isopipes::ISOPIPES_HSTPIPICR0_ISOPIPES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize)
                as *const crate::Reg<
                    isopipes_hstpipicr0_isopipes::ISOPIPES_HSTPIPICR0_ISOPIPES_SPEC,
                >)
        }
    }
    #[doc = "0x560 - Host Pipe Clear Register (n = 0)"]
    #[inline(always)]
    pub fn intpipes_hstpipicr0_intpipes(
        &self,
    ) -> &crate::Reg<intpipes_hstpipicr0_intpipes::INTPIPES_HSTPIPICR0_INTPIPES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize)
                as *const crate::Reg<
                    intpipes_hstpipicr0_intpipes::INTPIPES_HSTPIPICR0_INTPIPES_SPEC,
                >)
        }
    }
    #[doc = "0x560..0x588 - Host Pipe Clear Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipicr(&self) -> &[crate::Reg<hstpipicr::HSTPIPICR_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize)
                as *const [crate::Reg<hstpipicr::HSTPIPICR_SPEC>; 10])
        }
    }
    #[doc = "0x590 - Host Pipe Set Register (n = 0)"]
    #[inline(always)]
    pub fn isopipes_hstpipifr0_isopipes(
        &self,
    ) -> &crate::Reg<isopipes_hstpipifr0_isopipes::ISOPIPES_HSTPIPIFR0_ISOPIPES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1424usize)
                as *const crate::Reg<
                    isopipes_hstpipifr0_isopipes::ISOPIPES_HSTPIPIFR0_ISOPIPES_SPEC,
                >)
        }
    }
    #[doc = "0x590 - Host Pipe Set Register (n = 0)"]
    #[inline(always)]
    pub fn intpipes_hstpipifr0_intpipes(
        &self,
    ) -> &crate::Reg<intpipes_hstpipifr0_intpipes::INTPIPES_HSTPIPIFR0_INTPIPES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1424usize)
                as *const crate::Reg<
                    intpipes_hstpipifr0_intpipes::INTPIPES_HSTPIPIFR0_INTPIPES_SPEC,
                >)
        }
    }
    #[doc = "0x590..0x5b8 - Host Pipe Set Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipifr(&self) -> &[crate::Reg<hstpipifr::HSTPIPIFR_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1424usize)
                as *const [crate::Reg<hstpipifr::HSTPIPIFR_SPEC>; 10])
        }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register (n = 0)"]
    #[inline(always)]
    pub fn isopipes_hstpipimr0_isopipes(
        &self,
    ) -> &crate::Reg<isopipes_hstpipimr0_isopipes::ISOPIPES_HSTPIPIMR0_ISOPIPES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1472usize)
                as *const crate::Reg<
                    isopipes_hstpipimr0_isopipes::ISOPIPES_HSTPIPIMR0_ISOPIPES_SPEC,
                >)
        }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register (n = 0)"]
    #[inline(always)]
    pub fn intpipes_hstpipimr0_intpipes(
        &self,
    ) -> &crate::Reg<intpipes_hstpipimr0_intpipes::INTPIPES_HSTPIPIMR0_INTPIPES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1472usize)
                as *const crate::Reg<
                    intpipes_hstpipimr0_intpipes::INTPIPES_HSTPIPIMR0_INTPIPES_SPEC,
                >)
        }
    }
    #[doc = "0x5c0..0x5e8 - Host Pipe Mask Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipimr(&self) -> &[crate::Reg<hstpipimr::HSTPIPIMR_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1472usize)
                as *const [crate::Reg<hstpipimr::HSTPIPIMR_SPEC>; 10])
        }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register (n = 0)"]
    #[inline(always)]
    pub fn isopipes_hstpipier0_isopipes(
        &self,
    ) -> &crate::Reg<isopipes_hstpipier0_isopipes::ISOPIPES_HSTPIPIER0_ISOPIPES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1520usize)
                as *const crate::Reg<
                    isopipes_hstpipier0_isopipes::ISOPIPES_HSTPIPIER0_ISOPIPES_SPEC,
                >)
        }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register (n = 0)"]
    #[inline(always)]
    pub fn intpipes_hstpipier0_intpipes(
        &self,
    ) -> &crate::Reg<intpipes_hstpipier0_intpipes::INTPIPES_HSTPIPIER0_INTPIPES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1520usize)
                as *const crate::Reg<
                    intpipes_hstpipier0_intpipes::INTPIPES_HSTPIPIER0_INTPIPES_SPEC,
                >)
        }
    }
    #[doc = "0x5f0..0x618 - Host Pipe Enable Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipier(&self) -> &[crate::Reg<hstpipier::HSTPIPIER_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1520usize)
                as *const [crate::Reg<hstpipier::HSTPIPIER_SPEC>; 10])
        }
    }
    #[doc = "0x620 - Host Pipe Disable Register (n = 0)"]
    #[inline(always)]
    pub fn isopipes_hstpipidr0_isopipes(
        &self,
    ) -> &crate::Reg<isopipes_hstpipidr0_isopipes::ISOPIPES_HSTPIPIDR0_ISOPIPES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1568usize)
                as *const crate::Reg<
                    isopipes_hstpipidr0_isopipes::ISOPIPES_HSTPIPIDR0_ISOPIPES_SPEC,
                >)
        }
    }
    #[doc = "0x620 - Host Pipe Disable Register (n = 0)"]
    #[inline(always)]
    pub fn intpipes_hstpipidr0_intpipes(
        &self,
    ) -> &crate::Reg<intpipes_hstpipidr0_intpipes::INTPIPES_HSTPIPIDR0_INTPIPES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1568usize)
                as *const crate::Reg<
                    intpipes_hstpipidr0_intpipes::INTPIPES_HSTPIPIDR0_INTPIPES_SPEC,
                >)
        }
    }
    #[doc = "0x620..0x648 - Host Pipe Disable Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipidr(&self) -> &[crate::Reg<hstpipidr::HSTPIPIDR_SPEC>; 10] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1568usize)
                as *const [crate::Reg<hstpipidr::HSTPIPIDR_SPEC>; 10])
        }
    }
}
#[doc = "DEVCTRL register accessor: an alias for `Reg<DEVCTRL_SPEC>`"]
pub type DEVCTRL = crate::Reg<devctrl::DEVCTRL_SPEC>;
#[doc = "Device General Control Register"]
pub mod devctrl;
#[doc = "DEVISR register accessor: an alias for `Reg<DEVISR_SPEC>`"]
pub type DEVISR = crate::Reg<devisr::DEVISR_SPEC>;
#[doc = "Device Global Interrupt Status Register"]
pub mod devisr;
#[doc = "DEVICR register accessor: an alias for `Reg<DEVICR_SPEC>`"]
pub type DEVICR = crate::Reg<devicr::DEVICR_SPEC>;
#[doc = "Device Global Interrupt Clear Register"]
pub mod devicr;
#[doc = "DEVIFR register accessor: an alias for `Reg<DEVIFR_SPEC>`"]
pub type DEVIFR = crate::Reg<devifr::DEVIFR_SPEC>;
#[doc = "Device Global Interrupt Set Register"]
pub mod devifr;
#[doc = "DEVIMR register accessor: an alias for `Reg<DEVIMR_SPEC>`"]
pub type DEVIMR = crate::Reg<devimr::DEVIMR_SPEC>;
#[doc = "Device Global Interrupt Mask Register"]
pub mod devimr;
#[doc = "DEVIDR register accessor: an alias for `Reg<DEVIDR_SPEC>`"]
pub type DEVIDR = crate::Reg<devidr::DEVIDR_SPEC>;
#[doc = "Device Global Interrupt Disable Register"]
pub mod devidr;
#[doc = "DEVIER register accessor: an alias for `Reg<DEVIER_SPEC>`"]
pub type DEVIER = crate::Reg<devier::DEVIER_SPEC>;
#[doc = "Device Global Interrupt Enable Register"]
pub mod devier;
#[doc = "DEVEPT register accessor: an alias for `Reg<DEVEPT_SPEC>`"]
pub type DEVEPT = crate::Reg<devept::DEVEPT_SPEC>;
#[doc = "Device Endpoint Register"]
pub mod devept;
#[doc = "DEVFNUM register accessor: an alias for `Reg<DEVFNUM_SPEC>`"]
pub type DEVFNUM = crate::Reg<devfnum::DEVFNUM_SPEC>;
#[doc = "Device Frame Number Register"]
pub mod devfnum;
#[doc = "DEVEPTCFG register accessor: an alias for `Reg<DEVEPTCFG_SPEC>`"]
pub type DEVEPTCFG = crate::Reg<deveptcfg::DEVEPTCFG_SPEC>;
#[doc = "Device Endpoint Configuration Register (n = 0)"]
pub mod deveptcfg;
#[doc = "DEVEPTISR register accessor: an alias for `Reg<DEVEPTISR_SPEC>`"]
pub type DEVEPTISR = crate::Reg<deveptisr::DEVEPTISR_SPEC>;
#[doc = "Device Endpoint Status Register (n = 0)"]
pub mod deveptisr;
#[doc = "ISOENPT_DEVEPTISR0_ISOENPT register accessor: an alias for `Reg<ISOENPT_DEVEPTISR0_ISOENPT_SPEC>`"]
pub type ISOENPT_DEVEPTISR0_ISOENPT =
    crate::Reg<isoenpt_deveptisr0_isoenpt::ISOENPT_DEVEPTISR0_ISOENPT_SPEC>;
#[doc = "Device Endpoint Status Register (n = 0)"]
pub mod isoenpt_deveptisr0_isoenpt;
#[doc = "DEVEPTICR register accessor: an alias for `Reg<DEVEPTICR_SPEC>`"]
pub type DEVEPTICR = crate::Reg<devepticr::DEVEPTICR_SPEC>;
#[doc = "Device Endpoint Clear Register (n = 0)"]
pub mod devepticr;
#[doc = "ISOENPT_DEVEPTICR0_ISOENPT register accessor: an alias for `Reg<ISOENPT_DEVEPTICR0_ISOENPT_SPEC>`"]
pub type ISOENPT_DEVEPTICR0_ISOENPT =
    crate::Reg<isoenpt_devepticr0_isoenpt::ISOENPT_DEVEPTICR0_ISOENPT_SPEC>;
#[doc = "Device Endpoint Clear Register (n = 0)"]
pub mod isoenpt_devepticr0_isoenpt;
#[doc = "DEVEPTIFR register accessor: an alias for `Reg<DEVEPTIFR_SPEC>`"]
pub type DEVEPTIFR = crate::Reg<deveptifr::DEVEPTIFR_SPEC>;
#[doc = "Device Endpoint Set Register (n = 0)"]
pub mod deveptifr;
#[doc = "ISOENPT_DEVEPTIFR0_ISOENPT register accessor: an alias for `Reg<ISOENPT_DEVEPTIFR0_ISOENPT_SPEC>`"]
pub type ISOENPT_DEVEPTIFR0_ISOENPT =
    crate::Reg<isoenpt_deveptifr0_isoenpt::ISOENPT_DEVEPTIFR0_ISOENPT_SPEC>;
#[doc = "Device Endpoint Set Register (n = 0)"]
pub mod isoenpt_deveptifr0_isoenpt;
#[doc = "DEVEPTIMR register accessor: an alias for `Reg<DEVEPTIMR_SPEC>`"]
pub type DEVEPTIMR = crate::Reg<deveptimr::DEVEPTIMR_SPEC>;
#[doc = "Device Endpoint Mask Register (n = 0)"]
pub mod deveptimr;
#[doc = "ISOENPT_DEVEPTIMR0_ISOENPT register accessor: an alias for `Reg<ISOENPT_DEVEPTIMR0_ISOENPT_SPEC>`"]
pub type ISOENPT_DEVEPTIMR0_ISOENPT =
    crate::Reg<isoenpt_deveptimr0_isoenpt::ISOENPT_DEVEPTIMR0_ISOENPT_SPEC>;
#[doc = "Device Endpoint Mask Register (n = 0)"]
pub mod isoenpt_deveptimr0_isoenpt;
#[doc = "DEVEPTIER register accessor: an alias for `Reg<DEVEPTIER_SPEC>`"]
pub type DEVEPTIER = crate::Reg<deveptier::DEVEPTIER_SPEC>;
#[doc = "Device Endpoint Enable Register (n = 0)"]
pub mod deveptier;
#[doc = "ISOENPT_DEVEPTIER0_ISOENPT register accessor: an alias for `Reg<ISOENPT_DEVEPTIER0_ISOENPT_SPEC>`"]
pub type ISOENPT_DEVEPTIER0_ISOENPT =
    crate::Reg<isoenpt_deveptier0_isoenpt::ISOENPT_DEVEPTIER0_ISOENPT_SPEC>;
#[doc = "Device Endpoint Enable Register (n = 0)"]
pub mod isoenpt_deveptier0_isoenpt;
#[doc = "DEVEPTIDR register accessor: an alias for `Reg<DEVEPTIDR_SPEC>`"]
pub type DEVEPTIDR = crate::Reg<deveptidr::DEVEPTIDR_SPEC>;
#[doc = "Device Endpoint Disable Register (n = 0)"]
pub mod deveptidr;
#[doc = "ISOENPT_DEVEPTIDR0_ISOENPT register accessor: an alias for `Reg<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC>`"]
pub type ISOENPT_DEVEPTIDR0_ISOENPT =
    crate::Reg<isoenpt_deveptidr0_isoenpt::ISOENPT_DEVEPTIDR0_ISOENPT_SPEC>;
#[doc = "Device Endpoint Disable Register (n = 0)"]
pub mod isoenpt_deveptidr0_isoenpt;
#[doc = "DEVDMANXTDSC1 register accessor: an alias for `Reg<DEVDMANXTDSC1_SPEC>`"]
pub type DEVDMANXTDSC1 = crate::Reg<devdmanxtdsc1::DEVDMANXTDSC1_SPEC>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod devdmanxtdsc1;
#[doc = "DEVDMAADDRESS1 register accessor: an alias for `Reg<DEVDMAADDRESS1_SPEC>`"]
pub type DEVDMAADDRESS1 = crate::Reg<devdmaaddress1::DEVDMAADDRESS1_SPEC>;
#[doc = "Device DMA Channel Address Register (n = 1)"]
pub mod devdmaaddress1;
#[doc = "DEVDMACONTROL1 register accessor: an alias for `Reg<DEVDMACONTROL1_SPEC>`"]
pub type DEVDMACONTROL1 = crate::Reg<devdmacontrol1::DEVDMACONTROL1_SPEC>;
#[doc = "Device DMA Channel Control Register (n = 1)"]
pub mod devdmacontrol1;
#[doc = "DEVDMASTATUS1 register accessor: an alias for `Reg<DEVDMASTATUS1_SPEC>`"]
pub type DEVDMASTATUS1 = crate::Reg<devdmastatus1::DEVDMASTATUS1_SPEC>;
#[doc = "Device DMA Channel Status Register (n = 1)"]
pub mod devdmastatus1;
#[doc = "DEVDMANXTDSC2 register accessor: an alias for `Reg<DEVDMANXTDSC2_SPEC>`"]
pub type DEVDMANXTDSC2 = crate::Reg<devdmanxtdsc2::DEVDMANXTDSC2_SPEC>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 2)"]
pub mod devdmanxtdsc2;
#[doc = "DEVDMAADDRESS2 register accessor: an alias for `Reg<DEVDMAADDRESS2_SPEC>`"]
pub type DEVDMAADDRESS2 = crate::Reg<devdmaaddress2::DEVDMAADDRESS2_SPEC>;
#[doc = "Device DMA Channel Address Register (n = 2)"]
pub mod devdmaaddress2;
#[doc = "DEVDMACONTROL2 register accessor: an alias for `Reg<DEVDMACONTROL2_SPEC>`"]
pub type DEVDMACONTROL2 = crate::Reg<devdmacontrol2::DEVDMACONTROL2_SPEC>;
#[doc = "Device DMA Channel Control Register (n = 2)"]
pub mod devdmacontrol2;
#[doc = "DEVDMASTATUS2 register accessor: an alias for `Reg<DEVDMASTATUS2_SPEC>`"]
pub type DEVDMASTATUS2 = crate::Reg<devdmastatus2::DEVDMASTATUS2_SPEC>;
#[doc = "Device DMA Channel Status Register (n = 2)"]
pub mod devdmastatus2;
#[doc = "DEVDMANXTDSC3 register accessor: an alias for `Reg<DEVDMANXTDSC3_SPEC>`"]
pub type DEVDMANXTDSC3 = crate::Reg<devdmanxtdsc3::DEVDMANXTDSC3_SPEC>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 3)"]
pub mod devdmanxtdsc3;
#[doc = "DEVDMAADDRESS3 register accessor: an alias for `Reg<DEVDMAADDRESS3_SPEC>`"]
pub type DEVDMAADDRESS3 = crate::Reg<devdmaaddress3::DEVDMAADDRESS3_SPEC>;
#[doc = "Device DMA Channel Address Register (n = 3)"]
pub mod devdmaaddress3;
#[doc = "DEVDMACONTROL3 register accessor: an alias for `Reg<DEVDMACONTROL3_SPEC>`"]
pub type DEVDMACONTROL3 = crate::Reg<devdmacontrol3::DEVDMACONTROL3_SPEC>;
#[doc = "Device DMA Channel Control Register (n = 3)"]
pub mod devdmacontrol3;
#[doc = "DEVDMASTATUS3 register accessor: an alias for `Reg<DEVDMASTATUS3_SPEC>`"]
pub type DEVDMASTATUS3 = crate::Reg<devdmastatus3::DEVDMASTATUS3_SPEC>;
#[doc = "Device DMA Channel Status Register (n = 3)"]
pub mod devdmastatus3;
#[doc = "DEVDMANXTDSC4 register accessor: an alias for `Reg<DEVDMANXTDSC4_SPEC>`"]
pub type DEVDMANXTDSC4 = crate::Reg<devdmanxtdsc4::DEVDMANXTDSC4_SPEC>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 4)"]
pub mod devdmanxtdsc4;
#[doc = "DEVDMAADDRESS4 register accessor: an alias for `Reg<DEVDMAADDRESS4_SPEC>`"]
pub type DEVDMAADDRESS4 = crate::Reg<devdmaaddress4::DEVDMAADDRESS4_SPEC>;
#[doc = "Device DMA Channel Address Register (n = 4)"]
pub mod devdmaaddress4;
#[doc = "DEVDMACONTROL4 register accessor: an alias for `Reg<DEVDMACONTROL4_SPEC>`"]
pub type DEVDMACONTROL4 = crate::Reg<devdmacontrol4::DEVDMACONTROL4_SPEC>;
#[doc = "Device DMA Channel Control Register (n = 4)"]
pub mod devdmacontrol4;
#[doc = "DEVDMASTATUS4 register accessor: an alias for `Reg<DEVDMASTATUS4_SPEC>`"]
pub type DEVDMASTATUS4 = crate::Reg<devdmastatus4::DEVDMASTATUS4_SPEC>;
#[doc = "Device DMA Channel Status Register (n = 4)"]
pub mod devdmastatus4;
#[doc = "DEVDMANXTDSC5 register accessor: an alias for `Reg<DEVDMANXTDSC5_SPEC>`"]
pub type DEVDMANXTDSC5 = crate::Reg<devdmanxtdsc5::DEVDMANXTDSC5_SPEC>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 5)"]
pub mod devdmanxtdsc5;
#[doc = "DEVDMAADDRESS5 register accessor: an alias for `Reg<DEVDMAADDRESS5_SPEC>`"]
pub type DEVDMAADDRESS5 = crate::Reg<devdmaaddress5::DEVDMAADDRESS5_SPEC>;
#[doc = "Device DMA Channel Address Register (n = 5)"]
pub mod devdmaaddress5;
#[doc = "DEVDMACONTROL5 register accessor: an alias for `Reg<DEVDMACONTROL5_SPEC>`"]
pub type DEVDMACONTROL5 = crate::Reg<devdmacontrol5::DEVDMACONTROL5_SPEC>;
#[doc = "Device DMA Channel Control Register (n = 5)"]
pub mod devdmacontrol5;
#[doc = "DEVDMASTATUS5 register accessor: an alias for `Reg<DEVDMASTATUS5_SPEC>`"]
pub type DEVDMASTATUS5 = crate::Reg<devdmastatus5::DEVDMASTATUS5_SPEC>;
#[doc = "Device DMA Channel Status Register (n = 5)"]
pub mod devdmastatus5;
#[doc = "DEVDMANXTDSC6 register accessor: an alias for `Reg<DEVDMANXTDSC6_SPEC>`"]
pub type DEVDMANXTDSC6 = crate::Reg<devdmanxtdsc6::DEVDMANXTDSC6_SPEC>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 6)"]
pub mod devdmanxtdsc6;
#[doc = "DEVDMAADDRESS6 register accessor: an alias for `Reg<DEVDMAADDRESS6_SPEC>`"]
pub type DEVDMAADDRESS6 = crate::Reg<devdmaaddress6::DEVDMAADDRESS6_SPEC>;
#[doc = "Device DMA Channel Address Register (n = 6)"]
pub mod devdmaaddress6;
#[doc = "DEVDMACONTROL6 register accessor: an alias for `Reg<DEVDMACONTROL6_SPEC>`"]
pub type DEVDMACONTROL6 = crate::Reg<devdmacontrol6::DEVDMACONTROL6_SPEC>;
#[doc = "Device DMA Channel Control Register (n = 6)"]
pub mod devdmacontrol6;
#[doc = "DEVDMASTATUS6 register accessor: an alias for `Reg<DEVDMASTATUS6_SPEC>`"]
pub type DEVDMASTATUS6 = crate::Reg<devdmastatus6::DEVDMASTATUS6_SPEC>;
#[doc = "Device DMA Channel Status Register (n = 6)"]
pub mod devdmastatus6;
#[doc = "DEVDMANXTDSC7 register accessor: an alias for `Reg<DEVDMANXTDSC7_SPEC>`"]
pub type DEVDMANXTDSC7 = crate::Reg<devdmanxtdsc7::DEVDMANXTDSC7_SPEC>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 7)"]
pub mod devdmanxtdsc7;
#[doc = "DEVDMAADDRESS7 register accessor: an alias for `Reg<DEVDMAADDRESS7_SPEC>`"]
pub type DEVDMAADDRESS7 = crate::Reg<devdmaaddress7::DEVDMAADDRESS7_SPEC>;
#[doc = "Device DMA Channel Address Register (n = 7)"]
pub mod devdmaaddress7;
#[doc = "DEVDMACONTROL7 register accessor: an alias for `Reg<DEVDMACONTROL7_SPEC>`"]
pub type DEVDMACONTROL7 = crate::Reg<devdmacontrol7::DEVDMACONTROL7_SPEC>;
#[doc = "Device DMA Channel Control Register (n = 7)"]
pub mod devdmacontrol7;
#[doc = "DEVDMASTATUS7 register accessor: an alias for `Reg<DEVDMASTATUS7_SPEC>`"]
pub type DEVDMASTATUS7 = crate::Reg<devdmastatus7::DEVDMASTATUS7_SPEC>;
#[doc = "Device DMA Channel Status Register (n = 7)"]
pub mod devdmastatus7;
#[doc = "HSTCTRL register accessor: an alias for `Reg<HSTCTRL_SPEC>`"]
pub type HSTCTRL = crate::Reg<hstctrl::HSTCTRL_SPEC>;
#[doc = "Host General Control Register"]
pub mod hstctrl;
#[doc = "HSTISR register accessor: an alias for `Reg<HSTISR_SPEC>`"]
pub type HSTISR = crate::Reg<hstisr::HSTISR_SPEC>;
#[doc = "Host Global Interrupt Status Register"]
pub mod hstisr;
#[doc = "HSTICR register accessor: an alias for `Reg<HSTICR_SPEC>`"]
pub type HSTICR = crate::Reg<hsticr::HSTICR_SPEC>;
#[doc = "Host Global Interrupt Clear Register"]
pub mod hsticr;
#[doc = "HSTIFR register accessor: an alias for `Reg<HSTIFR_SPEC>`"]
pub type HSTIFR = crate::Reg<hstifr::HSTIFR_SPEC>;
#[doc = "Host Global Interrupt Set Register"]
pub mod hstifr;
#[doc = "HSTIMR register accessor: an alias for `Reg<HSTIMR_SPEC>`"]
pub type HSTIMR = crate::Reg<hstimr::HSTIMR_SPEC>;
#[doc = "Host Global Interrupt Mask Register"]
pub mod hstimr;
#[doc = "HSTIDR register accessor: an alias for `Reg<HSTIDR_SPEC>`"]
pub type HSTIDR = crate::Reg<hstidr::HSTIDR_SPEC>;
#[doc = "Host Global Interrupt Disable Register"]
pub mod hstidr;
#[doc = "HSTIER register accessor: an alias for `Reg<HSTIER_SPEC>`"]
pub type HSTIER = crate::Reg<hstier::HSTIER_SPEC>;
#[doc = "Host Global Interrupt Enable Register"]
pub mod hstier;
#[doc = "HSTPIP register accessor: an alias for `Reg<HSTPIP_SPEC>`"]
pub type HSTPIP = crate::Reg<hstpip::HSTPIP_SPEC>;
#[doc = "Host Pipe Register"]
pub mod hstpip;
#[doc = "HSTFNUM register accessor: an alias for `Reg<HSTFNUM_SPEC>`"]
pub type HSTFNUM = crate::Reg<hstfnum::HSTFNUM_SPEC>;
#[doc = "Host Frame Number Register"]
pub mod hstfnum;
#[doc = "HSTADDR1 register accessor: an alias for `Reg<HSTADDR1_SPEC>`"]
pub type HSTADDR1 = crate::Reg<hstaddr1::HSTADDR1_SPEC>;
#[doc = "Host Address 1 Register"]
pub mod hstaddr1;
#[doc = "HSTADDR2 register accessor: an alias for `Reg<HSTADDR2_SPEC>`"]
pub type HSTADDR2 = crate::Reg<hstaddr2::HSTADDR2_SPEC>;
#[doc = "Host Address 2 Register"]
pub mod hstaddr2;
#[doc = "HSTADDR3 register accessor: an alias for `Reg<HSTADDR3_SPEC>`"]
pub type HSTADDR3 = crate::Reg<hstaddr3::HSTADDR3_SPEC>;
#[doc = "Host Address 3 Register"]
pub mod hstaddr3;
#[doc = "HSTPIPCFG register accessor: an alias for `Reg<HSTPIPCFG_SPEC>`"]
pub type HSTPIPCFG = crate::Reg<hstpipcfg::HSTPIPCFG_SPEC>;
#[doc = "Host Pipe Configuration Register (n = 0)"]
pub mod hstpipcfg;
#[doc = "HSBOHSCP_HSTPIPCFG0_HSBOHSCP register accessor: an alias for `Reg<HSBOHSCP_HSTPIPCFG0_HSBOHSCP_SPEC>`"]
pub type HSBOHSCP_HSTPIPCFG0_HSBOHSCP =
    crate::Reg<hsbohscp_hstpipcfg0_hsbohscp::HSBOHSCP_HSTPIPCFG0_HSBOHSCP_SPEC>;
#[doc = "Host Pipe Configuration Register (n = 0)"]
pub mod hsbohscp_hstpipcfg0_hsbohscp;
#[doc = "HSTPIPISR register accessor: an alias for `Reg<HSTPIPISR_SPEC>`"]
pub type HSTPIPISR = crate::Reg<hstpipisr::HSTPIPISR_SPEC>;
#[doc = "Host Pipe Status Register (n = 0)"]
pub mod hstpipisr;
#[doc = "INTPIPES_HSTPIPISR0_INTPIPES register accessor: an alias for `Reg<INTPIPES_HSTPIPISR0_INTPIPES_SPEC>`"]
pub type INTPIPES_HSTPIPISR0_INTPIPES =
    crate::Reg<intpipes_hstpipisr0_intpipes::INTPIPES_HSTPIPISR0_INTPIPES_SPEC>;
#[doc = "Host Pipe Status Register (n = 0)"]
pub mod intpipes_hstpipisr0_intpipes;
#[doc = "ISOPIPES_HSTPIPISR0_ISOPIPES register accessor: an alias for `Reg<ISOPIPES_HSTPIPISR0_ISOPIPES_SPEC>`"]
pub type ISOPIPES_HSTPIPISR0_ISOPIPES =
    crate::Reg<isopipes_hstpipisr0_isopipes::ISOPIPES_HSTPIPISR0_ISOPIPES_SPEC>;
#[doc = "Host Pipe Status Register (n = 0)"]
pub mod isopipes_hstpipisr0_isopipes;
#[doc = "HSTPIPICR register accessor: an alias for `Reg<HSTPIPICR_SPEC>`"]
pub type HSTPIPICR = crate::Reg<hstpipicr::HSTPIPICR_SPEC>;
#[doc = "Host Pipe Clear Register (n = 0)"]
pub mod hstpipicr;
#[doc = "INTPIPES_HSTPIPICR0_INTPIPES register accessor: an alias for `Reg<INTPIPES_HSTPIPICR0_INTPIPES_SPEC>`"]
pub type INTPIPES_HSTPIPICR0_INTPIPES =
    crate::Reg<intpipes_hstpipicr0_intpipes::INTPIPES_HSTPIPICR0_INTPIPES_SPEC>;
#[doc = "Host Pipe Clear Register (n = 0)"]
pub mod intpipes_hstpipicr0_intpipes;
#[doc = "ISOPIPES_HSTPIPICR0_ISOPIPES register accessor: an alias for `Reg<ISOPIPES_HSTPIPICR0_ISOPIPES_SPEC>`"]
pub type ISOPIPES_HSTPIPICR0_ISOPIPES =
    crate::Reg<isopipes_hstpipicr0_isopipes::ISOPIPES_HSTPIPICR0_ISOPIPES_SPEC>;
#[doc = "Host Pipe Clear Register (n = 0)"]
pub mod isopipes_hstpipicr0_isopipes;
#[doc = "HSTPIPIFR register accessor: an alias for `Reg<HSTPIPIFR_SPEC>`"]
pub type HSTPIPIFR = crate::Reg<hstpipifr::HSTPIPIFR_SPEC>;
#[doc = "Host Pipe Set Register (n = 0)"]
pub mod hstpipifr;
#[doc = "INTPIPES_HSTPIPIFR0_INTPIPES register accessor: an alias for `Reg<INTPIPES_HSTPIPIFR0_INTPIPES_SPEC>`"]
pub type INTPIPES_HSTPIPIFR0_INTPIPES =
    crate::Reg<intpipes_hstpipifr0_intpipes::INTPIPES_HSTPIPIFR0_INTPIPES_SPEC>;
#[doc = "Host Pipe Set Register (n = 0)"]
pub mod intpipes_hstpipifr0_intpipes;
#[doc = "ISOPIPES_HSTPIPIFR0_ISOPIPES register accessor: an alias for `Reg<ISOPIPES_HSTPIPIFR0_ISOPIPES_SPEC>`"]
pub type ISOPIPES_HSTPIPIFR0_ISOPIPES =
    crate::Reg<isopipes_hstpipifr0_isopipes::ISOPIPES_HSTPIPIFR0_ISOPIPES_SPEC>;
#[doc = "Host Pipe Set Register (n = 0)"]
pub mod isopipes_hstpipifr0_isopipes;
#[doc = "HSTPIPIMR register accessor: an alias for `Reg<HSTPIPIMR_SPEC>`"]
pub type HSTPIPIMR = crate::Reg<hstpipimr::HSTPIPIMR_SPEC>;
#[doc = "Host Pipe Mask Register (n = 0)"]
pub mod hstpipimr;
#[doc = "INTPIPES_HSTPIPIMR0_INTPIPES register accessor: an alias for `Reg<INTPIPES_HSTPIPIMR0_INTPIPES_SPEC>`"]
pub type INTPIPES_HSTPIPIMR0_INTPIPES =
    crate::Reg<intpipes_hstpipimr0_intpipes::INTPIPES_HSTPIPIMR0_INTPIPES_SPEC>;
#[doc = "Host Pipe Mask Register (n = 0)"]
pub mod intpipes_hstpipimr0_intpipes;
#[doc = "ISOPIPES_HSTPIPIMR0_ISOPIPES register accessor: an alias for `Reg<ISOPIPES_HSTPIPIMR0_ISOPIPES_SPEC>`"]
pub type ISOPIPES_HSTPIPIMR0_ISOPIPES =
    crate::Reg<isopipes_hstpipimr0_isopipes::ISOPIPES_HSTPIPIMR0_ISOPIPES_SPEC>;
#[doc = "Host Pipe Mask Register (n = 0)"]
pub mod isopipes_hstpipimr0_isopipes;
#[doc = "HSTPIPIER register accessor: an alias for `Reg<HSTPIPIER_SPEC>`"]
pub type HSTPIPIER = crate::Reg<hstpipier::HSTPIPIER_SPEC>;
#[doc = "Host Pipe Enable Register (n = 0)"]
pub mod hstpipier;
#[doc = "INTPIPES_HSTPIPIER0_INTPIPES register accessor: an alias for `Reg<INTPIPES_HSTPIPIER0_INTPIPES_SPEC>`"]
pub type INTPIPES_HSTPIPIER0_INTPIPES =
    crate::Reg<intpipes_hstpipier0_intpipes::INTPIPES_HSTPIPIER0_INTPIPES_SPEC>;
#[doc = "Host Pipe Enable Register (n = 0)"]
pub mod intpipes_hstpipier0_intpipes;
#[doc = "ISOPIPES_HSTPIPIER0_ISOPIPES register accessor: an alias for `Reg<ISOPIPES_HSTPIPIER0_ISOPIPES_SPEC>`"]
pub type ISOPIPES_HSTPIPIER0_ISOPIPES =
    crate::Reg<isopipes_hstpipier0_isopipes::ISOPIPES_HSTPIPIER0_ISOPIPES_SPEC>;
#[doc = "Host Pipe Enable Register (n = 0)"]
pub mod isopipes_hstpipier0_isopipes;
#[doc = "HSTPIPIDR register accessor: an alias for `Reg<HSTPIPIDR_SPEC>`"]
pub type HSTPIPIDR = crate::Reg<hstpipidr::HSTPIPIDR_SPEC>;
#[doc = "Host Pipe Disable Register (n = 0)"]
pub mod hstpipidr;
#[doc = "INTPIPES_HSTPIPIDR0_INTPIPES register accessor: an alias for `Reg<INTPIPES_HSTPIPIDR0_INTPIPES_SPEC>`"]
pub type INTPIPES_HSTPIPIDR0_INTPIPES =
    crate::Reg<intpipes_hstpipidr0_intpipes::INTPIPES_HSTPIPIDR0_INTPIPES_SPEC>;
#[doc = "Host Pipe Disable Register (n = 0)"]
pub mod intpipes_hstpipidr0_intpipes;
#[doc = "ISOPIPES_HSTPIPIDR0_ISOPIPES register accessor: an alias for `Reg<ISOPIPES_HSTPIPIDR0_ISOPIPES_SPEC>`"]
pub type ISOPIPES_HSTPIPIDR0_ISOPIPES =
    crate::Reg<isopipes_hstpipidr0_isopipes::ISOPIPES_HSTPIPIDR0_ISOPIPES_SPEC>;
#[doc = "Host Pipe Disable Register (n = 0)"]
pub mod isopipes_hstpipidr0_isopipes;
#[doc = "HSTPIPINRQ register accessor: an alias for `Reg<HSTPIPINRQ_SPEC>`"]
pub type HSTPIPINRQ = crate::Reg<hstpipinrq::HSTPIPINRQ_SPEC>;
#[doc = "Host Pipe IN Request Register (n = 0)"]
pub mod hstpipinrq;
#[doc = "HSTPIPERR register accessor: an alias for `Reg<HSTPIPERR_SPEC>`"]
pub type HSTPIPERR = crate::Reg<hstpiperr::HSTPIPERR_SPEC>;
#[doc = "Host Pipe Error Register (n = 0)"]
pub mod hstpiperr;
#[doc = "HSTDMANXTDSC1 register accessor: an alias for `Reg<HSTDMANXTDSC1_SPEC>`"]
pub type HSTDMANXTDSC1 = crate::Reg<hstdmanxtdsc1::HSTDMANXTDSC1_SPEC>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod hstdmanxtdsc1;
#[doc = "HSTDMAADDRESS1 register accessor: an alias for `Reg<HSTDMAADDRESS1_SPEC>`"]
pub type HSTDMAADDRESS1 = crate::Reg<hstdmaaddress1::HSTDMAADDRESS1_SPEC>;
#[doc = "Host DMA Channel Address Register (n = 1)"]
pub mod hstdmaaddress1;
#[doc = "HSTDMACONTROL1 register accessor: an alias for `Reg<HSTDMACONTROL1_SPEC>`"]
pub type HSTDMACONTROL1 = crate::Reg<hstdmacontrol1::HSTDMACONTROL1_SPEC>;
#[doc = "Host DMA Channel Control Register (n = 1)"]
pub mod hstdmacontrol1;
#[doc = "HSTDMASTATUS1 register accessor: an alias for `Reg<HSTDMASTATUS1_SPEC>`"]
pub type HSTDMASTATUS1 = crate::Reg<hstdmastatus1::HSTDMASTATUS1_SPEC>;
#[doc = "Host DMA Channel Status Register (n = 1)"]
pub mod hstdmastatus1;
#[doc = "HSTDMANXTDSC2 register accessor: an alias for `Reg<HSTDMANXTDSC2_SPEC>`"]
pub type HSTDMANXTDSC2 = crate::Reg<hstdmanxtdsc2::HSTDMANXTDSC2_SPEC>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 2)"]
pub mod hstdmanxtdsc2;
#[doc = "HSTDMAADDRESS2 register accessor: an alias for `Reg<HSTDMAADDRESS2_SPEC>`"]
pub type HSTDMAADDRESS2 = crate::Reg<hstdmaaddress2::HSTDMAADDRESS2_SPEC>;
#[doc = "Host DMA Channel Address Register (n = 2)"]
pub mod hstdmaaddress2;
#[doc = "HSTDMACONTROL2 register accessor: an alias for `Reg<HSTDMACONTROL2_SPEC>`"]
pub type HSTDMACONTROL2 = crate::Reg<hstdmacontrol2::HSTDMACONTROL2_SPEC>;
#[doc = "Host DMA Channel Control Register (n = 2)"]
pub mod hstdmacontrol2;
#[doc = "HSTDMASTATUS2 register accessor: an alias for `Reg<HSTDMASTATUS2_SPEC>`"]
pub type HSTDMASTATUS2 = crate::Reg<hstdmastatus2::HSTDMASTATUS2_SPEC>;
#[doc = "Host DMA Channel Status Register (n = 2)"]
pub mod hstdmastatus2;
#[doc = "HSTDMANXTDSC3 register accessor: an alias for `Reg<HSTDMANXTDSC3_SPEC>`"]
pub type HSTDMANXTDSC3 = crate::Reg<hstdmanxtdsc3::HSTDMANXTDSC3_SPEC>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 3)"]
pub mod hstdmanxtdsc3;
#[doc = "HSTDMAADDRESS3 register accessor: an alias for `Reg<HSTDMAADDRESS3_SPEC>`"]
pub type HSTDMAADDRESS3 = crate::Reg<hstdmaaddress3::HSTDMAADDRESS3_SPEC>;
#[doc = "Host DMA Channel Address Register (n = 3)"]
pub mod hstdmaaddress3;
#[doc = "HSTDMACONTROL3 register accessor: an alias for `Reg<HSTDMACONTROL3_SPEC>`"]
pub type HSTDMACONTROL3 = crate::Reg<hstdmacontrol3::HSTDMACONTROL3_SPEC>;
#[doc = "Host DMA Channel Control Register (n = 3)"]
pub mod hstdmacontrol3;
#[doc = "HSTDMASTATUS3 register accessor: an alias for `Reg<HSTDMASTATUS3_SPEC>`"]
pub type HSTDMASTATUS3 = crate::Reg<hstdmastatus3::HSTDMASTATUS3_SPEC>;
#[doc = "Host DMA Channel Status Register (n = 3)"]
pub mod hstdmastatus3;
#[doc = "HSTDMANXTDSC4 register accessor: an alias for `Reg<HSTDMANXTDSC4_SPEC>`"]
pub type HSTDMANXTDSC4 = crate::Reg<hstdmanxtdsc4::HSTDMANXTDSC4_SPEC>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 4)"]
pub mod hstdmanxtdsc4;
#[doc = "HSTDMAADDRESS4 register accessor: an alias for `Reg<HSTDMAADDRESS4_SPEC>`"]
pub type HSTDMAADDRESS4 = crate::Reg<hstdmaaddress4::HSTDMAADDRESS4_SPEC>;
#[doc = "Host DMA Channel Address Register (n = 4)"]
pub mod hstdmaaddress4;
#[doc = "HSTDMACONTROL4 register accessor: an alias for `Reg<HSTDMACONTROL4_SPEC>`"]
pub type HSTDMACONTROL4 = crate::Reg<hstdmacontrol4::HSTDMACONTROL4_SPEC>;
#[doc = "Host DMA Channel Control Register (n = 4)"]
pub mod hstdmacontrol4;
#[doc = "HSTDMASTATUS4 register accessor: an alias for `Reg<HSTDMASTATUS4_SPEC>`"]
pub type HSTDMASTATUS4 = crate::Reg<hstdmastatus4::HSTDMASTATUS4_SPEC>;
#[doc = "Host DMA Channel Status Register (n = 4)"]
pub mod hstdmastatus4;
#[doc = "HSTDMANXTDSC5 register accessor: an alias for `Reg<HSTDMANXTDSC5_SPEC>`"]
pub type HSTDMANXTDSC5 = crate::Reg<hstdmanxtdsc5::HSTDMANXTDSC5_SPEC>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 5)"]
pub mod hstdmanxtdsc5;
#[doc = "HSTDMAADDRESS5 register accessor: an alias for `Reg<HSTDMAADDRESS5_SPEC>`"]
pub type HSTDMAADDRESS5 = crate::Reg<hstdmaaddress5::HSTDMAADDRESS5_SPEC>;
#[doc = "Host DMA Channel Address Register (n = 5)"]
pub mod hstdmaaddress5;
#[doc = "HSTDMACONTROL5 register accessor: an alias for `Reg<HSTDMACONTROL5_SPEC>`"]
pub type HSTDMACONTROL5 = crate::Reg<hstdmacontrol5::HSTDMACONTROL5_SPEC>;
#[doc = "Host DMA Channel Control Register (n = 5)"]
pub mod hstdmacontrol5;
#[doc = "HSTDMASTATUS5 register accessor: an alias for `Reg<HSTDMASTATUS5_SPEC>`"]
pub type HSTDMASTATUS5 = crate::Reg<hstdmastatus5::HSTDMASTATUS5_SPEC>;
#[doc = "Host DMA Channel Status Register (n = 5)"]
pub mod hstdmastatus5;
#[doc = "HSTDMANXTDSC6 register accessor: an alias for `Reg<HSTDMANXTDSC6_SPEC>`"]
pub type HSTDMANXTDSC6 = crate::Reg<hstdmanxtdsc6::HSTDMANXTDSC6_SPEC>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 6)"]
pub mod hstdmanxtdsc6;
#[doc = "HSTDMAADDRESS6 register accessor: an alias for `Reg<HSTDMAADDRESS6_SPEC>`"]
pub type HSTDMAADDRESS6 = crate::Reg<hstdmaaddress6::HSTDMAADDRESS6_SPEC>;
#[doc = "Host DMA Channel Address Register (n = 6)"]
pub mod hstdmaaddress6;
#[doc = "HSTDMACONTROL6 register accessor: an alias for `Reg<HSTDMACONTROL6_SPEC>`"]
pub type HSTDMACONTROL6 = crate::Reg<hstdmacontrol6::HSTDMACONTROL6_SPEC>;
#[doc = "Host DMA Channel Control Register (n = 6)"]
pub mod hstdmacontrol6;
#[doc = "HSTDMASTATUS6 register accessor: an alias for `Reg<HSTDMASTATUS6_SPEC>`"]
pub type HSTDMASTATUS6 = crate::Reg<hstdmastatus6::HSTDMASTATUS6_SPEC>;
#[doc = "Host DMA Channel Status Register (n = 6)"]
pub mod hstdmastatus6;
#[doc = "HSTDMANXTDSC7 register accessor: an alias for `Reg<HSTDMANXTDSC7_SPEC>`"]
pub type HSTDMANXTDSC7 = crate::Reg<hstdmanxtdsc7::HSTDMANXTDSC7_SPEC>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 7)"]
pub mod hstdmanxtdsc7;
#[doc = "HSTDMAADDRESS7 register accessor: an alias for `Reg<HSTDMAADDRESS7_SPEC>`"]
pub type HSTDMAADDRESS7 = crate::Reg<hstdmaaddress7::HSTDMAADDRESS7_SPEC>;
#[doc = "Host DMA Channel Address Register (n = 7)"]
pub mod hstdmaaddress7;
#[doc = "HSTDMACONTROL7 register accessor: an alias for `Reg<HSTDMACONTROL7_SPEC>`"]
pub type HSTDMACONTROL7 = crate::Reg<hstdmacontrol7::HSTDMACONTROL7_SPEC>;
#[doc = "Host DMA Channel Control Register (n = 7)"]
pub mod hstdmacontrol7;
#[doc = "HSTDMASTATUS7 register accessor: an alias for `Reg<HSTDMASTATUS7_SPEC>`"]
pub type HSTDMASTATUS7 = crate::Reg<hstdmastatus7::HSTDMASTATUS7_SPEC>;
#[doc = "Host DMA Channel Status Register (n = 7)"]
pub mod hstdmastatus7;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "General Control Register"]
pub mod ctrl;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "General Status Register"]
pub mod sr;
#[doc = "SCR register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "General Status Clear Register"]
pub mod scr;
#[doc = "SFR register accessor: an alias for `Reg<SFR_SPEC>`"]
pub type SFR = crate::Reg<sfr::SFR_SPEC>;
#[doc = "General Status Set Register"]
pub mod sfr;
#[doc = "FSM register accessor: an alias for `Reg<FSM_SPEC>`"]
pub type FSM = crate::Reg<fsm::FSM_SPEC>;
#[doc = "General Finite State Machine Register"]
pub mod fsm;
