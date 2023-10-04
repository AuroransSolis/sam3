#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device General Control Register"]
    pub devctrl: DEVCTRL,
    #[doc = "0x04 - Device Global Interrupt Status Register"]
    pub devisr: DEVISR,
    #[doc = "0x08 - Device Global Interrupt Clear Register"]
    pub devicr: DEVICR,
    #[doc = "0x0c - Device Global Interrupt Set Register"]
    pub devifr: DEVIFR,
    #[doc = "0x10 - Device Global Interrupt Mask Register"]
    pub devimr: DEVIMR,
    #[doc = "0x14 - Device Global Interrupt Disable Register"]
    pub devidr: DEVIDR,
    #[doc = "0x18 - Device Global Interrupt Enable Register"]
    pub devier: DEVIER,
    #[doc = "0x1c - Device Endpoint Register"]
    pub devept: DEVEPT,
    #[doc = "0x20 - Device Frame Number Register"]
    pub devfnum: DEVFNUM,
    _reserved9: [u8; 0xdc],
    #[doc = "0x100 - Device Endpoint Configuration Register (n = 0) 0"]
    pub deveptcfg0: DEVEPTCFG0,
    #[doc = "0x104 - Device Endpoint Configuration Register (n = 0) 1"]
    pub deveptcfg1: DEVEPTCFG1,
    #[doc = "0x108 - Device Endpoint Configuration Register (n = 0) 2"]
    pub deveptcfg2: DEVEPTCFG2,
    #[doc = "0x10c - Device Endpoint Configuration Register (n = 0) 3"]
    pub deveptcfg3: DEVEPTCFG3,
    #[doc = "0x110 - Device Endpoint Configuration Register (n = 0) 4"]
    pub deveptcfg4: DEVEPTCFG4,
    #[doc = "0x114 - Device Endpoint Configuration Register (n = 0) 5"]
    pub deveptcfg5: DEVEPTCFG5,
    #[doc = "0x118 - Device Endpoint Configuration Register (n = 0) 6"]
    pub deveptcfg6: DEVEPTCFG6,
    #[doc = "0x11c - Device Endpoint Configuration Register (n = 0) 7"]
    pub deveptcfg7: DEVEPTCFG7,
    #[doc = "0x120 - Device Endpoint Configuration Register (n = 0) 8"]
    pub deveptcfg8: DEVEPTCFG8,
    #[doc = "0x124 - Device Endpoint Configuration Register (n = 0) 9"]
    pub deveptcfg9: DEVEPTCFG9,
    _reserved19: [u8; 0x08],
    _reserved_19_deveptisr0: [u8; 0x04],
    #[doc = "0x134 - Device Endpoint Status Register (n = 0) 1"]
    pub deveptisr1: DEVEPTISR1,
    #[doc = "0x138 - Device Endpoint Status Register (n = 0) 2"]
    pub deveptisr2: DEVEPTISR2,
    #[doc = "0x13c - Device Endpoint Status Register (n = 0) 3"]
    pub deveptisr3: DEVEPTISR3,
    #[doc = "0x140 - Device Endpoint Status Register (n = 0) 4"]
    pub deveptisr4: DEVEPTISR4,
    #[doc = "0x144 - Device Endpoint Status Register (n = 0) 5"]
    pub deveptisr5: DEVEPTISR5,
    #[doc = "0x148 - Device Endpoint Status Register (n = 0) 6"]
    pub deveptisr6: DEVEPTISR6,
    #[doc = "0x14c - Device Endpoint Status Register (n = 0) 7"]
    pub deveptisr7: DEVEPTISR7,
    #[doc = "0x150 - Device Endpoint Status Register (n = 0) 8"]
    pub deveptisr8: DEVEPTISR8,
    #[doc = "0x154 - Device Endpoint Status Register (n = 0) 9"]
    pub deveptisr9: DEVEPTISR9,
    _reserved29: [u8; 0x08],
    _reserved_29_devepticr0: [u8; 0x04],
    #[doc = "0x164 - Device Endpoint Clear Register (n = 0) 1"]
    pub devepticr1: DEVEPTICR1,
    #[doc = "0x168 - Device Endpoint Clear Register (n = 0) 2"]
    pub devepticr2: DEVEPTICR2,
    #[doc = "0x16c - Device Endpoint Clear Register (n = 0) 3"]
    pub devepticr3: DEVEPTICR3,
    #[doc = "0x170 - Device Endpoint Clear Register (n = 0) 4"]
    pub devepticr4: DEVEPTICR4,
    #[doc = "0x174 - Device Endpoint Clear Register (n = 0) 5"]
    pub devepticr5: DEVEPTICR5,
    #[doc = "0x178 - Device Endpoint Clear Register (n = 0) 6"]
    pub devepticr6: DEVEPTICR6,
    #[doc = "0x17c - Device Endpoint Clear Register (n = 0) 7"]
    pub devepticr7: DEVEPTICR7,
    #[doc = "0x180 - Device Endpoint Clear Register (n = 0) 8"]
    pub devepticr8: DEVEPTICR8,
    #[doc = "0x184 - Device Endpoint Clear Register (n = 0) 9"]
    pub devepticr9: DEVEPTICR9,
    _reserved39: [u8; 0x08],
    _reserved_39_deveptifr0: [u8; 0x04],
    #[doc = "0x194 - Device Endpoint Set Register (n = 0) 1"]
    pub deveptifr1: DEVEPTIFR1,
    #[doc = "0x198 - Device Endpoint Set Register (n = 0) 2"]
    pub deveptifr2: DEVEPTIFR2,
    #[doc = "0x19c - Device Endpoint Set Register (n = 0) 3"]
    pub deveptifr3: DEVEPTIFR3,
    #[doc = "0x1a0 - Device Endpoint Set Register (n = 0) 4"]
    pub deveptifr4: DEVEPTIFR4,
    #[doc = "0x1a4 - Device Endpoint Set Register (n = 0) 5"]
    pub deveptifr5: DEVEPTIFR5,
    #[doc = "0x1a8 - Device Endpoint Set Register (n = 0) 6"]
    pub deveptifr6: DEVEPTIFR6,
    #[doc = "0x1ac - Device Endpoint Set Register (n = 0) 7"]
    pub deveptifr7: DEVEPTIFR7,
    #[doc = "0x1b0 - Device Endpoint Set Register (n = 0) 8"]
    pub deveptifr8: DEVEPTIFR8,
    #[doc = "0x1b4 - Device Endpoint Set Register (n = 0) 9"]
    pub deveptifr9: DEVEPTIFR9,
    _reserved49: [u8; 0x08],
    _reserved_49_deveptimr0: [u8; 0x04],
    #[doc = "0x1c4 - Device Endpoint Mask Register (n = 0) 1"]
    pub deveptimr1: DEVEPTIMR1,
    #[doc = "0x1c8 - Device Endpoint Mask Register (n = 0) 2"]
    pub deveptimr2: DEVEPTIMR2,
    #[doc = "0x1cc - Device Endpoint Mask Register (n = 0) 3"]
    pub deveptimr3: DEVEPTIMR3,
    #[doc = "0x1d0 - Device Endpoint Mask Register (n = 0) 4"]
    pub deveptimr4: DEVEPTIMR4,
    #[doc = "0x1d4 - Device Endpoint Mask Register (n = 0) 5"]
    pub deveptimr5: DEVEPTIMR5,
    #[doc = "0x1d8 - Device Endpoint Mask Register (n = 0) 6"]
    pub deveptimr6: DEVEPTIMR6,
    #[doc = "0x1dc - Device Endpoint Mask Register (n = 0) 7"]
    pub deveptimr7: DEVEPTIMR7,
    #[doc = "0x1e0 - Device Endpoint Mask Register (n = 0) 8"]
    pub deveptimr8: DEVEPTIMR8,
    #[doc = "0x1e4 - Device Endpoint Mask Register (n = 0) 9"]
    pub deveptimr9: DEVEPTIMR9,
    _reserved59: [u8; 0x08],
    _reserved_59_deveptier0: [u8; 0x04],
    #[doc = "0x1f4 - Device Endpoint Enable Register (n = 0) 1"]
    pub deveptier1: DEVEPTIER1,
    #[doc = "0x1f8 - Device Endpoint Enable Register (n = 0) 2"]
    pub deveptier2: DEVEPTIER2,
    #[doc = "0x1fc - Device Endpoint Enable Register (n = 0) 3"]
    pub deveptier3: DEVEPTIER3,
    #[doc = "0x200 - Device Endpoint Enable Register (n = 0) 4"]
    pub deveptier4: DEVEPTIER4,
    #[doc = "0x204 - Device Endpoint Enable Register (n = 0) 5"]
    pub deveptier5: DEVEPTIER5,
    #[doc = "0x208 - Device Endpoint Enable Register (n = 0) 6"]
    pub deveptier6: DEVEPTIER6,
    #[doc = "0x20c - Device Endpoint Enable Register (n = 0) 7"]
    pub deveptier7: DEVEPTIER7,
    #[doc = "0x210 - Device Endpoint Enable Register (n = 0) 8"]
    pub deveptier8: DEVEPTIER8,
    #[doc = "0x214 - Device Endpoint Enable Register (n = 0) 9"]
    pub deveptier9: DEVEPTIER9,
    _reserved69: [u8; 0x08],
    _reserved_69_deveptidr0: [u8; 0x04],
    #[doc = "0x224 - Device Endpoint Disable Register (n = 0) 1"]
    pub deveptidr1: DEVEPTIDR1,
    #[doc = "0x228 - Device Endpoint Disable Register (n = 0) 2"]
    pub deveptidr2: DEVEPTIDR2,
    #[doc = "0x22c - Device Endpoint Disable Register (n = 0) 3"]
    pub deveptidr3: DEVEPTIDR3,
    #[doc = "0x230 - Device Endpoint Disable Register (n = 0) 4"]
    pub deveptidr4: DEVEPTIDR4,
    #[doc = "0x234 - Device Endpoint Disable Register (n = 0) 5"]
    pub deveptidr5: DEVEPTIDR5,
    #[doc = "0x238 - Device Endpoint Disable Register (n = 0) 6"]
    pub deveptidr6: DEVEPTIDR6,
    #[doc = "0x23c - Device Endpoint Disable Register (n = 0) 7"]
    pub deveptidr7: DEVEPTIDR7,
    #[doc = "0x240 - Device Endpoint Disable Register (n = 0) 8"]
    pub deveptidr8: DEVEPTIDR8,
    #[doc = "0x244 - Device Endpoint Disable Register (n = 0) 9"]
    pub deveptidr9: DEVEPTIDR9,
    _reserved79: [u8; 0xc8],
    #[doc = "0x310 - Device DMA Channel Next Descriptor Address Register (n = 1)"]
    pub devdmanxtdsc1: DEVDMANXTDSC1,
    #[doc = "0x314 - Device DMA Channel Address Register (n = 1)"]
    pub devdmaaddress1: DEVDMAADDRESS1,
    #[doc = "0x318 - Device DMA Channel Control Register (n = 1)"]
    pub devdmacontrol1: DEVDMACONTROL1,
    #[doc = "0x31c - Device DMA Channel Status Register (n = 1)"]
    pub devdmastatus1: DEVDMASTATUS1,
    #[doc = "0x320 - Device DMA Channel Next Descriptor Address Register (n = 2)"]
    pub devdmanxtdsc2: DEVDMANXTDSC2,
    #[doc = "0x324 - Device DMA Channel Address Register (n = 2)"]
    pub devdmaaddress2: DEVDMAADDRESS2,
    #[doc = "0x328 - Device DMA Channel Control Register (n = 2)"]
    pub devdmacontrol2: DEVDMACONTROL2,
    #[doc = "0x32c - Device DMA Channel Status Register (n = 2)"]
    pub devdmastatus2: DEVDMASTATUS2,
    #[doc = "0x330 - Device DMA Channel Next Descriptor Address Register (n = 3)"]
    pub devdmanxtdsc3: DEVDMANXTDSC3,
    #[doc = "0x334 - Device DMA Channel Address Register (n = 3)"]
    pub devdmaaddress3: DEVDMAADDRESS3,
    #[doc = "0x338 - Device DMA Channel Control Register (n = 3)"]
    pub devdmacontrol3: DEVDMACONTROL3,
    #[doc = "0x33c - Device DMA Channel Status Register (n = 3)"]
    pub devdmastatus3: DEVDMASTATUS3,
    #[doc = "0x340 - Device DMA Channel Next Descriptor Address Register (n = 4)"]
    pub devdmanxtdsc4: DEVDMANXTDSC4,
    #[doc = "0x344 - Device DMA Channel Address Register (n = 4)"]
    pub devdmaaddress4: DEVDMAADDRESS4,
    #[doc = "0x348 - Device DMA Channel Control Register (n = 4)"]
    pub devdmacontrol4: DEVDMACONTROL4,
    #[doc = "0x34c - Device DMA Channel Status Register (n = 4)"]
    pub devdmastatus4: DEVDMASTATUS4,
    #[doc = "0x350 - Device DMA Channel Next Descriptor Address Register (n = 5)"]
    pub devdmanxtdsc5: DEVDMANXTDSC5,
    #[doc = "0x354 - Device DMA Channel Address Register (n = 5)"]
    pub devdmaaddress5: DEVDMAADDRESS5,
    #[doc = "0x358 - Device DMA Channel Control Register (n = 5)"]
    pub devdmacontrol5: DEVDMACONTROL5,
    #[doc = "0x35c - Device DMA Channel Status Register (n = 5)"]
    pub devdmastatus5: DEVDMASTATUS5,
    #[doc = "0x360 - Device DMA Channel Next Descriptor Address Register (n = 6)"]
    pub devdmanxtdsc6: DEVDMANXTDSC6,
    #[doc = "0x364 - Device DMA Channel Address Register (n = 6)"]
    pub devdmaaddress6: DEVDMAADDRESS6,
    #[doc = "0x368 - Device DMA Channel Control Register (n = 6)"]
    pub devdmacontrol6: DEVDMACONTROL6,
    #[doc = "0x36c - Device DMA Channel Status Register (n = 6)"]
    pub devdmastatus6: DEVDMASTATUS6,
    #[doc = "0x370 - Device DMA Channel Next Descriptor Address Register (n = 7)"]
    pub devdmanxtdsc7: DEVDMANXTDSC7,
    #[doc = "0x374 - Device DMA Channel Address Register (n = 7)"]
    pub devdmaaddress7: DEVDMAADDRESS7,
    #[doc = "0x378 - Device DMA Channel Control Register (n = 7)"]
    pub devdmacontrol7: DEVDMACONTROL7,
    #[doc = "0x37c - Device DMA Channel Status Register (n = 7)"]
    pub devdmastatus7: DEVDMASTATUS7,
    _reserved107: [u8; 0x80],
    #[doc = "0x400 - Host General Control Register"]
    pub hstctrl: HSTCTRL,
    #[doc = "0x404 - Host Global Interrupt Status Register"]
    pub hstisr: HSTISR,
    #[doc = "0x408 - Host Global Interrupt Clear Register"]
    pub hsticr: HSTICR,
    #[doc = "0x40c - Host Global Interrupt Set Register"]
    pub hstifr: HSTIFR,
    #[doc = "0x410 - Host Global Interrupt Mask Register"]
    pub hstimr: HSTIMR,
    #[doc = "0x414 - Host Global Interrupt Disable Register"]
    pub hstidr: HSTIDR,
    #[doc = "0x418 - Host Global Interrupt Enable Register"]
    pub hstier: HSTIER,
    #[doc = "0x41c - Host Pipe Register"]
    pub hstpip: HSTPIP,
    #[doc = "0x420 - Host Frame Number Register"]
    pub hstfnum: HSTFNUM,
    #[doc = "0x424 - Host Address 1 Register"]
    pub hstaddr1: HSTADDR1,
    #[doc = "0x428 - Host Address 2 Register"]
    pub hstaddr2: HSTADDR2,
    #[doc = "0x42c - Host Address 3 Register"]
    pub hstaddr3: HSTADDR3,
    _reserved119: [u8; 0xd0],
    _reserved_119_hstpipcfg0: [u8; 0x04],
    #[doc = "0x504 - Host Pipe Configuration Register (n = 0) 1"]
    pub hstpipcfg1: HSTPIPCFG1,
    #[doc = "0x508 - Host Pipe Configuration Register (n = 0) 2"]
    pub hstpipcfg2: HSTPIPCFG2,
    #[doc = "0x50c - Host Pipe Configuration Register (n = 0) 3"]
    pub hstpipcfg3: HSTPIPCFG3,
    #[doc = "0x510 - Host Pipe Configuration Register (n = 0) 4"]
    pub hstpipcfg4: HSTPIPCFG4,
    #[doc = "0x514 - Host Pipe Configuration Register (n = 0) 5"]
    pub hstpipcfg5: HSTPIPCFG5,
    #[doc = "0x518 - Host Pipe Configuration Register (n = 0) 6"]
    pub hstpipcfg6: HSTPIPCFG6,
    #[doc = "0x51c - Host Pipe Configuration Register (n = 0) 7"]
    pub hstpipcfg7: HSTPIPCFG7,
    #[doc = "0x520 - Host Pipe Configuration Register (n = 0) 8"]
    pub hstpipcfg8: HSTPIPCFG8,
    #[doc = "0x524 - Host Pipe Configuration Register (n = 0) 9"]
    pub hstpipcfg9: HSTPIPCFG9,
    _reserved129: [u8; 0x08],
    _reserved_129_hstpipisr0: [u8; 0x04],
    #[doc = "0x534 - Host Pipe Status Register (n = 0) 1"]
    pub hstpipisr1: HSTPIPISR1,
    #[doc = "0x538 - Host Pipe Status Register (n = 0) 2"]
    pub hstpipisr2: HSTPIPISR2,
    #[doc = "0x53c - Host Pipe Status Register (n = 0) 3"]
    pub hstpipisr3: HSTPIPISR3,
    #[doc = "0x540 - Host Pipe Status Register (n = 0) 4"]
    pub hstpipisr4: HSTPIPISR4,
    #[doc = "0x544 - Host Pipe Status Register (n = 0) 5"]
    pub hstpipisr5: HSTPIPISR5,
    #[doc = "0x548 - Host Pipe Status Register (n = 0) 6"]
    pub hstpipisr6: HSTPIPISR6,
    #[doc = "0x54c - Host Pipe Status Register (n = 0) 7"]
    pub hstpipisr7: HSTPIPISR7,
    #[doc = "0x550 - Host Pipe Status Register (n = 0) 8"]
    pub hstpipisr8: HSTPIPISR8,
    #[doc = "0x554 - Host Pipe Status Register (n = 0) 9"]
    pub hstpipisr9: HSTPIPISR9,
    _reserved139: [u8; 0x08],
    _reserved_139_hstpipicr0: [u8; 0x04],
    #[doc = "0x564 - Host Pipe Clear Register (n = 0) 1"]
    pub hstpipicr1: HSTPIPICR1,
    #[doc = "0x568 - Host Pipe Clear Register (n = 0) 2"]
    pub hstpipicr2: HSTPIPICR2,
    #[doc = "0x56c - Host Pipe Clear Register (n = 0) 3"]
    pub hstpipicr3: HSTPIPICR3,
    #[doc = "0x570 - Host Pipe Clear Register (n = 0) 4"]
    pub hstpipicr4: HSTPIPICR4,
    #[doc = "0x574 - Host Pipe Clear Register (n = 0) 5"]
    pub hstpipicr5: HSTPIPICR5,
    #[doc = "0x578 - Host Pipe Clear Register (n = 0) 6"]
    pub hstpipicr6: HSTPIPICR6,
    #[doc = "0x57c - Host Pipe Clear Register (n = 0) 7"]
    pub hstpipicr7: HSTPIPICR7,
    #[doc = "0x580 - Host Pipe Clear Register (n = 0) 8"]
    pub hstpipicr8: HSTPIPICR8,
    #[doc = "0x584 - Host Pipe Clear Register (n = 0) 9"]
    pub hstpipicr9: HSTPIPICR9,
    _reserved149: [u8; 0x08],
    _reserved_149_hstpipifr0: [u8; 0x04],
    #[doc = "0x594 - Host Pipe Set Register (n = 0) 1"]
    pub hstpipifr1: HSTPIPIFR1,
    #[doc = "0x598 - Host Pipe Set Register (n = 0) 2"]
    pub hstpipifr2: HSTPIPIFR2,
    #[doc = "0x59c - Host Pipe Set Register (n = 0) 3"]
    pub hstpipifr3: HSTPIPIFR3,
    #[doc = "0x5a0 - Host Pipe Set Register (n = 0) 4"]
    pub hstpipifr4: HSTPIPIFR4,
    #[doc = "0x5a4 - Host Pipe Set Register (n = 0) 5"]
    pub hstpipifr5: HSTPIPIFR5,
    #[doc = "0x5a8 - Host Pipe Set Register (n = 0) 6"]
    pub hstpipifr6: HSTPIPIFR6,
    #[doc = "0x5ac - Host Pipe Set Register (n = 0) 7"]
    pub hstpipifr7: HSTPIPIFR7,
    #[doc = "0x5b0 - Host Pipe Set Register (n = 0) 8"]
    pub hstpipifr8: HSTPIPIFR8,
    #[doc = "0x5b4 - Host Pipe Set Register (n = 0) 9"]
    pub hstpipifr9: HSTPIPIFR9,
    _reserved159: [u8; 0x08],
    _reserved_159_hstpipimr0: [u8; 0x04],
    #[doc = "0x5c4 - Host Pipe Mask Register (n = 0) 1"]
    pub hstpipimr1: HSTPIPIMR1,
    #[doc = "0x5c8 - Host Pipe Mask Register (n = 0) 2"]
    pub hstpipimr2: HSTPIPIMR2,
    #[doc = "0x5cc - Host Pipe Mask Register (n = 0) 3"]
    pub hstpipimr3: HSTPIPIMR3,
    #[doc = "0x5d0 - Host Pipe Mask Register (n = 0) 4"]
    pub hstpipimr4: HSTPIPIMR4,
    #[doc = "0x5d4 - Host Pipe Mask Register (n = 0) 5"]
    pub hstpipimr5: HSTPIPIMR5,
    #[doc = "0x5d8 - Host Pipe Mask Register (n = 0) 6"]
    pub hstpipimr6: HSTPIPIMR6,
    #[doc = "0x5dc - Host Pipe Mask Register (n = 0) 7"]
    pub hstpipimr7: HSTPIPIMR7,
    #[doc = "0x5e0 - Host Pipe Mask Register (n = 0) 8"]
    pub hstpipimr8: HSTPIPIMR8,
    #[doc = "0x5e4 - Host Pipe Mask Register (n = 0) 9"]
    pub hstpipimr9: HSTPIPIMR9,
    _reserved169: [u8; 0x08],
    _reserved_169_hstpipier0: [u8; 0x04],
    #[doc = "0x5f4 - Host Pipe Enable Register (n = 0) 1"]
    pub hstpipier1: HSTPIPIER1,
    #[doc = "0x5f8 - Host Pipe Enable Register (n = 0) 2"]
    pub hstpipier2: HSTPIPIER2,
    #[doc = "0x5fc - Host Pipe Enable Register (n = 0) 3"]
    pub hstpipier3: HSTPIPIER3,
    #[doc = "0x600 - Host Pipe Enable Register (n = 0) 4"]
    pub hstpipier4: HSTPIPIER4,
    #[doc = "0x604 - Host Pipe Enable Register (n = 0) 5"]
    pub hstpipier5: HSTPIPIER5,
    #[doc = "0x608 - Host Pipe Enable Register (n = 0) 6"]
    pub hstpipier6: HSTPIPIER6,
    #[doc = "0x60c - Host Pipe Enable Register (n = 0) 7"]
    pub hstpipier7: HSTPIPIER7,
    #[doc = "0x610 - Host Pipe Enable Register (n = 0) 8"]
    pub hstpipier8: HSTPIPIER8,
    #[doc = "0x614 - Host Pipe Enable Register (n = 0) 9"]
    pub hstpipier9: HSTPIPIER9,
    _reserved179: [u8; 0x08],
    _reserved_179_hstpipidr0: [u8; 0x04],
    #[doc = "0x624 - Host Pipe Disable Register (n = 0) 1"]
    pub hstpipidr1: HSTPIPIDR1,
    #[doc = "0x628 - Host Pipe Disable Register (n = 0) 2"]
    pub hstpipidr2: HSTPIPIDR2,
    #[doc = "0x62c - Host Pipe Disable Register (n = 0) 3"]
    pub hstpipidr3: HSTPIPIDR3,
    #[doc = "0x630 - Host Pipe Disable Register (n = 0) 4"]
    pub hstpipidr4: HSTPIPIDR4,
    #[doc = "0x634 - Host Pipe Disable Register (n = 0) 5"]
    pub hstpipidr5: HSTPIPIDR5,
    #[doc = "0x638 - Host Pipe Disable Register (n = 0) 6"]
    pub hstpipidr6: HSTPIPIDR6,
    #[doc = "0x63c - Host Pipe Disable Register (n = 0) 7"]
    pub hstpipidr7: HSTPIPIDR7,
    #[doc = "0x640 - Host Pipe Disable Register (n = 0) 8"]
    pub hstpipidr8: HSTPIPIDR8,
    #[doc = "0x644 - Host Pipe Disable Register (n = 0) 9"]
    pub hstpipidr9: HSTPIPIDR9,
    _reserved189: [u8; 0x08],
    #[doc = "0x650 - Host Pipe IN Request Register (n = 0) 0"]
    pub hstpipinrq0: HSTPIPINRQ0,
    #[doc = "0x654 - Host Pipe IN Request Register (n = 0) 1"]
    pub hstpipinrq1: HSTPIPINRQ1,
    #[doc = "0x658 - Host Pipe IN Request Register (n = 0) 2"]
    pub hstpipinrq2: HSTPIPINRQ2,
    #[doc = "0x65c - Host Pipe IN Request Register (n = 0) 3"]
    pub hstpipinrq3: HSTPIPINRQ3,
    #[doc = "0x660 - Host Pipe IN Request Register (n = 0) 4"]
    pub hstpipinrq4: HSTPIPINRQ4,
    #[doc = "0x664 - Host Pipe IN Request Register (n = 0) 5"]
    pub hstpipinrq5: HSTPIPINRQ5,
    #[doc = "0x668 - Host Pipe IN Request Register (n = 0) 6"]
    pub hstpipinrq6: HSTPIPINRQ6,
    #[doc = "0x66c - Host Pipe IN Request Register (n = 0) 7"]
    pub hstpipinrq7: HSTPIPINRQ7,
    #[doc = "0x670 - Host Pipe IN Request Register (n = 0) 8"]
    pub hstpipinrq8: HSTPIPINRQ8,
    #[doc = "0x674 - Host Pipe IN Request Register (n = 0) 9"]
    pub hstpipinrq9: HSTPIPINRQ9,
    _reserved199: [u8; 0x08],
    #[doc = "0x680 - Host Pipe Error Register (n = 0) 0"]
    pub hstpiperr0: HSTPIPERR0,
    #[doc = "0x684 - Host Pipe Error Register (n = 0) 1"]
    pub hstpiperr1: HSTPIPERR1,
    #[doc = "0x688 - Host Pipe Error Register (n = 0) 2"]
    pub hstpiperr2: HSTPIPERR2,
    #[doc = "0x68c - Host Pipe Error Register (n = 0) 3"]
    pub hstpiperr3: HSTPIPERR3,
    #[doc = "0x690 - Host Pipe Error Register (n = 0) 4"]
    pub hstpiperr4: HSTPIPERR4,
    #[doc = "0x694 - Host Pipe Error Register (n = 0) 5"]
    pub hstpiperr5: HSTPIPERR5,
    #[doc = "0x698 - Host Pipe Error Register (n = 0) 6"]
    pub hstpiperr6: HSTPIPERR6,
    #[doc = "0x69c - Host Pipe Error Register (n = 0) 7"]
    pub hstpiperr7: HSTPIPERR7,
    #[doc = "0x6a0 - Host Pipe Error Register (n = 0) 8"]
    pub hstpiperr8: HSTPIPERR8,
    #[doc = "0x6a4 - Host Pipe Error Register (n = 0) 9"]
    pub hstpiperr9: HSTPIPERR9,
    _reserved209: [u8; 0x68],
    #[doc = "0x710 - Host DMA Channel Next Descriptor Address Register (n = 1)"]
    pub hstdmanxtdsc1: HSTDMANXTDSC1,
    #[doc = "0x714 - Host DMA Channel Address Register (n = 1)"]
    pub hstdmaaddress1: HSTDMAADDRESS1,
    #[doc = "0x718 - Host DMA Channel Control Register (n = 1)"]
    pub hstdmacontrol1: HSTDMACONTROL1,
    #[doc = "0x71c - Host DMA Channel Status Register (n = 1)"]
    pub hstdmastatus1: HSTDMASTATUS1,
    #[doc = "0x720 - Host DMA Channel Next Descriptor Address Register (n = 2)"]
    pub hstdmanxtdsc2: HSTDMANXTDSC2,
    #[doc = "0x724 - Host DMA Channel Address Register (n = 2)"]
    pub hstdmaaddress2: HSTDMAADDRESS2,
    #[doc = "0x728 - Host DMA Channel Control Register (n = 2)"]
    pub hstdmacontrol2: HSTDMACONTROL2,
    #[doc = "0x72c - Host DMA Channel Status Register (n = 2)"]
    pub hstdmastatus2: HSTDMASTATUS2,
    #[doc = "0x730 - Host DMA Channel Next Descriptor Address Register (n = 3)"]
    pub hstdmanxtdsc3: HSTDMANXTDSC3,
    #[doc = "0x734 - Host DMA Channel Address Register (n = 3)"]
    pub hstdmaaddress3: HSTDMAADDRESS3,
    #[doc = "0x738 - Host DMA Channel Control Register (n = 3)"]
    pub hstdmacontrol3: HSTDMACONTROL3,
    #[doc = "0x73c - Host DMA Channel Status Register (n = 3)"]
    pub hstdmastatus3: HSTDMASTATUS3,
    #[doc = "0x740 - Host DMA Channel Next Descriptor Address Register (n = 4)"]
    pub hstdmanxtdsc4: HSTDMANXTDSC4,
    #[doc = "0x744 - Host DMA Channel Address Register (n = 4)"]
    pub hstdmaaddress4: HSTDMAADDRESS4,
    #[doc = "0x748 - Host DMA Channel Control Register (n = 4)"]
    pub hstdmacontrol4: HSTDMACONTROL4,
    #[doc = "0x74c - Host DMA Channel Status Register (n = 4)"]
    pub hstdmastatus4: HSTDMASTATUS4,
    #[doc = "0x750 - Host DMA Channel Next Descriptor Address Register (n = 5)"]
    pub hstdmanxtdsc5: HSTDMANXTDSC5,
    #[doc = "0x754 - Host DMA Channel Address Register (n = 5)"]
    pub hstdmaaddress5: HSTDMAADDRESS5,
    #[doc = "0x758 - Host DMA Channel Control Register (n = 5)"]
    pub hstdmacontrol5: HSTDMACONTROL5,
    #[doc = "0x75c - Host DMA Channel Status Register (n = 5)"]
    pub hstdmastatus5: HSTDMASTATUS5,
    #[doc = "0x760 - Host DMA Channel Next Descriptor Address Register (n = 6)"]
    pub hstdmanxtdsc6: HSTDMANXTDSC6,
    #[doc = "0x764 - Host DMA Channel Address Register (n = 6)"]
    pub hstdmaaddress6: HSTDMAADDRESS6,
    #[doc = "0x768 - Host DMA Channel Control Register (n = 6)"]
    pub hstdmacontrol6: HSTDMACONTROL6,
    #[doc = "0x76c - Host DMA Channel Status Register (n = 6)"]
    pub hstdmastatus6: HSTDMASTATUS6,
    #[doc = "0x770 - Host DMA Channel Next Descriptor Address Register (n = 7)"]
    pub hstdmanxtdsc7: HSTDMANXTDSC7,
    #[doc = "0x774 - Host DMA Channel Address Register (n = 7)"]
    pub hstdmaaddress7: HSTDMAADDRESS7,
    #[doc = "0x778 - Host DMA Channel Control Register (n = 7)"]
    pub hstdmacontrol7: HSTDMACONTROL7,
    #[doc = "0x77c - Host DMA Channel Status Register (n = 7)"]
    pub hstdmastatus7: HSTDMASTATUS7,
    _reserved237: [u8; 0x80],
    #[doc = "0x800 - General Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x804 - General Status Register"]
    pub sr: SR,
    #[doc = "0x808 - General Status Clear Register"]
    pub scr: SCR,
    #[doc = "0x80c - General Status Set Register"]
    pub sfr: SFR,
    _reserved241: [u8; 0x1c],
    #[doc = "0x82c - General Finite State Machine Register"]
    pub fsm: FSM,
}
impl RegisterBlock {
    #[doc = "0x130 - Device Endpoint Status Register (n = 0)"]
    #[inline(always)]
    pub const fn isoenpt_deveptisr0_isoenpt(&self) -> &ISOENPT_DEVEPTISR0_ISOENPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(304usize).cast() }
    }
    #[doc = "0x130 - Device Endpoint Status Register (n = 0) 0"]
    #[inline(always)]
    pub const fn deveptisr0(&self) -> &DEVEPTISR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(304usize).cast() }
    }
    #[doc = "0x160 - Device Endpoint Clear Register (n = 0)"]
    #[inline(always)]
    pub const fn isoenpt_devepticr0_isoenpt(&self) -> &ISOENPT_DEVEPTICR0_ISOENPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(352usize).cast() }
    }
    #[doc = "0x160 - Device Endpoint Clear Register (n = 0) 0"]
    #[inline(always)]
    pub const fn devepticr0(&self) -> &DEVEPTICR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(352usize).cast() }
    }
    #[doc = "0x190 - Device Endpoint Set Register (n = 0)"]
    #[inline(always)]
    pub const fn isoenpt_deveptifr0_isoenpt(&self) -> &ISOENPT_DEVEPTIFR0_ISOENPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(400usize).cast() }
    }
    #[doc = "0x190 - Device Endpoint Set Register (n = 0) 0"]
    #[inline(always)]
    pub const fn deveptifr0(&self) -> &DEVEPTIFR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(400usize).cast() }
    }
    #[doc = "0x1c0 - Device Endpoint Mask Register (n = 0)"]
    #[inline(always)]
    pub const fn isoenpt_deveptimr0_isoenpt(&self) -> &ISOENPT_DEVEPTIMR0_ISOENPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(448usize).cast() }
    }
    #[doc = "0x1c0 - Device Endpoint Mask Register (n = 0) 0"]
    #[inline(always)]
    pub const fn deveptimr0(&self) -> &DEVEPTIMR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(448usize).cast() }
    }
    #[doc = "0x1f0 - Device Endpoint Enable Register (n = 0)"]
    #[inline(always)]
    pub const fn isoenpt_deveptier0_isoenpt(&self) -> &ISOENPT_DEVEPTIER0_ISOENPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(496usize).cast() }
    }
    #[doc = "0x1f0 - Device Endpoint Enable Register (n = 0) 0"]
    #[inline(always)]
    pub const fn deveptier0(&self) -> &DEVEPTIER0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(496usize).cast() }
    }
    #[doc = "0x220 - Device Endpoint Disable Register (n = 0)"]
    #[inline(always)]
    pub const fn isoenpt_deveptidr0_isoenpt(&self) -> &ISOENPT_DEVEPTIDR0_ISOENPT {
        unsafe { &*(self as *const Self).cast::<u8>().add(544usize).cast() }
    }
    #[doc = "0x220 - Device Endpoint Disable Register (n = 0) 0"]
    #[inline(always)]
    pub const fn deveptidr0(&self) -> &DEVEPTIDR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(544usize).cast() }
    }
    #[doc = "0x500 - Host Pipe Configuration Register (n = 0)"]
    #[inline(always)]
    pub const fn hsbohscp_hstpipcfg0_hsbohscp(&self) -> &HSBOHSCP_HSTPIPCFG0_HSBOHSCP {
        unsafe { &*(self as *const Self).cast::<u8>().add(1280usize).cast() }
    }
    #[doc = "0x500 - Host Pipe Configuration Register (n = 0) 0"]
    #[inline(always)]
    pub const fn hstpipcfg0(&self) -> &HSTPIPCFG0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1280usize).cast() }
    }
    #[doc = "0x530 - Host Pipe Status Register (n = 0)"]
    #[inline(always)]
    pub const fn isopipes_hstpipisr0_isopipes(&self) -> &ISOPIPES_HSTPIPISR0_ISOPIPES {
        unsafe { &*(self as *const Self).cast::<u8>().add(1328usize).cast() }
    }
    #[doc = "0x530 - Host Pipe Status Register (n = 0)"]
    #[inline(always)]
    pub const fn intpipes_hstpipisr0_intpipes(&self) -> &INTPIPES_HSTPIPISR0_INTPIPES {
        unsafe { &*(self as *const Self).cast::<u8>().add(1328usize).cast() }
    }
    #[doc = "0x530 - Host Pipe Status Register (n = 0) 0"]
    #[inline(always)]
    pub const fn hstpipisr0(&self) -> &HSTPIPISR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1328usize).cast() }
    }
    #[doc = "0x560 - Host Pipe Clear Register (n = 0)"]
    #[inline(always)]
    pub const fn isopipes_hstpipicr0_isopipes(&self) -> &ISOPIPES_HSTPIPICR0_ISOPIPES {
        unsafe { &*(self as *const Self).cast::<u8>().add(1376usize).cast() }
    }
    #[doc = "0x560 - Host Pipe Clear Register (n = 0)"]
    #[inline(always)]
    pub const fn intpipes_hstpipicr0_intpipes(&self) -> &INTPIPES_HSTPIPICR0_INTPIPES {
        unsafe { &*(self as *const Self).cast::<u8>().add(1376usize).cast() }
    }
    #[doc = "0x560 - Host Pipe Clear Register (n = 0) 0"]
    #[inline(always)]
    pub const fn hstpipicr0(&self) -> &HSTPIPICR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1376usize).cast() }
    }
    #[doc = "0x590 - Host Pipe Set Register (n = 0)"]
    #[inline(always)]
    pub const fn isopipes_hstpipifr0_isopipes(&self) -> &ISOPIPES_HSTPIPIFR0_ISOPIPES {
        unsafe { &*(self as *const Self).cast::<u8>().add(1424usize).cast() }
    }
    #[doc = "0x590 - Host Pipe Set Register (n = 0)"]
    #[inline(always)]
    pub const fn intpipes_hstpipifr0_intpipes(&self) -> &INTPIPES_HSTPIPIFR0_INTPIPES {
        unsafe { &*(self as *const Self).cast::<u8>().add(1424usize).cast() }
    }
    #[doc = "0x590 - Host Pipe Set Register (n = 0) 0"]
    #[inline(always)]
    pub const fn hstpipifr0(&self) -> &HSTPIPIFR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1424usize).cast() }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register (n = 0)"]
    #[inline(always)]
    pub const fn isopipes_hstpipimr0_isopipes(&self) -> &ISOPIPES_HSTPIPIMR0_ISOPIPES {
        unsafe { &*(self as *const Self).cast::<u8>().add(1472usize).cast() }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register (n = 0)"]
    #[inline(always)]
    pub const fn intpipes_hstpipimr0_intpipes(&self) -> &INTPIPES_HSTPIPIMR0_INTPIPES {
        unsafe { &*(self as *const Self).cast::<u8>().add(1472usize).cast() }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register (n = 0) 0"]
    #[inline(always)]
    pub const fn hstpipimr0(&self) -> &HSTPIPIMR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1472usize).cast() }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register (n = 0)"]
    #[inline(always)]
    pub const fn isopipes_hstpipier0_isopipes(&self) -> &ISOPIPES_HSTPIPIER0_ISOPIPES {
        unsafe { &*(self as *const Self).cast::<u8>().add(1520usize).cast() }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register (n = 0)"]
    #[inline(always)]
    pub const fn intpipes_hstpipier0_intpipes(&self) -> &INTPIPES_HSTPIPIER0_INTPIPES {
        unsafe { &*(self as *const Self).cast::<u8>().add(1520usize).cast() }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register (n = 0) 0"]
    #[inline(always)]
    pub const fn hstpipier0(&self) -> &HSTPIPIER0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1520usize).cast() }
    }
    #[doc = "0x620 - Host Pipe Disable Register (n = 0)"]
    #[inline(always)]
    pub const fn isopipes_hstpipidr0_isopipes(&self) -> &ISOPIPES_HSTPIPIDR0_ISOPIPES {
        unsafe { &*(self as *const Self).cast::<u8>().add(1568usize).cast() }
    }
    #[doc = "0x620 - Host Pipe Disable Register (n = 0)"]
    #[inline(always)]
    pub const fn intpipes_hstpipidr0_intpipes(&self) -> &INTPIPES_HSTPIPIDR0_INTPIPES {
        unsafe { &*(self as *const Self).cast::<u8>().add(1568usize).cast() }
    }
    #[doc = "0x620 - Host Pipe Disable Register (n = 0) 0"]
    #[inline(always)]
    pub const fn hstpipidr0(&self) -> &HSTPIPIDR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1568usize).cast() }
    }
}
#[doc = "DEVCTRL (rw) register accessor: Device General Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devctrl`]
module"]
pub type DEVCTRL = crate::Reg<devctrl::DEVCTRL_SPEC>;
#[doc = "Device General Control Register"]
pub mod devctrl;
#[doc = "DEVISR (r) register accessor: Device Global Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devisr`]
module"]
pub type DEVISR = crate::Reg<devisr::DEVISR_SPEC>;
#[doc = "Device Global Interrupt Status Register"]
pub mod devisr;
#[doc = "DEVICR (w) register accessor: Device Global Interrupt Clear Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devicr`]
module"]
pub type DEVICR = crate::Reg<devicr::DEVICR_SPEC>;
#[doc = "Device Global Interrupt Clear Register"]
pub mod devicr;
#[doc = "DEVIFR (w) register accessor: Device Global Interrupt Set Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devifr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devifr`]
module"]
pub type DEVIFR = crate::Reg<devifr::DEVIFR_SPEC>;
#[doc = "Device Global Interrupt Set Register"]
pub mod devifr;
#[doc = "DEVIMR (r) register accessor: Device Global Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devimr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devimr`]
module"]
pub type DEVIMR = crate::Reg<devimr::DEVIMR_SPEC>;
#[doc = "Device Global Interrupt Mask Register"]
pub mod devimr;
#[doc = "DEVIDR (w) register accessor: Device Global Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devidr`]
module"]
pub type DEVIDR = crate::Reg<devidr::DEVIDR_SPEC>;
#[doc = "Device Global Interrupt Disable Register"]
pub mod devidr;
#[doc = "DEVIER (w) register accessor: Device Global Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devier`]
module"]
pub type DEVIER = crate::Reg<devier::DEVIER_SPEC>;
#[doc = "Device Global Interrupt Enable Register"]
pub mod devier;
#[doc = "DEVEPT (rw) register accessor: Device Endpoint Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devept::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devept::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devept`]
module"]
pub type DEVEPT = crate::Reg<devept::DEVEPT_SPEC>;
#[doc = "Device Endpoint Register"]
pub mod devept;
#[doc = "DEVFNUM (r) register accessor: Device Frame Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devfnum::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devfnum`]
module"]
pub type DEVFNUM = crate::Reg<devfnum::DEVFNUM_SPEC>;
#[doc = "Device Frame Number Register"]
pub mod devfnum;
#[doc = "DEVEPTCFG0 (rw) register accessor: Device Endpoint Configuration Register (n = 0) 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptcfg0`]
module"]
pub type DEVEPTCFG0 = crate::Reg<deveptcfg0::DEVEPTCFG0_SPEC>;
#[doc = "Device Endpoint Configuration Register (n = 0) 0"]
pub mod deveptcfg0;
#[doc = "DEVEPTCFG1 (rw) register accessor: Device Endpoint Configuration Register (n = 0) 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptcfg1`]
module"]
pub type DEVEPTCFG1 = crate::Reg<deveptcfg1::DEVEPTCFG1_SPEC>;
#[doc = "Device Endpoint Configuration Register (n = 0) 1"]
pub mod deveptcfg1;
#[doc = "DEVEPTCFG2 (rw) register accessor: Device Endpoint Configuration Register (n = 0) 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptcfg2`]
module"]
pub type DEVEPTCFG2 = crate::Reg<deveptcfg2::DEVEPTCFG2_SPEC>;
#[doc = "Device Endpoint Configuration Register (n = 0) 2"]
pub mod deveptcfg2;
#[doc = "DEVEPTCFG3 (rw) register accessor: Device Endpoint Configuration Register (n = 0) 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptcfg3`]
module"]
pub type DEVEPTCFG3 = crate::Reg<deveptcfg3::DEVEPTCFG3_SPEC>;
#[doc = "Device Endpoint Configuration Register (n = 0) 3"]
pub mod deveptcfg3;
#[doc = "DEVEPTCFG4 (rw) register accessor: Device Endpoint Configuration Register (n = 0) 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptcfg4`]
module"]
pub type DEVEPTCFG4 = crate::Reg<deveptcfg4::DEVEPTCFG4_SPEC>;
#[doc = "Device Endpoint Configuration Register (n = 0) 4"]
pub mod deveptcfg4;
#[doc = "DEVEPTCFG5 (rw) register accessor: Device Endpoint Configuration Register (n = 0) 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptcfg5`]
module"]
pub type DEVEPTCFG5 = crate::Reg<deveptcfg5::DEVEPTCFG5_SPEC>;
#[doc = "Device Endpoint Configuration Register (n = 0) 5"]
pub mod deveptcfg5;
#[doc = "DEVEPTCFG6 (rw) register accessor: Device Endpoint Configuration Register (n = 0) 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg6::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptcfg6`]
module"]
pub type DEVEPTCFG6 = crate::Reg<deveptcfg6::DEVEPTCFG6_SPEC>;
#[doc = "Device Endpoint Configuration Register (n = 0) 6"]
pub mod deveptcfg6;
#[doc = "DEVEPTCFG7 (rw) register accessor: Device Endpoint Configuration Register (n = 0) 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg7::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptcfg7`]
module"]
pub type DEVEPTCFG7 = crate::Reg<deveptcfg7::DEVEPTCFG7_SPEC>;
#[doc = "Device Endpoint Configuration Register (n = 0) 7"]
pub mod deveptcfg7;
#[doc = "DEVEPTCFG8 (rw) register accessor: Device Endpoint Configuration Register (n = 0) 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg8::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptcfg8`]
module"]
pub type DEVEPTCFG8 = crate::Reg<deveptcfg8::DEVEPTCFG8_SPEC>;
#[doc = "Device Endpoint Configuration Register (n = 0) 8"]
pub mod deveptcfg8;
#[doc = "DEVEPTCFG9 (rw) register accessor: Device Endpoint Configuration Register (n = 0) 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg9::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptcfg9`]
module"]
pub type DEVEPTCFG9 = crate::Reg<deveptcfg9::DEVEPTCFG9_SPEC>;
#[doc = "Device Endpoint Configuration Register (n = 0) 9"]
pub mod deveptcfg9;
#[doc = "DEVEPTISR0 (r) register accessor: Device Endpoint Status Register (n = 0) 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptisr0`]
module"]
pub type DEVEPTISR0 = crate::Reg<deveptisr0::DEVEPTISR0_SPEC>;
#[doc = "Device Endpoint Status Register (n = 0) 0"]
pub mod deveptisr0;
#[doc = "DEVEPTISR1 (r) register accessor: Device Endpoint Status Register (n = 0) 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptisr1`]
module"]
pub type DEVEPTISR1 = crate::Reg<deveptisr1::DEVEPTISR1_SPEC>;
#[doc = "Device Endpoint Status Register (n = 0) 1"]
pub mod deveptisr1;
#[doc = "DEVEPTISR2 (r) register accessor: Device Endpoint Status Register (n = 0) 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptisr2`]
module"]
pub type DEVEPTISR2 = crate::Reg<deveptisr2::DEVEPTISR2_SPEC>;
#[doc = "Device Endpoint Status Register (n = 0) 2"]
pub mod deveptisr2;
#[doc = "DEVEPTISR3 (r) register accessor: Device Endpoint Status Register (n = 0) 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptisr3`]
module"]
pub type DEVEPTISR3 = crate::Reg<deveptisr3::DEVEPTISR3_SPEC>;
#[doc = "Device Endpoint Status Register (n = 0) 3"]
pub mod deveptisr3;
#[doc = "DEVEPTISR4 (r) register accessor: Device Endpoint Status Register (n = 0) 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptisr4`]
module"]
pub type DEVEPTISR4 = crate::Reg<deveptisr4::DEVEPTISR4_SPEC>;
#[doc = "Device Endpoint Status Register (n = 0) 4"]
pub mod deveptisr4;
#[doc = "DEVEPTISR5 (r) register accessor: Device Endpoint Status Register (n = 0) 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptisr5`]
module"]
pub type DEVEPTISR5 = crate::Reg<deveptisr5::DEVEPTISR5_SPEC>;
#[doc = "Device Endpoint Status Register (n = 0) 5"]
pub mod deveptisr5;
#[doc = "DEVEPTISR6 (r) register accessor: Device Endpoint Status Register (n = 0) 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptisr6`]
module"]
pub type DEVEPTISR6 = crate::Reg<deveptisr6::DEVEPTISR6_SPEC>;
#[doc = "Device Endpoint Status Register (n = 0) 6"]
pub mod deveptisr6;
#[doc = "DEVEPTISR7 (r) register accessor: Device Endpoint Status Register (n = 0) 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptisr7`]
module"]
pub type DEVEPTISR7 = crate::Reg<deveptisr7::DEVEPTISR7_SPEC>;
#[doc = "Device Endpoint Status Register (n = 0) 7"]
pub mod deveptisr7;
#[doc = "DEVEPTISR8 (r) register accessor: Device Endpoint Status Register (n = 0) 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptisr8`]
module"]
pub type DEVEPTISR8 = crate::Reg<deveptisr8::DEVEPTISR8_SPEC>;
#[doc = "Device Endpoint Status Register (n = 0) 8"]
pub mod deveptisr8;
#[doc = "DEVEPTISR9 (r) register accessor: Device Endpoint Status Register (n = 0) 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptisr9`]
module"]
pub type DEVEPTISR9 = crate::Reg<deveptisr9::DEVEPTISR9_SPEC>;
#[doc = "Device Endpoint Status Register (n = 0) 9"]
pub mod deveptisr9;
#[doc = "ISOENPT_DEVEPTISR0_ISOENPT (r) register accessor: Device Endpoint Status Register (n = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoenpt_deveptisr0_isoenpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`isoenpt_deveptisr0_isoenpt`]
module"]
pub type ISOENPT_DEVEPTISR0_ISOENPT =
    crate::Reg<isoenpt_deveptisr0_isoenpt::ISOENPT_DEVEPTISR0_ISOENPT_SPEC>;
#[doc = "Device Endpoint Status Register (n = 0)"]
pub mod isoenpt_deveptisr0_isoenpt;
#[doc = "DEVEPTICR0 (w) register accessor: Device Endpoint Clear Register (n = 0) 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devepticr0`]
module"]
pub type DEVEPTICR0 = crate::Reg<devepticr0::DEVEPTICR0_SPEC>;
#[doc = "Device Endpoint Clear Register (n = 0) 0"]
pub mod devepticr0;
#[doc = "DEVEPTICR1 (w) register accessor: Device Endpoint Clear Register (n = 0) 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devepticr1`]
module"]
pub type DEVEPTICR1 = crate::Reg<devepticr1::DEVEPTICR1_SPEC>;
#[doc = "Device Endpoint Clear Register (n = 0) 1"]
pub mod devepticr1;
#[doc = "DEVEPTICR2 (w) register accessor: Device Endpoint Clear Register (n = 0) 2\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devepticr2`]
module"]
pub type DEVEPTICR2 = crate::Reg<devepticr2::DEVEPTICR2_SPEC>;
#[doc = "Device Endpoint Clear Register (n = 0) 2"]
pub mod devepticr2;
#[doc = "DEVEPTICR3 (w) register accessor: Device Endpoint Clear Register (n = 0) 3\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devepticr3`]
module"]
pub type DEVEPTICR3 = crate::Reg<devepticr3::DEVEPTICR3_SPEC>;
#[doc = "Device Endpoint Clear Register (n = 0) 3"]
pub mod devepticr3;
#[doc = "DEVEPTICR4 (w) register accessor: Device Endpoint Clear Register (n = 0) 4\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devepticr4`]
module"]
pub type DEVEPTICR4 = crate::Reg<devepticr4::DEVEPTICR4_SPEC>;
#[doc = "Device Endpoint Clear Register (n = 0) 4"]
pub mod devepticr4;
#[doc = "DEVEPTICR5 (w) register accessor: Device Endpoint Clear Register (n = 0) 5\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devepticr5`]
module"]
pub type DEVEPTICR5 = crate::Reg<devepticr5::DEVEPTICR5_SPEC>;
#[doc = "Device Endpoint Clear Register (n = 0) 5"]
pub mod devepticr5;
#[doc = "DEVEPTICR6 (w) register accessor: Device Endpoint Clear Register (n = 0) 6\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devepticr6`]
module"]
pub type DEVEPTICR6 = crate::Reg<devepticr6::DEVEPTICR6_SPEC>;
#[doc = "Device Endpoint Clear Register (n = 0) 6"]
pub mod devepticr6;
#[doc = "DEVEPTICR7 (w) register accessor: Device Endpoint Clear Register (n = 0) 7\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devepticr7`]
module"]
pub type DEVEPTICR7 = crate::Reg<devepticr7::DEVEPTICR7_SPEC>;
#[doc = "Device Endpoint Clear Register (n = 0) 7"]
pub mod devepticr7;
#[doc = "DEVEPTICR8 (w) register accessor: Device Endpoint Clear Register (n = 0) 8\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devepticr8`]
module"]
pub type DEVEPTICR8 = crate::Reg<devepticr8::DEVEPTICR8_SPEC>;
#[doc = "Device Endpoint Clear Register (n = 0) 8"]
pub mod devepticr8;
#[doc = "DEVEPTICR9 (w) register accessor: Device Endpoint Clear Register (n = 0) 9\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr9::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devepticr9`]
module"]
pub type DEVEPTICR9 = crate::Reg<devepticr9::DEVEPTICR9_SPEC>;
#[doc = "Device Endpoint Clear Register (n = 0) 9"]
pub mod devepticr9;
#[doc = "ISOENPT_DEVEPTICR0_ISOENPT (w) register accessor: Device Endpoint Clear Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoenpt_devepticr0_isoenpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`isoenpt_devepticr0_isoenpt`]
module"]
pub type ISOENPT_DEVEPTICR0_ISOENPT =
    crate::Reg<isoenpt_devepticr0_isoenpt::ISOENPT_DEVEPTICR0_ISOENPT_SPEC>;
#[doc = "Device Endpoint Clear Register (n = 0)"]
pub mod isoenpt_devepticr0_isoenpt;
#[doc = "DEVEPTIFR0 (w) register accessor: Device Endpoint Set Register (n = 0) 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptifr0`]
module"]
pub type DEVEPTIFR0 = crate::Reg<deveptifr0::DEVEPTIFR0_SPEC>;
#[doc = "Device Endpoint Set Register (n = 0) 0"]
pub mod deveptifr0;
#[doc = "DEVEPTIFR1 (w) register accessor: Device Endpoint Set Register (n = 0) 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptifr1`]
module"]
pub type DEVEPTIFR1 = crate::Reg<deveptifr1::DEVEPTIFR1_SPEC>;
#[doc = "Device Endpoint Set Register (n = 0) 1"]
pub mod deveptifr1;
#[doc = "DEVEPTIFR2 (w) register accessor: Device Endpoint Set Register (n = 0) 2\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptifr2`]
module"]
pub type DEVEPTIFR2 = crate::Reg<deveptifr2::DEVEPTIFR2_SPEC>;
#[doc = "Device Endpoint Set Register (n = 0) 2"]
pub mod deveptifr2;
#[doc = "DEVEPTIFR3 (w) register accessor: Device Endpoint Set Register (n = 0) 3\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptifr3`]
module"]
pub type DEVEPTIFR3 = crate::Reg<deveptifr3::DEVEPTIFR3_SPEC>;
#[doc = "Device Endpoint Set Register (n = 0) 3"]
pub mod deveptifr3;
#[doc = "DEVEPTIFR4 (w) register accessor: Device Endpoint Set Register (n = 0) 4\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptifr4`]
module"]
pub type DEVEPTIFR4 = crate::Reg<deveptifr4::DEVEPTIFR4_SPEC>;
#[doc = "Device Endpoint Set Register (n = 0) 4"]
pub mod deveptifr4;
#[doc = "DEVEPTIFR5 (w) register accessor: Device Endpoint Set Register (n = 0) 5\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptifr5`]
module"]
pub type DEVEPTIFR5 = crate::Reg<deveptifr5::DEVEPTIFR5_SPEC>;
#[doc = "Device Endpoint Set Register (n = 0) 5"]
pub mod deveptifr5;
#[doc = "DEVEPTIFR6 (w) register accessor: Device Endpoint Set Register (n = 0) 6\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptifr6`]
module"]
pub type DEVEPTIFR6 = crate::Reg<deveptifr6::DEVEPTIFR6_SPEC>;
#[doc = "Device Endpoint Set Register (n = 0) 6"]
pub mod deveptifr6;
#[doc = "DEVEPTIFR7 (w) register accessor: Device Endpoint Set Register (n = 0) 7\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptifr7`]
module"]
pub type DEVEPTIFR7 = crate::Reg<deveptifr7::DEVEPTIFR7_SPEC>;
#[doc = "Device Endpoint Set Register (n = 0) 7"]
pub mod deveptifr7;
#[doc = "DEVEPTIFR8 (w) register accessor: Device Endpoint Set Register (n = 0) 8\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptifr8`]
module"]
pub type DEVEPTIFR8 = crate::Reg<deveptifr8::DEVEPTIFR8_SPEC>;
#[doc = "Device Endpoint Set Register (n = 0) 8"]
pub mod deveptifr8;
#[doc = "DEVEPTIFR9 (w) register accessor: Device Endpoint Set Register (n = 0) 9\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr9::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptifr9`]
module"]
pub type DEVEPTIFR9 = crate::Reg<deveptifr9::DEVEPTIFR9_SPEC>;
#[doc = "Device Endpoint Set Register (n = 0) 9"]
pub mod deveptifr9;
#[doc = "ISOENPT_DEVEPTIFR0_ISOENPT (w) register accessor: Device Endpoint Set Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoenpt_deveptifr0_isoenpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`isoenpt_deveptifr0_isoenpt`]
module"]
pub type ISOENPT_DEVEPTIFR0_ISOENPT =
    crate::Reg<isoenpt_deveptifr0_isoenpt::ISOENPT_DEVEPTIFR0_ISOENPT_SPEC>;
#[doc = "Device Endpoint Set Register (n = 0)"]
pub mod isoenpt_deveptifr0_isoenpt;
#[doc = "DEVEPTIMR0 (r) register accessor: Device Endpoint Mask Register (n = 0) 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptimr0`]
module"]
pub type DEVEPTIMR0 = crate::Reg<deveptimr0::DEVEPTIMR0_SPEC>;
#[doc = "Device Endpoint Mask Register (n = 0) 0"]
pub mod deveptimr0;
#[doc = "DEVEPTIMR1 (r) register accessor: Device Endpoint Mask Register (n = 0) 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptimr1`]
module"]
pub type DEVEPTIMR1 = crate::Reg<deveptimr1::DEVEPTIMR1_SPEC>;
#[doc = "Device Endpoint Mask Register (n = 0) 1"]
pub mod deveptimr1;
#[doc = "DEVEPTIMR2 (r) register accessor: Device Endpoint Mask Register (n = 0) 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptimr2`]
module"]
pub type DEVEPTIMR2 = crate::Reg<deveptimr2::DEVEPTIMR2_SPEC>;
#[doc = "Device Endpoint Mask Register (n = 0) 2"]
pub mod deveptimr2;
#[doc = "DEVEPTIMR3 (r) register accessor: Device Endpoint Mask Register (n = 0) 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptimr3`]
module"]
pub type DEVEPTIMR3 = crate::Reg<deveptimr3::DEVEPTIMR3_SPEC>;
#[doc = "Device Endpoint Mask Register (n = 0) 3"]
pub mod deveptimr3;
#[doc = "DEVEPTIMR4 (r) register accessor: Device Endpoint Mask Register (n = 0) 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptimr4`]
module"]
pub type DEVEPTIMR4 = crate::Reg<deveptimr4::DEVEPTIMR4_SPEC>;
#[doc = "Device Endpoint Mask Register (n = 0) 4"]
pub mod deveptimr4;
#[doc = "DEVEPTIMR5 (r) register accessor: Device Endpoint Mask Register (n = 0) 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptimr5`]
module"]
pub type DEVEPTIMR5 = crate::Reg<deveptimr5::DEVEPTIMR5_SPEC>;
#[doc = "Device Endpoint Mask Register (n = 0) 5"]
pub mod deveptimr5;
#[doc = "DEVEPTIMR6 (r) register accessor: Device Endpoint Mask Register (n = 0) 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptimr6`]
module"]
pub type DEVEPTIMR6 = crate::Reg<deveptimr6::DEVEPTIMR6_SPEC>;
#[doc = "Device Endpoint Mask Register (n = 0) 6"]
pub mod deveptimr6;
#[doc = "DEVEPTIMR7 (r) register accessor: Device Endpoint Mask Register (n = 0) 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptimr7`]
module"]
pub type DEVEPTIMR7 = crate::Reg<deveptimr7::DEVEPTIMR7_SPEC>;
#[doc = "Device Endpoint Mask Register (n = 0) 7"]
pub mod deveptimr7;
#[doc = "DEVEPTIMR8 (r) register accessor: Device Endpoint Mask Register (n = 0) 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptimr8`]
module"]
pub type DEVEPTIMR8 = crate::Reg<deveptimr8::DEVEPTIMR8_SPEC>;
#[doc = "Device Endpoint Mask Register (n = 0) 8"]
pub mod deveptimr8;
#[doc = "DEVEPTIMR9 (r) register accessor: Device Endpoint Mask Register (n = 0) 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptimr9`]
module"]
pub type DEVEPTIMR9 = crate::Reg<deveptimr9::DEVEPTIMR9_SPEC>;
#[doc = "Device Endpoint Mask Register (n = 0) 9"]
pub mod deveptimr9;
#[doc = "ISOENPT_DEVEPTIMR0_ISOENPT (r) register accessor: Device Endpoint Mask Register (n = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoenpt_deveptimr0_isoenpt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`isoenpt_deveptimr0_isoenpt`]
module"]
pub type ISOENPT_DEVEPTIMR0_ISOENPT =
    crate::Reg<isoenpt_deveptimr0_isoenpt::ISOENPT_DEVEPTIMR0_ISOENPT_SPEC>;
#[doc = "Device Endpoint Mask Register (n = 0)"]
pub mod isoenpt_deveptimr0_isoenpt;
#[doc = "DEVEPTIER0 (w) register accessor: Device Endpoint Enable Register (n = 0) 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptier0`]
module"]
pub type DEVEPTIER0 = crate::Reg<deveptier0::DEVEPTIER0_SPEC>;
#[doc = "Device Endpoint Enable Register (n = 0) 0"]
pub mod deveptier0;
#[doc = "DEVEPTIER1 (w) register accessor: Device Endpoint Enable Register (n = 0) 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptier1`]
module"]
pub type DEVEPTIER1 = crate::Reg<deveptier1::DEVEPTIER1_SPEC>;
#[doc = "Device Endpoint Enable Register (n = 0) 1"]
pub mod deveptier1;
#[doc = "DEVEPTIER2 (w) register accessor: Device Endpoint Enable Register (n = 0) 2\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptier2`]
module"]
pub type DEVEPTIER2 = crate::Reg<deveptier2::DEVEPTIER2_SPEC>;
#[doc = "Device Endpoint Enable Register (n = 0) 2"]
pub mod deveptier2;
#[doc = "DEVEPTIER3 (w) register accessor: Device Endpoint Enable Register (n = 0) 3\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptier3`]
module"]
pub type DEVEPTIER3 = crate::Reg<deveptier3::DEVEPTIER3_SPEC>;
#[doc = "Device Endpoint Enable Register (n = 0) 3"]
pub mod deveptier3;
#[doc = "DEVEPTIER4 (w) register accessor: Device Endpoint Enable Register (n = 0) 4\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptier4`]
module"]
pub type DEVEPTIER4 = crate::Reg<deveptier4::DEVEPTIER4_SPEC>;
#[doc = "Device Endpoint Enable Register (n = 0) 4"]
pub mod deveptier4;
#[doc = "DEVEPTIER5 (w) register accessor: Device Endpoint Enable Register (n = 0) 5\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptier5`]
module"]
pub type DEVEPTIER5 = crate::Reg<deveptier5::DEVEPTIER5_SPEC>;
#[doc = "Device Endpoint Enable Register (n = 0) 5"]
pub mod deveptier5;
#[doc = "DEVEPTIER6 (w) register accessor: Device Endpoint Enable Register (n = 0) 6\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptier6`]
module"]
pub type DEVEPTIER6 = crate::Reg<deveptier6::DEVEPTIER6_SPEC>;
#[doc = "Device Endpoint Enable Register (n = 0) 6"]
pub mod deveptier6;
#[doc = "DEVEPTIER7 (w) register accessor: Device Endpoint Enable Register (n = 0) 7\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptier7`]
module"]
pub type DEVEPTIER7 = crate::Reg<deveptier7::DEVEPTIER7_SPEC>;
#[doc = "Device Endpoint Enable Register (n = 0) 7"]
pub mod deveptier7;
#[doc = "DEVEPTIER8 (w) register accessor: Device Endpoint Enable Register (n = 0) 8\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptier8`]
module"]
pub type DEVEPTIER8 = crate::Reg<deveptier8::DEVEPTIER8_SPEC>;
#[doc = "Device Endpoint Enable Register (n = 0) 8"]
pub mod deveptier8;
#[doc = "DEVEPTIER9 (w) register accessor: Device Endpoint Enable Register (n = 0) 9\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier9::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptier9`]
module"]
pub type DEVEPTIER9 = crate::Reg<deveptier9::DEVEPTIER9_SPEC>;
#[doc = "Device Endpoint Enable Register (n = 0) 9"]
pub mod deveptier9;
#[doc = "ISOENPT_DEVEPTIER0_ISOENPT (w) register accessor: Device Endpoint Enable Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoenpt_deveptier0_isoenpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`isoenpt_deveptier0_isoenpt`]
module"]
pub type ISOENPT_DEVEPTIER0_ISOENPT =
    crate::Reg<isoenpt_deveptier0_isoenpt::ISOENPT_DEVEPTIER0_ISOENPT_SPEC>;
#[doc = "Device Endpoint Enable Register (n = 0)"]
pub mod isoenpt_deveptier0_isoenpt;
#[doc = "DEVEPTIDR0 (w) register accessor: Device Endpoint Disable Register (n = 0) 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptidr0`]
module"]
pub type DEVEPTIDR0 = crate::Reg<deveptidr0::DEVEPTIDR0_SPEC>;
#[doc = "Device Endpoint Disable Register (n = 0) 0"]
pub mod deveptidr0;
#[doc = "DEVEPTIDR1 (w) register accessor: Device Endpoint Disable Register (n = 0) 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptidr1`]
module"]
pub type DEVEPTIDR1 = crate::Reg<deveptidr1::DEVEPTIDR1_SPEC>;
#[doc = "Device Endpoint Disable Register (n = 0) 1"]
pub mod deveptidr1;
#[doc = "DEVEPTIDR2 (w) register accessor: Device Endpoint Disable Register (n = 0) 2\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptidr2`]
module"]
pub type DEVEPTIDR2 = crate::Reg<deveptidr2::DEVEPTIDR2_SPEC>;
#[doc = "Device Endpoint Disable Register (n = 0) 2"]
pub mod deveptidr2;
#[doc = "DEVEPTIDR3 (w) register accessor: Device Endpoint Disable Register (n = 0) 3\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptidr3`]
module"]
pub type DEVEPTIDR3 = crate::Reg<deveptidr3::DEVEPTIDR3_SPEC>;
#[doc = "Device Endpoint Disable Register (n = 0) 3"]
pub mod deveptidr3;
#[doc = "DEVEPTIDR4 (w) register accessor: Device Endpoint Disable Register (n = 0) 4\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptidr4`]
module"]
pub type DEVEPTIDR4 = crate::Reg<deveptidr4::DEVEPTIDR4_SPEC>;
#[doc = "Device Endpoint Disable Register (n = 0) 4"]
pub mod deveptidr4;
#[doc = "DEVEPTIDR5 (w) register accessor: Device Endpoint Disable Register (n = 0) 5\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptidr5`]
module"]
pub type DEVEPTIDR5 = crate::Reg<deveptidr5::DEVEPTIDR5_SPEC>;
#[doc = "Device Endpoint Disable Register (n = 0) 5"]
pub mod deveptidr5;
#[doc = "DEVEPTIDR6 (w) register accessor: Device Endpoint Disable Register (n = 0) 6\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptidr6`]
module"]
pub type DEVEPTIDR6 = crate::Reg<deveptidr6::DEVEPTIDR6_SPEC>;
#[doc = "Device Endpoint Disable Register (n = 0) 6"]
pub mod deveptidr6;
#[doc = "DEVEPTIDR7 (w) register accessor: Device Endpoint Disable Register (n = 0) 7\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptidr7`]
module"]
pub type DEVEPTIDR7 = crate::Reg<deveptidr7::DEVEPTIDR7_SPEC>;
#[doc = "Device Endpoint Disable Register (n = 0) 7"]
pub mod deveptidr7;
#[doc = "DEVEPTIDR8 (w) register accessor: Device Endpoint Disable Register (n = 0) 8\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptidr8`]
module"]
pub type DEVEPTIDR8 = crate::Reg<deveptidr8::DEVEPTIDR8_SPEC>;
#[doc = "Device Endpoint Disable Register (n = 0) 8"]
pub mod deveptidr8;
#[doc = "DEVEPTIDR9 (w) register accessor: Device Endpoint Disable Register (n = 0) 9\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr9::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptidr9`]
module"]
pub type DEVEPTIDR9 = crate::Reg<deveptidr9::DEVEPTIDR9_SPEC>;
#[doc = "Device Endpoint Disable Register (n = 0) 9"]
pub mod deveptidr9;
#[doc = "ISOENPT_DEVEPTIDR0_ISOENPT (w) register accessor: Device Endpoint Disable Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoenpt_deveptidr0_isoenpt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`isoenpt_deveptidr0_isoenpt`]
module"]
pub type ISOENPT_DEVEPTIDR0_ISOENPT =
    crate::Reg<isoenpt_deveptidr0_isoenpt::ISOENPT_DEVEPTIDR0_ISOENPT_SPEC>;
#[doc = "Device Endpoint Disable Register (n = 0)"]
pub mod isoenpt_deveptidr0_isoenpt;
#[doc = "DEVDMANXTDSC1 (rw) register accessor: Device DMA Channel Next Descriptor Address Register (n = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmanxtdsc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmanxtdsc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmanxtdsc1`]
module"]
pub type DEVDMANXTDSC1 = crate::Reg<devdmanxtdsc1::DEVDMANXTDSC1_SPEC>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod devdmanxtdsc1;
#[doc = "DEVDMAADDRESS1 (rw) register accessor: Device DMA Channel Address Register (n = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmaaddress1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmaaddress1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmaaddress1`]
module"]
pub type DEVDMAADDRESS1 = crate::Reg<devdmaaddress1::DEVDMAADDRESS1_SPEC>;
#[doc = "Device DMA Channel Address Register (n = 1)"]
pub mod devdmaaddress1;
#[doc = "DEVDMACONTROL1 (rw) register accessor: Device DMA Channel Control Register (n = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmacontrol1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmacontrol1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmacontrol1`]
module"]
pub type DEVDMACONTROL1 = crate::Reg<devdmacontrol1::DEVDMACONTROL1_SPEC>;
#[doc = "Device DMA Channel Control Register (n = 1)"]
pub mod devdmacontrol1;
#[doc = "DEVDMASTATUS1 (rw) register accessor: Device DMA Channel Status Register (n = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmastatus1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmastatus1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmastatus1`]
module"]
pub type DEVDMASTATUS1 = crate::Reg<devdmastatus1::DEVDMASTATUS1_SPEC>;
#[doc = "Device DMA Channel Status Register (n = 1)"]
pub mod devdmastatus1;
#[doc = "DEVDMANXTDSC2 (rw) register accessor: Device DMA Channel Next Descriptor Address Register (n = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmanxtdsc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmanxtdsc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmanxtdsc2`]
module"]
pub type DEVDMANXTDSC2 = crate::Reg<devdmanxtdsc2::DEVDMANXTDSC2_SPEC>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 2)"]
pub mod devdmanxtdsc2;
#[doc = "DEVDMAADDRESS2 (rw) register accessor: Device DMA Channel Address Register (n = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmaaddress2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmaaddress2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmaaddress2`]
module"]
pub type DEVDMAADDRESS2 = crate::Reg<devdmaaddress2::DEVDMAADDRESS2_SPEC>;
#[doc = "Device DMA Channel Address Register (n = 2)"]
pub mod devdmaaddress2;
#[doc = "DEVDMACONTROL2 (rw) register accessor: Device DMA Channel Control Register (n = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmacontrol2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmacontrol2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmacontrol2`]
module"]
pub type DEVDMACONTROL2 = crate::Reg<devdmacontrol2::DEVDMACONTROL2_SPEC>;
#[doc = "Device DMA Channel Control Register (n = 2)"]
pub mod devdmacontrol2;
#[doc = "DEVDMASTATUS2 (rw) register accessor: Device DMA Channel Status Register (n = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmastatus2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmastatus2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmastatus2`]
module"]
pub type DEVDMASTATUS2 = crate::Reg<devdmastatus2::DEVDMASTATUS2_SPEC>;
#[doc = "Device DMA Channel Status Register (n = 2)"]
pub mod devdmastatus2;
#[doc = "DEVDMANXTDSC3 (rw) register accessor: Device DMA Channel Next Descriptor Address Register (n = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmanxtdsc3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmanxtdsc3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmanxtdsc3`]
module"]
pub type DEVDMANXTDSC3 = crate::Reg<devdmanxtdsc3::DEVDMANXTDSC3_SPEC>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 3)"]
pub mod devdmanxtdsc3;
#[doc = "DEVDMAADDRESS3 (rw) register accessor: Device DMA Channel Address Register (n = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmaaddress3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmaaddress3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmaaddress3`]
module"]
pub type DEVDMAADDRESS3 = crate::Reg<devdmaaddress3::DEVDMAADDRESS3_SPEC>;
#[doc = "Device DMA Channel Address Register (n = 3)"]
pub mod devdmaaddress3;
#[doc = "DEVDMACONTROL3 (rw) register accessor: Device DMA Channel Control Register (n = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmacontrol3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmacontrol3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmacontrol3`]
module"]
pub type DEVDMACONTROL3 = crate::Reg<devdmacontrol3::DEVDMACONTROL3_SPEC>;
#[doc = "Device DMA Channel Control Register (n = 3)"]
pub mod devdmacontrol3;
#[doc = "DEVDMASTATUS3 (rw) register accessor: Device DMA Channel Status Register (n = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmastatus3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmastatus3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmastatus3`]
module"]
pub type DEVDMASTATUS3 = crate::Reg<devdmastatus3::DEVDMASTATUS3_SPEC>;
#[doc = "Device DMA Channel Status Register (n = 3)"]
pub mod devdmastatus3;
#[doc = "DEVDMANXTDSC4 (rw) register accessor: Device DMA Channel Next Descriptor Address Register (n = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmanxtdsc4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmanxtdsc4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmanxtdsc4`]
module"]
pub type DEVDMANXTDSC4 = crate::Reg<devdmanxtdsc4::DEVDMANXTDSC4_SPEC>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 4)"]
pub mod devdmanxtdsc4;
#[doc = "DEVDMAADDRESS4 (rw) register accessor: Device DMA Channel Address Register (n = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmaaddress4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmaaddress4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmaaddress4`]
module"]
pub type DEVDMAADDRESS4 = crate::Reg<devdmaaddress4::DEVDMAADDRESS4_SPEC>;
#[doc = "Device DMA Channel Address Register (n = 4)"]
pub mod devdmaaddress4;
#[doc = "DEVDMACONTROL4 (rw) register accessor: Device DMA Channel Control Register (n = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmacontrol4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmacontrol4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmacontrol4`]
module"]
pub type DEVDMACONTROL4 = crate::Reg<devdmacontrol4::DEVDMACONTROL4_SPEC>;
#[doc = "Device DMA Channel Control Register (n = 4)"]
pub mod devdmacontrol4;
#[doc = "DEVDMASTATUS4 (rw) register accessor: Device DMA Channel Status Register (n = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmastatus4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmastatus4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmastatus4`]
module"]
pub type DEVDMASTATUS4 = crate::Reg<devdmastatus4::DEVDMASTATUS4_SPEC>;
#[doc = "Device DMA Channel Status Register (n = 4)"]
pub mod devdmastatus4;
#[doc = "DEVDMANXTDSC5 (rw) register accessor: Device DMA Channel Next Descriptor Address Register (n = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmanxtdsc5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmanxtdsc5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmanxtdsc5`]
module"]
pub type DEVDMANXTDSC5 = crate::Reg<devdmanxtdsc5::DEVDMANXTDSC5_SPEC>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 5)"]
pub mod devdmanxtdsc5;
#[doc = "DEVDMAADDRESS5 (rw) register accessor: Device DMA Channel Address Register (n = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmaaddress5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmaaddress5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmaaddress5`]
module"]
pub type DEVDMAADDRESS5 = crate::Reg<devdmaaddress5::DEVDMAADDRESS5_SPEC>;
#[doc = "Device DMA Channel Address Register (n = 5)"]
pub mod devdmaaddress5;
#[doc = "DEVDMACONTROL5 (rw) register accessor: Device DMA Channel Control Register (n = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmacontrol5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmacontrol5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmacontrol5`]
module"]
pub type DEVDMACONTROL5 = crate::Reg<devdmacontrol5::DEVDMACONTROL5_SPEC>;
#[doc = "Device DMA Channel Control Register (n = 5)"]
pub mod devdmacontrol5;
#[doc = "DEVDMASTATUS5 (rw) register accessor: Device DMA Channel Status Register (n = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmastatus5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmastatus5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmastatus5`]
module"]
pub type DEVDMASTATUS5 = crate::Reg<devdmastatus5::DEVDMASTATUS5_SPEC>;
#[doc = "Device DMA Channel Status Register (n = 5)"]
pub mod devdmastatus5;
#[doc = "DEVDMANXTDSC6 (rw) register accessor: Device DMA Channel Next Descriptor Address Register (n = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmanxtdsc6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmanxtdsc6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmanxtdsc6`]
module"]
pub type DEVDMANXTDSC6 = crate::Reg<devdmanxtdsc6::DEVDMANXTDSC6_SPEC>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 6)"]
pub mod devdmanxtdsc6;
#[doc = "DEVDMAADDRESS6 (rw) register accessor: Device DMA Channel Address Register (n = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmaaddress6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmaaddress6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmaaddress6`]
module"]
pub type DEVDMAADDRESS6 = crate::Reg<devdmaaddress6::DEVDMAADDRESS6_SPEC>;
#[doc = "Device DMA Channel Address Register (n = 6)"]
pub mod devdmaaddress6;
#[doc = "DEVDMACONTROL6 (rw) register accessor: Device DMA Channel Control Register (n = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmacontrol6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmacontrol6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmacontrol6`]
module"]
pub type DEVDMACONTROL6 = crate::Reg<devdmacontrol6::DEVDMACONTROL6_SPEC>;
#[doc = "Device DMA Channel Control Register (n = 6)"]
pub mod devdmacontrol6;
#[doc = "DEVDMASTATUS6 (rw) register accessor: Device DMA Channel Status Register (n = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmastatus6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmastatus6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmastatus6`]
module"]
pub type DEVDMASTATUS6 = crate::Reg<devdmastatus6::DEVDMASTATUS6_SPEC>;
#[doc = "Device DMA Channel Status Register (n = 6)"]
pub mod devdmastatus6;
#[doc = "DEVDMANXTDSC7 (rw) register accessor: Device DMA Channel Next Descriptor Address Register (n = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmanxtdsc7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmanxtdsc7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmanxtdsc7`]
module"]
pub type DEVDMANXTDSC7 = crate::Reg<devdmanxtdsc7::DEVDMANXTDSC7_SPEC>;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 7)"]
pub mod devdmanxtdsc7;
#[doc = "DEVDMAADDRESS7 (rw) register accessor: Device DMA Channel Address Register (n = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmaaddress7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmaaddress7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmaaddress7`]
module"]
pub type DEVDMAADDRESS7 = crate::Reg<devdmaaddress7::DEVDMAADDRESS7_SPEC>;
#[doc = "Device DMA Channel Address Register (n = 7)"]
pub mod devdmaaddress7;
#[doc = "DEVDMACONTROL7 (rw) register accessor: Device DMA Channel Control Register (n = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmacontrol7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmacontrol7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmacontrol7`]
module"]
pub type DEVDMACONTROL7 = crate::Reg<devdmacontrol7::DEVDMACONTROL7_SPEC>;
#[doc = "Device DMA Channel Control Register (n = 7)"]
pub mod devdmacontrol7;
#[doc = "DEVDMASTATUS7 (rw) register accessor: Device DMA Channel Status Register (n = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmastatus7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmastatus7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmastatus7`]
module"]
pub type DEVDMASTATUS7 = crate::Reg<devdmastatus7::DEVDMASTATUS7_SPEC>;
#[doc = "Device DMA Channel Status Register (n = 7)"]
pub mod devdmastatus7;
#[doc = "HSTCTRL (rw) register accessor: Host General Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstctrl`]
module"]
pub type HSTCTRL = crate::Reg<hstctrl::HSTCTRL_SPEC>;
#[doc = "Host General Control Register"]
pub mod hstctrl;
#[doc = "HSTISR (r) register accessor: Host Global Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstisr`]
module"]
pub type HSTISR = crate::Reg<hstisr::HSTISR_SPEC>;
#[doc = "Host Global Interrupt Status Register"]
pub mod hstisr;
#[doc = "HSTICR (w) register accessor: Host Global Interrupt Clear Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsticr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsticr`]
module"]
pub type HSTICR = crate::Reg<hsticr::HSTICR_SPEC>;
#[doc = "Host Global Interrupt Clear Register"]
pub mod hsticr;
#[doc = "HSTIFR (w) register accessor: Host Global Interrupt Set Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstifr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstifr`]
module"]
pub type HSTIFR = crate::Reg<hstifr::HSTIFR_SPEC>;
#[doc = "Host Global Interrupt Set Register"]
pub mod hstifr;
#[doc = "HSTIMR (r) register accessor: Host Global Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstimr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstimr`]
module"]
pub type HSTIMR = crate::Reg<hstimr::HSTIMR_SPEC>;
#[doc = "Host Global Interrupt Mask Register"]
pub mod hstimr;
#[doc = "HSTIDR (w) register accessor: Host Global Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstidr`]
module"]
pub type HSTIDR = crate::Reg<hstidr::HSTIDR_SPEC>;
#[doc = "Host Global Interrupt Disable Register"]
pub mod hstidr;
#[doc = "HSTIER (w) register accessor: Host Global Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstier`]
module"]
pub type HSTIER = crate::Reg<hstier::HSTIER_SPEC>;
#[doc = "Host Global Interrupt Enable Register"]
pub mod hstier;
#[doc = "HSTPIP (rw) register accessor: Host Pipe Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpip::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpip::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpip`]
module"]
pub type HSTPIP = crate::Reg<hstpip::HSTPIP_SPEC>;
#[doc = "Host Pipe Register"]
pub mod hstpip;
#[doc = "HSTFNUM (rw) register accessor: Host Frame Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstfnum::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstfnum::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstfnum`]
module"]
pub type HSTFNUM = crate::Reg<hstfnum::HSTFNUM_SPEC>;
#[doc = "Host Frame Number Register"]
pub mod hstfnum;
#[doc = "HSTADDR1 (rw) register accessor: Host Address 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstaddr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstaddr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstaddr1`]
module"]
pub type HSTADDR1 = crate::Reg<hstaddr1::HSTADDR1_SPEC>;
#[doc = "Host Address 1 Register"]
pub mod hstaddr1;
#[doc = "HSTADDR2 (rw) register accessor: Host Address 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstaddr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstaddr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstaddr2`]
module"]
pub type HSTADDR2 = crate::Reg<hstaddr2::HSTADDR2_SPEC>;
#[doc = "Host Address 2 Register"]
pub mod hstaddr2;
#[doc = "HSTADDR3 (rw) register accessor: Host Address 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstaddr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstaddr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstaddr3`]
module"]
pub type HSTADDR3 = crate::Reg<hstaddr3::HSTADDR3_SPEC>;
#[doc = "Host Address 3 Register"]
pub mod hstaddr3;
#[doc = "HSTPIPCFG0 (rw) register accessor: Host Pipe Configuration Register (n = 0) 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipcfg0`]
module"]
pub type HSTPIPCFG0 = crate::Reg<hstpipcfg0::HSTPIPCFG0_SPEC>;
#[doc = "Host Pipe Configuration Register (n = 0) 0"]
pub mod hstpipcfg0;
#[doc = "HSTPIPCFG1 (rw) register accessor: Host Pipe Configuration Register (n = 0) 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipcfg1`]
module"]
pub type HSTPIPCFG1 = crate::Reg<hstpipcfg1::HSTPIPCFG1_SPEC>;
#[doc = "Host Pipe Configuration Register (n = 0) 1"]
pub mod hstpipcfg1;
#[doc = "HSTPIPCFG2 (rw) register accessor: Host Pipe Configuration Register (n = 0) 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipcfg2`]
module"]
pub type HSTPIPCFG2 = crate::Reg<hstpipcfg2::HSTPIPCFG2_SPEC>;
#[doc = "Host Pipe Configuration Register (n = 0) 2"]
pub mod hstpipcfg2;
#[doc = "HSTPIPCFG3 (rw) register accessor: Host Pipe Configuration Register (n = 0) 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipcfg3`]
module"]
pub type HSTPIPCFG3 = crate::Reg<hstpipcfg3::HSTPIPCFG3_SPEC>;
#[doc = "Host Pipe Configuration Register (n = 0) 3"]
pub mod hstpipcfg3;
#[doc = "HSTPIPCFG4 (rw) register accessor: Host Pipe Configuration Register (n = 0) 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipcfg4`]
module"]
pub type HSTPIPCFG4 = crate::Reg<hstpipcfg4::HSTPIPCFG4_SPEC>;
#[doc = "Host Pipe Configuration Register (n = 0) 4"]
pub mod hstpipcfg4;
#[doc = "HSTPIPCFG5 (rw) register accessor: Host Pipe Configuration Register (n = 0) 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipcfg5`]
module"]
pub type HSTPIPCFG5 = crate::Reg<hstpipcfg5::HSTPIPCFG5_SPEC>;
#[doc = "Host Pipe Configuration Register (n = 0) 5"]
pub mod hstpipcfg5;
#[doc = "HSTPIPCFG6 (rw) register accessor: Host Pipe Configuration Register (n = 0) 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg6::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipcfg6`]
module"]
pub type HSTPIPCFG6 = crate::Reg<hstpipcfg6::HSTPIPCFG6_SPEC>;
#[doc = "Host Pipe Configuration Register (n = 0) 6"]
pub mod hstpipcfg6;
#[doc = "HSTPIPCFG7 (rw) register accessor: Host Pipe Configuration Register (n = 0) 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg7::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipcfg7`]
module"]
pub type HSTPIPCFG7 = crate::Reg<hstpipcfg7::HSTPIPCFG7_SPEC>;
#[doc = "Host Pipe Configuration Register (n = 0) 7"]
pub mod hstpipcfg7;
#[doc = "HSTPIPCFG8 (rw) register accessor: Host Pipe Configuration Register (n = 0) 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg8::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipcfg8`]
module"]
pub type HSTPIPCFG8 = crate::Reg<hstpipcfg8::HSTPIPCFG8_SPEC>;
#[doc = "Host Pipe Configuration Register (n = 0) 8"]
pub mod hstpipcfg8;
#[doc = "HSTPIPCFG9 (rw) register accessor: Host Pipe Configuration Register (n = 0) 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg9::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipcfg9`]
module"]
pub type HSTPIPCFG9 = crate::Reg<hstpipcfg9::HSTPIPCFG9_SPEC>;
#[doc = "Host Pipe Configuration Register (n = 0) 9"]
pub mod hstpipcfg9;
#[doc = "HSBOHSCP_HSTPIPCFG0_HSBOHSCP (rw) register accessor: Host Pipe Configuration Register (n = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsbohscp_hstpipcfg0_hsbohscp::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsbohscp_hstpipcfg0_hsbohscp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsbohscp_hstpipcfg0_hsbohscp`]
module"]
pub type HSBOHSCP_HSTPIPCFG0_HSBOHSCP =
    crate::Reg<hsbohscp_hstpipcfg0_hsbohscp::HSBOHSCP_HSTPIPCFG0_HSBOHSCP_SPEC>;
#[doc = "Host Pipe Configuration Register (n = 0)"]
pub mod hsbohscp_hstpipcfg0_hsbohscp;
#[doc = "HSTPIPISR0 (r) register accessor: Host Pipe Status Register (n = 0) 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipisr0`]
module"]
pub type HSTPIPISR0 = crate::Reg<hstpipisr0::HSTPIPISR0_SPEC>;
#[doc = "Host Pipe Status Register (n = 0) 0"]
pub mod hstpipisr0;
#[doc = "HSTPIPISR1 (r) register accessor: Host Pipe Status Register (n = 0) 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipisr1`]
module"]
pub type HSTPIPISR1 = crate::Reg<hstpipisr1::HSTPIPISR1_SPEC>;
#[doc = "Host Pipe Status Register (n = 0) 1"]
pub mod hstpipisr1;
#[doc = "HSTPIPISR2 (r) register accessor: Host Pipe Status Register (n = 0) 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipisr2`]
module"]
pub type HSTPIPISR2 = crate::Reg<hstpipisr2::HSTPIPISR2_SPEC>;
#[doc = "Host Pipe Status Register (n = 0) 2"]
pub mod hstpipisr2;
#[doc = "HSTPIPISR3 (r) register accessor: Host Pipe Status Register (n = 0) 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipisr3`]
module"]
pub type HSTPIPISR3 = crate::Reg<hstpipisr3::HSTPIPISR3_SPEC>;
#[doc = "Host Pipe Status Register (n = 0) 3"]
pub mod hstpipisr3;
#[doc = "HSTPIPISR4 (r) register accessor: Host Pipe Status Register (n = 0) 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipisr4`]
module"]
pub type HSTPIPISR4 = crate::Reg<hstpipisr4::HSTPIPISR4_SPEC>;
#[doc = "Host Pipe Status Register (n = 0) 4"]
pub mod hstpipisr4;
#[doc = "HSTPIPISR5 (r) register accessor: Host Pipe Status Register (n = 0) 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipisr5`]
module"]
pub type HSTPIPISR5 = crate::Reg<hstpipisr5::HSTPIPISR5_SPEC>;
#[doc = "Host Pipe Status Register (n = 0) 5"]
pub mod hstpipisr5;
#[doc = "HSTPIPISR6 (r) register accessor: Host Pipe Status Register (n = 0) 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipisr6`]
module"]
pub type HSTPIPISR6 = crate::Reg<hstpipisr6::HSTPIPISR6_SPEC>;
#[doc = "Host Pipe Status Register (n = 0) 6"]
pub mod hstpipisr6;
#[doc = "HSTPIPISR7 (r) register accessor: Host Pipe Status Register (n = 0) 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipisr7`]
module"]
pub type HSTPIPISR7 = crate::Reg<hstpipisr7::HSTPIPISR7_SPEC>;
#[doc = "Host Pipe Status Register (n = 0) 7"]
pub mod hstpipisr7;
#[doc = "HSTPIPISR8 (r) register accessor: Host Pipe Status Register (n = 0) 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipisr8`]
module"]
pub type HSTPIPISR8 = crate::Reg<hstpipisr8::HSTPIPISR8_SPEC>;
#[doc = "Host Pipe Status Register (n = 0) 8"]
pub mod hstpipisr8;
#[doc = "HSTPIPISR9 (r) register accessor: Host Pipe Status Register (n = 0) 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipisr9`]
module"]
pub type HSTPIPISR9 = crate::Reg<hstpipisr9::HSTPIPISR9_SPEC>;
#[doc = "Host Pipe Status Register (n = 0) 9"]
pub mod hstpipisr9;
#[doc = "INTPIPES_HSTPIPISR0_INTPIPES (r) register accessor: Host Pipe Status Register (n = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intpipes_hstpipisr0_intpipes::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intpipes_hstpipisr0_intpipes`]
module"]
pub type INTPIPES_HSTPIPISR0_INTPIPES =
    crate::Reg<intpipes_hstpipisr0_intpipes::INTPIPES_HSTPIPISR0_INTPIPES_SPEC>;
#[doc = "Host Pipe Status Register (n = 0)"]
pub mod intpipes_hstpipisr0_intpipes;
#[doc = "ISOPIPES_HSTPIPISR0_ISOPIPES (r) register accessor: Host Pipe Status Register (n = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isopipes_hstpipisr0_isopipes::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`isopipes_hstpipisr0_isopipes`]
module"]
pub type ISOPIPES_HSTPIPISR0_ISOPIPES =
    crate::Reg<isopipes_hstpipisr0_isopipes::ISOPIPES_HSTPIPISR0_ISOPIPES_SPEC>;
#[doc = "Host Pipe Status Register (n = 0)"]
pub mod isopipes_hstpipisr0_isopipes;
#[doc = "HSTPIPICR0 (w) register accessor: Host Pipe Clear Register (n = 0) 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipicr0`]
module"]
pub type HSTPIPICR0 = crate::Reg<hstpipicr0::HSTPIPICR0_SPEC>;
#[doc = "Host Pipe Clear Register (n = 0) 0"]
pub mod hstpipicr0;
#[doc = "HSTPIPICR1 (w) register accessor: Host Pipe Clear Register (n = 0) 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipicr1`]
module"]
pub type HSTPIPICR1 = crate::Reg<hstpipicr1::HSTPIPICR1_SPEC>;
#[doc = "Host Pipe Clear Register (n = 0) 1"]
pub mod hstpipicr1;
#[doc = "HSTPIPICR2 (w) register accessor: Host Pipe Clear Register (n = 0) 2\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipicr2`]
module"]
pub type HSTPIPICR2 = crate::Reg<hstpipicr2::HSTPIPICR2_SPEC>;
#[doc = "Host Pipe Clear Register (n = 0) 2"]
pub mod hstpipicr2;
#[doc = "HSTPIPICR3 (w) register accessor: Host Pipe Clear Register (n = 0) 3\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipicr3`]
module"]
pub type HSTPIPICR3 = crate::Reg<hstpipicr3::HSTPIPICR3_SPEC>;
#[doc = "Host Pipe Clear Register (n = 0) 3"]
pub mod hstpipicr3;
#[doc = "HSTPIPICR4 (w) register accessor: Host Pipe Clear Register (n = 0) 4\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipicr4`]
module"]
pub type HSTPIPICR4 = crate::Reg<hstpipicr4::HSTPIPICR4_SPEC>;
#[doc = "Host Pipe Clear Register (n = 0) 4"]
pub mod hstpipicr4;
#[doc = "HSTPIPICR5 (w) register accessor: Host Pipe Clear Register (n = 0) 5\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipicr5`]
module"]
pub type HSTPIPICR5 = crate::Reg<hstpipicr5::HSTPIPICR5_SPEC>;
#[doc = "Host Pipe Clear Register (n = 0) 5"]
pub mod hstpipicr5;
#[doc = "HSTPIPICR6 (w) register accessor: Host Pipe Clear Register (n = 0) 6\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipicr6`]
module"]
pub type HSTPIPICR6 = crate::Reg<hstpipicr6::HSTPIPICR6_SPEC>;
#[doc = "Host Pipe Clear Register (n = 0) 6"]
pub mod hstpipicr6;
#[doc = "HSTPIPICR7 (w) register accessor: Host Pipe Clear Register (n = 0) 7\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipicr7`]
module"]
pub type HSTPIPICR7 = crate::Reg<hstpipicr7::HSTPIPICR7_SPEC>;
#[doc = "Host Pipe Clear Register (n = 0) 7"]
pub mod hstpipicr7;
#[doc = "HSTPIPICR8 (w) register accessor: Host Pipe Clear Register (n = 0) 8\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipicr8`]
module"]
pub type HSTPIPICR8 = crate::Reg<hstpipicr8::HSTPIPICR8_SPEC>;
#[doc = "Host Pipe Clear Register (n = 0) 8"]
pub mod hstpipicr8;
#[doc = "HSTPIPICR9 (w) register accessor: Host Pipe Clear Register (n = 0) 9\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr9::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipicr9`]
module"]
pub type HSTPIPICR9 = crate::Reg<hstpipicr9::HSTPIPICR9_SPEC>;
#[doc = "Host Pipe Clear Register (n = 0) 9"]
pub mod hstpipicr9;
#[doc = "INTPIPES_HSTPIPICR0_INTPIPES (w) register accessor: Host Pipe Clear Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intpipes_hstpipicr0_intpipes::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intpipes_hstpipicr0_intpipes`]
module"]
pub type INTPIPES_HSTPIPICR0_INTPIPES =
    crate::Reg<intpipes_hstpipicr0_intpipes::INTPIPES_HSTPIPICR0_INTPIPES_SPEC>;
#[doc = "Host Pipe Clear Register (n = 0)"]
pub mod intpipes_hstpipicr0_intpipes;
#[doc = "ISOPIPES_HSTPIPICR0_ISOPIPES (w) register accessor: Host Pipe Clear Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isopipes_hstpipicr0_isopipes::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`isopipes_hstpipicr0_isopipes`]
module"]
pub type ISOPIPES_HSTPIPICR0_ISOPIPES =
    crate::Reg<isopipes_hstpipicr0_isopipes::ISOPIPES_HSTPIPICR0_ISOPIPES_SPEC>;
#[doc = "Host Pipe Clear Register (n = 0)"]
pub mod isopipes_hstpipicr0_isopipes;
#[doc = "HSTPIPIFR0 (w) register accessor: Host Pipe Set Register (n = 0) 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipifr0`]
module"]
pub type HSTPIPIFR0 = crate::Reg<hstpipifr0::HSTPIPIFR0_SPEC>;
#[doc = "Host Pipe Set Register (n = 0) 0"]
pub mod hstpipifr0;
#[doc = "HSTPIPIFR1 (w) register accessor: Host Pipe Set Register (n = 0) 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipifr1`]
module"]
pub type HSTPIPIFR1 = crate::Reg<hstpipifr1::HSTPIPIFR1_SPEC>;
#[doc = "Host Pipe Set Register (n = 0) 1"]
pub mod hstpipifr1;
#[doc = "HSTPIPIFR2 (w) register accessor: Host Pipe Set Register (n = 0) 2\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipifr2`]
module"]
pub type HSTPIPIFR2 = crate::Reg<hstpipifr2::HSTPIPIFR2_SPEC>;
#[doc = "Host Pipe Set Register (n = 0) 2"]
pub mod hstpipifr2;
#[doc = "HSTPIPIFR3 (w) register accessor: Host Pipe Set Register (n = 0) 3\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipifr3`]
module"]
pub type HSTPIPIFR3 = crate::Reg<hstpipifr3::HSTPIPIFR3_SPEC>;
#[doc = "Host Pipe Set Register (n = 0) 3"]
pub mod hstpipifr3;
#[doc = "HSTPIPIFR4 (w) register accessor: Host Pipe Set Register (n = 0) 4\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipifr4`]
module"]
pub type HSTPIPIFR4 = crate::Reg<hstpipifr4::HSTPIPIFR4_SPEC>;
#[doc = "Host Pipe Set Register (n = 0) 4"]
pub mod hstpipifr4;
#[doc = "HSTPIPIFR5 (w) register accessor: Host Pipe Set Register (n = 0) 5\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipifr5`]
module"]
pub type HSTPIPIFR5 = crate::Reg<hstpipifr5::HSTPIPIFR5_SPEC>;
#[doc = "Host Pipe Set Register (n = 0) 5"]
pub mod hstpipifr5;
#[doc = "HSTPIPIFR6 (w) register accessor: Host Pipe Set Register (n = 0) 6\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipifr6`]
module"]
pub type HSTPIPIFR6 = crate::Reg<hstpipifr6::HSTPIPIFR6_SPEC>;
#[doc = "Host Pipe Set Register (n = 0) 6"]
pub mod hstpipifr6;
#[doc = "HSTPIPIFR7 (w) register accessor: Host Pipe Set Register (n = 0) 7\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipifr7`]
module"]
pub type HSTPIPIFR7 = crate::Reg<hstpipifr7::HSTPIPIFR7_SPEC>;
#[doc = "Host Pipe Set Register (n = 0) 7"]
pub mod hstpipifr7;
#[doc = "HSTPIPIFR8 (w) register accessor: Host Pipe Set Register (n = 0) 8\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipifr8`]
module"]
pub type HSTPIPIFR8 = crate::Reg<hstpipifr8::HSTPIPIFR8_SPEC>;
#[doc = "Host Pipe Set Register (n = 0) 8"]
pub mod hstpipifr8;
#[doc = "HSTPIPIFR9 (w) register accessor: Host Pipe Set Register (n = 0) 9\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr9::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipifr9`]
module"]
pub type HSTPIPIFR9 = crate::Reg<hstpipifr9::HSTPIPIFR9_SPEC>;
#[doc = "Host Pipe Set Register (n = 0) 9"]
pub mod hstpipifr9;
#[doc = "INTPIPES_HSTPIPIFR0_INTPIPES (w) register accessor: Host Pipe Set Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intpipes_hstpipifr0_intpipes::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intpipes_hstpipifr0_intpipes`]
module"]
pub type INTPIPES_HSTPIPIFR0_INTPIPES =
    crate::Reg<intpipes_hstpipifr0_intpipes::INTPIPES_HSTPIPIFR0_INTPIPES_SPEC>;
#[doc = "Host Pipe Set Register (n = 0)"]
pub mod intpipes_hstpipifr0_intpipes;
#[doc = "ISOPIPES_HSTPIPIFR0_ISOPIPES (w) register accessor: Host Pipe Set Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isopipes_hstpipifr0_isopipes::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`isopipes_hstpipifr0_isopipes`]
module"]
pub type ISOPIPES_HSTPIPIFR0_ISOPIPES =
    crate::Reg<isopipes_hstpipifr0_isopipes::ISOPIPES_HSTPIPIFR0_ISOPIPES_SPEC>;
#[doc = "Host Pipe Set Register (n = 0)"]
pub mod isopipes_hstpipifr0_isopipes;
#[doc = "HSTPIPIMR0 (r) register accessor: Host Pipe Mask Register (n = 0) 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipimr0`]
module"]
pub type HSTPIPIMR0 = crate::Reg<hstpipimr0::HSTPIPIMR0_SPEC>;
#[doc = "Host Pipe Mask Register (n = 0) 0"]
pub mod hstpipimr0;
#[doc = "HSTPIPIMR1 (r) register accessor: Host Pipe Mask Register (n = 0) 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipimr1`]
module"]
pub type HSTPIPIMR1 = crate::Reg<hstpipimr1::HSTPIPIMR1_SPEC>;
#[doc = "Host Pipe Mask Register (n = 0) 1"]
pub mod hstpipimr1;
#[doc = "HSTPIPIMR2 (r) register accessor: Host Pipe Mask Register (n = 0) 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipimr2`]
module"]
pub type HSTPIPIMR2 = crate::Reg<hstpipimr2::HSTPIPIMR2_SPEC>;
#[doc = "Host Pipe Mask Register (n = 0) 2"]
pub mod hstpipimr2;
#[doc = "HSTPIPIMR3 (r) register accessor: Host Pipe Mask Register (n = 0) 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipimr3`]
module"]
pub type HSTPIPIMR3 = crate::Reg<hstpipimr3::HSTPIPIMR3_SPEC>;
#[doc = "Host Pipe Mask Register (n = 0) 3"]
pub mod hstpipimr3;
#[doc = "HSTPIPIMR4 (r) register accessor: Host Pipe Mask Register (n = 0) 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipimr4`]
module"]
pub type HSTPIPIMR4 = crate::Reg<hstpipimr4::HSTPIPIMR4_SPEC>;
#[doc = "Host Pipe Mask Register (n = 0) 4"]
pub mod hstpipimr4;
#[doc = "HSTPIPIMR5 (r) register accessor: Host Pipe Mask Register (n = 0) 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipimr5`]
module"]
pub type HSTPIPIMR5 = crate::Reg<hstpipimr5::HSTPIPIMR5_SPEC>;
#[doc = "Host Pipe Mask Register (n = 0) 5"]
pub mod hstpipimr5;
#[doc = "HSTPIPIMR6 (r) register accessor: Host Pipe Mask Register (n = 0) 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipimr6`]
module"]
pub type HSTPIPIMR6 = crate::Reg<hstpipimr6::HSTPIPIMR6_SPEC>;
#[doc = "Host Pipe Mask Register (n = 0) 6"]
pub mod hstpipimr6;
#[doc = "HSTPIPIMR7 (r) register accessor: Host Pipe Mask Register (n = 0) 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipimr7`]
module"]
pub type HSTPIPIMR7 = crate::Reg<hstpipimr7::HSTPIPIMR7_SPEC>;
#[doc = "Host Pipe Mask Register (n = 0) 7"]
pub mod hstpipimr7;
#[doc = "HSTPIPIMR8 (r) register accessor: Host Pipe Mask Register (n = 0) 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipimr8`]
module"]
pub type HSTPIPIMR8 = crate::Reg<hstpipimr8::HSTPIPIMR8_SPEC>;
#[doc = "Host Pipe Mask Register (n = 0) 8"]
pub mod hstpipimr8;
#[doc = "HSTPIPIMR9 (r) register accessor: Host Pipe Mask Register (n = 0) 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipimr9`]
module"]
pub type HSTPIPIMR9 = crate::Reg<hstpipimr9::HSTPIPIMR9_SPEC>;
#[doc = "Host Pipe Mask Register (n = 0) 9"]
pub mod hstpipimr9;
#[doc = "INTPIPES_HSTPIPIMR0_INTPIPES (r) register accessor: Host Pipe Mask Register (n = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intpipes_hstpipimr0_intpipes::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intpipes_hstpipimr0_intpipes`]
module"]
pub type INTPIPES_HSTPIPIMR0_INTPIPES =
    crate::Reg<intpipes_hstpipimr0_intpipes::INTPIPES_HSTPIPIMR0_INTPIPES_SPEC>;
#[doc = "Host Pipe Mask Register (n = 0)"]
pub mod intpipes_hstpipimr0_intpipes;
#[doc = "ISOPIPES_HSTPIPIMR0_ISOPIPES (r) register accessor: Host Pipe Mask Register (n = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isopipes_hstpipimr0_isopipes::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`isopipes_hstpipimr0_isopipes`]
module"]
pub type ISOPIPES_HSTPIPIMR0_ISOPIPES =
    crate::Reg<isopipes_hstpipimr0_isopipes::ISOPIPES_HSTPIPIMR0_ISOPIPES_SPEC>;
#[doc = "Host Pipe Mask Register (n = 0)"]
pub mod isopipes_hstpipimr0_isopipes;
#[doc = "HSTPIPIER0 (w) register accessor: Host Pipe Enable Register (n = 0) 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipier0`]
module"]
pub type HSTPIPIER0 = crate::Reg<hstpipier0::HSTPIPIER0_SPEC>;
#[doc = "Host Pipe Enable Register (n = 0) 0"]
pub mod hstpipier0;
#[doc = "HSTPIPIER1 (w) register accessor: Host Pipe Enable Register (n = 0) 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipier1`]
module"]
pub type HSTPIPIER1 = crate::Reg<hstpipier1::HSTPIPIER1_SPEC>;
#[doc = "Host Pipe Enable Register (n = 0) 1"]
pub mod hstpipier1;
#[doc = "HSTPIPIER2 (w) register accessor: Host Pipe Enable Register (n = 0) 2\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipier2`]
module"]
pub type HSTPIPIER2 = crate::Reg<hstpipier2::HSTPIPIER2_SPEC>;
#[doc = "Host Pipe Enable Register (n = 0) 2"]
pub mod hstpipier2;
#[doc = "HSTPIPIER3 (w) register accessor: Host Pipe Enable Register (n = 0) 3\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipier3`]
module"]
pub type HSTPIPIER3 = crate::Reg<hstpipier3::HSTPIPIER3_SPEC>;
#[doc = "Host Pipe Enable Register (n = 0) 3"]
pub mod hstpipier3;
#[doc = "HSTPIPIER4 (w) register accessor: Host Pipe Enable Register (n = 0) 4\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipier4`]
module"]
pub type HSTPIPIER4 = crate::Reg<hstpipier4::HSTPIPIER4_SPEC>;
#[doc = "Host Pipe Enable Register (n = 0) 4"]
pub mod hstpipier4;
#[doc = "HSTPIPIER5 (w) register accessor: Host Pipe Enable Register (n = 0) 5\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipier5`]
module"]
pub type HSTPIPIER5 = crate::Reg<hstpipier5::HSTPIPIER5_SPEC>;
#[doc = "Host Pipe Enable Register (n = 0) 5"]
pub mod hstpipier5;
#[doc = "HSTPIPIER6 (w) register accessor: Host Pipe Enable Register (n = 0) 6\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipier6`]
module"]
pub type HSTPIPIER6 = crate::Reg<hstpipier6::HSTPIPIER6_SPEC>;
#[doc = "Host Pipe Enable Register (n = 0) 6"]
pub mod hstpipier6;
#[doc = "HSTPIPIER7 (w) register accessor: Host Pipe Enable Register (n = 0) 7\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipier7`]
module"]
pub type HSTPIPIER7 = crate::Reg<hstpipier7::HSTPIPIER7_SPEC>;
#[doc = "Host Pipe Enable Register (n = 0) 7"]
pub mod hstpipier7;
#[doc = "HSTPIPIER8 (w) register accessor: Host Pipe Enable Register (n = 0) 8\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipier8`]
module"]
pub type HSTPIPIER8 = crate::Reg<hstpipier8::HSTPIPIER8_SPEC>;
#[doc = "Host Pipe Enable Register (n = 0) 8"]
pub mod hstpipier8;
#[doc = "HSTPIPIER9 (w) register accessor: Host Pipe Enable Register (n = 0) 9\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier9::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipier9`]
module"]
pub type HSTPIPIER9 = crate::Reg<hstpipier9::HSTPIPIER9_SPEC>;
#[doc = "Host Pipe Enable Register (n = 0) 9"]
pub mod hstpipier9;
#[doc = "INTPIPES_HSTPIPIER0_INTPIPES (w) register accessor: Host Pipe Enable Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intpipes_hstpipier0_intpipes::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intpipes_hstpipier0_intpipes`]
module"]
pub type INTPIPES_HSTPIPIER0_INTPIPES =
    crate::Reg<intpipes_hstpipier0_intpipes::INTPIPES_HSTPIPIER0_INTPIPES_SPEC>;
#[doc = "Host Pipe Enable Register (n = 0)"]
pub mod intpipes_hstpipier0_intpipes;
#[doc = "ISOPIPES_HSTPIPIER0_ISOPIPES (w) register accessor: Host Pipe Enable Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isopipes_hstpipier0_isopipes::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`isopipes_hstpipier0_isopipes`]
module"]
pub type ISOPIPES_HSTPIPIER0_ISOPIPES =
    crate::Reg<isopipes_hstpipier0_isopipes::ISOPIPES_HSTPIPIER0_ISOPIPES_SPEC>;
#[doc = "Host Pipe Enable Register (n = 0)"]
pub mod isopipes_hstpipier0_isopipes;
#[doc = "HSTPIPIDR0 (w) register accessor: Host Pipe Disable Register (n = 0) 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipidr0`]
module"]
pub type HSTPIPIDR0 = crate::Reg<hstpipidr0::HSTPIPIDR0_SPEC>;
#[doc = "Host Pipe Disable Register (n = 0) 0"]
pub mod hstpipidr0;
#[doc = "HSTPIPIDR1 (w) register accessor: Host Pipe Disable Register (n = 0) 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipidr1`]
module"]
pub type HSTPIPIDR1 = crate::Reg<hstpipidr1::HSTPIPIDR1_SPEC>;
#[doc = "Host Pipe Disable Register (n = 0) 1"]
pub mod hstpipidr1;
#[doc = "HSTPIPIDR2 (w) register accessor: Host Pipe Disable Register (n = 0) 2\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipidr2`]
module"]
pub type HSTPIPIDR2 = crate::Reg<hstpipidr2::HSTPIPIDR2_SPEC>;
#[doc = "Host Pipe Disable Register (n = 0) 2"]
pub mod hstpipidr2;
#[doc = "HSTPIPIDR3 (w) register accessor: Host Pipe Disable Register (n = 0) 3\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipidr3`]
module"]
pub type HSTPIPIDR3 = crate::Reg<hstpipidr3::HSTPIPIDR3_SPEC>;
#[doc = "Host Pipe Disable Register (n = 0) 3"]
pub mod hstpipidr3;
#[doc = "HSTPIPIDR4 (w) register accessor: Host Pipe Disable Register (n = 0) 4\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipidr4`]
module"]
pub type HSTPIPIDR4 = crate::Reg<hstpipidr4::HSTPIPIDR4_SPEC>;
#[doc = "Host Pipe Disable Register (n = 0) 4"]
pub mod hstpipidr4;
#[doc = "HSTPIPIDR5 (w) register accessor: Host Pipe Disable Register (n = 0) 5\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipidr5`]
module"]
pub type HSTPIPIDR5 = crate::Reg<hstpipidr5::HSTPIPIDR5_SPEC>;
#[doc = "Host Pipe Disable Register (n = 0) 5"]
pub mod hstpipidr5;
#[doc = "HSTPIPIDR6 (w) register accessor: Host Pipe Disable Register (n = 0) 6\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipidr6`]
module"]
pub type HSTPIPIDR6 = crate::Reg<hstpipidr6::HSTPIPIDR6_SPEC>;
#[doc = "Host Pipe Disable Register (n = 0) 6"]
pub mod hstpipidr6;
#[doc = "HSTPIPIDR7 (w) register accessor: Host Pipe Disable Register (n = 0) 7\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipidr7`]
module"]
pub type HSTPIPIDR7 = crate::Reg<hstpipidr7::HSTPIPIDR7_SPEC>;
#[doc = "Host Pipe Disable Register (n = 0) 7"]
pub mod hstpipidr7;
#[doc = "HSTPIPIDR8 (w) register accessor: Host Pipe Disable Register (n = 0) 8\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipidr8`]
module"]
pub type HSTPIPIDR8 = crate::Reg<hstpipidr8::HSTPIPIDR8_SPEC>;
#[doc = "Host Pipe Disable Register (n = 0) 8"]
pub mod hstpipidr8;
#[doc = "HSTPIPIDR9 (w) register accessor: Host Pipe Disable Register (n = 0) 9\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr9::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipidr9`]
module"]
pub type HSTPIPIDR9 = crate::Reg<hstpipidr9::HSTPIPIDR9_SPEC>;
#[doc = "Host Pipe Disable Register (n = 0) 9"]
pub mod hstpipidr9;
#[doc = "INTPIPES_HSTPIPIDR0_INTPIPES (w) register accessor: Host Pipe Disable Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intpipes_hstpipidr0_intpipes::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intpipes_hstpipidr0_intpipes`]
module"]
pub type INTPIPES_HSTPIPIDR0_INTPIPES =
    crate::Reg<intpipes_hstpipidr0_intpipes::INTPIPES_HSTPIPIDR0_INTPIPES_SPEC>;
#[doc = "Host Pipe Disable Register (n = 0)"]
pub mod intpipes_hstpipidr0_intpipes;
#[doc = "ISOPIPES_HSTPIPIDR0_ISOPIPES (w) register accessor: Host Pipe Disable Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isopipes_hstpipidr0_isopipes::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`isopipes_hstpipidr0_isopipes`]
module"]
pub type ISOPIPES_HSTPIPIDR0_ISOPIPES =
    crate::Reg<isopipes_hstpipidr0_isopipes::ISOPIPES_HSTPIPIDR0_ISOPIPES_SPEC>;
#[doc = "Host Pipe Disable Register (n = 0)"]
pub mod isopipes_hstpipidr0_isopipes;
#[doc = "HSTPIPINRQ0 (rw) register accessor: Host Pipe IN Request Register (n = 0) 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipinrq0`]
module"]
pub type HSTPIPINRQ0 = crate::Reg<hstpipinrq0::HSTPIPINRQ0_SPEC>;
#[doc = "Host Pipe IN Request Register (n = 0) 0"]
pub mod hstpipinrq0;
#[doc = "HSTPIPINRQ1 (rw) register accessor: Host Pipe IN Request Register (n = 0) 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipinrq1`]
module"]
pub type HSTPIPINRQ1 = crate::Reg<hstpipinrq1::HSTPIPINRQ1_SPEC>;
#[doc = "Host Pipe IN Request Register (n = 0) 1"]
pub mod hstpipinrq1;
#[doc = "HSTPIPINRQ2 (rw) register accessor: Host Pipe IN Request Register (n = 0) 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipinrq2`]
module"]
pub type HSTPIPINRQ2 = crate::Reg<hstpipinrq2::HSTPIPINRQ2_SPEC>;
#[doc = "Host Pipe IN Request Register (n = 0) 2"]
pub mod hstpipinrq2;
#[doc = "HSTPIPINRQ3 (rw) register accessor: Host Pipe IN Request Register (n = 0) 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipinrq3`]
module"]
pub type HSTPIPINRQ3 = crate::Reg<hstpipinrq3::HSTPIPINRQ3_SPEC>;
#[doc = "Host Pipe IN Request Register (n = 0) 3"]
pub mod hstpipinrq3;
#[doc = "HSTPIPINRQ4 (rw) register accessor: Host Pipe IN Request Register (n = 0) 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipinrq4`]
module"]
pub type HSTPIPINRQ4 = crate::Reg<hstpipinrq4::HSTPIPINRQ4_SPEC>;
#[doc = "Host Pipe IN Request Register (n = 0) 4"]
pub mod hstpipinrq4;
#[doc = "HSTPIPINRQ5 (rw) register accessor: Host Pipe IN Request Register (n = 0) 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipinrq5`]
module"]
pub type HSTPIPINRQ5 = crate::Reg<hstpipinrq5::HSTPIPINRQ5_SPEC>;
#[doc = "Host Pipe IN Request Register (n = 0) 5"]
pub mod hstpipinrq5;
#[doc = "HSTPIPINRQ6 (rw) register accessor: Host Pipe IN Request Register (n = 0) 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq6::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipinrq6`]
module"]
pub type HSTPIPINRQ6 = crate::Reg<hstpipinrq6::HSTPIPINRQ6_SPEC>;
#[doc = "Host Pipe IN Request Register (n = 0) 6"]
pub mod hstpipinrq6;
#[doc = "HSTPIPINRQ7 (rw) register accessor: Host Pipe IN Request Register (n = 0) 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq7::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipinrq7`]
module"]
pub type HSTPIPINRQ7 = crate::Reg<hstpipinrq7::HSTPIPINRQ7_SPEC>;
#[doc = "Host Pipe IN Request Register (n = 0) 7"]
pub mod hstpipinrq7;
#[doc = "HSTPIPINRQ8 (rw) register accessor: Host Pipe IN Request Register (n = 0) 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq8::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipinrq8`]
module"]
pub type HSTPIPINRQ8 = crate::Reg<hstpipinrq8::HSTPIPINRQ8_SPEC>;
#[doc = "Host Pipe IN Request Register (n = 0) 8"]
pub mod hstpipinrq8;
#[doc = "HSTPIPINRQ9 (rw) register accessor: Host Pipe IN Request Register (n = 0) 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq9::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipinrq9`]
module"]
pub type HSTPIPINRQ9 = crate::Reg<hstpipinrq9::HSTPIPINRQ9_SPEC>;
#[doc = "Host Pipe IN Request Register (n = 0) 9"]
pub mod hstpipinrq9;
#[doc = "HSTPIPERR0 (rw) register accessor: Host Pipe Error Register (n = 0) 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpiperr0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpiperr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpiperr0`]
module"]
pub type HSTPIPERR0 = crate::Reg<hstpiperr0::HSTPIPERR0_SPEC>;
#[doc = "Host Pipe Error Register (n = 0) 0"]
pub mod hstpiperr0;
#[doc = "HSTPIPERR1 (rw) register accessor: Host Pipe Error Register (n = 0) 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpiperr1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpiperr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpiperr1`]
module"]
pub type HSTPIPERR1 = crate::Reg<hstpiperr1::HSTPIPERR1_SPEC>;
#[doc = "Host Pipe Error Register (n = 0) 1"]
pub mod hstpiperr1;
#[doc = "HSTPIPERR2 (rw) register accessor: Host Pipe Error Register (n = 0) 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpiperr2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpiperr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpiperr2`]
module"]
pub type HSTPIPERR2 = crate::Reg<hstpiperr2::HSTPIPERR2_SPEC>;
#[doc = "Host Pipe Error Register (n = 0) 2"]
pub mod hstpiperr2;
#[doc = "HSTPIPERR3 (rw) register accessor: Host Pipe Error Register (n = 0) 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpiperr3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpiperr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpiperr3`]
module"]
pub type HSTPIPERR3 = crate::Reg<hstpiperr3::HSTPIPERR3_SPEC>;
#[doc = "Host Pipe Error Register (n = 0) 3"]
pub mod hstpiperr3;
#[doc = "HSTPIPERR4 (rw) register accessor: Host Pipe Error Register (n = 0) 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpiperr4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpiperr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpiperr4`]
module"]
pub type HSTPIPERR4 = crate::Reg<hstpiperr4::HSTPIPERR4_SPEC>;
#[doc = "Host Pipe Error Register (n = 0) 4"]
pub mod hstpiperr4;
#[doc = "HSTPIPERR5 (rw) register accessor: Host Pipe Error Register (n = 0) 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpiperr5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpiperr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpiperr5`]
module"]
pub type HSTPIPERR5 = crate::Reg<hstpiperr5::HSTPIPERR5_SPEC>;
#[doc = "Host Pipe Error Register (n = 0) 5"]
pub mod hstpiperr5;
#[doc = "HSTPIPERR6 (rw) register accessor: Host Pipe Error Register (n = 0) 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpiperr6::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpiperr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpiperr6`]
module"]
pub type HSTPIPERR6 = crate::Reg<hstpiperr6::HSTPIPERR6_SPEC>;
#[doc = "Host Pipe Error Register (n = 0) 6"]
pub mod hstpiperr6;
#[doc = "HSTPIPERR7 (rw) register accessor: Host Pipe Error Register (n = 0) 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpiperr7::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpiperr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpiperr7`]
module"]
pub type HSTPIPERR7 = crate::Reg<hstpiperr7::HSTPIPERR7_SPEC>;
#[doc = "Host Pipe Error Register (n = 0) 7"]
pub mod hstpiperr7;
#[doc = "HSTPIPERR8 (rw) register accessor: Host Pipe Error Register (n = 0) 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpiperr8::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpiperr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpiperr8`]
module"]
pub type HSTPIPERR8 = crate::Reg<hstpiperr8::HSTPIPERR8_SPEC>;
#[doc = "Host Pipe Error Register (n = 0) 8"]
pub mod hstpiperr8;
#[doc = "HSTPIPERR9 (rw) register accessor: Host Pipe Error Register (n = 0) 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpiperr9::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpiperr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpiperr9`]
module"]
pub type HSTPIPERR9 = crate::Reg<hstpiperr9::HSTPIPERR9_SPEC>;
#[doc = "Host Pipe Error Register (n = 0) 9"]
pub mod hstpiperr9;
#[doc = "HSTDMANXTDSC1 (rw) register accessor: Host DMA Channel Next Descriptor Address Register (n = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmanxtdsc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmanxtdsc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmanxtdsc1`]
module"]
pub type HSTDMANXTDSC1 = crate::Reg<hstdmanxtdsc1::HSTDMANXTDSC1_SPEC>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod hstdmanxtdsc1;
#[doc = "HSTDMAADDRESS1 (rw) register accessor: Host DMA Channel Address Register (n = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmaaddress1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmaaddress1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmaaddress1`]
module"]
pub type HSTDMAADDRESS1 = crate::Reg<hstdmaaddress1::HSTDMAADDRESS1_SPEC>;
#[doc = "Host DMA Channel Address Register (n = 1)"]
pub mod hstdmaaddress1;
#[doc = "HSTDMACONTROL1 (rw) register accessor: Host DMA Channel Control Register (n = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmacontrol1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmacontrol1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmacontrol1`]
module"]
pub type HSTDMACONTROL1 = crate::Reg<hstdmacontrol1::HSTDMACONTROL1_SPEC>;
#[doc = "Host DMA Channel Control Register (n = 1)"]
pub mod hstdmacontrol1;
#[doc = "HSTDMASTATUS1 (rw) register accessor: Host DMA Channel Status Register (n = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmastatus1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmastatus1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmastatus1`]
module"]
pub type HSTDMASTATUS1 = crate::Reg<hstdmastatus1::HSTDMASTATUS1_SPEC>;
#[doc = "Host DMA Channel Status Register (n = 1)"]
pub mod hstdmastatus1;
#[doc = "HSTDMANXTDSC2 (rw) register accessor: Host DMA Channel Next Descriptor Address Register (n = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmanxtdsc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmanxtdsc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmanxtdsc2`]
module"]
pub type HSTDMANXTDSC2 = crate::Reg<hstdmanxtdsc2::HSTDMANXTDSC2_SPEC>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 2)"]
pub mod hstdmanxtdsc2;
#[doc = "HSTDMAADDRESS2 (rw) register accessor: Host DMA Channel Address Register (n = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmaaddress2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmaaddress2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmaaddress2`]
module"]
pub type HSTDMAADDRESS2 = crate::Reg<hstdmaaddress2::HSTDMAADDRESS2_SPEC>;
#[doc = "Host DMA Channel Address Register (n = 2)"]
pub mod hstdmaaddress2;
#[doc = "HSTDMACONTROL2 (rw) register accessor: Host DMA Channel Control Register (n = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmacontrol2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmacontrol2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmacontrol2`]
module"]
pub type HSTDMACONTROL2 = crate::Reg<hstdmacontrol2::HSTDMACONTROL2_SPEC>;
#[doc = "Host DMA Channel Control Register (n = 2)"]
pub mod hstdmacontrol2;
#[doc = "HSTDMASTATUS2 (rw) register accessor: Host DMA Channel Status Register (n = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmastatus2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmastatus2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmastatus2`]
module"]
pub type HSTDMASTATUS2 = crate::Reg<hstdmastatus2::HSTDMASTATUS2_SPEC>;
#[doc = "Host DMA Channel Status Register (n = 2)"]
pub mod hstdmastatus2;
#[doc = "HSTDMANXTDSC3 (rw) register accessor: Host DMA Channel Next Descriptor Address Register (n = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmanxtdsc3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmanxtdsc3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmanxtdsc3`]
module"]
pub type HSTDMANXTDSC3 = crate::Reg<hstdmanxtdsc3::HSTDMANXTDSC3_SPEC>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 3)"]
pub mod hstdmanxtdsc3;
#[doc = "HSTDMAADDRESS3 (rw) register accessor: Host DMA Channel Address Register (n = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmaaddress3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmaaddress3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmaaddress3`]
module"]
pub type HSTDMAADDRESS3 = crate::Reg<hstdmaaddress3::HSTDMAADDRESS3_SPEC>;
#[doc = "Host DMA Channel Address Register (n = 3)"]
pub mod hstdmaaddress3;
#[doc = "HSTDMACONTROL3 (rw) register accessor: Host DMA Channel Control Register (n = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmacontrol3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmacontrol3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmacontrol3`]
module"]
pub type HSTDMACONTROL3 = crate::Reg<hstdmacontrol3::HSTDMACONTROL3_SPEC>;
#[doc = "Host DMA Channel Control Register (n = 3)"]
pub mod hstdmacontrol3;
#[doc = "HSTDMASTATUS3 (rw) register accessor: Host DMA Channel Status Register (n = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmastatus3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmastatus3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmastatus3`]
module"]
pub type HSTDMASTATUS3 = crate::Reg<hstdmastatus3::HSTDMASTATUS3_SPEC>;
#[doc = "Host DMA Channel Status Register (n = 3)"]
pub mod hstdmastatus3;
#[doc = "HSTDMANXTDSC4 (rw) register accessor: Host DMA Channel Next Descriptor Address Register (n = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmanxtdsc4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmanxtdsc4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmanxtdsc4`]
module"]
pub type HSTDMANXTDSC4 = crate::Reg<hstdmanxtdsc4::HSTDMANXTDSC4_SPEC>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 4)"]
pub mod hstdmanxtdsc4;
#[doc = "HSTDMAADDRESS4 (rw) register accessor: Host DMA Channel Address Register (n = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmaaddress4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmaaddress4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmaaddress4`]
module"]
pub type HSTDMAADDRESS4 = crate::Reg<hstdmaaddress4::HSTDMAADDRESS4_SPEC>;
#[doc = "Host DMA Channel Address Register (n = 4)"]
pub mod hstdmaaddress4;
#[doc = "HSTDMACONTROL4 (rw) register accessor: Host DMA Channel Control Register (n = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmacontrol4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmacontrol4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmacontrol4`]
module"]
pub type HSTDMACONTROL4 = crate::Reg<hstdmacontrol4::HSTDMACONTROL4_SPEC>;
#[doc = "Host DMA Channel Control Register (n = 4)"]
pub mod hstdmacontrol4;
#[doc = "HSTDMASTATUS4 (rw) register accessor: Host DMA Channel Status Register (n = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmastatus4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmastatus4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmastatus4`]
module"]
pub type HSTDMASTATUS4 = crate::Reg<hstdmastatus4::HSTDMASTATUS4_SPEC>;
#[doc = "Host DMA Channel Status Register (n = 4)"]
pub mod hstdmastatus4;
#[doc = "HSTDMANXTDSC5 (rw) register accessor: Host DMA Channel Next Descriptor Address Register (n = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmanxtdsc5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmanxtdsc5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmanxtdsc5`]
module"]
pub type HSTDMANXTDSC5 = crate::Reg<hstdmanxtdsc5::HSTDMANXTDSC5_SPEC>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 5)"]
pub mod hstdmanxtdsc5;
#[doc = "HSTDMAADDRESS5 (rw) register accessor: Host DMA Channel Address Register (n = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmaaddress5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmaaddress5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmaaddress5`]
module"]
pub type HSTDMAADDRESS5 = crate::Reg<hstdmaaddress5::HSTDMAADDRESS5_SPEC>;
#[doc = "Host DMA Channel Address Register (n = 5)"]
pub mod hstdmaaddress5;
#[doc = "HSTDMACONTROL5 (rw) register accessor: Host DMA Channel Control Register (n = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmacontrol5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmacontrol5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmacontrol5`]
module"]
pub type HSTDMACONTROL5 = crate::Reg<hstdmacontrol5::HSTDMACONTROL5_SPEC>;
#[doc = "Host DMA Channel Control Register (n = 5)"]
pub mod hstdmacontrol5;
#[doc = "HSTDMASTATUS5 (rw) register accessor: Host DMA Channel Status Register (n = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmastatus5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmastatus5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmastatus5`]
module"]
pub type HSTDMASTATUS5 = crate::Reg<hstdmastatus5::HSTDMASTATUS5_SPEC>;
#[doc = "Host DMA Channel Status Register (n = 5)"]
pub mod hstdmastatus5;
#[doc = "HSTDMANXTDSC6 (rw) register accessor: Host DMA Channel Next Descriptor Address Register (n = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmanxtdsc6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmanxtdsc6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmanxtdsc6`]
module"]
pub type HSTDMANXTDSC6 = crate::Reg<hstdmanxtdsc6::HSTDMANXTDSC6_SPEC>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 6)"]
pub mod hstdmanxtdsc6;
#[doc = "HSTDMAADDRESS6 (rw) register accessor: Host DMA Channel Address Register (n = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmaaddress6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmaaddress6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmaaddress6`]
module"]
pub type HSTDMAADDRESS6 = crate::Reg<hstdmaaddress6::HSTDMAADDRESS6_SPEC>;
#[doc = "Host DMA Channel Address Register (n = 6)"]
pub mod hstdmaaddress6;
#[doc = "HSTDMACONTROL6 (rw) register accessor: Host DMA Channel Control Register (n = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmacontrol6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmacontrol6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmacontrol6`]
module"]
pub type HSTDMACONTROL6 = crate::Reg<hstdmacontrol6::HSTDMACONTROL6_SPEC>;
#[doc = "Host DMA Channel Control Register (n = 6)"]
pub mod hstdmacontrol6;
#[doc = "HSTDMASTATUS6 (rw) register accessor: Host DMA Channel Status Register (n = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmastatus6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmastatus6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmastatus6`]
module"]
pub type HSTDMASTATUS6 = crate::Reg<hstdmastatus6::HSTDMASTATUS6_SPEC>;
#[doc = "Host DMA Channel Status Register (n = 6)"]
pub mod hstdmastatus6;
#[doc = "HSTDMANXTDSC7 (rw) register accessor: Host DMA Channel Next Descriptor Address Register (n = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmanxtdsc7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmanxtdsc7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmanxtdsc7`]
module"]
pub type HSTDMANXTDSC7 = crate::Reg<hstdmanxtdsc7::HSTDMANXTDSC7_SPEC>;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 7)"]
pub mod hstdmanxtdsc7;
#[doc = "HSTDMAADDRESS7 (rw) register accessor: Host DMA Channel Address Register (n = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmaaddress7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmaaddress7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmaaddress7`]
module"]
pub type HSTDMAADDRESS7 = crate::Reg<hstdmaaddress7::HSTDMAADDRESS7_SPEC>;
#[doc = "Host DMA Channel Address Register (n = 7)"]
pub mod hstdmaaddress7;
#[doc = "HSTDMACONTROL7 (rw) register accessor: Host DMA Channel Control Register (n = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmacontrol7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmacontrol7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmacontrol7`]
module"]
pub type HSTDMACONTROL7 = crate::Reg<hstdmacontrol7::HSTDMACONTROL7_SPEC>;
#[doc = "Host DMA Channel Control Register (n = 7)"]
pub mod hstdmacontrol7;
#[doc = "HSTDMASTATUS7 (rw) register accessor: Host DMA Channel Status Register (n = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmastatus7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmastatus7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmastatus7`]
module"]
pub type HSTDMASTATUS7 = crate::Reg<hstdmastatus7::HSTDMASTATUS7_SPEC>;
#[doc = "Host DMA Channel Status Register (n = 7)"]
pub mod hstdmastatus7;
#[doc = "CTRL (rw) register accessor: General Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "General Control Register"]
pub mod ctrl;
#[doc = "SR (r) register accessor: General Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "General Status Register"]
pub mod sr;
#[doc = "SCR (w) register accessor: General Status Clear Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scr`]
module"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "General Status Clear Register"]
pub mod scr;
#[doc = "SFR (w) register accessor: General Status Set Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sfr`]
module"]
pub type SFR = crate::Reg<sfr::SFR_SPEC>;
#[doc = "General Status Set Register"]
pub mod sfr;
#[doc = "FSM (r) register accessor: General Finite State Machine Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fsm`]
module"]
pub type FSM = crate::Reg<fsm::FSM_SPEC>;
#[doc = "General Finite State Machine Register"]
pub mod fsm;
