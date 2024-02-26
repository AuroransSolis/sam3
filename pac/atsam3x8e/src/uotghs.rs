#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    devctrl: Devctrl,
    devisr: Devisr,
    devicr: Devicr,
    devifr: Devifr,
    devimr: Devimr,
    devidr: Devidr,
    devier: Devier,
    devept: Devept,
    devfnum: Devfnum,
    _reserved9: [u8; 0xdc],
    deveptcfg0: Deveptcfg0,
    deveptcfg1: Deveptcfg1,
    deveptcfg2: Deveptcfg2,
    deveptcfg3: Deveptcfg3,
    deveptcfg4: Deveptcfg4,
    deveptcfg5: Deveptcfg5,
    deveptcfg6: Deveptcfg6,
    deveptcfg7: Deveptcfg7,
    deveptcfg8: Deveptcfg8,
    deveptcfg9: Deveptcfg9,
    _reserved19: [u8; 0x08],
    _reserved_19_deveptisr0: [u8; 0x04],
    deveptisr1: Deveptisr1,
    deveptisr2: Deveptisr2,
    deveptisr3: Deveptisr3,
    deveptisr4: Deveptisr4,
    deveptisr5: Deveptisr5,
    deveptisr6: Deveptisr6,
    deveptisr7: Deveptisr7,
    deveptisr8: Deveptisr8,
    deveptisr9: Deveptisr9,
    _reserved29: [u8; 0x08],
    _reserved_29_devepticr0: [u8; 0x04],
    devepticr1: Devepticr1,
    devepticr2: Devepticr2,
    devepticr3: Devepticr3,
    devepticr4: Devepticr4,
    devepticr5: Devepticr5,
    devepticr6: Devepticr6,
    devepticr7: Devepticr7,
    devepticr8: Devepticr8,
    devepticr9: Devepticr9,
    _reserved39: [u8; 0x08],
    _reserved_39_deveptifr0: [u8; 0x04],
    deveptifr1: Deveptifr1,
    deveptifr2: Deveptifr2,
    deveptifr3: Deveptifr3,
    deveptifr4: Deveptifr4,
    deveptifr5: Deveptifr5,
    deveptifr6: Deveptifr6,
    deveptifr7: Deveptifr7,
    deveptifr8: Deveptifr8,
    deveptifr9: Deveptifr9,
    _reserved49: [u8; 0x08],
    _reserved_49_deveptimr0: [u8; 0x04],
    deveptimr1: Deveptimr1,
    deveptimr2: Deveptimr2,
    deveptimr3: Deveptimr3,
    deveptimr4: Deveptimr4,
    deveptimr5: Deveptimr5,
    deveptimr6: Deveptimr6,
    deveptimr7: Deveptimr7,
    deveptimr8: Deveptimr8,
    deveptimr9: Deveptimr9,
    _reserved59: [u8; 0x08],
    _reserved_59_deveptier0: [u8; 0x04],
    deveptier1: Deveptier1,
    deveptier2: Deveptier2,
    deveptier3: Deveptier3,
    deveptier4: Deveptier4,
    deveptier5: Deveptier5,
    deveptier6: Deveptier6,
    deveptier7: Deveptier7,
    deveptier8: Deveptier8,
    deveptier9: Deveptier9,
    _reserved69: [u8; 0x08],
    _reserved_69_deveptidr0: [u8; 0x04],
    deveptidr1: Deveptidr1,
    deveptidr2: Deveptidr2,
    deveptidr3: Deveptidr3,
    deveptidr4: Deveptidr4,
    deveptidr5: Deveptidr5,
    deveptidr6: Deveptidr6,
    deveptidr7: Deveptidr7,
    deveptidr8: Deveptidr8,
    deveptidr9: Deveptidr9,
    _reserved79: [u8; 0xc8],
    devdmanxtdsc1: Devdmanxtdsc1,
    devdmaaddress1: Devdmaaddress1,
    devdmacontrol1: Devdmacontrol1,
    devdmastatus1: Devdmastatus1,
    devdmanxtdsc2: Devdmanxtdsc2,
    devdmaaddress2: Devdmaaddress2,
    devdmacontrol2: Devdmacontrol2,
    devdmastatus2: Devdmastatus2,
    devdmanxtdsc3: Devdmanxtdsc3,
    devdmaaddress3: Devdmaaddress3,
    devdmacontrol3: Devdmacontrol3,
    devdmastatus3: Devdmastatus3,
    devdmanxtdsc4: Devdmanxtdsc4,
    devdmaaddress4: Devdmaaddress4,
    devdmacontrol4: Devdmacontrol4,
    devdmastatus4: Devdmastatus4,
    devdmanxtdsc5: Devdmanxtdsc5,
    devdmaaddress5: Devdmaaddress5,
    devdmacontrol5: Devdmacontrol5,
    devdmastatus5: Devdmastatus5,
    devdmanxtdsc6: Devdmanxtdsc6,
    devdmaaddress6: Devdmaaddress6,
    devdmacontrol6: Devdmacontrol6,
    devdmastatus6: Devdmastatus6,
    devdmanxtdsc7: Devdmanxtdsc7,
    devdmaaddress7: Devdmaaddress7,
    devdmacontrol7: Devdmacontrol7,
    devdmastatus7: Devdmastatus7,
    _reserved107: [u8; 0x80],
    hstctrl: Hstctrl,
    hstisr: Hstisr,
    hsticr: Hsticr,
    hstifr: Hstifr,
    hstimr: Hstimr,
    hstidr: Hstidr,
    hstier: Hstier,
    hstpip: Hstpip,
    hstfnum: Hstfnum,
    hstaddr1: Hstaddr1,
    hstaddr2: Hstaddr2,
    hstaddr3: Hstaddr3,
    _reserved119: [u8; 0xd0],
    _reserved_119_hstpipcfg0: [u8; 0x04],
    hstpipcfg1: Hstpipcfg1,
    hstpipcfg2: Hstpipcfg2,
    hstpipcfg3: Hstpipcfg3,
    hstpipcfg4: Hstpipcfg4,
    hstpipcfg5: Hstpipcfg5,
    hstpipcfg6: Hstpipcfg6,
    hstpipcfg7: Hstpipcfg7,
    hstpipcfg8: Hstpipcfg8,
    hstpipcfg9: Hstpipcfg9,
    _reserved129: [u8; 0x08],
    _reserved_129_hstpipisr0: [u8; 0x04],
    hstpipisr1: Hstpipisr1,
    hstpipisr2: Hstpipisr2,
    hstpipisr3: Hstpipisr3,
    hstpipisr4: Hstpipisr4,
    hstpipisr5: Hstpipisr5,
    hstpipisr6: Hstpipisr6,
    hstpipisr7: Hstpipisr7,
    hstpipisr8: Hstpipisr8,
    hstpipisr9: Hstpipisr9,
    _reserved139: [u8; 0x08],
    _reserved_139_hstpipicr0: [u8; 0x04],
    hstpipicr1: Hstpipicr1,
    hstpipicr2: Hstpipicr2,
    hstpipicr3: Hstpipicr3,
    hstpipicr4: Hstpipicr4,
    hstpipicr5: Hstpipicr5,
    hstpipicr6: Hstpipicr6,
    hstpipicr7: Hstpipicr7,
    hstpipicr8: Hstpipicr8,
    hstpipicr9: Hstpipicr9,
    _reserved149: [u8; 0x08],
    _reserved_149_hstpipifr0: [u8; 0x04],
    hstpipifr1: Hstpipifr1,
    hstpipifr2: Hstpipifr2,
    hstpipifr3: Hstpipifr3,
    hstpipifr4: Hstpipifr4,
    hstpipifr5: Hstpipifr5,
    hstpipifr6: Hstpipifr6,
    hstpipifr7: Hstpipifr7,
    hstpipifr8: Hstpipifr8,
    hstpipifr9: Hstpipifr9,
    _reserved159: [u8; 0x08],
    _reserved_159_hstpipimr0: [u8; 0x04],
    hstpipimr1: Hstpipimr1,
    hstpipimr2: Hstpipimr2,
    hstpipimr3: Hstpipimr3,
    hstpipimr4: Hstpipimr4,
    hstpipimr5: Hstpipimr5,
    hstpipimr6: Hstpipimr6,
    hstpipimr7: Hstpipimr7,
    hstpipimr8: Hstpipimr8,
    hstpipimr9: Hstpipimr9,
    _reserved169: [u8; 0x08],
    _reserved_169_hstpipier0: [u8; 0x04],
    hstpipier1: Hstpipier1,
    hstpipier2: Hstpipier2,
    hstpipier3: Hstpipier3,
    hstpipier4: Hstpipier4,
    hstpipier5: Hstpipier5,
    hstpipier6: Hstpipier6,
    hstpipier7: Hstpipier7,
    hstpipier8: Hstpipier8,
    hstpipier9: Hstpipier9,
    _reserved179: [u8; 0x08],
    _reserved_179_hstpipidr0: [u8; 0x04],
    hstpipidr1: Hstpipidr1,
    hstpipidr2: Hstpipidr2,
    hstpipidr3: Hstpipidr3,
    hstpipidr4: Hstpipidr4,
    hstpipidr5: Hstpipidr5,
    hstpipidr6: Hstpipidr6,
    hstpipidr7: Hstpipidr7,
    hstpipidr8: Hstpipidr8,
    hstpipidr9: Hstpipidr9,
    _reserved189: [u8; 0x08],
    hstpipinrq0: Hstpipinrq0,
    hstpipinrq1: Hstpipinrq1,
    hstpipinrq2: Hstpipinrq2,
    hstpipinrq3: Hstpipinrq3,
    hstpipinrq4: Hstpipinrq4,
    hstpipinrq5: Hstpipinrq5,
    hstpipinrq6: Hstpipinrq6,
    hstpipinrq7: Hstpipinrq7,
    hstpipinrq8: Hstpipinrq8,
    hstpipinrq9: Hstpipinrq9,
    _reserved199: [u8; 0x08],
    hstpiperr0: Hstpiperr0,
    hstpiperr1: Hstpiperr1,
    hstpiperr2: Hstpiperr2,
    hstpiperr3: Hstpiperr3,
    hstpiperr4: Hstpiperr4,
    hstpiperr5: Hstpiperr5,
    hstpiperr6: Hstpiperr6,
    hstpiperr7: Hstpiperr7,
    hstpiperr8: Hstpiperr8,
    hstpiperr9: Hstpiperr9,
    _reserved209: [u8; 0x68],
    hstdmanxtdsc1: Hstdmanxtdsc1,
    hstdmaaddress1: Hstdmaaddress1,
    hstdmacontrol1: Hstdmacontrol1,
    hstdmastatus1: Hstdmastatus1,
    hstdmanxtdsc2: Hstdmanxtdsc2,
    hstdmaaddress2: Hstdmaaddress2,
    hstdmacontrol2: Hstdmacontrol2,
    hstdmastatus2: Hstdmastatus2,
    hstdmanxtdsc3: Hstdmanxtdsc3,
    hstdmaaddress3: Hstdmaaddress3,
    hstdmacontrol3: Hstdmacontrol3,
    hstdmastatus3: Hstdmastatus3,
    hstdmanxtdsc4: Hstdmanxtdsc4,
    hstdmaaddress4: Hstdmaaddress4,
    hstdmacontrol4: Hstdmacontrol4,
    hstdmastatus4: Hstdmastatus4,
    hstdmanxtdsc5: Hstdmanxtdsc5,
    hstdmaaddress5: Hstdmaaddress5,
    hstdmacontrol5: Hstdmacontrol5,
    hstdmastatus5: Hstdmastatus5,
    hstdmanxtdsc6: Hstdmanxtdsc6,
    hstdmaaddress6: Hstdmaaddress6,
    hstdmacontrol6: Hstdmacontrol6,
    hstdmastatus6: Hstdmastatus6,
    hstdmanxtdsc7: Hstdmanxtdsc7,
    hstdmaaddress7: Hstdmaaddress7,
    hstdmacontrol7: Hstdmacontrol7,
    hstdmastatus7: Hstdmastatus7,
    _reserved237: [u8; 0x80],
    ctrl: Ctrl,
    sr: Sr,
    scr: Scr,
    sfr: Sfr,
    _reserved241: [u8; 0x1c],
    fsm: Fsm,
}
impl RegisterBlock {
    #[doc = "0x00 - Device General Control Register"]
    #[inline(always)]
    pub const fn devctrl(&self) -> &Devctrl {
        &self.devctrl
    }
    #[doc = "0x04 - Device Global Interrupt Status Register"]
    #[inline(always)]
    pub const fn devisr(&self) -> &Devisr {
        &self.devisr
    }
    #[doc = "0x08 - Device Global Interrupt Clear Register"]
    #[inline(always)]
    pub const fn devicr(&self) -> &Devicr {
        &self.devicr
    }
    #[doc = "0x0c - Device Global Interrupt Set Register"]
    #[inline(always)]
    pub const fn devifr(&self) -> &Devifr {
        &self.devifr
    }
    #[doc = "0x10 - Device Global Interrupt Mask Register"]
    #[inline(always)]
    pub const fn devimr(&self) -> &Devimr {
        &self.devimr
    }
    #[doc = "0x14 - Device Global Interrupt Disable Register"]
    #[inline(always)]
    pub const fn devidr(&self) -> &Devidr {
        &self.devidr
    }
    #[doc = "0x18 - Device Global Interrupt Enable Register"]
    #[inline(always)]
    pub const fn devier(&self) -> &Devier {
        &self.devier
    }
    #[doc = "0x1c - Device Endpoint Register"]
    #[inline(always)]
    pub const fn devept(&self) -> &Devept {
        &self.devept
    }
    #[doc = "0x20 - Device Frame Number Register"]
    #[inline(always)]
    pub const fn devfnum(&self) -> &Devfnum {
        &self.devfnum
    }
    #[doc = "0x100 - Device Endpoint Configuration Register (n = 0) 0"]
    #[inline(always)]
    pub const fn deveptcfg0(&self) -> &Deveptcfg0 {
        &self.deveptcfg0
    }
    #[doc = "0x104 - Device Endpoint Configuration Register (n = 0) 1"]
    #[inline(always)]
    pub const fn deveptcfg1(&self) -> &Deveptcfg1 {
        &self.deveptcfg1
    }
    #[doc = "0x108 - Device Endpoint Configuration Register (n = 0) 2"]
    #[inline(always)]
    pub const fn deveptcfg2(&self) -> &Deveptcfg2 {
        &self.deveptcfg2
    }
    #[doc = "0x10c - Device Endpoint Configuration Register (n = 0) 3"]
    #[inline(always)]
    pub const fn deveptcfg3(&self) -> &Deveptcfg3 {
        &self.deveptcfg3
    }
    #[doc = "0x110 - Device Endpoint Configuration Register (n = 0) 4"]
    #[inline(always)]
    pub const fn deveptcfg4(&self) -> &Deveptcfg4 {
        &self.deveptcfg4
    }
    #[doc = "0x114 - Device Endpoint Configuration Register (n = 0) 5"]
    #[inline(always)]
    pub const fn deveptcfg5(&self) -> &Deveptcfg5 {
        &self.deveptcfg5
    }
    #[doc = "0x118 - Device Endpoint Configuration Register (n = 0) 6"]
    #[inline(always)]
    pub const fn deveptcfg6(&self) -> &Deveptcfg6 {
        &self.deveptcfg6
    }
    #[doc = "0x11c - Device Endpoint Configuration Register (n = 0) 7"]
    #[inline(always)]
    pub const fn deveptcfg7(&self) -> &Deveptcfg7 {
        &self.deveptcfg7
    }
    #[doc = "0x120 - Device Endpoint Configuration Register (n = 0) 8"]
    #[inline(always)]
    pub const fn deveptcfg8(&self) -> &Deveptcfg8 {
        &self.deveptcfg8
    }
    #[doc = "0x124 - Device Endpoint Configuration Register (n = 0) 9"]
    #[inline(always)]
    pub const fn deveptcfg9(&self) -> &Deveptcfg9 {
        &self.deveptcfg9
    }
    #[doc = "0x130 - Device Endpoint Status Register (n = 0)"]
    #[inline(always)]
    pub const fn isoenpt_deveptisr0_isoenpt(&self) -> &IsoenptDeveptisr0Isoenpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(304).cast() }
    }
    #[doc = "0x130 - Device Endpoint Status Register (n = 0) 0"]
    #[inline(always)]
    pub const fn deveptisr0(&self) -> &Deveptisr0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(304).cast() }
    }
    #[doc = "0x134 - Device Endpoint Status Register (n = 0) 1"]
    #[inline(always)]
    pub const fn deveptisr1(&self) -> &Deveptisr1 {
        &self.deveptisr1
    }
    #[doc = "0x138 - Device Endpoint Status Register (n = 0) 2"]
    #[inline(always)]
    pub const fn deveptisr2(&self) -> &Deveptisr2 {
        &self.deveptisr2
    }
    #[doc = "0x13c - Device Endpoint Status Register (n = 0) 3"]
    #[inline(always)]
    pub const fn deveptisr3(&self) -> &Deveptisr3 {
        &self.deveptisr3
    }
    #[doc = "0x140 - Device Endpoint Status Register (n = 0) 4"]
    #[inline(always)]
    pub const fn deveptisr4(&self) -> &Deveptisr4 {
        &self.deveptisr4
    }
    #[doc = "0x144 - Device Endpoint Status Register (n = 0) 5"]
    #[inline(always)]
    pub const fn deveptisr5(&self) -> &Deveptisr5 {
        &self.deveptisr5
    }
    #[doc = "0x148 - Device Endpoint Status Register (n = 0) 6"]
    #[inline(always)]
    pub const fn deveptisr6(&self) -> &Deveptisr6 {
        &self.deveptisr6
    }
    #[doc = "0x14c - Device Endpoint Status Register (n = 0) 7"]
    #[inline(always)]
    pub const fn deveptisr7(&self) -> &Deveptisr7 {
        &self.deveptisr7
    }
    #[doc = "0x150 - Device Endpoint Status Register (n = 0) 8"]
    #[inline(always)]
    pub const fn deveptisr8(&self) -> &Deveptisr8 {
        &self.deveptisr8
    }
    #[doc = "0x154 - Device Endpoint Status Register (n = 0) 9"]
    #[inline(always)]
    pub const fn deveptisr9(&self) -> &Deveptisr9 {
        &self.deveptisr9
    }
    #[doc = "0x160 - Device Endpoint Clear Register (n = 0)"]
    #[inline(always)]
    pub const fn isoenpt_devepticr0_isoenpt(&self) -> &IsoenptDevepticr0Isoenpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(352).cast() }
    }
    #[doc = "0x160 - Device Endpoint Clear Register (n = 0) 0"]
    #[inline(always)]
    pub const fn devepticr0(&self) -> &Devepticr0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(352).cast() }
    }
    #[doc = "0x164 - Device Endpoint Clear Register (n = 0) 1"]
    #[inline(always)]
    pub const fn devepticr1(&self) -> &Devepticr1 {
        &self.devepticr1
    }
    #[doc = "0x168 - Device Endpoint Clear Register (n = 0) 2"]
    #[inline(always)]
    pub const fn devepticr2(&self) -> &Devepticr2 {
        &self.devepticr2
    }
    #[doc = "0x16c - Device Endpoint Clear Register (n = 0) 3"]
    #[inline(always)]
    pub const fn devepticr3(&self) -> &Devepticr3 {
        &self.devepticr3
    }
    #[doc = "0x170 - Device Endpoint Clear Register (n = 0) 4"]
    #[inline(always)]
    pub const fn devepticr4(&self) -> &Devepticr4 {
        &self.devepticr4
    }
    #[doc = "0x174 - Device Endpoint Clear Register (n = 0) 5"]
    #[inline(always)]
    pub const fn devepticr5(&self) -> &Devepticr5 {
        &self.devepticr5
    }
    #[doc = "0x178 - Device Endpoint Clear Register (n = 0) 6"]
    #[inline(always)]
    pub const fn devepticr6(&self) -> &Devepticr6 {
        &self.devepticr6
    }
    #[doc = "0x17c - Device Endpoint Clear Register (n = 0) 7"]
    #[inline(always)]
    pub const fn devepticr7(&self) -> &Devepticr7 {
        &self.devepticr7
    }
    #[doc = "0x180 - Device Endpoint Clear Register (n = 0) 8"]
    #[inline(always)]
    pub const fn devepticr8(&self) -> &Devepticr8 {
        &self.devepticr8
    }
    #[doc = "0x184 - Device Endpoint Clear Register (n = 0) 9"]
    #[inline(always)]
    pub const fn devepticr9(&self) -> &Devepticr9 {
        &self.devepticr9
    }
    #[doc = "0x190 - Device Endpoint Set Register (n = 0)"]
    #[inline(always)]
    pub const fn isoenpt_deveptifr0_isoenpt(&self) -> &IsoenptDeveptifr0Isoenpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(400).cast() }
    }
    #[doc = "0x190 - Device Endpoint Set Register (n = 0) 0"]
    #[inline(always)]
    pub const fn deveptifr0(&self) -> &Deveptifr0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(400).cast() }
    }
    #[doc = "0x194 - Device Endpoint Set Register (n = 0) 1"]
    #[inline(always)]
    pub const fn deveptifr1(&self) -> &Deveptifr1 {
        &self.deveptifr1
    }
    #[doc = "0x198 - Device Endpoint Set Register (n = 0) 2"]
    #[inline(always)]
    pub const fn deveptifr2(&self) -> &Deveptifr2 {
        &self.deveptifr2
    }
    #[doc = "0x19c - Device Endpoint Set Register (n = 0) 3"]
    #[inline(always)]
    pub const fn deveptifr3(&self) -> &Deveptifr3 {
        &self.deveptifr3
    }
    #[doc = "0x1a0 - Device Endpoint Set Register (n = 0) 4"]
    #[inline(always)]
    pub const fn deveptifr4(&self) -> &Deveptifr4 {
        &self.deveptifr4
    }
    #[doc = "0x1a4 - Device Endpoint Set Register (n = 0) 5"]
    #[inline(always)]
    pub const fn deveptifr5(&self) -> &Deveptifr5 {
        &self.deveptifr5
    }
    #[doc = "0x1a8 - Device Endpoint Set Register (n = 0) 6"]
    #[inline(always)]
    pub const fn deveptifr6(&self) -> &Deveptifr6 {
        &self.deveptifr6
    }
    #[doc = "0x1ac - Device Endpoint Set Register (n = 0) 7"]
    #[inline(always)]
    pub const fn deveptifr7(&self) -> &Deveptifr7 {
        &self.deveptifr7
    }
    #[doc = "0x1b0 - Device Endpoint Set Register (n = 0) 8"]
    #[inline(always)]
    pub const fn deveptifr8(&self) -> &Deveptifr8 {
        &self.deveptifr8
    }
    #[doc = "0x1b4 - Device Endpoint Set Register (n = 0) 9"]
    #[inline(always)]
    pub const fn deveptifr9(&self) -> &Deveptifr9 {
        &self.deveptifr9
    }
    #[doc = "0x1c0 - Device Endpoint Mask Register (n = 0)"]
    #[inline(always)]
    pub const fn isoenpt_deveptimr0_isoenpt(&self) -> &IsoenptDeveptimr0Isoenpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(448).cast() }
    }
    #[doc = "0x1c0 - Device Endpoint Mask Register (n = 0) 0"]
    #[inline(always)]
    pub const fn deveptimr0(&self) -> &Deveptimr0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(448).cast() }
    }
    #[doc = "0x1c4 - Device Endpoint Mask Register (n = 0) 1"]
    #[inline(always)]
    pub const fn deveptimr1(&self) -> &Deveptimr1 {
        &self.deveptimr1
    }
    #[doc = "0x1c8 - Device Endpoint Mask Register (n = 0) 2"]
    #[inline(always)]
    pub const fn deveptimr2(&self) -> &Deveptimr2 {
        &self.deveptimr2
    }
    #[doc = "0x1cc - Device Endpoint Mask Register (n = 0) 3"]
    #[inline(always)]
    pub const fn deveptimr3(&self) -> &Deveptimr3 {
        &self.deveptimr3
    }
    #[doc = "0x1d0 - Device Endpoint Mask Register (n = 0) 4"]
    #[inline(always)]
    pub const fn deveptimr4(&self) -> &Deveptimr4 {
        &self.deveptimr4
    }
    #[doc = "0x1d4 - Device Endpoint Mask Register (n = 0) 5"]
    #[inline(always)]
    pub const fn deveptimr5(&self) -> &Deveptimr5 {
        &self.deveptimr5
    }
    #[doc = "0x1d8 - Device Endpoint Mask Register (n = 0) 6"]
    #[inline(always)]
    pub const fn deveptimr6(&self) -> &Deveptimr6 {
        &self.deveptimr6
    }
    #[doc = "0x1dc - Device Endpoint Mask Register (n = 0) 7"]
    #[inline(always)]
    pub const fn deveptimr7(&self) -> &Deveptimr7 {
        &self.deveptimr7
    }
    #[doc = "0x1e0 - Device Endpoint Mask Register (n = 0) 8"]
    #[inline(always)]
    pub const fn deveptimr8(&self) -> &Deveptimr8 {
        &self.deveptimr8
    }
    #[doc = "0x1e4 - Device Endpoint Mask Register (n = 0) 9"]
    #[inline(always)]
    pub const fn deveptimr9(&self) -> &Deveptimr9 {
        &self.deveptimr9
    }
    #[doc = "0x1f0 - Device Endpoint Enable Register (n = 0)"]
    #[inline(always)]
    pub const fn isoenpt_deveptier0_isoenpt(&self) -> &IsoenptDeveptier0Isoenpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(496).cast() }
    }
    #[doc = "0x1f0 - Device Endpoint Enable Register (n = 0) 0"]
    #[inline(always)]
    pub const fn deveptier0(&self) -> &Deveptier0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(496).cast() }
    }
    #[doc = "0x1f4 - Device Endpoint Enable Register (n = 0) 1"]
    #[inline(always)]
    pub const fn deveptier1(&self) -> &Deveptier1 {
        &self.deveptier1
    }
    #[doc = "0x1f8 - Device Endpoint Enable Register (n = 0) 2"]
    #[inline(always)]
    pub const fn deveptier2(&self) -> &Deveptier2 {
        &self.deveptier2
    }
    #[doc = "0x1fc - Device Endpoint Enable Register (n = 0) 3"]
    #[inline(always)]
    pub const fn deveptier3(&self) -> &Deveptier3 {
        &self.deveptier3
    }
    #[doc = "0x200 - Device Endpoint Enable Register (n = 0) 4"]
    #[inline(always)]
    pub const fn deveptier4(&self) -> &Deveptier4 {
        &self.deveptier4
    }
    #[doc = "0x204 - Device Endpoint Enable Register (n = 0) 5"]
    #[inline(always)]
    pub const fn deveptier5(&self) -> &Deveptier5 {
        &self.deveptier5
    }
    #[doc = "0x208 - Device Endpoint Enable Register (n = 0) 6"]
    #[inline(always)]
    pub const fn deveptier6(&self) -> &Deveptier6 {
        &self.deveptier6
    }
    #[doc = "0x20c - Device Endpoint Enable Register (n = 0) 7"]
    #[inline(always)]
    pub const fn deveptier7(&self) -> &Deveptier7 {
        &self.deveptier7
    }
    #[doc = "0x210 - Device Endpoint Enable Register (n = 0) 8"]
    #[inline(always)]
    pub const fn deveptier8(&self) -> &Deveptier8 {
        &self.deveptier8
    }
    #[doc = "0x214 - Device Endpoint Enable Register (n = 0) 9"]
    #[inline(always)]
    pub const fn deveptier9(&self) -> &Deveptier9 {
        &self.deveptier9
    }
    #[doc = "0x220 - Device Endpoint Disable Register (n = 0)"]
    #[inline(always)]
    pub const fn isoenpt_deveptidr0_isoenpt(&self) -> &IsoenptDeveptidr0Isoenpt {
        unsafe { &*(self as *const Self).cast::<u8>().add(544).cast() }
    }
    #[doc = "0x220 - Device Endpoint Disable Register (n = 0) 0"]
    #[inline(always)]
    pub const fn deveptidr0(&self) -> &Deveptidr0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(544).cast() }
    }
    #[doc = "0x224 - Device Endpoint Disable Register (n = 0) 1"]
    #[inline(always)]
    pub const fn deveptidr1(&self) -> &Deveptidr1 {
        &self.deveptidr1
    }
    #[doc = "0x228 - Device Endpoint Disable Register (n = 0) 2"]
    #[inline(always)]
    pub const fn deveptidr2(&self) -> &Deveptidr2 {
        &self.deveptidr2
    }
    #[doc = "0x22c - Device Endpoint Disable Register (n = 0) 3"]
    #[inline(always)]
    pub const fn deveptidr3(&self) -> &Deveptidr3 {
        &self.deveptidr3
    }
    #[doc = "0x230 - Device Endpoint Disable Register (n = 0) 4"]
    #[inline(always)]
    pub const fn deveptidr4(&self) -> &Deveptidr4 {
        &self.deveptidr4
    }
    #[doc = "0x234 - Device Endpoint Disable Register (n = 0) 5"]
    #[inline(always)]
    pub const fn deveptidr5(&self) -> &Deveptidr5 {
        &self.deveptidr5
    }
    #[doc = "0x238 - Device Endpoint Disable Register (n = 0) 6"]
    #[inline(always)]
    pub const fn deveptidr6(&self) -> &Deveptidr6 {
        &self.deveptidr6
    }
    #[doc = "0x23c - Device Endpoint Disable Register (n = 0) 7"]
    #[inline(always)]
    pub const fn deveptidr7(&self) -> &Deveptidr7 {
        &self.deveptidr7
    }
    #[doc = "0x240 - Device Endpoint Disable Register (n = 0) 8"]
    #[inline(always)]
    pub const fn deveptidr8(&self) -> &Deveptidr8 {
        &self.deveptidr8
    }
    #[doc = "0x244 - Device Endpoint Disable Register (n = 0) 9"]
    #[inline(always)]
    pub const fn deveptidr9(&self) -> &Deveptidr9 {
        &self.deveptidr9
    }
    #[doc = "0x310 - Device DMA Channel Next Descriptor Address Register (n = 1)"]
    #[inline(always)]
    pub const fn devdmanxtdsc1(&self) -> &Devdmanxtdsc1 {
        &self.devdmanxtdsc1
    }
    #[doc = "0x314 - Device DMA Channel Address Register (n = 1)"]
    #[inline(always)]
    pub const fn devdmaaddress1(&self) -> &Devdmaaddress1 {
        &self.devdmaaddress1
    }
    #[doc = "0x318 - Device DMA Channel Control Register (n = 1)"]
    #[inline(always)]
    pub const fn devdmacontrol1(&self) -> &Devdmacontrol1 {
        &self.devdmacontrol1
    }
    #[doc = "0x31c - Device DMA Channel Status Register (n = 1)"]
    #[inline(always)]
    pub const fn devdmastatus1(&self) -> &Devdmastatus1 {
        &self.devdmastatus1
    }
    #[doc = "0x320 - Device DMA Channel Next Descriptor Address Register (n = 2)"]
    #[inline(always)]
    pub const fn devdmanxtdsc2(&self) -> &Devdmanxtdsc2 {
        &self.devdmanxtdsc2
    }
    #[doc = "0x324 - Device DMA Channel Address Register (n = 2)"]
    #[inline(always)]
    pub const fn devdmaaddress2(&self) -> &Devdmaaddress2 {
        &self.devdmaaddress2
    }
    #[doc = "0x328 - Device DMA Channel Control Register (n = 2)"]
    #[inline(always)]
    pub const fn devdmacontrol2(&self) -> &Devdmacontrol2 {
        &self.devdmacontrol2
    }
    #[doc = "0x32c - Device DMA Channel Status Register (n = 2)"]
    #[inline(always)]
    pub const fn devdmastatus2(&self) -> &Devdmastatus2 {
        &self.devdmastatus2
    }
    #[doc = "0x330 - Device DMA Channel Next Descriptor Address Register (n = 3)"]
    #[inline(always)]
    pub const fn devdmanxtdsc3(&self) -> &Devdmanxtdsc3 {
        &self.devdmanxtdsc3
    }
    #[doc = "0x334 - Device DMA Channel Address Register (n = 3)"]
    #[inline(always)]
    pub const fn devdmaaddress3(&self) -> &Devdmaaddress3 {
        &self.devdmaaddress3
    }
    #[doc = "0x338 - Device DMA Channel Control Register (n = 3)"]
    #[inline(always)]
    pub const fn devdmacontrol3(&self) -> &Devdmacontrol3 {
        &self.devdmacontrol3
    }
    #[doc = "0x33c - Device DMA Channel Status Register (n = 3)"]
    #[inline(always)]
    pub const fn devdmastatus3(&self) -> &Devdmastatus3 {
        &self.devdmastatus3
    }
    #[doc = "0x340 - Device DMA Channel Next Descriptor Address Register (n = 4)"]
    #[inline(always)]
    pub const fn devdmanxtdsc4(&self) -> &Devdmanxtdsc4 {
        &self.devdmanxtdsc4
    }
    #[doc = "0x344 - Device DMA Channel Address Register (n = 4)"]
    #[inline(always)]
    pub const fn devdmaaddress4(&self) -> &Devdmaaddress4 {
        &self.devdmaaddress4
    }
    #[doc = "0x348 - Device DMA Channel Control Register (n = 4)"]
    #[inline(always)]
    pub const fn devdmacontrol4(&self) -> &Devdmacontrol4 {
        &self.devdmacontrol4
    }
    #[doc = "0x34c - Device DMA Channel Status Register (n = 4)"]
    #[inline(always)]
    pub const fn devdmastatus4(&self) -> &Devdmastatus4 {
        &self.devdmastatus4
    }
    #[doc = "0x350 - Device DMA Channel Next Descriptor Address Register (n = 5)"]
    #[inline(always)]
    pub const fn devdmanxtdsc5(&self) -> &Devdmanxtdsc5 {
        &self.devdmanxtdsc5
    }
    #[doc = "0x354 - Device DMA Channel Address Register (n = 5)"]
    #[inline(always)]
    pub const fn devdmaaddress5(&self) -> &Devdmaaddress5 {
        &self.devdmaaddress5
    }
    #[doc = "0x358 - Device DMA Channel Control Register (n = 5)"]
    #[inline(always)]
    pub const fn devdmacontrol5(&self) -> &Devdmacontrol5 {
        &self.devdmacontrol5
    }
    #[doc = "0x35c - Device DMA Channel Status Register (n = 5)"]
    #[inline(always)]
    pub const fn devdmastatus5(&self) -> &Devdmastatus5 {
        &self.devdmastatus5
    }
    #[doc = "0x360 - Device DMA Channel Next Descriptor Address Register (n = 6)"]
    #[inline(always)]
    pub const fn devdmanxtdsc6(&self) -> &Devdmanxtdsc6 {
        &self.devdmanxtdsc6
    }
    #[doc = "0x364 - Device DMA Channel Address Register (n = 6)"]
    #[inline(always)]
    pub const fn devdmaaddress6(&self) -> &Devdmaaddress6 {
        &self.devdmaaddress6
    }
    #[doc = "0x368 - Device DMA Channel Control Register (n = 6)"]
    #[inline(always)]
    pub const fn devdmacontrol6(&self) -> &Devdmacontrol6 {
        &self.devdmacontrol6
    }
    #[doc = "0x36c - Device DMA Channel Status Register (n = 6)"]
    #[inline(always)]
    pub const fn devdmastatus6(&self) -> &Devdmastatus6 {
        &self.devdmastatus6
    }
    #[doc = "0x370 - Device DMA Channel Next Descriptor Address Register (n = 7)"]
    #[inline(always)]
    pub const fn devdmanxtdsc7(&self) -> &Devdmanxtdsc7 {
        &self.devdmanxtdsc7
    }
    #[doc = "0x374 - Device DMA Channel Address Register (n = 7)"]
    #[inline(always)]
    pub const fn devdmaaddress7(&self) -> &Devdmaaddress7 {
        &self.devdmaaddress7
    }
    #[doc = "0x378 - Device DMA Channel Control Register (n = 7)"]
    #[inline(always)]
    pub const fn devdmacontrol7(&self) -> &Devdmacontrol7 {
        &self.devdmacontrol7
    }
    #[doc = "0x37c - Device DMA Channel Status Register (n = 7)"]
    #[inline(always)]
    pub const fn devdmastatus7(&self) -> &Devdmastatus7 {
        &self.devdmastatus7
    }
    #[doc = "0x400 - Host General Control Register"]
    #[inline(always)]
    pub const fn hstctrl(&self) -> &Hstctrl {
        &self.hstctrl
    }
    #[doc = "0x404 - Host Global Interrupt Status Register"]
    #[inline(always)]
    pub const fn hstisr(&self) -> &Hstisr {
        &self.hstisr
    }
    #[doc = "0x408 - Host Global Interrupt Clear Register"]
    #[inline(always)]
    pub const fn hsticr(&self) -> &Hsticr {
        &self.hsticr
    }
    #[doc = "0x40c - Host Global Interrupt Set Register"]
    #[inline(always)]
    pub const fn hstifr(&self) -> &Hstifr {
        &self.hstifr
    }
    #[doc = "0x410 - Host Global Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hstimr(&self) -> &Hstimr {
        &self.hstimr
    }
    #[doc = "0x414 - Host Global Interrupt Disable Register"]
    #[inline(always)]
    pub const fn hstidr(&self) -> &Hstidr {
        &self.hstidr
    }
    #[doc = "0x418 - Host Global Interrupt Enable Register"]
    #[inline(always)]
    pub const fn hstier(&self) -> &Hstier {
        &self.hstier
    }
    #[doc = "0x41c - Host Pipe Register"]
    #[inline(always)]
    pub const fn hstpip(&self) -> &Hstpip {
        &self.hstpip
    }
    #[doc = "0x420 - Host Frame Number Register"]
    #[inline(always)]
    pub const fn hstfnum(&self) -> &Hstfnum {
        &self.hstfnum
    }
    #[doc = "0x424 - Host Address 1 Register"]
    #[inline(always)]
    pub const fn hstaddr1(&self) -> &Hstaddr1 {
        &self.hstaddr1
    }
    #[doc = "0x428 - Host Address 2 Register"]
    #[inline(always)]
    pub const fn hstaddr2(&self) -> &Hstaddr2 {
        &self.hstaddr2
    }
    #[doc = "0x42c - Host Address 3 Register"]
    #[inline(always)]
    pub const fn hstaddr3(&self) -> &Hstaddr3 {
        &self.hstaddr3
    }
    #[doc = "0x500 - Host Pipe Configuration Register (n = 0)"]
    #[inline(always)]
    pub const fn hsbohscp_hstpipcfg0_hsbohscp(&self) -> &HsbohscpHstpipcfg0Hsbohscp {
        unsafe { &*(self as *const Self).cast::<u8>().add(1280).cast() }
    }
    #[doc = "0x500 - Host Pipe Configuration Register (n = 0) 0"]
    #[inline(always)]
    pub const fn hstpipcfg0(&self) -> &Hstpipcfg0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1280).cast() }
    }
    #[doc = "0x504 - Host Pipe Configuration Register (n = 0) 1"]
    #[inline(always)]
    pub const fn hstpipcfg1(&self) -> &Hstpipcfg1 {
        &self.hstpipcfg1
    }
    #[doc = "0x508 - Host Pipe Configuration Register (n = 0) 2"]
    #[inline(always)]
    pub const fn hstpipcfg2(&self) -> &Hstpipcfg2 {
        &self.hstpipcfg2
    }
    #[doc = "0x50c - Host Pipe Configuration Register (n = 0) 3"]
    #[inline(always)]
    pub const fn hstpipcfg3(&self) -> &Hstpipcfg3 {
        &self.hstpipcfg3
    }
    #[doc = "0x510 - Host Pipe Configuration Register (n = 0) 4"]
    #[inline(always)]
    pub const fn hstpipcfg4(&self) -> &Hstpipcfg4 {
        &self.hstpipcfg4
    }
    #[doc = "0x514 - Host Pipe Configuration Register (n = 0) 5"]
    #[inline(always)]
    pub const fn hstpipcfg5(&self) -> &Hstpipcfg5 {
        &self.hstpipcfg5
    }
    #[doc = "0x518 - Host Pipe Configuration Register (n = 0) 6"]
    #[inline(always)]
    pub const fn hstpipcfg6(&self) -> &Hstpipcfg6 {
        &self.hstpipcfg6
    }
    #[doc = "0x51c - Host Pipe Configuration Register (n = 0) 7"]
    #[inline(always)]
    pub const fn hstpipcfg7(&self) -> &Hstpipcfg7 {
        &self.hstpipcfg7
    }
    #[doc = "0x520 - Host Pipe Configuration Register (n = 0) 8"]
    #[inline(always)]
    pub const fn hstpipcfg8(&self) -> &Hstpipcfg8 {
        &self.hstpipcfg8
    }
    #[doc = "0x524 - Host Pipe Configuration Register (n = 0) 9"]
    #[inline(always)]
    pub const fn hstpipcfg9(&self) -> &Hstpipcfg9 {
        &self.hstpipcfg9
    }
    #[doc = "0x530 - Host Pipe Status Register (n = 0)"]
    #[inline(always)]
    pub const fn isopipes_hstpipisr0_isopipes(&self) -> &IsopipesHstpipisr0Isopipes {
        unsafe { &*(self as *const Self).cast::<u8>().add(1328).cast() }
    }
    #[doc = "0x530 - Host Pipe Status Register (n = 0)"]
    #[inline(always)]
    pub const fn intpipes_hstpipisr0_intpipes(&self) -> &IntpipesHstpipisr0Intpipes {
        unsafe { &*(self as *const Self).cast::<u8>().add(1328).cast() }
    }
    #[doc = "0x530 - Host Pipe Status Register (n = 0) 0"]
    #[inline(always)]
    pub const fn hstpipisr0(&self) -> &Hstpipisr0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1328).cast() }
    }
    #[doc = "0x534 - Host Pipe Status Register (n = 0) 1"]
    #[inline(always)]
    pub const fn hstpipisr1(&self) -> &Hstpipisr1 {
        &self.hstpipisr1
    }
    #[doc = "0x538 - Host Pipe Status Register (n = 0) 2"]
    #[inline(always)]
    pub const fn hstpipisr2(&self) -> &Hstpipisr2 {
        &self.hstpipisr2
    }
    #[doc = "0x53c - Host Pipe Status Register (n = 0) 3"]
    #[inline(always)]
    pub const fn hstpipisr3(&self) -> &Hstpipisr3 {
        &self.hstpipisr3
    }
    #[doc = "0x540 - Host Pipe Status Register (n = 0) 4"]
    #[inline(always)]
    pub const fn hstpipisr4(&self) -> &Hstpipisr4 {
        &self.hstpipisr4
    }
    #[doc = "0x544 - Host Pipe Status Register (n = 0) 5"]
    #[inline(always)]
    pub const fn hstpipisr5(&self) -> &Hstpipisr5 {
        &self.hstpipisr5
    }
    #[doc = "0x548 - Host Pipe Status Register (n = 0) 6"]
    #[inline(always)]
    pub const fn hstpipisr6(&self) -> &Hstpipisr6 {
        &self.hstpipisr6
    }
    #[doc = "0x54c - Host Pipe Status Register (n = 0) 7"]
    #[inline(always)]
    pub const fn hstpipisr7(&self) -> &Hstpipisr7 {
        &self.hstpipisr7
    }
    #[doc = "0x550 - Host Pipe Status Register (n = 0) 8"]
    #[inline(always)]
    pub const fn hstpipisr8(&self) -> &Hstpipisr8 {
        &self.hstpipisr8
    }
    #[doc = "0x554 - Host Pipe Status Register (n = 0) 9"]
    #[inline(always)]
    pub const fn hstpipisr9(&self) -> &Hstpipisr9 {
        &self.hstpipisr9
    }
    #[doc = "0x560 - Host Pipe Clear Register (n = 0)"]
    #[inline(always)]
    pub const fn isopipes_hstpipicr0_isopipes(&self) -> &IsopipesHstpipicr0Isopipes {
        unsafe { &*(self as *const Self).cast::<u8>().add(1376).cast() }
    }
    #[doc = "0x560 - Host Pipe Clear Register (n = 0)"]
    #[inline(always)]
    pub const fn intpipes_hstpipicr0_intpipes(&self) -> &IntpipesHstpipicr0Intpipes {
        unsafe { &*(self as *const Self).cast::<u8>().add(1376).cast() }
    }
    #[doc = "0x560 - Host Pipe Clear Register (n = 0) 0"]
    #[inline(always)]
    pub const fn hstpipicr0(&self) -> &Hstpipicr0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1376).cast() }
    }
    #[doc = "0x564 - Host Pipe Clear Register (n = 0) 1"]
    #[inline(always)]
    pub const fn hstpipicr1(&self) -> &Hstpipicr1 {
        &self.hstpipicr1
    }
    #[doc = "0x568 - Host Pipe Clear Register (n = 0) 2"]
    #[inline(always)]
    pub const fn hstpipicr2(&self) -> &Hstpipicr2 {
        &self.hstpipicr2
    }
    #[doc = "0x56c - Host Pipe Clear Register (n = 0) 3"]
    #[inline(always)]
    pub const fn hstpipicr3(&self) -> &Hstpipicr3 {
        &self.hstpipicr3
    }
    #[doc = "0x570 - Host Pipe Clear Register (n = 0) 4"]
    #[inline(always)]
    pub const fn hstpipicr4(&self) -> &Hstpipicr4 {
        &self.hstpipicr4
    }
    #[doc = "0x574 - Host Pipe Clear Register (n = 0) 5"]
    #[inline(always)]
    pub const fn hstpipicr5(&self) -> &Hstpipicr5 {
        &self.hstpipicr5
    }
    #[doc = "0x578 - Host Pipe Clear Register (n = 0) 6"]
    #[inline(always)]
    pub const fn hstpipicr6(&self) -> &Hstpipicr6 {
        &self.hstpipicr6
    }
    #[doc = "0x57c - Host Pipe Clear Register (n = 0) 7"]
    #[inline(always)]
    pub const fn hstpipicr7(&self) -> &Hstpipicr7 {
        &self.hstpipicr7
    }
    #[doc = "0x580 - Host Pipe Clear Register (n = 0) 8"]
    #[inline(always)]
    pub const fn hstpipicr8(&self) -> &Hstpipicr8 {
        &self.hstpipicr8
    }
    #[doc = "0x584 - Host Pipe Clear Register (n = 0) 9"]
    #[inline(always)]
    pub const fn hstpipicr9(&self) -> &Hstpipicr9 {
        &self.hstpipicr9
    }
    #[doc = "0x590 - Host Pipe Set Register (n = 0)"]
    #[inline(always)]
    pub const fn isopipes_hstpipifr0_isopipes(&self) -> &IsopipesHstpipifr0Isopipes {
        unsafe { &*(self as *const Self).cast::<u8>().add(1424).cast() }
    }
    #[doc = "0x590 - Host Pipe Set Register (n = 0)"]
    #[inline(always)]
    pub const fn intpipes_hstpipifr0_intpipes(&self) -> &IntpipesHstpipifr0Intpipes {
        unsafe { &*(self as *const Self).cast::<u8>().add(1424).cast() }
    }
    #[doc = "0x590 - Host Pipe Set Register (n = 0) 0"]
    #[inline(always)]
    pub const fn hstpipifr0(&self) -> &Hstpipifr0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1424).cast() }
    }
    #[doc = "0x594 - Host Pipe Set Register (n = 0) 1"]
    #[inline(always)]
    pub const fn hstpipifr1(&self) -> &Hstpipifr1 {
        &self.hstpipifr1
    }
    #[doc = "0x598 - Host Pipe Set Register (n = 0) 2"]
    #[inline(always)]
    pub const fn hstpipifr2(&self) -> &Hstpipifr2 {
        &self.hstpipifr2
    }
    #[doc = "0x59c - Host Pipe Set Register (n = 0) 3"]
    #[inline(always)]
    pub const fn hstpipifr3(&self) -> &Hstpipifr3 {
        &self.hstpipifr3
    }
    #[doc = "0x5a0 - Host Pipe Set Register (n = 0) 4"]
    #[inline(always)]
    pub const fn hstpipifr4(&self) -> &Hstpipifr4 {
        &self.hstpipifr4
    }
    #[doc = "0x5a4 - Host Pipe Set Register (n = 0) 5"]
    #[inline(always)]
    pub const fn hstpipifr5(&self) -> &Hstpipifr5 {
        &self.hstpipifr5
    }
    #[doc = "0x5a8 - Host Pipe Set Register (n = 0) 6"]
    #[inline(always)]
    pub const fn hstpipifr6(&self) -> &Hstpipifr6 {
        &self.hstpipifr6
    }
    #[doc = "0x5ac - Host Pipe Set Register (n = 0) 7"]
    #[inline(always)]
    pub const fn hstpipifr7(&self) -> &Hstpipifr7 {
        &self.hstpipifr7
    }
    #[doc = "0x5b0 - Host Pipe Set Register (n = 0) 8"]
    #[inline(always)]
    pub const fn hstpipifr8(&self) -> &Hstpipifr8 {
        &self.hstpipifr8
    }
    #[doc = "0x5b4 - Host Pipe Set Register (n = 0) 9"]
    #[inline(always)]
    pub const fn hstpipifr9(&self) -> &Hstpipifr9 {
        &self.hstpipifr9
    }
    #[doc = "0x5c0 - Host Pipe Mask Register (n = 0)"]
    #[inline(always)]
    pub const fn isopipes_hstpipimr0_isopipes(&self) -> &IsopipesHstpipimr0Isopipes {
        unsafe { &*(self as *const Self).cast::<u8>().add(1472).cast() }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register (n = 0)"]
    #[inline(always)]
    pub const fn intpipes_hstpipimr0_intpipes(&self) -> &IntpipesHstpipimr0Intpipes {
        unsafe { &*(self as *const Self).cast::<u8>().add(1472).cast() }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register (n = 0) 0"]
    #[inline(always)]
    pub const fn hstpipimr0(&self) -> &Hstpipimr0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1472).cast() }
    }
    #[doc = "0x5c4 - Host Pipe Mask Register (n = 0) 1"]
    #[inline(always)]
    pub const fn hstpipimr1(&self) -> &Hstpipimr1 {
        &self.hstpipimr1
    }
    #[doc = "0x5c8 - Host Pipe Mask Register (n = 0) 2"]
    #[inline(always)]
    pub const fn hstpipimr2(&self) -> &Hstpipimr2 {
        &self.hstpipimr2
    }
    #[doc = "0x5cc - Host Pipe Mask Register (n = 0) 3"]
    #[inline(always)]
    pub const fn hstpipimr3(&self) -> &Hstpipimr3 {
        &self.hstpipimr3
    }
    #[doc = "0x5d0 - Host Pipe Mask Register (n = 0) 4"]
    #[inline(always)]
    pub const fn hstpipimr4(&self) -> &Hstpipimr4 {
        &self.hstpipimr4
    }
    #[doc = "0x5d4 - Host Pipe Mask Register (n = 0) 5"]
    #[inline(always)]
    pub const fn hstpipimr5(&self) -> &Hstpipimr5 {
        &self.hstpipimr5
    }
    #[doc = "0x5d8 - Host Pipe Mask Register (n = 0) 6"]
    #[inline(always)]
    pub const fn hstpipimr6(&self) -> &Hstpipimr6 {
        &self.hstpipimr6
    }
    #[doc = "0x5dc - Host Pipe Mask Register (n = 0) 7"]
    #[inline(always)]
    pub const fn hstpipimr7(&self) -> &Hstpipimr7 {
        &self.hstpipimr7
    }
    #[doc = "0x5e0 - Host Pipe Mask Register (n = 0) 8"]
    #[inline(always)]
    pub const fn hstpipimr8(&self) -> &Hstpipimr8 {
        &self.hstpipimr8
    }
    #[doc = "0x5e4 - Host Pipe Mask Register (n = 0) 9"]
    #[inline(always)]
    pub const fn hstpipimr9(&self) -> &Hstpipimr9 {
        &self.hstpipimr9
    }
    #[doc = "0x5f0 - Host Pipe Enable Register (n = 0)"]
    #[inline(always)]
    pub const fn isopipes_hstpipier0_isopipes(&self) -> &IsopipesHstpipier0Isopipes {
        unsafe { &*(self as *const Self).cast::<u8>().add(1520).cast() }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register (n = 0)"]
    #[inline(always)]
    pub const fn intpipes_hstpipier0_intpipes(&self) -> &IntpipesHstpipier0Intpipes {
        unsafe { &*(self as *const Self).cast::<u8>().add(1520).cast() }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register (n = 0) 0"]
    #[inline(always)]
    pub const fn hstpipier0(&self) -> &Hstpipier0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1520).cast() }
    }
    #[doc = "0x5f4 - Host Pipe Enable Register (n = 0) 1"]
    #[inline(always)]
    pub const fn hstpipier1(&self) -> &Hstpipier1 {
        &self.hstpipier1
    }
    #[doc = "0x5f8 - Host Pipe Enable Register (n = 0) 2"]
    #[inline(always)]
    pub const fn hstpipier2(&self) -> &Hstpipier2 {
        &self.hstpipier2
    }
    #[doc = "0x5fc - Host Pipe Enable Register (n = 0) 3"]
    #[inline(always)]
    pub const fn hstpipier3(&self) -> &Hstpipier3 {
        &self.hstpipier3
    }
    #[doc = "0x600 - Host Pipe Enable Register (n = 0) 4"]
    #[inline(always)]
    pub const fn hstpipier4(&self) -> &Hstpipier4 {
        &self.hstpipier4
    }
    #[doc = "0x604 - Host Pipe Enable Register (n = 0) 5"]
    #[inline(always)]
    pub const fn hstpipier5(&self) -> &Hstpipier5 {
        &self.hstpipier5
    }
    #[doc = "0x608 - Host Pipe Enable Register (n = 0) 6"]
    #[inline(always)]
    pub const fn hstpipier6(&self) -> &Hstpipier6 {
        &self.hstpipier6
    }
    #[doc = "0x60c - Host Pipe Enable Register (n = 0) 7"]
    #[inline(always)]
    pub const fn hstpipier7(&self) -> &Hstpipier7 {
        &self.hstpipier7
    }
    #[doc = "0x610 - Host Pipe Enable Register (n = 0) 8"]
    #[inline(always)]
    pub const fn hstpipier8(&self) -> &Hstpipier8 {
        &self.hstpipier8
    }
    #[doc = "0x614 - Host Pipe Enable Register (n = 0) 9"]
    #[inline(always)]
    pub const fn hstpipier9(&self) -> &Hstpipier9 {
        &self.hstpipier9
    }
    #[doc = "0x620 - Host Pipe Disable Register (n = 0)"]
    #[inline(always)]
    pub const fn isopipes_hstpipidr0_isopipes(&self) -> &IsopipesHstpipidr0Isopipes {
        unsafe { &*(self as *const Self).cast::<u8>().add(1568).cast() }
    }
    #[doc = "0x620 - Host Pipe Disable Register (n = 0)"]
    #[inline(always)]
    pub const fn intpipes_hstpipidr0_intpipes(&self) -> &IntpipesHstpipidr0Intpipes {
        unsafe { &*(self as *const Self).cast::<u8>().add(1568).cast() }
    }
    #[doc = "0x620 - Host Pipe Disable Register (n = 0) 0"]
    #[inline(always)]
    pub const fn hstpipidr0(&self) -> &Hstpipidr0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1568).cast() }
    }
    #[doc = "0x624 - Host Pipe Disable Register (n = 0) 1"]
    #[inline(always)]
    pub const fn hstpipidr1(&self) -> &Hstpipidr1 {
        &self.hstpipidr1
    }
    #[doc = "0x628 - Host Pipe Disable Register (n = 0) 2"]
    #[inline(always)]
    pub const fn hstpipidr2(&self) -> &Hstpipidr2 {
        &self.hstpipidr2
    }
    #[doc = "0x62c - Host Pipe Disable Register (n = 0) 3"]
    #[inline(always)]
    pub const fn hstpipidr3(&self) -> &Hstpipidr3 {
        &self.hstpipidr3
    }
    #[doc = "0x630 - Host Pipe Disable Register (n = 0) 4"]
    #[inline(always)]
    pub const fn hstpipidr4(&self) -> &Hstpipidr4 {
        &self.hstpipidr4
    }
    #[doc = "0x634 - Host Pipe Disable Register (n = 0) 5"]
    #[inline(always)]
    pub const fn hstpipidr5(&self) -> &Hstpipidr5 {
        &self.hstpipidr5
    }
    #[doc = "0x638 - Host Pipe Disable Register (n = 0) 6"]
    #[inline(always)]
    pub const fn hstpipidr6(&self) -> &Hstpipidr6 {
        &self.hstpipidr6
    }
    #[doc = "0x63c - Host Pipe Disable Register (n = 0) 7"]
    #[inline(always)]
    pub const fn hstpipidr7(&self) -> &Hstpipidr7 {
        &self.hstpipidr7
    }
    #[doc = "0x640 - Host Pipe Disable Register (n = 0) 8"]
    #[inline(always)]
    pub const fn hstpipidr8(&self) -> &Hstpipidr8 {
        &self.hstpipidr8
    }
    #[doc = "0x644 - Host Pipe Disable Register (n = 0) 9"]
    #[inline(always)]
    pub const fn hstpipidr9(&self) -> &Hstpipidr9 {
        &self.hstpipidr9
    }
    #[doc = "0x650 - Host Pipe IN Request Register (n = 0) 0"]
    #[inline(always)]
    pub const fn hstpipinrq0(&self) -> &Hstpipinrq0 {
        &self.hstpipinrq0
    }
    #[doc = "0x654 - Host Pipe IN Request Register (n = 0) 1"]
    #[inline(always)]
    pub const fn hstpipinrq1(&self) -> &Hstpipinrq1 {
        &self.hstpipinrq1
    }
    #[doc = "0x658 - Host Pipe IN Request Register (n = 0) 2"]
    #[inline(always)]
    pub const fn hstpipinrq2(&self) -> &Hstpipinrq2 {
        &self.hstpipinrq2
    }
    #[doc = "0x65c - Host Pipe IN Request Register (n = 0) 3"]
    #[inline(always)]
    pub const fn hstpipinrq3(&self) -> &Hstpipinrq3 {
        &self.hstpipinrq3
    }
    #[doc = "0x660 - Host Pipe IN Request Register (n = 0) 4"]
    #[inline(always)]
    pub const fn hstpipinrq4(&self) -> &Hstpipinrq4 {
        &self.hstpipinrq4
    }
    #[doc = "0x664 - Host Pipe IN Request Register (n = 0) 5"]
    #[inline(always)]
    pub const fn hstpipinrq5(&self) -> &Hstpipinrq5 {
        &self.hstpipinrq5
    }
    #[doc = "0x668 - Host Pipe IN Request Register (n = 0) 6"]
    #[inline(always)]
    pub const fn hstpipinrq6(&self) -> &Hstpipinrq6 {
        &self.hstpipinrq6
    }
    #[doc = "0x66c - Host Pipe IN Request Register (n = 0) 7"]
    #[inline(always)]
    pub const fn hstpipinrq7(&self) -> &Hstpipinrq7 {
        &self.hstpipinrq7
    }
    #[doc = "0x670 - Host Pipe IN Request Register (n = 0) 8"]
    #[inline(always)]
    pub const fn hstpipinrq8(&self) -> &Hstpipinrq8 {
        &self.hstpipinrq8
    }
    #[doc = "0x674 - Host Pipe IN Request Register (n = 0) 9"]
    #[inline(always)]
    pub const fn hstpipinrq9(&self) -> &Hstpipinrq9 {
        &self.hstpipinrq9
    }
    #[doc = "0x680 - Host Pipe Error Register (n = 0) 0"]
    #[inline(always)]
    pub const fn hstpiperr0(&self) -> &Hstpiperr0 {
        &self.hstpiperr0
    }
    #[doc = "0x684 - Host Pipe Error Register (n = 0) 1"]
    #[inline(always)]
    pub const fn hstpiperr1(&self) -> &Hstpiperr1 {
        &self.hstpiperr1
    }
    #[doc = "0x688 - Host Pipe Error Register (n = 0) 2"]
    #[inline(always)]
    pub const fn hstpiperr2(&self) -> &Hstpiperr2 {
        &self.hstpiperr2
    }
    #[doc = "0x68c - Host Pipe Error Register (n = 0) 3"]
    #[inline(always)]
    pub const fn hstpiperr3(&self) -> &Hstpiperr3 {
        &self.hstpiperr3
    }
    #[doc = "0x690 - Host Pipe Error Register (n = 0) 4"]
    #[inline(always)]
    pub const fn hstpiperr4(&self) -> &Hstpiperr4 {
        &self.hstpiperr4
    }
    #[doc = "0x694 - Host Pipe Error Register (n = 0) 5"]
    #[inline(always)]
    pub const fn hstpiperr5(&self) -> &Hstpiperr5 {
        &self.hstpiperr5
    }
    #[doc = "0x698 - Host Pipe Error Register (n = 0) 6"]
    #[inline(always)]
    pub const fn hstpiperr6(&self) -> &Hstpiperr6 {
        &self.hstpiperr6
    }
    #[doc = "0x69c - Host Pipe Error Register (n = 0) 7"]
    #[inline(always)]
    pub const fn hstpiperr7(&self) -> &Hstpiperr7 {
        &self.hstpiperr7
    }
    #[doc = "0x6a0 - Host Pipe Error Register (n = 0) 8"]
    #[inline(always)]
    pub const fn hstpiperr8(&self) -> &Hstpiperr8 {
        &self.hstpiperr8
    }
    #[doc = "0x6a4 - Host Pipe Error Register (n = 0) 9"]
    #[inline(always)]
    pub const fn hstpiperr9(&self) -> &Hstpiperr9 {
        &self.hstpiperr9
    }
    #[doc = "0x710 - Host DMA Channel Next Descriptor Address Register (n = 1)"]
    #[inline(always)]
    pub const fn hstdmanxtdsc1(&self) -> &Hstdmanxtdsc1 {
        &self.hstdmanxtdsc1
    }
    #[doc = "0x714 - Host DMA Channel Address Register (n = 1)"]
    #[inline(always)]
    pub const fn hstdmaaddress1(&self) -> &Hstdmaaddress1 {
        &self.hstdmaaddress1
    }
    #[doc = "0x718 - Host DMA Channel Control Register (n = 1)"]
    #[inline(always)]
    pub const fn hstdmacontrol1(&self) -> &Hstdmacontrol1 {
        &self.hstdmacontrol1
    }
    #[doc = "0x71c - Host DMA Channel Status Register (n = 1)"]
    #[inline(always)]
    pub const fn hstdmastatus1(&self) -> &Hstdmastatus1 {
        &self.hstdmastatus1
    }
    #[doc = "0x720 - Host DMA Channel Next Descriptor Address Register (n = 2)"]
    #[inline(always)]
    pub const fn hstdmanxtdsc2(&self) -> &Hstdmanxtdsc2 {
        &self.hstdmanxtdsc2
    }
    #[doc = "0x724 - Host DMA Channel Address Register (n = 2)"]
    #[inline(always)]
    pub const fn hstdmaaddress2(&self) -> &Hstdmaaddress2 {
        &self.hstdmaaddress2
    }
    #[doc = "0x728 - Host DMA Channel Control Register (n = 2)"]
    #[inline(always)]
    pub const fn hstdmacontrol2(&self) -> &Hstdmacontrol2 {
        &self.hstdmacontrol2
    }
    #[doc = "0x72c - Host DMA Channel Status Register (n = 2)"]
    #[inline(always)]
    pub const fn hstdmastatus2(&self) -> &Hstdmastatus2 {
        &self.hstdmastatus2
    }
    #[doc = "0x730 - Host DMA Channel Next Descriptor Address Register (n = 3)"]
    #[inline(always)]
    pub const fn hstdmanxtdsc3(&self) -> &Hstdmanxtdsc3 {
        &self.hstdmanxtdsc3
    }
    #[doc = "0x734 - Host DMA Channel Address Register (n = 3)"]
    #[inline(always)]
    pub const fn hstdmaaddress3(&self) -> &Hstdmaaddress3 {
        &self.hstdmaaddress3
    }
    #[doc = "0x738 - Host DMA Channel Control Register (n = 3)"]
    #[inline(always)]
    pub const fn hstdmacontrol3(&self) -> &Hstdmacontrol3 {
        &self.hstdmacontrol3
    }
    #[doc = "0x73c - Host DMA Channel Status Register (n = 3)"]
    #[inline(always)]
    pub const fn hstdmastatus3(&self) -> &Hstdmastatus3 {
        &self.hstdmastatus3
    }
    #[doc = "0x740 - Host DMA Channel Next Descriptor Address Register (n = 4)"]
    #[inline(always)]
    pub const fn hstdmanxtdsc4(&self) -> &Hstdmanxtdsc4 {
        &self.hstdmanxtdsc4
    }
    #[doc = "0x744 - Host DMA Channel Address Register (n = 4)"]
    #[inline(always)]
    pub const fn hstdmaaddress4(&self) -> &Hstdmaaddress4 {
        &self.hstdmaaddress4
    }
    #[doc = "0x748 - Host DMA Channel Control Register (n = 4)"]
    #[inline(always)]
    pub const fn hstdmacontrol4(&self) -> &Hstdmacontrol4 {
        &self.hstdmacontrol4
    }
    #[doc = "0x74c - Host DMA Channel Status Register (n = 4)"]
    #[inline(always)]
    pub const fn hstdmastatus4(&self) -> &Hstdmastatus4 {
        &self.hstdmastatus4
    }
    #[doc = "0x750 - Host DMA Channel Next Descriptor Address Register (n = 5)"]
    #[inline(always)]
    pub const fn hstdmanxtdsc5(&self) -> &Hstdmanxtdsc5 {
        &self.hstdmanxtdsc5
    }
    #[doc = "0x754 - Host DMA Channel Address Register (n = 5)"]
    #[inline(always)]
    pub const fn hstdmaaddress5(&self) -> &Hstdmaaddress5 {
        &self.hstdmaaddress5
    }
    #[doc = "0x758 - Host DMA Channel Control Register (n = 5)"]
    #[inline(always)]
    pub const fn hstdmacontrol5(&self) -> &Hstdmacontrol5 {
        &self.hstdmacontrol5
    }
    #[doc = "0x75c - Host DMA Channel Status Register (n = 5)"]
    #[inline(always)]
    pub const fn hstdmastatus5(&self) -> &Hstdmastatus5 {
        &self.hstdmastatus5
    }
    #[doc = "0x760 - Host DMA Channel Next Descriptor Address Register (n = 6)"]
    #[inline(always)]
    pub const fn hstdmanxtdsc6(&self) -> &Hstdmanxtdsc6 {
        &self.hstdmanxtdsc6
    }
    #[doc = "0x764 - Host DMA Channel Address Register (n = 6)"]
    #[inline(always)]
    pub const fn hstdmaaddress6(&self) -> &Hstdmaaddress6 {
        &self.hstdmaaddress6
    }
    #[doc = "0x768 - Host DMA Channel Control Register (n = 6)"]
    #[inline(always)]
    pub const fn hstdmacontrol6(&self) -> &Hstdmacontrol6 {
        &self.hstdmacontrol6
    }
    #[doc = "0x76c - Host DMA Channel Status Register (n = 6)"]
    #[inline(always)]
    pub const fn hstdmastatus6(&self) -> &Hstdmastatus6 {
        &self.hstdmastatus6
    }
    #[doc = "0x770 - Host DMA Channel Next Descriptor Address Register (n = 7)"]
    #[inline(always)]
    pub const fn hstdmanxtdsc7(&self) -> &Hstdmanxtdsc7 {
        &self.hstdmanxtdsc7
    }
    #[doc = "0x774 - Host DMA Channel Address Register (n = 7)"]
    #[inline(always)]
    pub const fn hstdmaaddress7(&self) -> &Hstdmaaddress7 {
        &self.hstdmaaddress7
    }
    #[doc = "0x778 - Host DMA Channel Control Register (n = 7)"]
    #[inline(always)]
    pub const fn hstdmacontrol7(&self) -> &Hstdmacontrol7 {
        &self.hstdmacontrol7
    }
    #[doc = "0x77c - Host DMA Channel Status Register (n = 7)"]
    #[inline(always)]
    pub const fn hstdmastatus7(&self) -> &Hstdmastatus7 {
        &self.hstdmastatus7
    }
    #[doc = "0x800 - General Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x804 - General Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x808 - General Status Clear Register"]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    #[doc = "0x80c - General Status Set Register"]
    #[inline(always)]
    pub const fn sfr(&self) -> &Sfr {
        &self.sfr
    }
    #[doc = "0x82c - General Finite State Machine Register"]
    #[inline(always)]
    pub const fn fsm(&self) -> &Fsm {
        &self.fsm
    }
}
#[doc = "DEVCTRL (rw) register accessor: Device General Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devctrl`]
module"]
#[doc(alias = "DEVCTRL")]
pub type Devctrl = crate::Reg<devctrl::DevctrlSpec>;
#[doc = "Device General Control Register"]
pub mod devctrl;
#[doc = "DEVISR (r) register accessor: Device Global Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devisr`]
module"]
#[doc(alias = "DEVISR")]
pub type Devisr = crate::Reg<devisr::DevisrSpec>;
#[doc = "Device Global Interrupt Status Register"]
pub mod devisr;
#[doc = "DEVICR (w) register accessor: Device Global Interrupt Clear Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devicr`]
module"]
#[doc(alias = "DEVICR")]
pub type Devicr = crate::Reg<devicr::DevicrSpec>;
#[doc = "Device Global Interrupt Clear Register"]
pub mod devicr;
#[doc = "DEVIFR (w) register accessor: Device Global Interrupt Set Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devifr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devifr`]
module"]
#[doc(alias = "DEVIFR")]
pub type Devifr = crate::Reg<devifr::DevifrSpec>;
#[doc = "Device Global Interrupt Set Register"]
pub mod devifr;
#[doc = "DEVIMR (r) register accessor: Device Global Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devimr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devimr`]
module"]
#[doc(alias = "DEVIMR")]
pub type Devimr = crate::Reg<devimr::DevimrSpec>;
#[doc = "Device Global Interrupt Mask Register"]
pub mod devimr;
#[doc = "DEVIDR (w) register accessor: Device Global Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devidr`]
module"]
#[doc(alias = "DEVIDR")]
pub type Devidr = crate::Reg<devidr::DevidrSpec>;
#[doc = "Device Global Interrupt Disable Register"]
pub mod devidr;
#[doc = "DEVIER (w) register accessor: Device Global Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devier`]
module"]
#[doc(alias = "DEVIER")]
pub type Devier = crate::Reg<devier::DevierSpec>;
#[doc = "Device Global Interrupt Enable Register"]
pub mod devier;
#[doc = "DEVEPT (rw) register accessor: Device Endpoint Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devept::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devept::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devept`]
module"]
#[doc(alias = "DEVEPT")]
pub type Devept = crate::Reg<devept::DeveptSpec>;
#[doc = "Device Endpoint Register"]
pub mod devept;
#[doc = "DEVFNUM (r) register accessor: Device Frame Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devfnum::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devfnum`]
module"]
#[doc(alias = "DEVFNUM")]
pub type Devfnum = crate::Reg<devfnum::DevfnumSpec>;
#[doc = "Device Frame Number Register"]
pub mod devfnum;
#[doc = "DEVEPTCFG0 (rw) register accessor: Device Endpoint Configuration Register (n = 0) 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptcfg0`]
module"]
#[doc(alias = "DEVEPTCFG0")]
pub type Deveptcfg0 = crate::Reg<deveptcfg0::Deveptcfg0Spec>;
#[doc = "Device Endpoint Configuration Register (n = 0) 0"]
pub mod deveptcfg0;
#[doc = "DEVEPTCFG1 (rw) register accessor: Device Endpoint Configuration Register (n = 0) 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptcfg1`]
module"]
#[doc(alias = "DEVEPTCFG1")]
pub type Deveptcfg1 = crate::Reg<deveptcfg1::Deveptcfg1Spec>;
#[doc = "Device Endpoint Configuration Register (n = 0) 1"]
pub mod deveptcfg1;
#[doc = "DEVEPTCFG2 (rw) register accessor: Device Endpoint Configuration Register (n = 0) 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptcfg2`]
module"]
#[doc(alias = "DEVEPTCFG2")]
pub type Deveptcfg2 = crate::Reg<deveptcfg2::Deveptcfg2Spec>;
#[doc = "Device Endpoint Configuration Register (n = 0) 2"]
pub mod deveptcfg2;
#[doc = "DEVEPTCFG3 (rw) register accessor: Device Endpoint Configuration Register (n = 0) 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptcfg3`]
module"]
#[doc(alias = "DEVEPTCFG3")]
pub type Deveptcfg3 = crate::Reg<deveptcfg3::Deveptcfg3Spec>;
#[doc = "Device Endpoint Configuration Register (n = 0) 3"]
pub mod deveptcfg3;
#[doc = "DEVEPTCFG4 (rw) register accessor: Device Endpoint Configuration Register (n = 0) 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptcfg4`]
module"]
#[doc(alias = "DEVEPTCFG4")]
pub type Deveptcfg4 = crate::Reg<deveptcfg4::Deveptcfg4Spec>;
#[doc = "Device Endpoint Configuration Register (n = 0) 4"]
pub mod deveptcfg4;
#[doc = "DEVEPTCFG5 (rw) register accessor: Device Endpoint Configuration Register (n = 0) 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptcfg5`]
module"]
#[doc(alias = "DEVEPTCFG5")]
pub type Deveptcfg5 = crate::Reg<deveptcfg5::Deveptcfg5Spec>;
#[doc = "Device Endpoint Configuration Register (n = 0) 5"]
pub mod deveptcfg5;
#[doc = "DEVEPTCFG6 (rw) register accessor: Device Endpoint Configuration Register (n = 0) 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg6::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptcfg6`]
module"]
#[doc(alias = "DEVEPTCFG6")]
pub type Deveptcfg6 = crate::Reg<deveptcfg6::Deveptcfg6Spec>;
#[doc = "Device Endpoint Configuration Register (n = 0) 6"]
pub mod deveptcfg6;
#[doc = "DEVEPTCFG7 (rw) register accessor: Device Endpoint Configuration Register (n = 0) 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg7::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptcfg7`]
module"]
#[doc(alias = "DEVEPTCFG7")]
pub type Deveptcfg7 = crate::Reg<deveptcfg7::Deveptcfg7Spec>;
#[doc = "Device Endpoint Configuration Register (n = 0) 7"]
pub mod deveptcfg7;
#[doc = "DEVEPTCFG8 (rw) register accessor: Device Endpoint Configuration Register (n = 0) 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg8::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptcfg8`]
module"]
#[doc(alias = "DEVEPTCFG8")]
pub type Deveptcfg8 = crate::Reg<deveptcfg8::Deveptcfg8Spec>;
#[doc = "Device Endpoint Configuration Register (n = 0) 8"]
pub mod deveptcfg8;
#[doc = "DEVEPTCFG9 (rw) register accessor: Device Endpoint Configuration Register (n = 0) 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg9::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptcfg9`]
module"]
#[doc(alias = "DEVEPTCFG9")]
pub type Deveptcfg9 = crate::Reg<deveptcfg9::Deveptcfg9Spec>;
#[doc = "Device Endpoint Configuration Register (n = 0) 9"]
pub mod deveptcfg9;
#[doc = "DEVEPTISR0 (r) register accessor: Device Endpoint Status Register (n = 0) 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptisr0`]
module"]
#[doc(alias = "DEVEPTISR0")]
pub type Deveptisr0 = crate::Reg<deveptisr0::Deveptisr0Spec>;
#[doc = "Device Endpoint Status Register (n = 0) 0"]
pub mod deveptisr0;
#[doc = "DEVEPTISR1 (r) register accessor: Device Endpoint Status Register (n = 0) 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptisr1`]
module"]
#[doc(alias = "DEVEPTISR1")]
pub type Deveptisr1 = crate::Reg<deveptisr1::Deveptisr1Spec>;
#[doc = "Device Endpoint Status Register (n = 0) 1"]
pub mod deveptisr1;
#[doc = "DEVEPTISR2 (r) register accessor: Device Endpoint Status Register (n = 0) 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptisr2`]
module"]
#[doc(alias = "DEVEPTISR2")]
pub type Deveptisr2 = crate::Reg<deveptisr2::Deveptisr2Spec>;
#[doc = "Device Endpoint Status Register (n = 0) 2"]
pub mod deveptisr2;
#[doc = "DEVEPTISR3 (r) register accessor: Device Endpoint Status Register (n = 0) 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptisr3`]
module"]
#[doc(alias = "DEVEPTISR3")]
pub type Deveptisr3 = crate::Reg<deveptisr3::Deveptisr3Spec>;
#[doc = "Device Endpoint Status Register (n = 0) 3"]
pub mod deveptisr3;
#[doc = "DEVEPTISR4 (r) register accessor: Device Endpoint Status Register (n = 0) 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptisr4`]
module"]
#[doc(alias = "DEVEPTISR4")]
pub type Deveptisr4 = crate::Reg<deveptisr4::Deveptisr4Spec>;
#[doc = "Device Endpoint Status Register (n = 0) 4"]
pub mod deveptisr4;
#[doc = "DEVEPTISR5 (r) register accessor: Device Endpoint Status Register (n = 0) 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptisr5`]
module"]
#[doc(alias = "DEVEPTISR5")]
pub type Deveptisr5 = crate::Reg<deveptisr5::Deveptisr5Spec>;
#[doc = "Device Endpoint Status Register (n = 0) 5"]
pub mod deveptisr5;
#[doc = "DEVEPTISR6 (r) register accessor: Device Endpoint Status Register (n = 0) 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptisr6`]
module"]
#[doc(alias = "DEVEPTISR6")]
pub type Deveptisr6 = crate::Reg<deveptisr6::Deveptisr6Spec>;
#[doc = "Device Endpoint Status Register (n = 0) 6"]
pub mod deveptisr6;
#[doc = "DEVEPTISR7 (r) register accessor: Device Endpoint Status Register (n = 0) 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptisr7`]
module"]
#[doc(alias = "DEVEPTISR7")]
pub type Deveptisr7 = crate::Reg<deveptisr7::Deveptisr7Spec>;
#[doc = "Device Endpoint Status Register (n = 0) 7"]
pub mod deveptisr7;
#[doc = "DEVEPTISR8 (r) register accessor: Device Endpoint Status Register (n = 0) 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptisr8`]
module"]
#[doc(alias = "DEVEPTISR8")]
pub type Deveptisr8 = crate::Reg<deveptisr8::Deveptisr8Spec>;
#[doc = "Device Endpoint Status Register (n = 0) 8"]
pub mod deveptisr8;
#[doc = "DEVEPTISR9 (r) register accessor: Device Endpoint Status Register (n = 0) 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptisr9`]
module"]
#[doc(alias = "DEVEPTISR9")]
pub type Deveptisr9 = crate::Reg<deveptisr9::Deveptisr9Spec>;
#[doc = "Device Endpoint Status Register (n = 0) 9"]
pub mod deveptisr9;
#[doc = "ISOENPT_DEVEPTISR0_ISOENPT (r) register accessor: Device Endpoint Status Register (n = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoenpt_deveptisr0_isoenpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoenpt_deveptisr0_isoenpt`]
module"]
#[doc(alias = "ISOENPT_DEVEPTISR0_ISOENPT")]
pub type IsoenptDeveptisr0Isoenpt =
    crate::Reg<isoenpt_deveptisr0_isoenpt::IsoenptDeveptisr0IsoenptSpec>;
#[doc = "Device Endpoint Status Register (n = 0)"]
pub mod isoenpt_deveptisr0_isoenpt;
#[doc = "DEVEPTICR0 (w) register accessor: Device Endpoint Clear Register (n = 0) 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devepticr0`]
module"]
#[doc(alias = "DEVEPTICR0")]
pub type Devepticr0 = crate::Reg<devepticr0::Devepticr0Spec>;
#[doc = "Device Endpoint Clear Register (n = 0) 0"]
pub mod devepticr0;
#[doc = "DEVEPTICR1 (w) register accessor: Device Endpoint Clear Register (n = 0) 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devepticr1`]
module"]
#[doc(alias = "DEVEPTICR1")]
pub type Devepticr1 = crate::Reg<devepticr1::Devepticr1Spec>;
#[doc = "Device Endpoint Clear Register (n = 0) 1"]
pub mod devepticr1;
#[doc = "DEVEPTICR2 (w) register accessor: Device Endpoint Clear Register (n = 0) 2\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devepticr2`]
module"]
#[doc(alias = "DEVEPTICR2")]
pub type Devepticr2 = crate::Reg<devepticr2::Devepticr2Spec>;
#[doc = "Device Endpoint Clear Register (n = 0) 2"]
pub mod devepticr2;
#[doc = "DEVEPTICR3 (w) register accessor: Device Endpoint Clear Register (n = 0) 3\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devepticr3`]
module"]
#[doc(alias = "DEVEPTICR3")]
pub type Devepticr3 = crate::Reg<devepticr3::Devepticr3Spec>;
#[doc = "Device Endpoint Clear Register (n = 0) 3"]
pub mod devepticr3;
#[doc = "DEVEPTICR4 (w) register accessor: Device Endpoint Clear Register (n = 0) 4\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devepticr4`]
module"]
#[doc(alias = "DEVEPTICR4")]
pub type Devepticr4 = crate::Reg<devepticr4::Devepticr4Spec>;
#[doc = "Device Endpoint Clear Register (n = 0) 4"]
pub mod devepticr4;
#[doc = "DEVEPTICR5 (w) register accessor: Device Endpoint Clear Register (n = 0) 5\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devepticr5`]
module"]
#[doc(alias = "DEVEPTICR5")]
pub type Devepticr5 = crate::Reg<devepticr5::Devepticr5Spec>;
#[doc = "Device Endpoint Clear Register (n = 0) 5"]
pub mod devepticr5;
#[doc = "DEVEPTICR6 (w) register accessor: Device Endpoint Clear Register (n = 0) 6\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devepticr6`]
module"]
#[doc(alias = "DEVEPTICR6")]
pub type Devepticr6 = crate::Reg<devepticr6::Devepticr6Spec>;
#[doc = "Device Endpoint Clear Register (n = 0) 6"]
pub mod devepticr6;
#[doc = "DEVEPTICR7 (w) register accessor: Device Endpoint Clear Register (n = 0) 7\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devepticr7`]
module"]
#[doc(alias = "DEVEPTICR7")]
pub type Devepticr7 = crate::Reg<devepticr7::Devepticr7Spec>;
#[doc = "Device Endpoint Clear Register (n = 0) 7"]
pub mod devepticr7;
#[doc = "DEVEPTICR8 (w) register accessor: Device Endpoint Clear Register (n = 0) 8\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devepticr8`]
module"]
#[doc(alias = "DEVEPTICR8")]
pub type Devepticr8 = crate::Reg<devepticr8::Devepticr8Spec>;
#[doc = "Device Endpoint Clear Register (n = 0) 8"]
pub mod devepticr8;
#[doc = "DEVEPTICR9 (w) register accessor: Device Endpoint Clear Register (n = 0) 9\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr9::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devepticr9`]
module"]
#[doc(alias = "DEVEPTICR9")]
pub type Devepticr9 = crate::Reg<devepticr9::Devepticr9Spec>;
#[doc = "Device Endpoint Clear Register (n = 0) 9"]
pub mod devepticr9;
#[doc = "ISOENPT_DEVEPTICR0_ISOENPT (w) register accessor: Device Endpoint Clear Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoenpt_devepticr0_isoenpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoenpt_devepticr0_isoenpt`]
module"]
#[doc(alias = "ISOENPT_DEVEPTICR0_ISOENPT")]
pub type IsoenptDevepticr0Isoenpt =
    crate::Reg<isoenpt_devepticr0_isoenpt::IsoenptDevepticr0IsoenptSpec>;
#[doc = "Device Endpoint Clear Register (n = 0)"]
pub mod isoenpt_devepticr0_isoenpt;
#[doc = "DEVEPTIFR0 (w) register accessor: Device Endpoint Set Register (n = 0) 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptifr0`]
module"]
#[doc(alias = "DEVEPTIFR0")]
pub type Deveptifr0 = crate::Reg<deveptifr0::Deveptifr0Spec>;
#[doc = "Device Endpoint Set Register (n = 0) 0"]
pub mod deveptifr0;
#[doc = "DEVEPTIFR1 (w) register accessor: Device Endpoint Set Register (n = 0) 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptifr1`]
module"]
#[doc(alias = "DEVEPTIFR1")]
pub type Deveptifr1 = crate::Reg<deveptifr1::Deveptifr1Spec>;
#[doc = "Device Endpoint Set Register (n = 0) 1"]
pub mod deveptifr1;
#[doc = "DEVEPTIFR2 (w) register accessor: Device Endpoint Set Register (n = 0) 2\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptifr2`]
module"]
#[doc(alias = "DEVEPTIFR2")]
pub type Deveptifr2 = crate::Reg<deveptifr2::Deveptifr2Spec>;
#[doc = "Device Endpoint Set Register (n = 0) 2"]
pub mod deveptifr2;
#[doc = "DEVEPTIFR3 (w) register accessor: Device Endpoint Set Register (n = 0) 3\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptifr3`]
module"]
#[doc(alias = "DEVEPTIFR3")]
pub type Deveptifr3 = crate::Reg<deveptifr3::Deveptifr3Spec>;
#[doc = "Device Endpoint Set Register (n = 0) 3"]
pub mod deveptifr3;
#[doc = "DEVEPTIFR4 (w) register accessor: Device Endpoint Set Register (n = 0) 4\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptifr4`]
module"]
#[doc(alias = "DEVEPTIFR4")]
pub type Deveptifr4 = crate::Reg<deveptifr4::Deveptifr4Spec>;
#[doc = "Device Endpoint Set Register (n = 0) 4"]
pub mod deveptifr4;
#[doc = "DEVEPTIFR5 (w) register accessor: Device Endpoint Set Register (n = 0) 5\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptifr5`]
module"]
#[doc(alias = "DEVEPTIFR5")]
pub type Deveptifr5 = crate::Reg<deveptifr5::Deveptifr5Spec>;
#[doc = "Device Endpoint Set Register (n = 0) 5"]
pub mod deveptifr5;
#[doc = "DEVEPTIFR6 (w) register accessor: Device Endpoint Set Register (n = 0) 6\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptifr6`]
module"]
#[doc(alias = "DEVEPTIFR6")]
pub type Deveptifr6 = crate::Reg<deveptifr6::Deveptifr6Spec>;
#[doc = "Device Endpoint Set Register (n = 0) 6"]
pub mod deveptifr6;
#[doc = "DEVEPTIFR7 (w) register accessor: Device Endpoint Set Register (n = 0) 7\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptifr7`]
module"]
#[doc(alias = "DEVEPTIFR7")]
pub type Deveptifr7 = crate::Reg<deveptifr7::Deveptifr7Spec>;
#[doc = "Device Endpoint Set Register (n = 0) 7"]
pub mod deveptifr7;
#[doc = "DEVEPTIFR8 (w) register accessor: Device Endpoint Set Register (n = 0) 8\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptifr8`]
module"]
#[doc(alias = "DEVEPTIFR8")]
pub type Deveptifr8 = crate::Reg<deveptifr8::Deveptifr8Spec>;
#[doc = "Device Endpoint Set Register (n = 0) 8"]
pub mod deveptifr8;
#[doc = "DEVEPTIFR9 (w) register accessor: Device Endpoint Set Register (n = 0) 9\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr9::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptifr9`]
module"]
#[doc(alias = "DEVEPTIFR9")]
pub type Deveptifr9 = crate::Reg<deveptifr9::Deveptifr9Spec>;
#[doc = "Device Endpoint Set Register (n = 0) 9"]
pub mod deveptifr9;
#[doc = "ISOENPT_DEVEPTIFR0_ISOENPT (w) register accessor: Device Endpoint Set Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoenpt_deveptifr0_isoenpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoenpt_deveptifr0_isoenpt`]
module"]
#[doc(alias = "ISOENPT_DEVEPTIFR0_ISOENPT")]
pub type IsoenptDeveptifr0Isoenpt =
    crate::Reg<isoenpt_deveptifr0_isoenpt::IsoenptDeveptifr0IsoenptSpec>;
#[doc = "Device Endpoint Set Register (n = 0)"]
pub mod isoenpt_deveptifr0_isoenpt;
#[doc = "DEVEPTIMR0 (r) register accessor: Device Endpoint Mask Register (n = 0) 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptimr0`]
module"]
#[doc(alias = "DEVEPTIMR0")]
pub type Deveptimr0 = crate::Reg<deveptimr0::Deveptimr0Spec>;
#[doc = "Device Endpoint Mask Register (n = 0) 0"]
pub mod deveptimr0;
#[doc = "DEVEPTIMR1 (r) register accessor: Device Endpoint Mask Register (n = 0) 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptimr1`]
module"]
#[doc(alias = "DEVEPTIMR1")]
pub type Deveptimr1 = crate::Reg<deveptimr1::Deveptimr1Spec>;
#[doc = "Device Endpoint Mask Register (n = 0) 1"]
pub mod deveptimr1;
#[doc = "DEVEPTIMR2 (r) register accessor: Device Endpoint Mask Register (n = 0) 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptimr2`]
module"]
#[doc(alias = "DEVEPTIMR2")]
pub type Deveptimr2 = crate::Reg<deveptimr2::Deveptimr2Spec>;
#[doc = "Device Endpoint Mask Register (n = 0) 2"]
pub mod deveptimr2;
#[doc = "DEVEPTIMR3 (r) register accessor: Device Endpoint Mask Register (n = 0) 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptimr3`]
module"]
#[doc(alias = "DEVEPTIMR3")]
pub type Deveptimr3 = crate::Reg<deveptimr3::Deveptimr3Spec>;
#[doc = "Device Endpoint Mask Register (n = 0) 3"]
pub mod deveptimr3;
#[doc = "DEVEPTIMR4 (r) register accessor: Device Endpoint Mask Register (n = 0) 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptimr4`]
module"]
#[doc(alias = "DEVEPTIMR4")]
pub type Deveptimr4 = crate::Reg<deveptimr4::Deveptimr4Spec>;
#[doc = "Device Endpoint Mask Register (n = 0) 4"]
pub mod deveptimr4;
#[doc = "DEVEPTIMR5 (r) register accessor: Device Endpoint Mask Register (n = 0) 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptimr5`]
module"]
#[doc(alias = "DEVEPTIMR5")]
pub type Deveptimr5 = crate::Reg<deveptimr5::Deveptimr5Spec>;
#[doc = "Device Endpoint Mask Register (n = 0) 5"]
pub mod deveptimr5;
#[doc = "DEVEPTIMR6 (r) register accessor: Device Endpoint Mask Register (n = 0) 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptimr6`]
module"]
#[doc(alias = "DEVEPTIMR6")]
pub type Deveptimr6 = crate::Reg<deveptimr6::Deveptimr6Spec>;
#[doc = "Device Endpoint Mask Register (n = 0) 6"]
pub mod deveptimr6;
#[doc = "DEVEPTIMR7 (r) register accessor: Device Endpoint Mask Register (n = 0) 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptimr7`]
module"]
#[doc(alias = "DEVEPTIMR7")]
pub type Deveptimr7 = crate::Reg<deveptimr7::Deveptimr7Spec>;
#[doc = "Device Endpoint Mask Register (n = 0) 7"]
pub mod deveptimr7;
#[doc = "DEVEPTIMR8 (r) register accessor: Device Endpoint Mask Register (n = 0) 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptimr8`]
module"]
#[doc(alias = "DEVEPTIMR8")]
pub type Deveptimr8 = crate::Reg<deveptimr8::Deveptimr8Spec>;
#[doc = "Device Endpoint Mask Register (n = 0) 8"]
pub mod deveptimr8;
#[doc = "DEVEPTIMR9 (r) register accessor: Device Endpoint Mask Register (n = 0) 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptimr9`]
module"]
#[doc(alias = "DEVEPTIMR9")]
pub type Deveptimr9 = crate::Reg<deveptimr9::Deveptimr9Spec>;
#[doc = "Device Endpoint Mask Register (n = 0) 9"]
pub mod deveptimr9;
#[doc = "ISOENPT_DEVEPTIMR0_ISOENPT (r) register accessor: Device Endpoint Mask Register (n = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoenpt_deveptimr0_isoenpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoenpt_deveptimr0_isoenpt`]
module"]
#[doc(alias = "ISOENPT_DEVEPTIMR0_ISOENPT")]
pub type IsoenptDeveptimr0Isoenpt =
    crate::Reg<isoenpt_deveptimr0_isoenpt::IsoenptDeveptimr0IsoenptSpec>;
#[doc = "Device Endpoint Mask Register (n = 0)"]
pub mod isoenpt_deveptimr0_isoenpt;
#[doc = "DEVEPTIER0 (w) register accessor: Device Endpoint Enable Register (n = 0) 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptier0`]
module"]
#[doc(alias = "DEVEPTIER0")]
pub type Deveptier0 = crate::Reg<deveptier0::Deveptier0Spec>;
#[doc = "Device Endpoint Enable Register (n = 0) 0"]
pub mod deveptier0;
#[doc = "DEVEPTIER1 (w) register accessor: Device Endpoint Enable Register (n = 0) 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptier1`]
module"]
#[doc(alias = "DEVEPTIER1")]
pub type Deveptier1 = crate::Reg<deveptier1::Deveptier1Spec>;
#[doc = "Device Endpoint Enable Register (n = 0) 1"]
pub mod deveptier1;
#[doc = "DEVEPTIER2 (w) register accessor: Device Endpoint Enable Register (n = 0) 2\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptier2`]
module"]
#[doc(alias = "DEVEPTIER2")]
pub type Deveptier2 = crate::Reg<deveptier2::Deveptier2Spec>;
#[doc = "Device Endpoint Enable Register (n = 0) 2"]
pub mod deveptier2;
#[doc = "DEVEPTIER3 (w) register accessor: Device Endpoint Enable Register (n = 0) 3\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptier3`]
module"]
#[doc(alias = "DEVEPTIER3")]
pub type Deveptier3 = crate::Reg<deveptier3::Deveptier3Spec>;
#[doc = "Device Endpoint Enable Register (n = 0) 3"]
pub mod deveptier3;
#[doc = "DEVEPTIER4 (w) register accessor: Device Endpoint Enable Register (n = 0) 4\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptier4`]
module"]
#[doc(alias = "DEVEPTIER4")]
pub type Deveptier4 = crate::Reg<deveptier4::Deveptier4Spec>;
#[doc = "Device Endpoint Enable Register (n = 0) 4"]
pub mod deveptier4;
#[doc = "DEVEPTIER5 (w) register accessor: Device Endpoint Enable Register (n = 0) 5\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptier5`]
module"]
#[doc(alias = "DEVEPTIER5")]
pub type Deveptier5 = crate::Reg<deveptier5::Deveptier5Spec>;
#[doc = "Device Endpoint Enable Register (n = 0) 5"]
pub mod deveptier5;
#[doc = "DEVEPTIER6 (w) register accessor: Device Endpoint Enable Register (n = 0) 6\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptier6`]
module"]
#[doc(alias = "DEVEPTIER6")]
pub type Deveptier6 = crate::Reg<deveptier6::Deveptier6Spec>;
#[doc = "Device Endpoint Enable Register (n = 0) 6"]
pub mod deveptier6;
#[doc = "DEVEPTIER7 (w) register accessor: Device Endpoint Enable Register (n = 0) 7\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptier7`]
module"]
#[doc(alias = "DEVEPTIER7")]
pub type Deveptier7 = crate::Reg<deveptier7::Deveptier7Spec>;
#[doc = "Device Endpoint Enable Register (n = 0) 7"]
pub mod deveptier7;
#[doc = "DEVEPTIER8 (w) register accessor: Device Endpoint Enable Register (n = 0) 8\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptier8`]
module"]
#[doc(alias = "DEVEPTIER8")]
pub type Deveptier8 = crate::Reg<deveptier8::Deveptier8Spec>;
#[doc = "Device Endpoint Enable Register (n = 0) 8"]
pub mod deveptier8;
#[doc = "DEVEPTIER9 (w) register accessor: Device Endpoint Enable Register (n = 0) 9\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier9::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptier9`]
module"]
#[doc(alias = "DEVEPTIER9")]
pub type Deveptier9 = crate::Reg<deveptier9::Deveptier9Spec>;
#[doc = "Device Endpoint Enable Register (n = 0) 9"]
pub mod deveptier9;
#[doc = "ISOENPT_DEVEPTIER0_ISOENPT (w) register accessor: Device Endpoint Enable Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoenpt_deveptier0_isoenpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoenpt_deveptier0_isoenpt`]
module"]
#[doc(alias = "ISOENPT_DEVEPTIER0_ISOENPT")]
pub type IsoenptDeveptier0Isoenpt =
    crate::Reg<isoenpt_deveptier0_isoenpt::IsoenptDeveptier0IsoenptSpec>;
#[doc = "Device Endpoint Enable Register (n = 0)"]
pub mod isoenpt_deveptier0_isoenpt;
#[doc = "DEVEPTIDR0 (w) register accessor: Device Endpoint Disable Register (n = 0) 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptidr0`]
module"]
#[doc(alias = "DEVEPTIDR0")]
pub type Deveptidr0 = crate::Reg<deveptidr0::Deveptidr0Spec>;
#[doc = "Device Endpoint Disable Register (n = 0) 0"]
pub mod deveptidr0;
#[doc = "DEVEPTIDR1 (w) register accessor: Device Endpoint Disable Register (n = 0) 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptidr1`]
module"]
#[doc(alias = "DEVEPTIDR1")]
pub type Deveptidr1 = crate::Reg<deveptidr1::Deveptidr1Spec>;
#[doc = "Device Endpoint Disable Register (n = 0) 1"]
pub mod deveptidr1;
#[doc = "DEVEPTIDR2 (w) register accessor: Device Endpoint Disable Register (n = 0) 2\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptidr2`]
module"]
#[doc(alias = "DEVEPTIDR2")]
pub type Deveptidr2 = crate::Reg<deveptidr2::Deveptidr2Spec>;
#[doc = "Device Endpoint Disable Register (n = 0) 2"]
pub mod deveptidr2;
#[doc = "DEVEPTIDR3 (w) register accessor: Device Endpoint Disable Register (n = 0) 3\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptidr3`]
module"]
#[doc(alias = "DEVEPTIDR3")]
pub type Deveptidr3 = crate::Reg<deveptidr3::Deveptidr3Spec>;
#[doc = "Device Endpoint Disable Register (n = 0) 3"]
pub mod deveptidr3;
#[doc = "DEVEPTIDR4 (w) register accessor: Device Endpoint Disable Register (n = 0) 4\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptidr4`]
module"]
#[doc(alias = "DEVEPTIDR4")]
pub type Deveptidr4 = crate::Reg<deveptidr4::Deveptidr4Spec>;
#[doc = "Device Endpoint Disable Register (n = 0) 4"]
pub mod deveptidr4;
#[doc = "DEVEPTIDR5 (w) register accessor: Device Endpoint Disable Register (n = 0) 5\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptidr5`]
module"]
#[doc(alias = "DEVEPTIDR5")]
pub type Deveptidr5 = crate::Reg<deveptidr5::Deveptidr5Spec>;
#[doc = "Device Endpoint Disable Register (n = 0) 5"]
pub mod deveptidr5;
#[doc = "DEVEPTIDR6 (w) register accessor: Device Endpoint Disable Register (n = 0) 6\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptidr6`]
module"]
#[doc(alias = "DEVEPTIDR6")]
pub type Deveptidr6 = crate::Reg<deveptidr6::Deveptidr6Spec>;
#[doc = "Device Endpoint Disable Register (n = 0) 6"]
pub mod deveptidr6;
#[doc = "DEVEPTIDR7 (w) register accessor: Device Endpoint Disable Register (n = 0) 7\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptidr7`]
module"]
#[doc(alias = "DEVEPTIDR7")]
pub type Deveptidr7 = crate::Reg<deveptidr7::Deveptidr7Spec>;
#[doc = "Device Endpoint Disable Register (n = 0) 7"]
pub mod deveptidr7;
#[doc = "DEVEPTIDR8 (w) register accessor: Device Endpoint Disable Register (n = 0) 8\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptidr8`]
module"]
#[doc(alias = "DEVEPTIDR8")]
pub type Deveptidr8 = crate::Reg<deveptidr8::Deveptidr8Spec>;
#[doc = "Device Endpoint Disable Register (n = 0) 8"]
pub mod deveptidr8;
#[doc = "DEVEPTIDR9 (w) register accessor: Device Endpoint Disable Register (n = 0) 9\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr9::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deveptidr9`]
module"]
#[doc(alias = "DEVEPTIDR9")]
pub type Deveptidr9 = crate::Reg<deveptidr9::Deveptidr9Spec>;
#[doc = "Device Endpoint Disable Register (n = 0) 9"]
pub mod deveptidr9;
#[doc = "ISOENPT_DEVEPTIDR0_ISOENPT (w) register accessor: Device Endpoint Disable Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoenpt_deveptidr0_isoenpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isoenpt_deveptidr0_isoenpt`]
module"]
#[doc(alias = "ISOENPT_DEVEPTIDR0_ISOENPT")]
pub type IsoenptDeveptidr0Isoenpt =
    crate::Reg<isoenpt_deveptidr0_isoenpt::IsoenptDeveptidr0IsoenptSpec>;
#[doc = "Device Endpoint Disable Register (n = 0)"]
pub mod isoenpt_deveptidr0_isoenpt;
#[doc = "DEVDMANXTDSC1 (rw) register accessor: Device DMA Channel Next Descriptor Address Register (n = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmanxtdsc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmanxtdsc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmanxtdsc1`]
module"]
#[doc(alias = "DEVDMANXTDSC1")]
pub type Devdmanxtdsc1 = crate::Reg<devdmanxtdsc1::Devdmanxtdsc1Spec>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod devdmanxtdsc1;
#[doc = "DEVDMAADDRESS1 (rw) register accessor: Device DMA Channel Address Register (n = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmaaddress1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmaaddress1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmaaddress1`]
module"]
#[doc(alias = "DEVDMAADDRESS1")]
pub type Devdmaaddress1 = crate::Reg<devdmaaddress1::Devdmaaddress1Spec>;
#[doc = "Device DMA Channel Address Register (n = 1)"]
pub mod devdmaaddress1;
#[doc = "DEVDMACONTROL1 (rw) register accessor: Device DMA Channel Control Register (n = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmacontrol1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmacontrol1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmacontrol1`]
module"]
#[doc(alias = "DEVDMACONTROL1")]
pub type Devdmacontrol1 = crate::Reg<devdmacontrol1::Devdmacontrol1Spec>;
#[doc = "Device DMA Channel Control Register (n = 1)"]
pub mod devdmacontrol1;
#[doc = "DEVDMASTATUS1 (rw) register accessor: Device DMA Channel Status Register (n = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmastatus1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmastatus1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmastatus1`]
module"]
#[doc(alias = "DEVDMASTATUS1")]
pub type Devdmastatus1 = crate::Reg<devdmastatus1::Devdmastatus1Spec>;
#[doc = "Device DMA Channel Status Register (n = 1)"]
pub mod devdmastatus1;
#[doc = "DEVDMANXTDSC2 (rw) register accessor: Device DMA Channel Next Descriptor Address Register (n = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmanxtdsc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmanxtdsc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmanxtdsc2`]
module"]
#[doc(alias = "DEVDMANXTDSC2")]
pub type Devdmanxtdsc2 = crate::Reg<devdmanxtdsc2::Devdmanxtdsc2Spec>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 2)"]
pub mod devdmanxtdsc2;
#[doc = "DEVDMAADDRESS2 (rw) register accessor: Device DMA Channel Address Register (n = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmaaddress2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmaaddress2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmaaddress2`]
module"]
#[doc(alias = "DEVDMAADDRESS2")]
pub type Devdmaaddress2 = crate::Reg<devdmaaddress2::Devdmaaddress2Spec>;
#[doc = "Device DMA Channel Address Register (n = 2)"]
pub mod devdmaaddress2;
#[doc = "DEVDMACONTROL2 (rw) register accessor: Device DMA Channel Control Register (n = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmacontrol2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmacontrol2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmacontrol2`]
module"]
#[doc(alias = "DEVDMACONTROL2")]
pub type Devdmacontrol2 = crate::Reg<devdmacontrol2::Devdmacontrol2Spec>;
#[doc = "Device DMA Channel Control Register (n = 2)"]
pub mod devdmacontrol2;
#[doc = "DEVDMASTATUS2 (rw) register accessor: Device DMA Channel Status Register (n = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmastatus2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmastatus2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmastatus2`]
module"]
#[doc(alias = "DEVDMASTATUS2")]
pub type Devdmastatus2 = crate::Reg<devdmastatus2::Devdmastatus2Spec>;
#[doc = "Device DMA Channel Status Register (n = 2)"]
pub mod devdmastatus2;
#[doc = "DEVDMANXTDSC3 (rw) register accessor: Device DMA Channel Next Descriptor Address Register (n = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmanxtdsc3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmanxtdsc3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmanxtdsc3`]
module"]
#[doc(alias = "DEVDMANXTDSC3")]
pub type Devdmanxtdsc3 = crate::Reg<devdmanxtdsc3::Devdmanxtdsc3Spec>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 3)"]
pub mod devdmanxtdsc3;
#[doc = "DEVDMAADDRESS3 (rw) register accessor: Device DMA Channel Address Register (n = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmaaddress3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmaaddress3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmaaddress3`]
module"]
#[doc(alias = "DEVDMAADDRESS3")]
pub type Devdmaaddress3 = crate::Reg<devdmaaddress3::Devdmaaddress3Spec>;
#[doc = "Device DMA Channel Address Register (n = 3)"]
pub mod devdmaaddress3;
#[doc = "DEVDMACONTROL3 (rw) register accessor: Device DMA Channel Control Register (n = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmacontrol3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmacontrol3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmacontrol3`]
module"]
#[doc(alias = "DEVDMACONTROL3")]
pub type Devdmacontrol3 = crate::Reg<devdmacontrol3::Devdmacontrol3Spec>;
#[doc = "Device DMA Channel Control Register (n = 3)"]
pub mod devdmacontrol3;
#[doc = "DEVDMASTATUS3 (rw) register accessor: Device DMA Channel Status Register (n = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmastatus3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmastatus3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmastatus3`]
module"]
#[doc(alias = "DEVDMASTATUS3")]
pub type Devdmastatus3 = crate::Reg<devdmastatus3::Devdmastatus3Spec>;
#[doc = "Device DMA Channel Status Register (n = 3)"]
pub mod devdmastatus3;
#[doc = "DEVDMANXTDSC4 (rw) register accessor: Device DMA Channel Next Descriptor Address Register (n = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmanxtdsc4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmanxtdsc4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmanxtdsc4`]
module"]
#[doc(alias = "DEVDMANXTDSC4")]
pub type Devdmanxtdsc4 = crate::Reg<devdmanxtdsc4::Devdmanxtdsc4Spec>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 4)"]
pub mod devdmanxtdsc4;
#[doc = "DEVDMAADDRESS4 (rw) register accessor: Device DMA Channel Address Register (n = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmaaddress4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmaaddress4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmaaddress4`]
module"]
#[doc(alias = "DEVDMAADDRESS4")]
pub type Devdmaaddress4 = crate::Reg<devdmaaddress4::Devdmaaddress4Spec>;
#[doc = "Device DMA Channel Address Register (n = 4)"]
pub mod devdmaaddress4;
#[doc = "DEVDMACONTROL4 (rw) register accessor: Device DMA Channel Control Register (n = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmacontrol4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmacontrol4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmacontrol4`]
module"]
#[doc(alias = "DEVDMACONTROL4")]
pub type Devdmacontrol4 = crate::Reg<devdmacontrol4::Devdmacontrol4Spec>;
#[doc = "Device DMA Channel Control Register (n = 4)"]
pub mod devdmacontrol4;
#[doc = "DEVDMASTATUS4 (rw) register accessor: Device DMA Channel Status Register (n = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmastatus4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmastatus4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmastatus4`]
module"]
#[doc(alias = "DEVDMASTATUS4")]
pub type Devdmastatus4 = crate::Reg<devdmastatus4::Devdmastatus4Spec>;
#[doc = "Device DMA Channel Status Register (n = 4)"]
pub mod devdmastatus4;
#[doc = "DEVDMANXTDSC5 (rw) register accessor: Device DMA Channel Next Descriptor Address Register (n = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmanxtdsc5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmanxtdsc5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmanxtdsc5`]
module"]
#[doc(alias = "DEVDMANXTDSC5")]
pub type Devdmanxtdsc5 = crate::Reg<devdmanxtdsc5::Devdmanxtdsc5Spec>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 5)"]
pub mod devdmanxtdsc5;
#[doc = "DEVDMAADDRESS5 (rw) register accessor: Device DMA Channel Address Register (n = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmaaddress5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmaaddress5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmaaddress5`]
module"]
#[doc(alias = "DEVDMAADDRESS5")]
pub type Devdmaaddress5 = crate::Reg<devdmaaddress5::Devdmaaddress5Spec>;
#[doc = "Device DMA Channel Address Register (n = 5)"]
pub mod devdmaaddress5;
#[doc = "DEVDMACONTROL5 (rw) register accessor: Device DMA Channel Control Register (n = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmacontrol5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmacontrol5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmacontrol5`]
module"]
#[doc(alias = "DEVDMACONTROL5")]
pub type Devdmacontrol5 = crate::Reg<devdmacontrol5::Devdmacontrol5Spec>;
#[doc = "Device DMA Channel Control Register (n = 5)"]
pub mod devdmacontrol5;
#[doc = "DEVDMASTATUS5 (rw) register accessor: Device DMA Channel Status Register (n = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmastatus5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmastatus5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmastatus5`]
module"]
#[doc(alias = "DEVDMASTATUS5")]
pub type Devdmastatus5 = crate::Reg<devdmastatus5::Devdmastatus5Spec>;
#[doc = "Device DMA Channel Status Register (n = 5)"]
pub mod devdmastatus5;
#[doc = "DEVDMANXTDSC6 (rw) register accessor: Device DMA Channel Next Descriptor Address Register (n = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmanxtdsc6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmanxtdsc6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmanxtdsc6`]
module"]
#[doc(alias = "DEVDMANXTDSC6")]
pub type Devdmanxtdsc6 = crate::Reg<devdmanxtdsc6::Devdmanxtdsc6Spec>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 6)"]
pub mod devdmanxtdsc6;
#[doc = "DEVDMAADDRESS6 (rw) register accessor: Device DMA Channel Address Register (n = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmaaddress6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmaaddress6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmaaddress6`]
module"]
#[doc(alias = "DEVDMAADDRESS6")]
pub type Devdmaaddress6 = crate::Reg<devdmaaddress6::Devdmaaddress6Spec>;
#[doc = "Device DMA Channel Address Register (n = 6)"]
pub mod devdmaaddress6;
#[doc = "DEVDMACONTROL6 (rw) register accessor: Device DMA Channel Control Register (n = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmacontrol6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmacontrol6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmacontrol6`]
module"]
#[doc(alias = "DEVDMACONTROL6")]
pub type Devdmacontrol6 = crate::Reg<devdmacontrol6::Devdmacontrol6Spec>;
#[doc = "Device DMA Channel Control Register (n = 6)"]
pub mod devdmacontrol6;
#[doc = "DEVDMASTATUS6 (rw) register accessor: Device DMA Channel Status Register (n = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmastatus6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmastatus6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmastatus6`]
module"]
#[doc(alias = "DEVDMASTATUS6")]
pub type Devdmastatus6 = crate::Reg<devdmastatus6::Devdmastatus6Spec>;
#[doc = "Device DMA Channel Status Register (n = 6)"]
pub mod devdmastatus6;
#[doc = "DEVDMANXTDSC7 (rw) register accessor: Device DMA Channel Next Descriptor Address Register (n = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmanxtdsc7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmanxtdsc7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmanxtdsc7`]
module"]
#[doc(alias = "DEVDMANXTDSC7")]
pub type Devdmanxtdsc7 = crate::Reg<devdmanxtdsc7::Devdmanxtdsc7Spec>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 7)"]
pub mod devdmanxtdsc7;
#[doc = "DEVDMAADDRESS7 (rw) register accessor: Device DMA Channel Address Register (n = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmaaddress7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmaaddress7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmaaddress7`]
module"]
#[doc(alias = "DEVDMAADDRESS7")]
pub type Devdmaaddress7 = crate::Reg<devdmaaddress7::Devdmaaddress7Spec>;
#[doc = "Device DMA Channel Address Register (n = 7)"]
pub mod devdmaaddress7;
#[doc = "DEVDMACONTROL7 (rw) register accessor: Device DMA Channel Control Register (n = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmacontrol7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmacontrol7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmacontrol7`]
module"]
#[doc(alias = "DEVDMACONTROL7")]
pub type Devdmacontrol7 = crate::Reg<devdmacontrol7::Devdmacontrol7Spec>;
#[doc = "Device DMA Channel Control Register (n = 7)"]
pub mod devdmacontrol7;
#[doc = "DEVDMASTATUS7 (rw) register accessor: Device DMA Channel Status Register (n = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmastatus7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmastatus7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdmastatus7`]
module"]
#[doc(alias = "DEVDMASTATUS7")]
pub type Devdmastatus7 = crate::Reg<devdmastatus7::Devdmastatus7Spec>;
#[doc = "Device DMA Channel Status Register (n = 7)"]
pub mod devdmastatus7;
#[doc = "HSTCTRL (rw) register accessor: Host General Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstctrl`]
module"]
#[doc(alias = "HSTCTRL")]
pub type Hstctrl = crate::Reg<hstctrl::HstctrlSpec>;
#[doc = "Host General Control Register"]
pub mod hstctrl;
#[doc = "HSTISR (r) register accessor: Host Global Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstisr`]
module"]
#[doc(alias = "HSTISR")]
pub type Hstisr = crate::Reg<hstisr::HstisrSpec>;
#[doc = "Host Global Interrupt Status Register"]
pub mod hstisr;
#[doc = "HSTICR (w) register accessor: Host Global Interrupt Clear Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsticr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsticr`]
module"]
#[doc(alias = "HSTICR")]
pub type Hsticr = crate::Reg<hsticr::HsticrSpec>;
#[doc = "Host Global Interrupt Clear Register"]
pub mod hsticr;
#[doc = "HSTIFR (w) register accessor: Host Global Interrupt Set Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstifr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstifr`]
module"]
#[doc(alias = "HSTIFR")]
pub type Hstifr = crate::Reg<hstifr::HstifrSpec>;
#[doc = "Host Global Interrupt Set Register"]
pub mod hstifr;
#[doc = "HSTIMR (r) register accessor: Host Global Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstimr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstimr`]
module"]
#[doc(alias = "HSTIMR")]
pub type Hstimr = crate::Reg<hstimr::HstimrSpec>;
#[doc = "Host Global Interrupt Mask Register"]
pub mod hstimr;
#[doc = "HSTIDR (w) register accessor: Host Global Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstidr`]
module"]
#[doc(alias = "HSTIDR")]
pub type Hstidr = crate::Reg<hstidr::HstidrSpec>;
#[doc = "Host Global Interrupt Disable Register"]
pub mod hstidr;
#[doc = "HSTIER (w) register accessor: Host Global Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstier`]
module"]
#[doc(alias = "HSTIER")]
pub type Hstier = crate::Reg<hstier::HstierSpec>;
#[doc = "Host Global Interrupt Enable Register"]
pub mod hstier;
#[doc = "HSTPIP (rw) register accessor: Host Pipe Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpip::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpip::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpip`]
module"]
#[doc(alias = "HSTPIP")]
pub type Hstpip = crate::Reg<hstpip::HstpipSpec>;
#[doc = "Host Pipe Register"]
pub mod hstpip;
#[doc = "HSTFNUM (rw) register accessor: Host Frame Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstfnum::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstfnum::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstfnum`]
module"]
#[doc(alias = "HSTFNUM")]
pub type Hstfnum = crate::Reg<hstfnum::HstfnumSpec>;
#[doc = "Host Frame Number Register"]
pub mod hstfnum;
#[doc = "HSTADDR1 (rw) register accessor: Host Address 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstaddr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstaddr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstaddr1`]
module"]
#[doc(alias = "HSTADDR1")]
pub type Hstaddr1 = crate::Reg<hstaddr1::Hstaddr1Spec>;
#[doc = "Host Address 1 Register"]
pub mod hstaddr1;
#[doc = "HSTADDR2 (rw) register accessor: Host Address 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstaddr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstaddr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstaddr2`]
module"]
#[doc(alias = "HSTADDR2")]
pub type Hstaddr2 = crate::Reg<hstaddr2::Hstaddr2Spec>;
#[doc = "Host Address 2 Register"]
pub mod hstaddr2;
#[doc = "HSTADDR3 (rw) register accessor: Host Address 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstaddr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstaddr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstaddr3`]
module"]
#[doc(alias = "HSTADDR3")]
pub type Hstaddr3 = crate::Reg<hstaddr3::Hstaddr3Spec>;
#[doc = "Host Address 3 Register"]
pub mod hstaddr3;
#[doc = "HSTPIPCFG0 (rw) register accessor: Host Pipe Configuration Register (n = 0) 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipcfg0`]
module"]
#[doc(alias = "HSTPIPCFG0")]
pub type Hstpipcfg0 = crate::Reg<hstpipcfg0::Hstpipcfg0Spec>;
#[doc = "Host Pipe Configuration Register (n = 0) 0"]
pub mod hstpipcfg0;
#[doc = "HSTPIPCFG1 (rw) register accessor: Host Pipe Configuration Register (n = 0) 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipcfg1`]
module"]
#[doc(alias = "HSTPIPCFG1")]
pub type Hstpipcfg1 = crate::Reg<hstpipcfg1::Hstpipcfg1Spec>;
#[doc = "Host Pipe Configuration Register (n = 0) 1"]
pub mod hstpipcfg1;
#[doc = "HSTPIPCFG2 (rw) register accessor: Host Pipe Configuration Register (n = 0) 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipcfg2`]
module"]
#[doc(alias = "HSTPIPCFG2")]
pub type Hstpipcfg2 = crate::Reg<hstpipcfg2::Hstpipcfg2Spec>;
#[doc = "Host Pipe Configuration Register (n = 0) 2"]
pub mod hstpipcfg2;
#[doc = "HSTPIPCFG3 (rw) register accessor: Host Pipe Configuration Register (n = 0) 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipcfg3`]
module"]
#[doc(alias = "HSTPIPCFG3")]
pub type Hstpipcfg3 = crate::Reg<hstpipcfg3::Hstpipcfg3Spec>;
#[doc = "Host Pipe Configuration Register (n = 0) 3"]
pub mod hstpipcfg3;
#[doc = "HSTPIPCFG4 (rw) register accessor: Host Pipe Configuration Register (n = 0) 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipcfg4`]
module"]
#[doc(alias = "HSTPIPCFG4")]
pub type Hstpipcfg4 = crate::Reg<hstpipcfg4::Hstpipcfg4Spec>;
#[doc = "Host Pipe Configuration Register (n = 0) 4"]
pub mod hstpipcfg4;
#[doc = "HSTPIPCFG5 (rw) register accessor: Host Pipe Configuration Register (n = 0) 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipcfg5`]
module"]
#[doc(alias = "HSTPIPCFG5")]
pub type Hstpipcfg5 = crate::Reg<hstpipcfg5::Hstpipcfg5Spec>;
#[doc = "Host Pipe Configuration Register (n = 0) 5"]
pub mod hstpipcfg5;
#[doc = "HSTPIPCFG6 (rw) register accessor: Host Pipe Configuration Register (n = 0) 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg6::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipcfg6`]
module"]
#[doc(alias = "HSTPIPCFG6")]
pub type Hstpipcfg6 = crate::Reg<hstpipcfg6::Hstpipcfg6Spec>;
#[doc = "Host Pipe Configuration Register (n = 0) 6"]
pub mod hstpipcfg6;
#[doc = "HSTPIPCFG7 (rw) register accessor: Host Pipe Configuration Register (n = 0) 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg7::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipcfg7`]
module"]
#[doc(alias = "HSTPIPCFG7")]
pub type Hstpipcfg7 = crate::Reg<hstpipcfg7::Hstpipcfg7Spec>;
#[doc = "Host Pipe Configuration Register (n = 0) 7"]
pub mod hstpipcfg7;
#[doc = "HSTPIPCFG8 (rw) register accessor: Host Pipe Configuration Register (n = 0) 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg8::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipcfg8`]
module"]
#[doc(alias = "HSTPIPCFG8")]
pub type Hstpipcfg8 = crate::Reg<hstpipcfg8::Hstpipcfg8Spec>;
#[doc = "Host Pipe Configuration Register (n = 0) 8"]
pub mod hstpipcfg8;
#[doc = "HSTPIPCFG9 (rw) register accessor: Host Pipe Configuration Register (n = 0) 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg9::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipcfg9`]
module"]
#[doc(alias = "HSTPIPCFG9")]
pub type Hstpipcfg9 = crate::Reg<hstpipcfg9::Hstpipcfg9Spec>;
#[doc = "Host Pipe Configuration Register (n = 0) 9"]
pub mod hstpipcfg9;
#[doc = "HSBOHSCP_HSTPIPCFG0_HSBOHSCP (rw) register accessor: Host Pipe Configuration Register (n = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsbohscp_hstpipcfg0_hsbohscp::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsbohscp_hstpipcfg0_hsbohscp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsbohscp_hstpipcfg0_hsbohscp`]
module"]
#[doc(alias = "HSBOHSCP_HSTPIPCFG0_HSBOHSCP")]
pub type HsbohscpHstpipcfg0Hsbohscp =
    crate::Reg<hsbohscp_hstpipcfg0_hsbohscp::HsbohscpHstpipcfg0HsbohscpSpec>;
#[doc = "Host Pipe Configuration Register (n = 0)"]
pub mod hsbohscp_hstpipcfg0_hsbohscp;
#[doc = "HSTPIPISR0 (r) register accessor: Host Pipe Status Register (n = 0) 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipisr0`]
module"]
#[doc(alias = "HSTPIPISR0")]
pub type Hstpipisr0 = crate::Reg<hstpipisr0::Hstpipisr0Spec>;
#[doc = "Host Pipe Status Register (n = 0) 0"]
pub mod hstpipisr0;
#[doc = "HSTPIPISR1 (r) register accessor: Host Pipe Status Register (n = 0) 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipisr1`]
module"]
#[doc(alias = "HSTPIPISR1")]
pub type Hstpipisr1 = crate::Reg<hstpipisr1::Hstpipisr1Spec>;
#[doc = "Host Pipe Status Register (n = 0) 1"]
pub mod hstpipisr1;
#[doc = "HSTPIPISR2 (r) register accessor: Host Pipe Status Register (n = 0) 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipisr2`]
module"]
#[doc(alias = "HSTPIPISR2")]
pub type Hstpipisr2 = crate::Reg<hstpipisr2::Hstpipisr2Spec>;
#[doc = "Host Pipe Status Register (n = 0) 2"]
pub mod hstpipisr2;
#[doc = "HSTPIPISR3 (r) register accessor: Host Pipe Status Register (n = 0) 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipisr3`]
module"]
#[doc(alias = "HSTPIPISR3")]
pub type Hstpipisr3 = crate::Reg<hstpipisr3::Hstpipisr3Spec>;
#[doc = "Host Pipe Status Register (n = 0) 3"]
pub mod hstpipisr3;
#[doc = "HSTPIPISR4 (r) register accessor: Host Pipe Status Register (n = 0) 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipisr4`]
module"]
#[doc(alias = "HSTPIPISR4")]
pub type Hstpipisr4 = crate::Reg<hstpipisr4::Hstpipisr4Spec>;
#[doc = "Host Pipe Status Register (n = 0) 4"]
pub mod hstpipisr4;
#[doc = "HSTPIPISR5 (r) register accessor: Host Pipe Status Register (n = 0) 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipisr5`]
module"]
#[doc(alias = "HSTPIPISR5")]
pub type Hstpipisr5 = crate::Reg<hstpipisr5::Hstpipisr5Spec>;
#[doc = "Host Pipe Status Register (n = 0) 5"]
pub mod hstpipisr5;
#[doc = "HSTPIPISR6 (r) register accessor: Host Pipe Status Register (n = 0) 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipisr6`]
module"]
#[doc(alias = "HSTPIPISR6")]
pub type Hstpipisr6 = crate::Reg<hstpipisr6::Hstpipisr6Spec>;
#[doc = "Host Pipe Status Register (n = 0) 6"]
pub mod hstpipisr6;
#[doc = "HSTPIPISR7 (r) register accessor: Host Pipe Status Register (n = 0) 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipisr7`]
module"]
#[doc(alias = "HSTPIPISR7")]
pub type Hstpipisr7 = crate::Reg<hstpipisr7::Hstpipisr7Spec>;
#[doc = "Host Pipe Status Register (n = 0) 7"]
pub mod hstpipisr7;
#[doc = "HSTPIPISR8 (r) register accessor: Host Pipe Status Register (n = 0) 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipisr8`]
module"]
#[doc(alias = "HSTPIPISR8")]
pub type Hstpipisr8 = crate::Reg<hstpipisr8::Hstpipisr8Spec>;
#[doc = "Host Pipe Status Register (n = 0) 8"]
pub mod hstpipisr8;
#[doc = "HSTPIPISR9 (r) register accessor: Host Pipe Status Register (n = 0) 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipisr9`]
module"]
#[doc(alias = "HSTPIPISR9")]
pub type Hstpipisr9 = crate::Reg<hstpipisr9::Hstpipisr9Spec>;
#[doc = "Host Pipe Status Register (n = 0) 9"]
pub mod hstpipisr9;
#[doc = "INTPIPES_HSTPIPISR0_INTPIPES (r) register accessor: Host Pipe Status Register (n = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intpipes_hstpipisr0_intpipes::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpipes_hstpipisr0_intpipes`]
module"]
#[doc(alias = "INTPIPES_HSTPIPISR0_INTPIPES")]
pub type IntpipesHstpipisr0Intpipes =
    crate::Reg<intpipes_hstpipisr0_intpipes::IntpipesHstpipisr0IntpipesSpec>;
#[doc = "Host Pipe Status Register (n = 0)"]
pub mod intpipes_hstpipisr0_intpipes;
#[doc = "ISOPIPES_HSTPIPISR0_ISOPIPES (r) register accessor: Host Pipe Status Register (n = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isopipes_hstpipisr0_isopipes::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isopipes_hstpipisr0_isopipes`]
module"]
#[doc(alias = "ISOPIPES_HSTPIPISR0_ISOPIPES")]
pub type IsopipesHstpipisr0Isopipes =
    crate::Reg<isopipes_hstpipisr0_isopipes::IsopipesHstpipisr0IsopipesSpec>;
#[doc = "Host Pipe Status Register (n = 0)"]
pub mod isopipes_hstpipisr0_isopipes;
#[doc = "HSTPIPICR0 (w) register accessor: Host Pipe Clear Register (n = 0) 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipicr0`]
module"]
#[doc(alias = "HSTPIPICR0")]
pub type Hstpipicr0 = crate::Reg<hstpipicr0::Hstpipicr0Spec>;
#[doc = "Host Pipe Clear Register (n = 0) 0"]
pub mod hstpipicr0;
#[doc = "HSTPIPICR1 (w) register accessor: Host Pipe Clear Register (n = 0) 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipicr1`]
module"]
#[doc(alias = "HSTPIPICR1")]
pub type Hstpipicr1 = crate::Reg<hstpipicr1::Hstpipicr1Spec>;
#[doc = "Host Pipe Clear Register (n = 0) 1"]
pub mod hstpipicr1;
#[doc = "HSTPIPICR2 (w) register accessor: Host Pipe Clear Register (n = 0) 2\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipicr2`]
module"]
#[doc(alias = "HSTPIPICR2")]
pub type Hstpipicr2 = crate::Reg<hstpipicr2::Hstpipicr2Spec>;
#[doc = "Host Pipe Clear Register (n = 0) 2"]
pub mod hstpipicr2;
#[doc = "HSTPIPICR3 (w) register accessor: Host Pipe Clear Register (n = 0) 3\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipicr3`]
module"]
#[doc(alias = "HSTPIPICR3")]
pub type Hstpipicr3 = crate::Reg<hstpipicr3::Hstpipicr3Spec>;
#[doc = "Host Pipe Clear Register (n = 0) 3"]
pub mod hstpipicr3;
#[doc = "HSTPIPICR4 (w) register accessor: Host Pipe Clear Register (n = 0) 4\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipicr4`]
module"]
#[doc(alias = "HSTPIPICR4")]
pub type Hstpipicr4 = crate::Reg<hstpipicr4::Hstpipicr4Spec>;
#[doc = "Host Pipe Clear Register (n = 0) 4"]
pub mod hstpipicr4;
#[doc = "HSTPIPICR5 (w) register accessor: Host Pipe Clear Register (n = 0) 5\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipicr5`]
module"]
#[doc(alias = "HSTPIPICR5")]
pub type Hstpipicr5 = crate::Reg<hstpipicr5::Hstpipicr5Spec>;
#[doc = "Host Pipe Clear Register (n = 0) 5"]
pub mod hstpipicr5;
#[doc = "HSTPIPICR6 (w) register accessor: Host Pipe Clear Register (n = 0) 6\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipicr6`]
module"]
#[doc(alias = "HSTPIPICR6")]
pub type Hstpipicr6 = crate::Reg<hstpipicr6::Hstpipicr6Spec>;
#[doc = "Host Pipe Clear Register (n = 0) 6"]
pub mod hstpipicr6;
#[doc = "HSTPIPICR7 (w) register accessor: Host Pipe Clear Register (n = 0) 7\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipicr7`]
module"]
#[doc(alias = "HSTPIPICR7")]
pub type Hstpipicr7 = crate::Reg<hstpipicr7::Hstpipicr7Spec>;
#[doc = "Host Pipe Clear Register (n = 0) 7"]
pub mod hstpipicr7;
#[doc = "HSTPIPICR8 (w) register accessor: Host Pipe Clear Register (n = 0) 8\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipicr8`]
module"]
#[doc(alias = "HSTPIPICR8")]
pub type Hstpipicr8 = crate::Reg<hstpipicr8::Hstpipicr8Spec>;
#[doc = "Host Pipe Clear Register (n = 0) 8"]
pub mod hstpipicr8;
#[doc = "HSTPIPICR9 (w) register accessor: Host Pipe Clear Register (n = 0) 9\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr9::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipicr9`]
module"]
#[doc(alias = "HSTPIPICR9")]
pub type Hstpipicr9 = crate::Reg<hstpipicr9::Hstpipicr9Spec>;
#[doc = "Host Pipe Clear Register (n = 0) 9"]
pub mod hstpipicr9;
#[doc = "INTPIPES_HSTPIPICR0_INTPIPES (w) register accessor: Host Pipe Clear Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intpipes_hstpipicr0_intpipes::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpipes_hstpipicr0_intpipes`]
module"]
#[doc(alias = "INTPIPES_HSTPIPICR0_INTPIPES")]
pub type IntpipesHstpipicr0Intpipes =
    crate::Reg<intpipes_hstpipicr0_intpipes::IntpipesHstpipicr0IntpipesSpec>;
#[doc = "Host Pipe Clear Register (n = 0)"]
pub mod intpipes_hstpipicr0_intpipes;
#[doc = "ISOPIPES_HSTPIPICR0_ISOPIPES (w) register accessor: Host Pipe Clear Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isopipes_hstpipicr0_isopipes::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isopipes_hstpipicr0_isopipes`]
module"]
#[doc(alias = "ISOPIPES_HSTPIPICR0_ISOPIPES")]
pub type IsopipesHstpipicr0Isopipes =
    crate::Reg<isopipes_hstpipicr0_isopipes::IsopipesHstpipicr0IsopipesSpec>;
#[doc = "Host Pipe Clear Register (n = 0)"]
pub mod isopipes_hstpipicr0_isopipes;
#[doc = "HSTPIPIFR0 (w) register accessor: Host Pipe Set Register (n = 0) 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipifr0`]
module"]
#[doc(alias = "HSTPIPIFR0")]
pub type Hstpipifr0 = crate::Reg<hstpipifr0::Hstpipifr0Spec>;
#[doc = "Host Pipe Set Register (n = 0) 0"]
pub mod hstpipifr0;
#[doc = "HSTPIPIFR1 (w) register accessor: Host Pipe Set Register (n = 0) 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipifr1`]
module"]
#[doc(alias = "HSTPIPIFR1")]
pub type Hstpipifr1 = crate::Reg<hstpipifr1::Hstpipifr1Spec>;
#[doc = "Host Pipe Set Register (n = 0) 1"]
pub mod hstpipifr1;
#[doc = "HSTPIPIFR2 (w) register accessor: Host Pipe Set Register (n = 0) 2\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipifr2`]
module"]
#[doc(alias = "HSTPIPIFR2")]
pub type Hstpipifr2 = crate::Reg<hstpipifr2::Hstpipifr2Spec>;
#[doc = "Host Pipe Set Register (n = 0) 2"]
pub mod hstpipifr2;
#[doc = "HSTPIPIFR3 (w) register accessor: Host Pipe Set Register (n = 0) 3\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipifr3`]
module"]
#[doc(alias = "HSTPIPIFR3")]
pub type Hstpipifr3 = crate::Reg<hstpipifr3::Hstpipifr3Spec>;
#[doc = "Host Pipe Set Register (n = 0) 3"]
pub mod hstpipifr3;
#[doc = "HSTPIPIFR4 (w) register accessor: Host Pipe Set Register (n = 0) 4\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipifr4`]
module"]
#[doc(alias = "HSTPIPIFR4")]
pub type Hstpipifr4 = crate::Reg<hstpipifr4::Hstpipifr4Spec>;
#[doc = "Host Pipe Set Register (n = 0) 4"]
pub mod hstpipifr4;
#[doc = "HSTPIPIFR5 (w) register accessor: Host Pipe Set Register (n = 0) 5\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipifr5`]
module"]
#[doc(alias = "HSTPIPIFR5")]
pub type Hstpipifr5 = crate::Reg<hstpipifr5::Hstpipifr5Spec>;
#[doc = "Host Pipe Set Register (n = 0) 5"]
pub mod hstpipifr5;
#[doc = "HSTPIPIFR6 (w) register accessor: Host Pipe Set Register (n = 0) 6\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipifr6`]
module"]
#[doc(alias = "HSTPIPIFR6")]
pub type Hstpipifr6 = crate::Reg<hstpipifr6::Hstpipifr6Spec>;
#[doc = "Host Pipe Set Register (n = 0) 6"]
pub mod hstpipifr6;
#[doc = "HSTPIPIFR7 (w) register accessor: Host Pipe Set Register (n = 0) 7\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipifr7`]
module"]
#[doc(alias = "HSTPIPIFR7")]
pub type Hstpipifr7 = crate::Reg<hstpipifr7::Hstpipifr7Spec>;
#[doc = "Host Pipe Set Register (n = 0) 7"]
pub mod hstpipifr7;
#[doc = "HSTPIPIFR8 (w) register accessor: Host Pipe Set Register (n = 0) 8\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipifr8`]
module"]
#[doc(alias = "HSTPIPIFR8")]
pub type Hstpipifr8 = crate::Reg<hstpipifr8::Hstpipifr8Spec>;
#[doc = "Host Pipe Set Register (n = 0) 8"]
pub mod hstpipifr8;
#[doc = "HSTPIPIFR9 (w) register accessor: Host Pipe Set Register (n = 0) 9\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr9::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipifr9`]
module"]
#[doc(alias = "HSTPIPIFR9")]
pub type Hstpipifr9 = crate::Reg<hstpipifr9::Hstpipifr9Spec>;
#[doc = "Host Pipe Set Register (n = 0) 9"]
pub mod hstpipifr9;
#[doc = "INTPIPES_HSTPIPIFR0_INTPIPES (w) register accessor: Host Pipe Set Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intpipes_hstpipifr0_intpipes::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpipes_hstpipifr0_intpipes`]
module"]
#[doc(alias = "INTPIPES_HSTPIPIFR0_INTPIPES")]
pub type IntpipesHstpipifr0Intpipes =
    crate::Reg<intpipes_hstpipifr0_intpipes::IntpipesHstpipifr0IntpipesSpec>;
#[doc = "Host Pipe Set Register (n = 0)"]
pub mod intpipes_hstpipifr0_intpipes;
#[doc = "ISOPIPES_HSTPIPIFR0_ISOPIPES (w) register accessor: Host Pipe Set Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isopipes_hstpipifr0_isopipes::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isopipes_hstpipifr0_isopipes`]
module"]
#[doc(alias = "ISOPIPES_HSTPIPIFR0_ISOPIPES")]
pub type IsopipesHstpipifr0Isopipes =
    crate::Reg<isopipes_hstpipifr0_isopipes::IsopipesHstpipifr0IsopipesSpec>;
#[doc = "Host Pipe Set Register (n = 0)"]
pub mod isopipes_hstpipifr0_isopipes;
#[doc = "HSTPIPIMR0 (r) register accessor: Host Pipe Mask Register (n = 0) 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipimr0`]
module"]
#[doc(alias = "HSTPIPIMR0")]
pub type Hstpipimr0 = crate::Reg<hstpipimr0::Hstpipimr0Spec>;
#[doc = "Host Pipe Mask Register (n = 0) 0"]
pub mod hstpipimr0;
#[doc = "HSTPIPIMR1 (r) register accessor: Host Pipe Mask Register (n = 0) 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipimr1`]
module"]
#[doc(alias = "HSTPIPIMR1")]
pub type Hstpipimr1 = crate::Reg<hstpipimr1::Hstpipimr1Spec>;
#[doc = "Host Pipe Mask Register (n = 0) 1"]
pub mod hstpipimr1;
#[doc = "HSTPIPIMR2 (r) register accessor: Host Pipe Mask Register (n = 0) 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipimr2`]
module"]
#[doc(alias = "HSTPIPIMR2")]
pub type Hstpipimr2 = crate::Reg<hstpipimr2::Hstpipimr2Spec>;
#[doc = "Host Pipe Mask Register (n = 0) 2"]
pub mod hstpipimr2;
#[doc = "HSTPIPIMR3 (r) register accessor: Host Pipe Mask Register (n = 0) 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipimr3`]
module"]
#[doc(alias = "HSTPIPIMR3")]
pub type Hstpipimr3 = crate::Reg<hstpipimr3::Hstpipimr3Spec>;
#[doc = "Host Pipe Mask Register (n = 0) 3"]
pub mod hstpipimr3;
#[doc = "HSTPIPIMR4 (r) register accessor: Host Pipe Mask Register (n = 0) 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipimr4`]
module"]
#[doc(alias = "HSTPIPIMR4")]
pub type Hstpipimr4 = crate::Reg<hstpipimr4::Hstpipimr4Spec>;
#[doc = "Host Pipe Mask Register (n = 0) 4"]
pub mod hstpipimr4;
#[doc = "HSTPIPIMR5 (r) register accessor: Host Pipe Mask Register (n = 0) 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipimr5`]
module"]
#[doc(alias = "HSTPIPIMR5")]
pub type Hstpipimr5 = crate::Reg<hstpipimr5::Hstpipimr5Spec>;
#[doc = "Host Pipe Mask Register (n = 0) 5"]
pub mod hstpipimr5;
#[doc = "HSTPIPIMR6 (r) register accessor: Host Pipe Mask Register (n = 0) 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipimr6`]
module"]
#[doc(alias = "HSTPIPIMR6")]
pub type Hstpipimr6 = crate::Reg<hstpipimr6::Hstpipimr6Spec>;
#[doc = "Host Pipe Mask Register (n = 0) 6"]
pub mod hstpipimr6;
#[doc = "HSTPIPIMR7 (r) register accessor: Host Pipe Mask Register (n = 0) 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipimr7`]
module"]
#[doc(alias = "HSTPIPIMR7")]
pub type Hstpipimr7 = crate::Reg<hstpipimr7::Hstpipimr7Spec>;
#[doc = "Host Pipe Mask Register (n = 0) 7"]
pub mod hstpipimr7;
#[doc = "HSTPIPIMR8 (r) register accessor: Host Pipe Mask Register (n = 0) 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipimr8`]
module"]
#[doc(alias = "HSTPIPIMR8")]
pub type Hstpipimr8 = crate::Reg<hstpipimr8::Hstpipimr8Spec>;
#[doc = "Host Pipe Mask Register (n = 0) 8"]
pub mod hstpipimr8;
#[doc = "HSTPIPIMR9 (r) register accessor: Host Pipe Mask Register (n = 0) 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipimr9`]
module"]
#[doc(alias = "HSTPIPIMR9")]
pub type Hstpipimr9 = crate::Reg<hstpipimr9::Hstpipimr9Spec>;
#[doc = "Host Pipe Mask Register (n = 0) 9"]
pub mod hstpipimr9;
#[doc = "INTPIPES_HSTPIPIMR0_INTPIPES (r) register accessor: Host Pipe Mask Register (n = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intpipes_hstpipimr0_intpipes::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpipes_hstpipimr0_intpipes`]
module"]
#[doc(alias = "INTPIPES_HSTPIPIMR0_INTPIPES")]
pub type IntpipesHstpipimr0Intpipes =
    crate::Reg<intpipes_hstpipimr0_intpipes::IntpipesHstpipimr0IntpipesSpec>;
#[doc = "Host Pipe Mask Register (n = 0)"]
pub mod intpipes_hstpipimr0_intpipes;
#[doc = "ISOPIPES_HSTPIPIMR0_ISOPIPES (r) register accessor: Host Pipe Mask Register (n = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isopipes_hstpipimr0_isopipes::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isopipes_hstpipimr0_isopipes`]
module"]
#[doc(alias = "ISOPIPES_HSTPIPIMR0_ISOPIPES")]
pub type IsopipesHstpipimr0Isopipes =
    crate::Reg<isopipes_hstpipimr0_isopipes::IsopipesHstpipimr0IsopipesSpec>;
#[doc = "Host Pipe Mask Register (n = 0)"]
pub mod isopipes_hstpipimr0_isopipes;
#[doc = "HSTPIPIER0 (w) register accessor: Host Pipe Enable Register (n = 0) 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipier0`]
module"]
#[doc(alias = "HSTPIPIER0")]
pub type Hstpipier0 = crate::Reg<hstpipier0::Hstpipier0Spec>;
#[doc = "Host Pipe Enable Register (n = 0) 0"]
pub mod hstpipier0;
#[doc = "HSTPIPIER1 (w) register accessor: Host Pipe Enable Register (n = 0) 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipier1`]
module"]
#[doc(alias = "HSTPIPIER1")]
pub type Hstpipier1 = crate::Reg<hstpipier1::Hstpipier1Spec>;
#[doc = "Host Pipe Enable Register (n = 0) 1"]
pub mod hstpipier1;
#[doc = "HSTPIPIER2 (w) register accessor: Host Pipe Enable Register (n = 0) 2\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipier2`]
module"]
#[doc(alias = "HSTPIPIER2")]
pub type Hstpipier2 = crate::Reg<hstpipier2::Hstpipier2Spec>;
#[doc = "Host Pipe Enable Register (n = 0) 2"]
pub mod hstpipier2;
#[doc = "HSTPIPIER3 (w) register accessor: Host Pipe Enable Register (n = 0) 3\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipier3`]
module"]
#[doc(alias = "HSTPIPIER3")]
pub type Hstpipier3 = crate::Reg<hstpipier3::Hstpipier3Spec>;
#[doc = "Host Pipe Enable Register (n = 0) 3"]
pub mod hstpipier3;
#[doc = "HSTPIPIER4 (w) register accessor: Host Pipe Enable Register (n = 0) 4\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipier4`]
module"]
#[doc(alias = "HSTPIPIER4")]
pub type Hstpipier4 = crate::Reg<hstpipier4::Hstpipier4Spec>;
#[doc = "Host Pipe Enable Register (n = 0) 4"]
pub mod hstpipier4;
#[doc = "HSTPIPIER5 (w) register accessor: Host Pipe Enable Register (n = 0) 5\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipier5`]
module"]
#[doc(alias = "HSTPIPIER5")]
pub type Hstpipier5 = crate::Reg<hstpipier5::Hstpipier5Spec>;
#[doc = "Host Pipe Enable Register (n = 0) 5"]
pub mod hstpipier5;
#[doc = "HSTPIPIER6 (w) register accessor: Host Pipe Enable Register (n = 0) 6\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipier6`]
module"]
#[doc(alias = "HSTPIPIER6")]
pub type Hstpipier6 = crate::Reg<hstpipier6::Hstpipier6Spec>;
#[doc = "Host Pipe Enable Register (n = 0) 6"]
pub mod hstpipier6;
#[doc = "HSTPIPIER7 (w) register accessor: Host Pipe Enable Register (n = 0) 7\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipier7`]
module"]
#[doc(alias = "HSTPIPIER7")]
pub type Hstpipier7 = crate::Reg<hstpipier7::Hstpipier7Spec>;
#[doc = "Host Pipe Enable Register (n = 0) 7"]
pub mod hstpipier7;
#[doc = "HSTPIPIER8 (w) register accessor: Host Pipe Enable Register (n = 0) 8\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipier8`]
module"]
#[doc(alias = "HSTPIPIER8")]
pub type Hstpipier8 = crate::Reg<hstpipier8::Hstpipier8Spec>;
#[doc = "Host Pipe Enable Register (n = 0) 8"]
pub mod hstpipier8;
#[doc = "HSTPIPIER9 (w) register accessor: Host Pipe Enable Register (n = 0) 9\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier9::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipier9`]
module"]
#[doc(alias = "HSTPIPIER9")]
pub type Hstpipier9 = crate::Reg<hstpipier9::Hstpipier9Spec>;
#[doc = "Host Pipe Enable Register (n = 0) 9"]
pub mod hstpipier9;
#[doc = "INTPIPES_HSTPIPIER0_INTPIPES (w) register accessor: Host Pipe Enable Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intpipes_hstpipier0_intpipes::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpipes_hstpipier0_intpipes`]
module"]
#[doc(alias = "INTPIPES_HSTPIPIER0_INTPIPES")]
pub type IntpipesHstpipier0Intpipes =
    crate::Reg<intpipes_hstpipier0_intpipes::IntpipesHstpipier0IntpipesSpec>;
#[doc = "Host Pipe Enable Register (n = 0)"]
pub mod intpipes_hstpipier0_intpipes;
#[doc = "ISOPIPES_HSTPIPIER0_ISOPIPES (w) register accessor: Host Pipe Enable Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isopipes_hstpipier0_isopipes::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isopipes_hstpipier0_isopipes`]
module"]
#[doc(alias = "ISOPIPES_HSTPIPIER0_ISOPIPES")]
pub type IsopipesHstpipier0Isopipes =
    crate::Reg<isopipes_hstpipier0_isopipes::IsopipesHstpipier0IsopipesSpec>;
#[doc = "Host Pipe Enable Register (n = 0)"]
pub mod isopipes_hstpipier0_isopipes;
#[doc = "HSTPIPIDR0 (w) register accessor: Host Pipe Disable Register (n = 0) 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipidr0`]
module"]
#[doc(alias = "HSTPIPIDR0")]
pub type Hstpipidr0 = crate::Reg<hstpipidr0::Hstpipidr0Spec>;
#[doc = "Host Pipe Disable Register (n = 0) 0"]
pub mod hstpipidr0;
#[doc = "HSTPIPIDR1 (w) register accessor: Host Pipe Disable Register (n = 0) 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipidr1`]
module"]
#[doc(alias = "HSTPIPIDR1")]
pub type Hstpipidr1 = crate::Reg<hstpipidr1::Hstpipidr1Spec>;
#[doc = "Host Pipe Disable Register (n = 0) 1"]
pub mod hstpipidr1;
#[doc = "HSTPIPIDR2 (w) register accessor: Host Pipe Disable Register (n = 0) 2\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipidr2`]
module"]
#[doc(alias = "HSTPIPIDR2")]
pub type Hstpipidr2 = crate::Reg<hstpipidr2::Hstpipidr2Spec>;
#[doc = "Host Pipe Disable Register (n = 0) 2"]
pub mod hstpipidr2;
#[doc = "HSTPIPIDR3 (w) register accessor: Host Pipe Disable Register (n = 0) 3\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipidr3`]
module"]
#[doc(alias = "HSTPIPIDR3")]
pub type Hstpipidr3 = crate::Reg<hstpipidr3::Hstpipidr3Spec>;
#[doc = "Host Pipe Disable Register (n = 0) 3"]
pub mod hstpipidr3;
#[doc = "HSTPIPIDR4 (w) register accessor: Host Pipe Disable Register (n = 0) 4\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipidr4`]
module"]
#[doc(alias = "HSTPIPIDR4")]
pub type Hstpipidr4 = crate::Reg<hstpipidr4::Hstpipidr4Spec>;
#[doc = "Host Pipe Disable Register (n = 0) 4"]
pub mod hstpipidr4;
#[doc = "HSTPIPIDR5 (w) register accessor: Host Pipe Disable Register (n = 0) 5\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipidr5`]
module"]
#[doc(alias = "HSTPIPIDR5")]
pub type Hstpipidr5 = crate::Reg<hstpipidr5::Hstpipidr5Spec>;
#[doc = "Host Pipe Disable Register (n = 0) 5"]
pub mod hstpipidr5;
#[doc = "HSTPIPIDR6 (w) register accessor: Host Pipe Disable Register (n = 0) 6\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipidr6`]
module"]
#[doc(alias = "HSTPIPIDR6")]
pub type Hstpipidr6 = crate::Reg<hstpipidr6::Hstpipidr6Spec>;
#[doc = "Host Pipe Disable Register (n = 0) 6"]
pub mod hstpipidr6;
#[doc = "HSTPIPIDR7 (w) register accessor: Host Pipe Disable Register (n = 0) 7\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipidr7`]
module"]
#[doc(alias = "HSTPIPIDR7")]
pub type Hstpipidr7 = crate::Reg<hstpipidr7::Hstpipidr7Spec>;
#[doc = "Host Pipe Disable Register (n = 0) 7"]
pub mod hstpipidr7;
#[doc = "HSTPIPIDR8 (w) register accessor: Host Pipe Disable Register (n = 0) 8\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipidr8`]
module"]
#[doc(alias = "HSTPIPIDR8")]
pub type Hstpipidr8 = crate::Reg<hstpipidr8::Hstpipidr8Spec>;
#[doc = "Host Pipe Disable Register (n = 0) 8"]
pub mod hstpipidr8;
#[doc = "HSTPIPIDR9 (w) register accessor: Host Pipe Disable Register (n = 0) 9\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr9::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipidr9`]
module"]
#[doc(alias = "HSTPIPIDR9")]
pub type Hstpipidr9 = crate::Reg<hstpipidr9::Hstpipidr9Spec>;
#[doc = "Host Pipe Disable Register (n = 0) 9"]
pub mod hstpipidr9;
#[doc = "INTPIPES_HSTPIPIDR0_INTPIPES (w) register accessor: Host Pipe Disable Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intpipes_hstpipidr0_intpipes::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpipes_hstpipidr0_intpipes`]
module"]
#[doc(alias = "INTPIPES_HSTPIPIDR0_INTPIPES")]
pub type IntpipesHstpipidr0Intpipes =
    crate::Reg<intpipes_hstpipidr0_intpipes::IntpipesHstpipidr0IntpipesSpec>;
#[doc = "Host Pipe Disable Register (n = 0)"]
pub mod intpipes_hstpipidr0_intpipes;
#[doc = "ISOPIPES_HSTPIPIDR0_ISOPIPES (w) register accessor: Host Pipe Disable Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isopipes_hstpipidr0_isopipes::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isopipes_hstpipidr0_isopipes`]
module"]
#[doc(alias = "ISOPIPES_HSTPIPIDR0_ISOPIPES")]
pub type IsopipesHstpipidr0Isopipes =
    crate::Reg<isopipes_hstpipidr0_isopipes::IsopipesHstpipidr0IsopipesSpec>;
#[doc = "Host Pipe Disable Register (n = 0)"]
pub mod isopipes_hstpipidr0_isopipes;
#[doc = "HSTPIPINRQ0 (rw) register accessor: Host Pipe IN Request Register (n = 0) 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipinrq0`]
module"]
#[doc(alias = "HSTPIPINRQ0")]
pub type Hstpipinrq0 = crate::Reg<hstpipinrq0::Hstpipinrq0Spec>;
#[doc = "Host Pipe IN Request Register (n = 0) 0"]
pub mod hstpipinrq0;
#[doc = "HSTPIPINRQ1 (rw) register accessor: Host Pipe IN Request Register (n = 0) 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipinrq1`]
module"]
#[doc(alias = "HSTPIPINRQ1")]
pub type Hstpipinrq1 = crate::Reg<hstpipinrq1::Hstpipinrq1Spec>;
#[doc = "Host Pipe IN Request Register (n = 0) 1"]
pub mod hstpipinrq1;
#[doc = "HSTPIPINRQ2 (rw) register accessor: Host Pipe IN Request Register (n = 0) 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipinrq2`]
module"]
#[doc(alias = "HSTPIPINRQ2")]
pub type Hstpipinrq2 = crate::Reg<hstpipinrq2::Hstpipinrq2Spec>;
#[doc = "Host Pipe IN Request Register (n = 0) 2"]
pub mod hstpipinrq2;
#[doc = "HSTPIPINRQ3 (rw) register accessor: Host Pipe IN Request Register (n = 0) 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipinrq3`]
module"]
#[doc(alias = "HSTPIPINRQ3")]
pub type Hstpipinrq3 = crate::Reg<hstpipinrq3::Hstpipinrq3Spec>;
#[doc = "Host Pipe IN Request Register (n = 0) 3"]
pub mod hstpipinrq3;
#[doc = "HSTPIPINRQ4 (rw) register accessor: Host Pipe IN Request Register (n = 0) 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipinrq4`]
module"]
#[doc(alias = "HSTPIPINRQ4")]
pub type Hstpipinrq4 = crate::Reg<hstpipinrq4::Hstpipinrq4Spec>;
#[doc = "Host Pipe IN Request Register (n = 0) 4"]
pub mod hstpipinrq4;
#[doc = "HSTPIPINRQ5 (rw) register accessor: Host Pipe IN Request Register (n = 0) 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipinrq5`]
module"]
#[doc(alias = "HSTPIPINRQ5")]
pub type Hstpipinrq5 = crate::Reg<hstpipinrq5::Hstpipinrq5Spec>;
#[doc = "Host Pipe IN Request Register (n = 0) 5"]
pub mod hstpipinrq5;
#[doc = "HSTPIPINRQ6 (rw) register accessor: Host Pipe IN Request Register (n = 0) 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq6::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipinrq6`]
module"]
#[doc(alias = "HSTPIPINRQ6")]
pub type Hstpipinrq6 = crate::Reg<hstpipinrq6::Hstpipinrq6Spec>;
#[doc = "Host Pipe IN Request Register (n = 0) 6"]
pub mod hstpipinrq6;
#[doc = "HSTPIPINRQ7 (rw) register accessor: Host Pipe IN Request Register (n = 0) 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq7::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipinrq7`]
module"]
#[doc(alias = "HSTPIPINRQ7")]
pub type Hstpipinrq7 = crate::Reg<hstpipinrq7::Hstpipinrq7Spec>;
#[doc = "Host Pipe IN Request Register (n = 0) 7"]
pub mod hstpipinrq7;
#[doc = "HSTPIPINRQ8 (rw) register accessor: Host Pipe IN Request Register (n = 0) 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq8::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipinrq8`]
module"]
#[doc(alias = "HSTPIPINRQ8")]
pub type Hstpipinrq8 = crate::Reg<hstpipinrq8::Hstpipinrq8Spec>;
#[doc = "Host Pipe IN Request Register (n = 0) 8"]
pub mod hstpipinrq8;
#[doc = "HSTPIPINRQ9 (rw) register accessor: Host Pipe IN Request Register (n = 0) 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq9::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpipinrq9`]
module"]
#[doc(alias = "HSTPIPINRQ9")]
pub type Hstpipinrq9 = crate::Reg<hstpipinrq9::Hstpipinrq9Spec>;
#[doc = "Host Pipe IN Request Register (n = 0) 9"]
pub mod hstpipinrq9;
#[doc = "HSTPIPERR0 (rw) register accessor: Host Pipe Error Register (n = 0) 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpiperr0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpiperr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpiperr0`]
module"]
#[doc(alias = "HSTPIPERR0")]
pub type Hstpiperr0 = crate::Reg<hstpiperr0::Hstpiperr0Spec>;
#[doc = "Host Pipe Error Register (n = 0) 0"]
pub mod hstpiperr0;
#[doc = "HSTPIPERR1 (rw) register accessor: Host Pipe Error Register (n = 0) 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpiperr1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpiperr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpiperr1`]
module"]
#[doc(alias = "HSTPIPERR1")]
pub type Hstpiperr1 = crate::Reg<hstpiperr1::Hstpiperr1Spec>;
#[doc = "Host Pipe Error Register (n = 0) 1"]
pub mod hstpiperr1;
#[doc = "HSTPIPERR2 (rw) register accessor: Host Pipe Error Register (n = 0) 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpiperr2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpiperr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpiperr2`]
module"]
#[doc(alias = "HSTPIPERR2")]
pub type Hstpiperr2 = crate::Reg<hstpiperr2::Hstpiperr2Spec>;
#[doc = "Host Pipe Error Register (n = 0) 2"]
pub mod hstpiperr2;
#[doc = "HSTPIPERR3 (rw) register accessor: Host Pipe Error Register (n = 0) 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpiperr3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpiperr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpiperr3`]
module"]
#[doc(alias = "HSTPIPERR3")]
pub type Hstpiperr3 = crate::Reg<hstpiperr3::Hstpiperr3Spec>;
#[doc = "Host Pipe Error Register (n = 0) 3"]
pub mod hstpiperr3;
#[doc = "HSTPIPERR4 (rw) register accessor: Host Pipe Error Register (n = 0) 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpiperr4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpiperr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpiperr4`]
module"]
#[doc(alias = "HSTPIPERR4")]
pub type Hstpiperr4 = crate::Reg<hstpiperr4::Hstpiperr4Spec>;
#[doc = "Host Pipe Error Register (n = 0) 4"]
pub mod hstpiperr4;
#[doc = "HSTPIPERR5 (rw) register accessor: Host Pipe Error Register (n = 0) 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpiperr5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpiperr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpiperr5`]
module"]
#[doc(alias = "HSTPIPERR5")]
pub type Hstpiperr5 = crate::Reg<hstpiperr5::Hstpiperr5Spec>;
#[doc = "Host Pipe Error Register (n = 0) 5"]
pub mod hstpiperr5;
#[doc = "HSTPIPERR6 (rw) register accessor: Host Pipe Error Register (n = 0) 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpiperr6::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpiperr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpiperr6`]
module"]
#[doc(alias = "HSTPIPERR6")]
pub type Hstpiperr6 = crate::Reg<hstpiperr6::Hstpiperr6Spec>;
#[doc = "Host Pipe Error Register (n = 0) 6"]
pub mod hstpiperr6;
#[doc = "HSTPIPERR7 (rw) register accessor: Host Pipe Error Register (n = 0) 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpiperr7::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpiperr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpiperr7`]
module"]
#[doc(alias = "HSTPIPERR7")]
pub type Hstpiperr7 = crate::Reg<hstpiperr7::Hstpiperr7Spec>;
#[doc = "Host Pipe Error Register (n = 0) 7"]
pub mod hstpiperr7;
#[doc = "HSTPIPERR8 (rw) register accessor: Host Pipe Error Register (n = 0) 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpiperr8::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpiperr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpiperr8`]
module"]
#[doc(alias = "HSTPIPERR8")]
pub type Hstpiperr8 = crate::Reg<hstpiperr8::Hstpiperr8Spec>;
#[doc = "Host Pipe Error Register (n = 0) 8"]
pub mod hstpiperr8;
#[doc = "HSTPIPERR9 (rw) register accessor: Host Pipe Error Register (n = 0) 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpiperr9::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpiperr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpiperr9`]
module"]
#[doc(alias = "HSTPIPERR9")]
pub type Hstpiperr9 = crate::Reg<hstpiperr9::Hstpiperr9Spec>;
#[doc = "Host Pipe Error Register (n = 0) 9"]
pub mod hstpiperr9;
#[doc = "HSTDMANXTDSC1 (rw) register accessor: Host DMA Channel Next Descriptor Address Register (n = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmanxtdsc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmanxtdsc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmanxtdsc1`]
module"]
#[doc(alias = "HSTDMANXTDSC1")]
pub type Hstdmanxtdsc1 = crate::Reg<hstdmanxtdsc1::Hstdmanxtdsc1Spec>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod hstdmanxtdsc1;
#[doc = "HSTDMAADDRESS1 (rw) register accessor: Host DMA Channel Address Register (n = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmaaddress1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmaaddress1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmaaddress1`]
module"]
#[doc(alias = "HSTDMAADDRESS1")]
pub type Hstdmaaddress1 = crate::Reg<hstdmaaddress1::Hstdmaaddress1Spec>;
#[doc = "Host DMA Channel Address Register (n = 1)"]
pub mod hstdmaaddress1;
#[doc = "HSTDMACONTROL1 (rw) register accessor: Host DMA Channel Control Register (n = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmacontrol1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmacontrol1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmacontrol1`]
module"]
#[doc(alias = "HSTDMACONTROL1")]
pub type Hstdmacontrol1 = crate::Reg<hstdmacontrol1::Hstdmacontrol1Spec>;
#[doc = "Host DMA Channel Control Register (n = 1)"]
pub mod hstdmacontrol1;
#[doc = "HSTDMASTATUS1 (rw) register accessor: Host DMA Channel Status Register (n = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmastatus1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmastatus1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmastatus1`]
module"]
#[doc(alias = "HSTDMASTATUS1")]
pub type Hstdmastatus1 = crate::Reg<hstdmastatus1::Hstdmastatus1Spec>;
#[doc = "Host DMA Channel Status Register (n = 1)"]
pub mod hstdmastatus1;
#[doc = "HSTDMANXTDSC2 (rw) register accessor: Host DMA Channel Next Descriptor Address Register (n = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmanxtdsc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmanxtdsc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmanxtdsc2`]
module"]
#[doc(alias = "HSTDMANXTDSC2")]
pub type Hstdmanxtdsc2 = crate::Reg<hstdmanxtdsc2::Hstdmanxtdsc2Spec>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 2)"]
pub mod hstdmanxtdsc2;
#[doc = "HSTDMAADDRESS2 (rw) register accessor: Host DMA Channel Address Register (n = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmaaddress2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmaaddress2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmaaddress2`]
module"]
#[doc(alias = "HSTDMAADDRESS2")]
pub type Hstdmaaddress2 = crate::Reg<hstdmaaddress2::Hstdmaaddress2Spec>;
#[doc = "Host DMA Channel Address Register (n = 2)"]
pub mod hstdmaaddress2;
#[doc = "HSTDMACONTROL2 (rw) register accessor: Host DMA Channel Control Register (n = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmacontrol2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmacontrol2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmacontrol2`]
module"]
#[doc(alias = "HSTDMACONTROL2")]
pub type Hstdmacontrol2 = crate::Reg<hstdmacontrol2::Hstdmacontrol2Spec>;
#[doc = "Host DMA Channel Control Register (n = 2)"]
pub mod hstdmacontrol2;
#[doc = "HSTDMASTATUS2 (rw) register accessor: Host DMA Channel Status Register (n = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmastatus2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmastatus2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmastatus2`]
module"]
#[doc(alias = "HSTDMASTATUS2")]
pub type Hstdmastatus2 = crate::Reg<hstdmastatus2::Hstdmastatus2Spec>;
#[doc = "Host DMA Channel Status Register (n = 2)"]
pub mod hstdmastatus2;
#[doc = "HSTDMANXTDSC3 (rw) register accessor: Host DMA Channel Next Descriptor Address Register (n = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmanxtdsc3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmanxtdsc3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmanxtdsc3`]
module"]
#[doc(alias = "HSTDMANXTDSC3")]
pub type Hstdmanxtdsc3 = crate::Reg<hstdmanxtdsc3::Hstdmanxtdsc3Spec>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 3)"]
pub mod hstdmanxtdsc3;
#[doc = "HSTDMAADDRESS3 (rw) register accessor: Host DMA Channel Address Register (n = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmaaddress3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmaaddress3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmaaddress3`]
module"]
#[doc(alias = "HSTDMAADDRESS3")]
pub type Hstdmaaddress3 = crate::Reg<hstdmaaddress3::Hstdmaaddress3Spec>;
#[doc = "Host DMA Channel Address Register (n = 3)"]
pub mod hstdmaaddress3;
#[doc = "HSTDMACONTROL3 (rw) register accessor: Host DMA Channel Control Register (n = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmacontrol3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmacontrol3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmacontrol3`]
module"]
#[doc(alias = "HSTDMACONTROL3")]
pub type Hstdmacontrol3 = crate::Reg<hstdmacontrol3::Hstdmacontrol3Spec>;
#[doc = "Host DMA Channel Control Register (n = 3)"]
pub mod hstdmacontrol3;
#[doc = "HSTDMASTATUS3 (rw) register accessor: Host DMA Channel Status Register (n = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmastatus3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmastatus3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmastatus3`]
module"]
#[doc(alias = "HSTDMASTATUS3")]
pub type Hstdmastatus3 = crate::Reg<hstdmastatus3::Hstdmastatus3Spec>;
#[doc = "Host DMA Channel Status Register (n = 3)"]
pub mod hstdmastatus3;
#[doc = "HSTDMANXTDSC4 (rw) register accessor: Host DMA Channel Next Descriptor Address Register (n = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmanxtdsc4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmanxtdsc4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmanxtdsc4`]
module"]
#[doc(alias = "HSTDMANXTDSC4")]
pub type Hstdmanxtdsc4 = crate::Reg<hstdmanxtdsc4::Hstdmanxtdsc4Spec>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 4)"]
pub mod hstdmanxtdsc4;
#[doc = "HSTDMAADDRESS4 (rw) register accessor: Host DMA Channel Address Register (n = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmaaddress4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmaaddress4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmaaddress4`]
module"]
#[doc(alias = "HSTDMAADDRESS4")]
pub type Hstdmaaddress4 = crate::Reg<hstdmaaddress4::Hstdmaaddress4Spec>;
#[doc = "Host DMA Channel Address Register (n = 4)"]
pub mod hstdmaaddress4;
#[doc = "HSTDMACONTROL4 (rw) register accessor: Host DMA Channel Control Register (n = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmacontrol4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmacontrol4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmacontrol4`]
module"]
#[doc(alias = "HSTDMACONTROL4")]
pub type Hstdmacontrol4 = crate::Reg<hstdmacontrol4::Hstdmacontrol4Spec>;
#[doc = "Host DMA Channel Control Register (n = 4)"]
pub mod hstdmacontrol4;
#[doc = "HSTDMASTATUS4 (rw) register accessor: Host DMA Channel Status Register (n = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmastatus4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmastatus4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmastatus4`]
module"]
#[doc(alias = "HSTDMASTATUS4")]
pub type Hstdmastatus4 = crate::Reg<hstdmastatus4::Hstdmastatus4Spec>;
#[doc = "Host DMA Channel Status Register (n = 4)"]
pub mod hstdmastatus4;
#[doc = "HSTDMANXTDSC5 (rw) register accessor: Host DMA Channel Next Descriptor Address Register (n = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmanxtdsc5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmanxtdsc5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmanxtdsc5`]
module"]
#[doc(alias = "HSTDMANXTDSC5")]
pub type Hstdmanxtdsc5 = crate::Reg<hstdmanxtdsc5::Hstdmanxtdsc5Spec>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 5)"]
pub mod hstdmanxtdsc5;
#[doc = "HSTDMAADDRESS5 (rw) register accessor: Host DMA Channel Address Register (n = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmaaddress5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmaaddress5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmaaddress5`]
module"]
#[doc(alias = "HSTDMAADDRESS5")]
pub type Hstdmaaddress5 = crate::Reg<hstdmaaddress5::Hstdmaaddress5Spec>;
#[doc = "Host DMA Channel Address Register (n = 5)"]
pub mod hstdmaaddress5;
#[doc = "HSTDMACONTROL5 (rw) register accessor: Host DMA Channel Control Register (n = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmacontrol5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmacontrol5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmacontrol5`]
module"]
#[doc(alias = "HSTDMACONTROL5")]
pub type Hstdmacontrol5 = crate::Reg<hstdmacontrol5::Hstdmacontrol5Spec>;
#[doc = "Host DMA Channel Control Register (n = 5)"]
pub mod hstdmacontrol5;
#[doc = "HSTDMASTATUS5 (rw) register accessor: Host DMA Channel Status Register (n = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmastatus5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmastatus5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmastatus5`]
module"]
#[doc(alias = "HSTDMASTATUS5")]
pub type Hstdmastatus5 = crate::Reg<hstdmastatus5::Hstdmastatus5Spec>;
#[doc = "Host DMA Channel Status Register (n = 5)"]
pub mod hstdmastatus5;
#[doc = "HSTDMANXTDSC6 (rw) register accessor: Host DMA Channel Next Descriptor Address Register (n = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmanxtdsc6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmanxtdsc6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmanxtdsc6`]
module"]
#[doc(alias = "HSTDMANXTDSC6")]
pub type Hstdmanxtdsc6 = crate::Reg<hstdmanxtdsc6::Hstdmanxtdsc6Spec>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 6)"]
pub mod hstdmanxtdsc6;
#[doc = "HSTDMAADDRESS6 (rw) register accessor: Host DMA Channel Address Register (n = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmaaddress6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmaaddress6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmaaddress6`]
module"]
#[doc(alias = "HSTDMAADDRESS6")]
pub type Hstdmaaddress6 = crate::Reg<hstdmaaddress6::Hstdmaaddress6Spec>;
#[doc = "Host DMA Channel Address Register (n = 6)"]
pub mod hstdmaaddress6;
#[doc = "HSTDMACONTROL6 (rw) register accessor: Host DMA Channel Control Register (n = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmacontrol6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmacontrol6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmacontrol6`]
module"]
#[doc(alias = "HSTDMACONTROL6")]
pub type Hstdmacontrol6 = crate::Reg<hstdmacontrol6::Hstdmacontrol6Spec>;
#[doc = "Host DMA Channel Control Register (n = 6)"]
pub mod hstdmacontrol6;
#[doc = "HSTDMASTATUS6 (rw) register accessor: Host DMA Channel Status Register (n = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmastatus6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmastatus6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmastatus6`]
module"]
#[doc(alias = "HSTDMASTATUS6")]
pub type Hstdmastatus6 = crate::Reg<hstdmastatus6::Hstdmastatus6Spec>;
#[doc = "Host DMA Channel Status Register (n = 6)"]
pub mod hstdmastatus6;
#[doc = "HSTDMANXTDSC7 (rw) register accessor: Host DMA Channel Next Descriptor Address Register (n = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmanxtdsc7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmanxtdsc7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmanxtdsc7`]
module"]
#[doc(alias = "HSTDMANXTDSC7")]
pub type Hstdmanxtdsc7 = crate::Reg<hstdmanxtdsc7::Hstdmanxtdsc7Spec>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 7)"]
pub mod hstdmanxtdsc7;
#[doc = "HSTDMAADDRESS7 (rw) register accessor: Host DMA Channel Address Register (n = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmaaddress7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmaaddress7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmaaddress7`]
module"]
#[doc(alias = "HSTDMAADDRESS7")]
pub type Hstdmaaddress7 = crate::Reg<hstdmaaddress7::Hstdmaaddress7Spec>;
#[doc = "Host DMA Channel Address Register (n = 7)"]
pub mod hstdmaaddress7;
#[doc = "HSTDMACONTROL7 (rw) register accessor: Host DMA Channel Control Register (n = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmacontrol7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmacontrol7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmacontrol7`]
module"]
#[doc(alias = "HSTDMACONTROL7")]
pub type Hstdmacontrol7 = crate::Reg<hstdmacontrol7::Hstdmacontrol7Spec>;
#[doc = "Host DMA Channel Control Register (n = 7)"]
pub mod hstdmacontrol7;
#[doc = "HSTDMASTATUS7 (rw) register accessor: Host DMA Channel Status Register (n = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmastatus7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmastatus7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstdmastatus7`]
module"]
#[doc(alias = "HSTDMASTATUS7")]
pub type Hstdmastatus7 = crate::Reg<hstdmastatus7::Hstdmastatus7Spec>;
#[doc = "Host DMA Channel Status Register (n = 7)"]
pub mod hstdmastatus7;
#[doc = "CTRL (rw) register accessor: General Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "General Control Register"]
pub mod ctrl;
#[doc = "SR (r) register accessor: General Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "General Status Register"]
pub mod sr;
#[doc = "SCR (w) register accessor: General Status Clear Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "General Status Clear Register"]
pub mod scr;
#[doc = "SFR (w) register accessor: General Status Set Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr`]
module"]
#[doc(alias = "SFR")]
pub type Sfr = crate::Reg<sfr::SfrSpec>;
#[doc = "General Status Set Register"]
pub mod sfr;
#[doc = "FSM (r) register accessor: General Finite State Machine Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm`]
module"]
#[doc(alias = "FSM")]
pub type Fsm = crate::Reg<fsm::FsmSpec>;
#[doc = "General Finite State Machine Register"]
pub mod fsm;
