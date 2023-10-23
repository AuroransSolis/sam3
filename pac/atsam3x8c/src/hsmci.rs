#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Mode Register"]
    pub mr: MR,
    #[doc = "0x08 - Data Timeout Register"]
    pub dtor: DTOR,
    #[doc = "0x0c - SD/SDIO Card Register"]
    pub sdcr: SDCR,
    #[doc = "0x10 - Argument Register"]
    pub argr: ARGR,
    #[doc = "0x14 - Command Register"]
    pub cmdr: CMDR,
    #[doc = "0x18 - Block Register"]
    pub blkr: BLKR,
    #[doc = "0x1c - Completion Signal Timeout Register"]
    pub cstor: CSTOR,
    #[doc = "0x20 - Response Register 0"]
    pub rspr0: RSPR0,
    #[doc = "0x24 - Response Register 1"]
    pub rspr1: RSPR1,
    #[doc = "0x28 - Response Register 2"]
    pub rspr2: RSPR2,
    #[doc = "0x2c - Response Register 3"]
    pub rspr3: RSPR3,
    #[doc = "0x30 - Receive Data Register"]
    pub rdr: RDR,
    #[doc = "0x34 - Transmit Data Register"]
    pub tdr: TDR,
    _reserved14: [u8; 0x08],
    #[doc = "0x40 - Status Register"]
    pub sr: SR,
    #[doc = "0x44 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x48 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x4c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x50 - DMA Configuration Register"]
    pub dma: DMA,
    #[doc = "0x54 - Configuration Register"]
    pub cfg: CFG,
    _reserved20: [u8; 0x8c],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub wpsr: WPSR,
    _reserved22: [u8; 0x0114],
    #[doc = "0x200 - FIFO Memory Aperture0 0"]
    pub fifo0: FIFO0,
    #[doc = "0x204 - FIFO Memory Aperture0 1"]
    pub fifo1: FIFO1,
    #[doc = "0x208 - FIFO Memory Aperture0 2"]
    pub fifo2: FIFO2,
    #[doc = "0x20c - FIFO Memory Aperture0 3"]
    pub fifo3: FIFO3,
    #[doc = "0x210 - FIFO Memory Aperture0 4"]
    pub fifo4: FIFO4,
    #[doc = "0x214 - FIFO Memory Aperture0 5"]
    pub fifo5: FIFO5,
    #[doc = "0x218 - FIFO Memory Aperture0 6"]
    pub fifo6: FIFO6,
    #[doc = "0x21c - FIFO Memory Aperture0 7"]
    pub fifo7: FIFO7,
    #[doc = "0x220 - FIFO Memory Aperture0 8"]
    pub fifo8: FIFO8,
    #[doc = "0x224 - FIFO Memory Aperture0 9"]
    pub fifo9: FIFO9,
    #[doc = "0x228 - FIFO Memory Aperture0 10"]
    pub fifo10: FIFO10,
    #[doc = "0x22c - FIFO Memory Aperture0 11"]
    pub fifo11: FIFO11,
    #[doc = "0x230 - FIFO Memory Aperture0 12"]
    pub fifo12: FIFO12,
    #[doc = "0x234 - FIFO Memory Aperture0 13"]
    pub fifo13: FIFO13,
    #[doc = "0x238 - FIFO Memory Aperture0 14"]
    pub fifo14: FIFO14,
    #[doc = "0x23c - FIFO Memory Aperture0 15"]
    pub fifo15: FIFO15,
    #[doc = "0x240 - FIFO Memory Aperture0 16"]
    pub fifo16: FIFO16,
    #[doc = "0x244 - FIFO Memory Aperture0 17"]
    pub fifo17: FIFO17,
    #[doc = "0x248 - FIFO Memory Aperture0 18"]
    pub fifo18: FIFO18,
    #[doc = "0x24c - FIFO Memory Aperture0 19"]
    pub fifo19: FIFO19,
    #[doc = "0x250 - FIFO Memory Aperture0 20"]
    pub fifo20: FIFO20,
    #[doc = "0x254 - FIFO Memory Aperture0 21"]
    pub fifo21: FIFO21,
    #[doc = "0x258 - FIFO Memory Aperture0 22"]
    pub fifo22: FIFO22,
    #[doc = "0x25c - FIFO Memory Aperture0 23"]
    pub fifo23: FIFO23,
    #[doc = "0x260 - FIFO Memory Aperture0 24"]
    pub fifo24: FIFO24,
    #[doc = "0x264 - FIFO Memory Aperture0 25"]
    pub fifo25: FIFO25,
    #[doc = "0x268 - FIFO Memory Aperture0 26"]
    pub fifo26: FIFO26,
    #[doc = "0x26c - FIFO Memory Aperture0 27"]
    pub fifo27: FIFO27,
    #[doc = "0x270 - FIFO Memory Aperture0 28"]
    pub fifo28: FIFO28,
    #[doc = "0x274 - FIFO Memory Aperture0 29"]
    pub fifo29: FIFO29,
    #[doc = "0x278 - FIFO Memory Aperture0 30"]
    pub fifo30: FIFO30,
    #[doc = "0x27c - FIFO Memory Aperture0 31"]
    pub fifo31: FIFO31,
    #[doc = "0x280 - FIFO Memory Aperture0 32"]
    pub fifo32: FIFO32,
    #[doc = "0x284 - FIFO Memory Aperture0 33"]
    pub fifo33: FIFO33,
    #[doc = "0x288 - FIFO Memory Aperture0 34"]
    pub fifo34: FIFO34,
    #[doc = "0x28c - FIFO Memory Aperture0 35"]
    pub fifo35: FIFO35,
    #[doc = "0x290 - FIFO Memory Aperture0 36"]
    pub fifo36: FIFO36,
    #[doc = "0x294 - FIFO Memory Aperture0 37"]
    pub fifo37: FIFO37,
    #[doc = "0x298 - FIFO Memory Aperture0 38"]
    pub fifo38: FIFO38,
    #[doc = "0x29c - FIFO Memory Aperture0 39"]
    pub fifo39: FIFO39,
    #[doc = "0x2a0 - FIFO Memory Aperture0 40"]
    pub fifo40: FIFO40,
    #[doc = "0x2a4 - FIFO Memory Aperture0 41"]
    pub fifo41: FIFO41,
    #[doc = "0x2a8 - FIFO Memory Aperture0 42"]
    pub fifo42: FIFO42,
    #[doc = "0x2ac - FIFO Memory Aperture0 43"]
    pub fifo43: FIFO43,
    #[doc = "0x2b0 - FIFO Memory Aperture0 44"]
    pub fifo44: FIFO44,
    #[doc = "0x2b4 - FIFO Memory Aperture0 45"]
    pub fifo45: FIFO45,
    #[doc = "0x2b8 - FIFO Memory Aperture0 46"]
    pub fifo46: FIFO46,
    #[doc = "0x2bc - FIFO Memory Aperture0 47"]
    pub fifo47: FIFO47,
    #[doc = "0x2c0 - FIFO Memory Aperture0 48"]
    pub fifo48: FIFO48,
    #[doc = "0x2c4 - FIFO Memory Aperture0 49"]
    pub fifo49: FIFO49,
    #[doc = "0x2c8 - FIFO Memory Aperture0 50"]
    pub fifo50: FIFO50,
    #[doc = "0x2cc - FIFO Memory Aperture0 51"]
    pub fifo51: FIFO51,
    #[doc = "0x2d0 - FIFO Memory Aperture0 52"]
    pub fifo52: FIFO52,
    #[doc = "0x2d4 - FIFO Memory Aperture0 53"]
    pub fifo53: FIFO53,
    #[doc = "0x2d8 - FIFO Memory Aperture0 54"]
    pub fifo54: FIFO54,
    #[doc = "0x2dc - FIFO Memory Aperture0 55"]
    pub fifo55: FIFO55,
    #[doc = "0x2e0 - FIFO Memory Aperture0 56"]
    pub fifo56: FIFO56,
    #[doc = "0x2e4 - FIFO Memory Aperture0 57"]
    pub fifo57: FIFO57,
    #[doc = "0x2e8 - FIFO Memory Aperture0 58"]
    pub fifo58: FIFO58,
    #[doc = "0x2ec - FIFO Memory Aperture0 59"]
    pub fifo59: FIFO59,
    #[doc = "0x2f0 - FIFO Memory Aperture0 60"]
    pub fifo60: FIFO60,
    #[doc = "0x2f4 - FIFO Memory Aperture0 61"]
    pub fifo61: FIFO61,
    #[doc = "0x2f8 - FIFO Memory Aperture0 62"]
    pub fifo62: FIFO62,
    #[doc = "0x2fc - FIFO Memory Aperture0 63"]
    pub fifo63: FIFO63,
    #[doc = "0x300 - FIFO Memory Aperture0 64"]
    pub fifo64: FIFO64,
    #[doc = "0x304 - FIFO Memory Aperture0 65"]
    pub fifo65: FIFO65,
    #[doc = "0x308 - FIFO Memory Aperture0 66"]
    pub fifo66: FIFO66,
    #[doc = "0x30c - FIFO Memory Aperture0 67"]
    pub fifo67: FIFO67,
    #[doc = "0x310 - FIFO Memory Aperture0 68"]
    pub fifo68: FIFO68,
    #[doc = "0x314 - FIFO Memory Aperture0 69"]
    pub fifo69: FIFO69,
    #[doc = "0x318 - FIFO Memory Aperture0 70"]
    pub fifo70: FIFO70,
    #[doc = "0x31c - FIFO Memory Aperture0 71"]
    pub fifo71: FIFO71,
    #[doc = "0x320 - FIFO Memory Aperture0 72"]
    pub fifo72: FIFO72,
    #[doc = "0x324 - FIFO Memory Aperture0 73"]
    pub fifo73: FIFO73,
    #[doc = "0x328 - FIFO Memory Aperture0 74"]
    pub fifo74: FIFO74,
    #[doc = "0x32c - FIFO Memory Aperture0 75"]
    pub fifo75: FIFO75,
    #[doc = "0x330 - FIFO Memory Aperture0 76"]
    pub fifo76: FIFO76,
    #[doc = "0x334 - FIFO Memory Aperture0 77"]
    pub fifo77: FIFO77,
    #[doc = "0x338 - FIFO Memory Aperture0 78"]
    pub fifo78: FIFO78,
    #[doc = "0x33c - FIFO Memory Aperture0 79"]
    pub fifo79: FIFO79,
    #[doc = "0x340 - FIFO Memory Aperture0 80"]
    pub fifo80: FIFO80,
    #[doc = "0x344 - FIFO Memory Aperture0 81"]
    pub fifo81: FIFO81,
    #[doc = "0x348 - FIFO Memory Aperture0 82"]
    pub fifo82: FIFO82,
    #[doc = "0x34c - FIFO Memory Aperture0 83"]
    pub fifo83: FIFO83,
    #[doc = "0x350 - FIFO Memory Aperture0 84"]
    pub fifo84: FIFO84,
    #[doc = "0x354 - FIFO Memory Aperture0 85"]
    pub fifo85: FIFO85,
    #[doc = "0x358 - FIFO Memory Aperture0 86"]
    pub fifo86: FIFO86,
    #[doc = "0x35c - FIFO Memory Aperture0 87"]
    pub fifo87: FIFO87,
    #[doc = "0x360 - FIFO Memory Aperture0 88"]
    pub fifo88: FIFO88,
    #[doc = "0x364 - FIFO Memory Aperture0 89"]
    pub fifo89: FIFO89,
    #[doc = "0x368 - FIFO Memory Aperture0 90"]
    pub fifo90: FIFO90,
    #[doc = "0x36c - FIFO Memory Aperture0 91"]
    pub fifo91: FIFO91,
    #[doc = "0x370 - FIFO Memory Aperture0 92"]
    pub fifo92: FIFO92,
    #[doc = "0x374 - FIFO Memory Aperture0 93"]
    pub fifo93: FIFO93,
    #[doc = "0x378 - FIFO Memory Aperture0 94"]
    pub fifo94: FIFO94,
    #[doc = "0x37c - FIFO Memory Aperture0 95"]
    pub fifo95: FIFO95,
    #[doc = "0x380 - FIFO Memory Aperture0 96"]
    pub fifo96: FIFO96,
    #[doc = "0x384 - FIFO Memory Aperture0 97"]
    pub fifo97: FIFO97,
    #[doc = "0x388 - FIFO Memory Aperture0 98"]
    pub fifo98: FIFO98,
    #[doc = "0x38c - FIFO Memory Aperture0 99"]
    pub fifo99: FIFO99,
    #[doc = "0x390 - FIFO Memory Aperture0 100"]
    pub fifo100: FIFO100,
    #[doc = "0x394 - FIFO Memory Aperture0 101"]
    pub fifo101: FIFO101,
    #[doc = "0x398 - FIFO Memory Aperture0 102"]
    pub fifo102: FIFO102,
    #[doc = "0x39c - FIFO Memory Aperture0 103"]
    pub fifo103: FIFO103,
    #[doc = "0x3a0 - FIFO Memory Aperture0 104"]
    pub fifo104: FIFO104,
    #[doc = "0x3a4 - FIFO Memory Aperture0 105"]
    pub fifo105: FIFO105,
    #[doc = "0x3a8 - FIFO Memory Aperture0 106"]
    pub fifo106: FIFO106,
    #[doc = "0x3ac - FIFO Memory Aperture0 107"]
    pub fifo107: FIFO107,
    #[doc = "0x3b0 - FIFO Memory Aperture0 108"]
    pub fifo108: FIFO108,
    #[doc = "0x3b4 - FIFO Memory Aperture0 109"]
    pub fifo109: FIFO109,
    #[doc = "0x3b8 - FIFO Memory Aperture0 110"]
    pub fifo110: FIFO110,
    #[doc = "0x3bc - FIFO Memory Aperture0 111"]
    pub fifo111: FIFO111,
    #[doc = "0x3c0 - FIFO Memory Aperture0 112"]
    pub fifo112: FIFO112,
    #[doc = "0x3c4 - FIFO Memory Aperture0 113"]
    pub fifo113: FIFO113,
    #[doc = "0x3c8 - FIFO Memory Aperture0 114"]
    pub fifo114: FIFO114,
    #[doc = "0x3cc - FIFO Memory Aperture0 115"]
    pub fifo115: FIFO115,
    #[doc = "0x3d0 - FIFO Memory Aperture0 116"]
    pub fifo116: FIFO116,
    #[doc = "0x3d4 - FIFO Memory Aperture0 117"]
    pub fifo117: FIFO117,
    #[doc = "0x3d8 - FIFO Memory Aperture0 118"]
    pub fifo118: FIFO118,
    #[doc = "0x3dc - FIFO Memory Aperture0 119"]
    pub fifo119: FIFO119,
    #[doc = "0x3e0 - FIFO Memory Aperture0 120"]
    pub fifo120: FIFO120,
    #[doc = "0x3e4 - FIFO Memory Aperture0 121"]
    pub fifo121: FIFO121,
    #[doc = "0x3e8 - FIFO Memory Aperture0 122"]
    pub fifo122: FIFO122,
    #[doc = "0x3ec - FIFO Memory Aperture0 123"]
    pub fifo123: FIFO123,
    #[doc = "0x3f0 - FIFO Memory Aperture0 124"]
    pub fifo124: FIFO124,
    #[doc = "0x3f4 - FIFO Memory Aperture0 125"]
    pub fifo125: FIFO125,
    #[doc = "0x3f8 - FIFO Memory Aperture0 126"]
    pub fifo126: FIFO126,
    #[doc = "0x3fc - FIFO Memory Aperture0 127"]
    pub fifo127: FIFO127,
    #[doc = "0x400 - FIFO Memory Aperture0 128"]
    pub fifo128: FIFO128,
    #[doc = "0x404 - FIFO Memory Aperture0 129"]
    pub fifo129: FIFO129,
    #[doc = "0x408 - FIFO Memory Aperture0 130"]
    pub fifo130: FIFO130,
    #[doc = "0x40c - FIFO Memory Aperture0 131"]
    pub fifo131: FIFO131,
    #[doc = "0x410 - FIFO Memory Aperture0 132"]
    pub fifo132: FIFO132,
    #[doc = "0x414 - FIFO Memory Aperture0 133"]
    pub fifo133: FIFO133,
    #[doc = "0x418 - FIFO Memory Aperture0 134"]
    pub fifo134: FIFO134,
    #[doc = "0x41c - FIFO Memory Aperture0 135"]
    pub fifo135: FIFO135,
    #[doc = "0x420 - FIFO Memory Aperture0 136"]
    pub fifo136: FIFO136,
    #[doc = "0x424 - FIFO Memory Aperture0 137"]
    pub fifo137: FIFO137,
    #[doc = "0x428 - FIFO Memory Aperture0 138"]
    pub fifo138: FIFO138,
    #[doc = "0x42c - FIFO Memory Aperture0 139"]
    pub fifo139: FIFO139,
    #[doc = "0x430 - FIFO Memory Aperture0 140"]
    pub fifo140: FIFO140,
    #[doc = "0x434 - FIFO Memory Aperture0 141"]
    pub fifo141: FIFO141,
    #[doc = "0x438 - FIFO Memory Aperture0 142"]
    pub fifo142: FIFO142,
    #[doc = "0x43c - FIFO Memory Aperture0 143"]
    pub fifo143: FIFO143,
    #[doc = "0x440 - FIFO Memory Aperture0 144"]
    pub fifo144: FIFO144,
    #[doc = "0x444 - FIFO Memory Aperture0 145"]
    pub fifo145: FIFO145,
    #[doc = "0x448 - FIFO Memory Aperture0 146"]
    pub fifo146: FIFO146,
    #[doc = "0x44c - FIFO Memory Aperture0 147"]
    pub fifo147: FIFO147,
    #[doc = "0x450 - FIFO Memory Aperture0 148"]
    pub fifo148: FIFO148,
    #[doc = "0x454 - FIFO Memory Aperture0 149"]
    pub fifo149: FIFO149,
    #[doc = "0x458 - FIFO Memory Aperture0 150"]
    pub fifo150: FIFO150,
    #[doc = "0x45c - FIFO Memory Aperture0 151"]
    pub fifo151: FIFO151,
    #[doc = "0x460 - FIFO Memory Aperture0 152"]
    pub fifo152: FIFO152,
    #[doc = "0x464 - FIFO Memory Aperture0 153"]
    pub fifo153: FIFO153,
    #[doc = "0x468 - FIFO Memory Aperture0 154"]
    pub fifo154: FIFO154,
    #[doc = "0x46c - FIFO Memory Aperture0 155"]
    pub fifo155: FIFO155,
    #[doc = "0x470 - FIFO Memory Aperture0 156"]
    pub fifo156: FIFO156,
    #[doc = "0x474 - FIFO Memory Aperture0 157"]
    pub fifo157: FIFO157,
    #[doc = "0x478 - FIFO Memory Aperture0 158"]
    pub fifo158: FIFO158,
    #[doc = "0x47c - FIFO Memory Aperture0 159"]
    pub fifo159: FIFO159,
    #[doc = "0x480 - FIFO Memory Aperture0 160"]
    pub fifo160: FIFO160,
    #[doc = "0x484 - FIFO Memory Aperture0 161"]
    pub fifo161: FIFO161,
    #[doc = "0x488 - FIFO Memory Aperture0 162"]
    pub fifo162: FIFO162,
    #[doc = "0x48c - FIFO Memory Aperture0 163"]
    pub fifo163: FIFO163,
    #[doc = "0x490 - FIFO Memory Aperture0 164"]
    pub fifo164: FIFO164,
    #[doc = "0x494 - FIFO Memory Aperture0 165"]
    pub fifo165: FIFO165,
    #[doc = "0x498 - FIFO Memory Aperture0 166"]
    pub fifo166: FIFO166,
    #[doc = "0x49c - FIFO Memory Aperture0 167"]
    pub fifo167: FIFO167,
    #[doc = "0x4a0 - FIFO Memory Aperture0 168"]
    pub fifo168: FIFO168,
    #[doc = "0x4a4 - FIFO Memory Aperture0 169"]
    pub fifo169: FIFO169,
    #[doc = "0x4a8 - FIFO Memory Aperture0 170"]
    pub fifo170: FIFO170,
    #[doc = "0x4ac - FIFO Memory Aperture0 171"]
    pub fifo171: FIFO171,
    #[doc = "0x4b0 - FIFO Memory Aperture0 172"]
    pub fifo172: FIFO172,
    #[doc = "0x4b4 - FIFO Memory Aperture0 173"]
    pub fifo173: FIFO173,
    #[doc = "0x4b8 - FIFO Memory Aperture0 174"]
    pub fifo174: FIFO174,
    #[doc = "0x4bc - FIFO Memory Aperture0 175"]
    pub fifo175: FIFO175,
    #[doc = "0x4c0 - FIFO Memory Aperture0 176"]
    pub fifo176: FIFO176,
    #[doc = "0x4c4 - FIFO Memory Aperture0 177"]
    pub fifo177: FIFO177,
    #[doc = "0x4c8 - FIFO Memory Aperture0 178"]
    pub fifo178: FIFO178,
    #[doc = "0x4cc - FIFO Memory Aperture0 179"]
    pub fifo179: FIFO179,
    #[doc = "0x4d0 - FIFO Memory Aperture0 180"]
    pub fifo180: FIFO180,
    #[doc = "0x4d4 - FIFO Memory Aperture0 181"]
    pub fifo181: FIFO181,
    #[doc = "0x4d8 - FIFO Memory Aperture0 182"]
    pub fifo182: FIFO182,
    #[doc = "0x4dc - FIFO Memory Aperture0 183"]
    pub fifo183: FIFO183,
    #[doc = "0x4e0 - FIFO Memory Aperture0 184"]
    pub fifo184: FIFO184,
    #[doc = "0x4e4 - FIFO Memory Aperture0 185"]
    pub fifo185: FIFO185,
    #[doc = "0x4e8 - FIFO Memory Aperture0 186"]
    pub fifo186: FIFO186,
    #[doc = "0x4ec - FIFO Memory Aperture0 187"]
    pub fifo187: FIFO187,
    #[doc = "0x4f0 - FIFO Memory Aperture0 188"]
    pub fifo188: FIFO188,
    #[doc = "0x4f4 - FIFO Memory Aperture0 189"]
    pub fifo189: FIFO189,
    #[doc = "0x4f8 - FIFO Memory Aperture0 190"]
    pub fifo190: FIFO190,
    #[doc = "0x4fc - FIFO Memory Aperture0 191"]
    pub fifo191: FIFO191,
    #[doc = "0x500 - FIFO Memory Aperture0 192"]
    pub fifo192: FIFO192,
    #[doc = "0x504 - FIFO Memory Aperture0 193"]
    pub fifo193: FIFO193,
    #[doc = "0x508 - FIFO Memory Aperture0 194"]
    pub fifo194: FIFO194,
    #[doc = "0x50c - FIFO Memory Aperture0 195"]
    pub fifo195: FIFO195,
    #[doc = "0x510 - FIFO Memory Aperture0 196"]
    pub fifo196: FIFO196,
    #[doc = "0x514 - FIFO Memory Aperture0 197"]
    pub fifo197: FIFO197,
    #[doc = "0x518 - FIFO Memory Aperture0 198"]
    pub fifo198: FIFO198,
    #[doc = "0x51c - FIFO Memory Aperture0 199"]
    pub fifo199: FIFO199,
    #[doc = "0x520 - FIFO Memory Aperture0 200"]
    pub fifo200: FIFO200,
    #[doc = "0x524 - FIFO Memory Aperture0 201"]
    pub fifo201: FIFO201,
    #[doc = "0x528 - FIFO Memory Aperture0 202"]
    pub fifo202: FIFO202,
    #[doc = "0x52c - FIFO Memory Aperture0 203"]
    pub fifo203: FIFO203,
    #[doc = "0x530 - FIFO Memory Aperture0 204"]
    pub fifo204: FIFO204,
    #[doc = "0x534 - FIFO Memory Aperture0 205"]
    pub fifo205: FIFO205,
    #[doc = "0x538 - FIFO Memory Aperture0 206"]
    pub fifo206: FIFO206,
    #[doc = "0x53c - FIFO Memory Aperture0 207"]
    pub fifo207: FIFO207,
    #[doc = "0x540 - FIFO Memory Aperture0 208"]
    pub fifo208: FIFO208,
    #[doc = "0x544 - FIFO Memory Aperture0 209"]
    pub fifo209: FIFO209,
    #[doc = "0x548 - FIFO Memory Aperture0 210"]
    pub fifo210: FIFO210,
    #[doc = "0x54c - FIFO Memory Aperture0 211"]
    pub fifo211: FIFO211,
    #[doc = "0x550 - FIFO Memory Aperture0 212"]
    pub fifo212: FIFO212,
    #[doc = "0x554 - FIFO Memory Aperture0 213"]
    pub fifo213: FIFO213,
    #[doc = "0x558 - FIFO Memory Aperture0 214"]
    pub fifo214: FIFO214,
    #[doc = "0x55c - FIFO Memory Aperture0 215"]
    pub fifo215: FIFO215,
    #[doc = "0x560 - FIFO Memory Aperture0 216"]
    pub fifo216: FIFO216,
    #[doc = "0x564 - FIFO Memory Aperture0 217"]
    pub fifo217: FIFO217,
    #[doc = "0x568 - FIFO Memory Aperture0 218"]
    pub fifo218: FIFO218,
    #[doc = "0x56c - FIFO Memory Aperture0 219"]
    pub fifo219: FIFO219,
    #[doc = "0x570 - FIFO Memory Aperture0 220"]
    pub fifo220: FIFO220,
    #[doc = "0x574 - FIFO Memory Aperture0 221"]
    pub fifo221: FIFO221,
    #[doc = "0x578 - FIFO Memory Aperture0 222"]
    pub fifo222: FIFO222,
    #[doc = "0x57c - FIFO Memory Aperture0 223"]
    pub fifo223: FIFO223,
    #[doc = "0x580 - FIFO Memory Aperture0 224"]
    pub fifo224: FIFO224,
    #[doc = "0x584 - FIFO Memory Aperture0 225"]
    pub fifo225: FIFO225,
    #[doc = "0x588 - FIFO Memory Aperture0 226"]
    pub fifo226: FIFO226,
    #[doc = "0x58c - FIFO Memory Aperture0 227"]
    pub fifo227: FIFO227,
    #[doc = "0x590 - FIFO Memory Aperture0 228"]
    pub fifo228: FIFO228,
    #[doc = "0x594 - FIFO Memory Aperture0 229"]
    pub fifo229: FIFO229,
    #[doc = "0x598 - FIFO Memory Aperture0 230"]
    pub fifo230: FIFO230,
    #[doc = "0x59c - FIFO Memory Aperture0 231"]
    pub fifo231: FIFO231,
    #[doc = "0x5a0 - FIFO Memory Aperture0 232"]
    pub fifo232: FIFO232,
    #[doc = "0x5a4 - FIFO Memory Aperture0 233"]
    pub fifo233: FIFO233,
    #[doc = "0x5a8 - FIFO Memory Aperture0 234"]
    pub fifo234: FIFO234,
    #[doc = "0x5ac - FIFO Memory Aperture0 235"]
    pub fifo235: FIFO235,
    #[doc = "0x5b0 - FIFO Memory Aperture0 236"]
    pub fifo236: FIFO236,
    #[doc = "0x5b4 - FIFO Memory Aperture0 237"]
    pub fifo237: FIFO237,
    #[doc = "0x5b8 - FIFO Memory Aperture0 238"]
    pub fifo238: FIFO238,
    #[doc = "0x5bc - FIFO Memory Aperture0 239"]
    pub fifo239: FIFO239,
    #[doc = "0x5c0 - FIFO Memory Aperture0 240"]
    pub fifo240: FIFO240,
    #[doc = "0x5c4 - FIFO Memory Aperture0 241"]
    pub fifo241: FIFO241,
    #[doc = "0x5c8 - FIFO Memory Aperture0 242"]
    pub fifo242: FIFO242,
    #[doc = "0x5cc - FIFO Memory Aperture0 243"]
    pub fifo243: FIFO243,
    #[doc = "0x5d0 - FIFO Memory Aperture0 244"]
    pub fifo244: FIFO244,
    #[doc = "0x5d4 - FIFO Memory Aperture0 245"]
    pub fifo245: FIFO245,
    #[doc = "0x5d8 - FIFO Memory Aperture0 246"]
    pub fifo246: FIFO246,
    #[doc = "0x5dc - FIFO Memory Aperture0 247"]
    pub fifo247: FIFO247,
    #[doc = "0x5e0 - FIFO Memory Aperture0 248"]
    pub fifo248: FIFO248,
    #[doc = "0x5e4 - FIFO Memory Aperture0 249"]
    pub fifo249: FIFO249,
    #[doc = "0x5e8 - FIFO Memory Aperture0 250"]
    pub fifo250: FIFO250,
    #[doc = "0x5ec - FIFO Memory Aperture0 251"]
    pub fifo251: FIFO251,
    #[doc = "0x5f0 - FIFO Memory Aperture0 252"]
    pub fifo252: FIFO252,
    #[doc = "0x5f4 - FIFO Memory Aperture0 253"]
    pub fifo253: FIFO253,
    #[doc = "0x5f8 - FIFO Memory Aperture0 254"]
    pub fifo254: FIFO254,
    #[doc = "0x5fc - FIFO Memory Aperture0 255"]
    pub fifo255: FIFO255,
}
#[doc = "CR (w) register accessor: Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MR (rw) register accessor: Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`]
module"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "DTOR (rw) register accessor: Data Timeout Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtor`]
module"]
pub type DTOR = crate::Reg<dtor::DTOR_SPEC>;
#[doc = "Data Timeout Register"]
pub mod dtor;
#[doc = "SDCR (rw) register accessor: SD/SDIO Card Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdcr`]
module"]
pub type SDCR = crate::Reg<sdcr::SDCR_SPEC>;
#[doc = "SD/SDIO Card Register"]
pub mod sdcr;
#[doc = "ARGR (rw) register accessor: Argument Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`argr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`argr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@argr`]
module"]
pub type ARGR = crate::Reg<argr::ARGR_SPEC>;
#[doc = "Argument Register"]
pub mod argr;
#[doc = "CMDR (w) register accessor: Command Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdr`]
module"]
pub type CMDR = crate::Reg<cmdr::CMDR_SPEC>;
#[doc = "Command Register"]
pub mod cmdr;
#[doc = "BLKR (rw) register accessor: Block Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blkr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blkr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blkr`]
module"]
pub type BLKR = crate::Reg<blkr::BLKR_SPEC>;
#[doc = "Block Register"]
pub mod blkr;
#[doc = "CSTOR (rw) register accessor: Completion Signal Timeout Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cstor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cstor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cstor`]
module"]
pub type CSTOR = crate::Reg<cstor::CSTOR_SPEC>;
#[doc = "Completion Signal Timeout Register"]
pub mod cstor;
#[doc = "RSPR0 (r) register accessor: Response Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rspr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rspr0`]
module"]
pub type RSPR0 = crate::Reg<rspr0::RSPR0_SPEC>;
#[doc = "Response Register 0"]
pub mod rspr0;
#[doc = "RSPR1 (r) register accessor: Response Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rspr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rspr1`]
module"]
pub type RSPR1 = crate::Reg<rspr1::RSPR1_SPEC>;
#[doc = "Response Register 1"]
pub mod rspr1;
#[doc = "RSPR2 (r) register accessor: Response Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rspr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rspr2`]
module"]
pub type RSPR2 = crate::Reg<rspr2::RSPR2_SPEC>;
#[doc = "Response Register 2"]
pub mod rspr2;
#[doc = "RSPR3 (r) register accessor: Response Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rspr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rspr3`]
module"]
pub type RSPR3 = crate::Reg<rspr3::RSPR3_SPEC>;
#[doc = "Response Register 3"]
pub mod rspr3;
#[doc = "RDR (r) register accessor: Receive Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdr`]
module"]
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
#[doc = "Receive Data Register"]
pub mod rdr;
#[doc = "TDR (w) register accessor: Transmit Data Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr`]
module"]
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
#[doc = "Transmit Data Register"]
pub mod tdr;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
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
#[doc = "DMA (rw) register accessor: DMA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma`]
module"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "DMA Configuration Register"]
pub mod dma;
#[doc = "CFG (rw) register accessor: Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "WPMR (rw) register accessor: Write Protection Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpmr::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`]
module"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: Write Protection Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`]
module"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
#[doc = "FIFO0 (rw) register accessor: FIFO Memory Aperture0 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo0`]
module"]
pub type FIFO0 = crate::Reg<fifo0::FIFO0_SPEC>;
#[doc = "FIFO Memory Aperture0 0"]
pub mod fifo0;
#[doc = "FIFO1 (rw) register accessor: FIFO Memory Aperture0 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo1`]
module"]
pub type FIFO1 = crate::Reg<fifo1::FIFO1_SPEC>;
#[doc = "FIFO Memory Aperture0 1"]
pub mod fifo1;
#[doc = "FIFO2 (rw) register accessor: FIFO Memory Aperture0 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo2`]
module"]
pub type FIFO2 = crate::Reg<fifo2::FIFO2_SPEC>;
#[doc = "FIFO Memory Aperture0 2"]
pub mod fifo2;
#[doc = "FIFO3 (rw) register accessor: FIFO Memory Aperture0 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo3`]
module"]
pub type FIFO3 = crate::Reg<fifo3::FIFO3_SPEC>;
#[doc = "FIFO Memory Aperture0 3"]
pub mod fifo3;
#[doc = "FIFO4 (rw) register accessor: FIFO Memory Aperture0 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo4`]
module"]
pub type FIFO4 = crate::Reg<fifo4::FIFO4_SPEC>;
#[doc = "FIFO Memory Aperture0 4"]
pub mod fifo4;
#[doc = "FIFO5 (rw) register accessor: FIFO Memory Aperture0 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo5`]
module"]
pub type FIFO5 = crate::Reg<fifo5::FIFO5_SPEC>;
#[doc = "FIFO Memory Aperture0 5"]
pub mod fifo5;
#[doc = "FIFO6 (rw) register accessor: FIFO Memory Aperture0 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo6::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo6`]
module"]
pub type FIFO6 = crate::Reg<fifo6::FIFO6_SPEC>;
#[doc = "FIFO Memory Aperture0 6"]
pub mod fifo6;
#[doc = "FIFO7 (rw) register accessor: FIFO Memory Aperture0 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo7::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo7`]
module"]
pub type FIFO7 = crate::Reg<fifo7::FIFO7_SPEC>;
#[doc = "FIFO Memory Aperture0 7"]
pub mod fifo7;
#[doc = "FIFO8 (rw) register accessor: FIFO Memory Aperture0 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo8::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo8`]
module"]
pub type FIFO8 = crate::Reg<fifo8::FIFO8_SPEC>;
#[doc = "FIFO Memory Aperture0 8"]
pub mod fifo8;
#[doc = "FIFO9 (rw) register accessor: FIFO Memory Aperture0 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo9::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo9`]
module"]
pub type FIFO9 = crate::Reg<fifo9::FIFO9_SPEC>;
#[doc = "FIFO Memory Aperture0 9"]
pub mod fifo9;
#[doc = "FIFO10 (rw) register accessor: FIFO Memory Aperture0 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo10::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo10`]
module"]
pub type FIFO10 = crate::Reg<fifo10::FIFO10_SPEC>;
#[doc = "FIFO Memory Aperture0 10"]
pub mod fifo10;
#[doc = "FIFO11 (rw) register accessor: FIFO Memory Aperture0 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo11::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo11`]
module"]
pub type FIFO11 = crate::Reg<fifo11::FIFO11_SPEC>;
#[doc = "FIFO Memory Aperture0 11"]
pub mod fifo11;
#[doc = "FIFO12 (rw) register accessor: FIFO Memory Aperture0 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo12::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo12`]
module"]
pub type FIFO12 = crate::Reg<fifo12::FIFO12_SPEC>;
#[doc = "FIFO Memory Aperture0 12"]
pub mod fifo12;
#[doc = "FIFO13 (rw) register accessor: FIFO Memory Aperture0 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo13::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo13`]
module"]
pub type FIFO13 = crate::Reg<fifo13::FIFO13_SPEC>;
#[doc = "FIFO Memory Aperture0 13"]
pub mod fifo13;
#[doc = "FIFO14 (rw) register accessor: FIFO Memory Aperture0 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo14::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo14`]
module"]
pub type FIFO14 = crate::Reg<fifo14::FIFO14_SPEC>;
#[doc = "FIFO Memory Aperture0 14"]
pub mod fifo14;
#[doc = "FIFO15 (rw) register accessor: FIFO Memory Aperture0 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo15::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo15`]
module"]
pub type FIFO15 = crate::Reg<fifo15::FIFO15_SPEC>;
#[doc = "FIFO Memory Aperture0 15"]
pub mod fifo15;
#[doc = "FIFO16 (rw) register accessor: FIFO Memory Aperture0 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo16::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo16`]
module"]
pub type FIFO16 = crate::Reg<fifo16::FIFO16_SPEC>;
#[doc = "FIFO Memory Aperture0 16"]
pub mod fifo16;
#[doc = "FIFO17 (rw) register accessor: FIFO Memory Aperture0 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo17::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo17`]
module"]
pub type FIFO17 = crate::Reg<fifo17::FIFO17_SPEC>;
#[doc = "FIFO Memory Aperture0 17"]
pub mod fifo17;
#[doc = "FIFO18 (rw) register accessor: FIFO Memory Aperture0 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo18::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo18`]
module"]
pub type FIFO18 = crate::Reg<fifo18::FIFO18_SPEC>;
#[doc = "FIFO Memory Aperture0 18"]
pub mod fifo18;
#[doc = "FIFO19 (rw) register accessor: FIFO Memory Aperture0 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo19::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo19`]
module"]
pub type FIFO19 = crate::Reg<fifo19::FIFO19_SPEC>;
#[doc = "FIFO Memory Aperture0 19"]
pub mod fifo19;
#[doc = "FIFO20 (rw) register accessor: FIFO Memory Aperture0 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo20::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo20`]
module"]
pub type FIFO20 = crate::Reg<fifo20::FIFO20_SPEC>;
#[doc = "FIFO Memory Aperture0 20"]
pub mod fifo20;
#[doc = "FIFO21 (rw) register accessor: FIFO Memory Aperture0 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo21::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo21`]
module"]
pub type FIFO21 = crate::Reg<fifo21::FIFO21_SPEC>;
#[doc = "FIFO Memory Aperture0 21"]
pub mod fifo21;
#[doc = "FIFO22 (rw) register accessor: FIFO Memory Aperture0 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo22::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo22`]
module"]
pub type FIFO22 = crate::Reg<fifo22::FIFO22_SPEC>;
#[doc = "FIFO Memory Aperture0 22"]
pub mod fifo22;
#[doc = "FIFO23 (rw) register accessor: FIFO Memory Aperture0 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo23::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo23`]
module"]
pub type FIFO23 = crate::Reg<fifo23::FIFO23_SPEC>;
#[doc = "FIFO Memory Aperture0 23"]
pub mod fifo23;
#[doc = "FIFO24 (rw) register accessor: FIFO Memory Aperture0 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo24::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo24`]
module"]
pub type FIFO24 = crate::Reg<fifo24::FIFO24_SPEC>;
#[doc = "FIFO Memory Aperture0 24"]
pub mod fifo24;
#[doc = "FIFO25 (rw) register accessor: FIFO Memory Aperture0 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo25::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo25`]
module"]
pub type FIFO25 = crate::Reg<fifo25::FIFO25_SPEC>;
#[doc = "FIFO Memory Aperture0 25"]
pub mod fifo25;
#[doc = "FIFO26 (rw) register accessor: FIFO Memory Aperture0 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo26::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo26`]
module"]
pub type FIFO26 = crate::Reg<fifo26::FIFO26_SPEC>;
#[doc = "FIFO Memory Aperture0 26"]
pub mod fifo26;
#[doc = "FIFO27 (rw) register accessor: FIFO Memory Aperture0 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo27::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo27`]
module"]
pub type FIFO27 = crate::Reg<fifo27::FIFO27_SPEC>;
#[doc = "FIFO Memory Aperture0 27"]
pub mod fifo27;
#[doc = "FIFO28 (rw) register accessor: FIFO Memory Aperture0 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo28::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo28`]
module"]
pub type FIFO28 = crate::Reg<fifo28::FIFO28_SPEC>;
#[doc = "FIFO Memory Aperture0 28"]
pub mod fifo28;
#[doc = "FIFO29 (rw) register accessor: FIFO Memory Aperture0 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo29::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo29`]
module"]
pub type FIFO29 = crate::Reg<fifo29::FIFO29_SPEC>;
#[doc = "FIFO Memory Aperture0 29"]
pub mod fifo29;
#[doc = "FIFO30 (rw) register accessor: FIFO Memory Aperture0 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo30::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo30`]
module"]
pub type FIFO30 = crate::Reg<fifo30::FIFO30_SPEC>;
#[doc = "FIFO Memory Aperture0 30"]
pub mod fifo30;
#[doc = "FIFO31 (rw) register accessor: FIFO Memory Aperture0 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo31::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo31`]
module"]
pub type FIFO31 = crate::Reg<fifo31::FIFO31_SPEC>;
#[doc = "FIFO Memory Aperture0 31"]
pub mod fifo31;
#[doc = "FIFO32 (rw) register accessor: FIFO Memory Aperture0 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo32::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo32`]
module"]
pub type FIFO32 = crate::Reg<fifo32::FIFO32_SPEC>;
#[doc = "FIFO Memory Aperture0 32"]
pub mod fifo32;
#[doc = "FIFO33 (rw) register accessor: FIFO Memory Aperture0 33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo33::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo33`]
module"]
pub type FIFO33 = crate::Reg<fifo33::FIFO33_SPEC>;
#[doc = "FIFO Memory Aperture0 33"]
pub mod fifo33;
#[doc = "FIFO34 (rw) register accessor: FIFO Memory Aperture0 34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo34::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo34`]
module"]
pub type FIFO34 = crate::Reg<fifo34::FIFO34_SPEC>;
#[doc = "FIFO Memory Aperture0 34"]
pub mod fifo34;
#[doc = "FIFO35 (rw) register accessor: FIFO Memory Aperture0 35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo35::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo35`]
module"]
pub type FIFO35 = crate::Reg<fifo35::FIFO35_SPEC>;
#[doc = "FIFO Memory Aperture0 35"]
pub mod fifo35;
#[doc = "FIFO36 (rw) register accessor: FIFO Memory Aperture0 36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo36::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo36`]
module"]
pub type FIFO36 = crate::Reg<fifo36::FIFO36_SPEC>;
#[doc = "FIFO Memory Aperture0 36"]
pub mod fifo36;
#[doc = "FIFO37 (rw) register accessor: FIFO Memory Aperture0 37\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo37::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo37`]
module"]
pub type FIFO37 = crate::Reg<fifo37::FIFO37_SPEC>;
#[doc = "FIFO Memory Aperture0 37"]
pub mod fifo37;
#[doc = "FIFO38 (rw) register accessor: FIFO Memory Aperture0 38\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo38::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo38`]
module"]
pub type FIFO38 = crate::Reg<fifo38::FIFO38_SPEC>;
#[doc = "FIFO Memory Aperture0 38"]
pub mod fifo38;
#[doc = "FIFO39 (rw) register accessor: FIFO Memory Aperture0 39\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo39::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo39`]
module"]
pub type FIFO39 = crate::Reg<fifo39::FIFO39_SPEC>;
#[doc = "FIFO Memory Aperture0 39"]
pub mod fifo39;
#[doc = "FIFO40 (rw) register accessor: FIFO Memory Aperture0 40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo40::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo40`]
module"]
pub type FIFO40 = crate::Reg<fifo40::FIFO40_SPEC>;
#[doc = "FIFO Memory Aperture0 40"]
pub mod fifo40;
#[doc = "FIFO41 (rw) register accessor: FIFO Memory Aperture0 41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo41::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo41`]
module"]
pub type FIFO41 = crate::Reg<fifo41::FIFO41_SPEC>;
#[doc = "FIFO Memory Aperture0 41"]
pub mod fifo41;
#[doc = "FIFO42 (rw) register accessor: FIFO Memory Aperture0 42\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo42::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo42`]
module"]
pub type FIFO42 = crate::Reg<fifo42::FIFO42_SPEC>;
#[doc = "FIFO Memory Aperture0 42"]
pub mod fifo42;
#[doc = "FIFO43 (rw) register accessor: FIFO Memory Aperture0 43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo43::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo43`]
module"]
pub type FIFO43 = crate::Reg<fifo43::FIFO43_SPEC>;
#[doc = "FIFO Memory Aperture0 43"]
pub mod fifo43;
#[doc = "FIFO44 (rw) register accessor: FIFO Memory Aperture0 44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo44::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo44`]
module"]
pub type FIFO44 = crate::Reg<fifo44::FIFO44_SPEC>;
#[doc = "FIFO Memory Aperture0 44"]
pub mod fifo44;
#[doc = "FIFO45 (rw) register accessor: FIFO Memory Aperture0 45\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo45::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo45`]
module"]
pub type FIFO45 = crate::Reg<fifo45::FIFO45_SPEC>;
#[doc = "FIFO Memory Aperture0 45"]
pub mod fifo45;
#[doc = "FIFO46 (rw) register accessor: FIFO Memory Aperture0 46\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo46::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo46`]
module"]
pub type FIFO46 = crate::Reg<fifo46::FIFO46_SPEC>;
#[doc = "FIFO Memory Aperture0 46"]
pub mod fifo46;
#[doc = "FIFO47 (rw) register accessor: FIFO Memory Aperture0 47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo47::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo47`]
module"]
pub type FIFO47 = crate::Reg<fifo47::FIFO47_SPEC>;
#[doc = "FIFO Memory Aperture0 47"]
pub mod fifo47;
#[doc = "FIFO48 (rw) register accessor: FIFO Memory Aperture0 48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo48::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo48`]
module"]
pub type FIFO48 = crate::Reg<fifo48::FIFO48_SPEC>;
#[doc = "FIFO Memory Aperture0 48"]
pub mod fifo48;
#[doc = "FIFO49 (rw) register accessor: FIFO Memory Aperture0 49\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo49::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo49`]
module"]
pub type FIFO49 = crate::Reg<fifo49::FIFO49_SPEC>;
#[doc = "FIFO Memory Aperture0 49"]
pub mod fifo49;
#[doc = "FIFO50 (rw) register accessor: FIFO Memory Aperture0 50\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo50::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo50::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo50`]
module"]
pub type FIFO50 = crate::Reg<fifo50::FIFO50_SPEC>;
#[doc = "FIFO Memory Aperture0 50"]
pub mod fifo50;
#[doc = "FIFO51 (rw) register accessor: FIFO Memory Aperture0 51\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo51::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo51::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo51`]
module"]
pub type FIFO51 = crate::Reg<fifo51::FIFO51_SPEC>;
#[doc = "FIFO Memory Aperture0 51"]
pub mod fifo51;
#[doc = "FIFO52 (rw) register accessor: FIFO Memory Aperture0 52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo52::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo52`]
module"]
pub type FIFO52 = crate::Reg<fifo52::FIFO52_SPEC>;
#[doc = "FIFO Memory Aperture0 52"]
pub mod fifo52;
#[doc = "FIFO53 (rw) register accessor: FIFO Memory Aperture0 53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo53::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo53::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo53`]
module"]
pub type FIFO53 = crate::Reg<fifo53::FIFO53_SPEC>;
#[doc = "FIFO Memory Aperture0 53"]
pub mod fifo53;
#[doc = "FIFO54 (rw) register accessor: FIFO Memory Aperture0 54\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo54::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo54::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo54`]
module"]
pub type FIFO54 = crate::Reg<fifo54::FIFO54_SPEC>;
#[doc = "FIFO Memory Aperture0 54"]
pub mod fifo54;
#[doc = "FIFO55 (rw) register accessor: FIFO Memory Aperture0 55\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo55::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo55`]
module"]
pub type FIFO55 = crate::Reg<fifo55::FIFO55_SPEC>;
#[doc = "FIFO Memory Aperture0 55"]
pub mod fifo55;
#[doc = "FIFO56 (rw) register accessor: FIFO Memory Aperture0 56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo56::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo56::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo56`]
module"]
pub type FIFO56 = crate::Reg<fifo56::FIFO56_SPEC>;
#[doc = "FIFO Memory Aperture0 56"]
pub mod fifo56;
#[doc = "FIFO57 (rw) register accessor: FIFO Memory Aperture0 57\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo57::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo57::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo57`]
module"]
pub type FIFO57 = crate::Reg<fifo57::FIFO57_SPEC>;
#[doc = "FIFO Memory Aperture0 57"]
pub mod fifo57;
#[doc = "FIFO58 (rw) register accessor: FIFO Memory Aperture0 58\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo58::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo58::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo58`]
module"]
pub type FIFO58 = crate::Reg<fifo58::FIFO58_SPEC>;
#[doc = "FIFO Memory Aperture0 58"]
pub mod fifo58;
#[doc = "FIFO59 (rw) register accessor: FIFO Memory Aperture0 59\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo59::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo59::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo59`]
module"]
pub type FIFO59 = crate::Reg<fifo59::FIFO59_SPEC>;
#[doc = "FIFO Memory Aperture0 59"]
pub mod fifo59;
#[doc = "FIFO60 (rw) register accessor: FIFO Memory Aperture0 60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo60::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo60::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo60`]
module"]
pub type FIFO60 = crate::Reg<fifo60::FIFO60_SPEC>;
#[doc = "FIFO Memory Aperture0 60"]
pub mod fifo60;
#[doc = "FIFO61 (rw) register accessor: FIFO Memory Aperture0 61\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo61::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo61::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo61`]
module"]
pub type FIFO61 = crate::Reg<fifo61::FIFO61_SPEC>;
#[doc = "FIFO Memory Aperture0 61"]
pub mod fifo61;
#[doc = "FIFO62 (rw) register accessor: FIFO Memory Aperture0 62\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo62::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo62::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo62`]
module"]
pub type FIFO62 = crate::Reg<fifo62::FIFO62_SPEC>;
#[doc = "FIFO Memory Aperture0 62"]
pub mod fifo62;
#[doc = "FIFO63 (rw) register accessor: FIFO Memory Aperture0 63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo63::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo63::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo63`]
module"]
pub type FIFO63 = crate::Reg<fifo63::FIFO63_SPEC>;
#[doc = "FIFO Memory Aperture0 63"]
pub mod fifo63;
#[doc = "FIFO64 (rw) register accessor: FIFO Memory Aperture0 64\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo64::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo64`]
module"]
pub type FIFO64 = crate::Reg<fifo64::FIFO64_SPEC>;
#[doc = "FIFO Memory Aperture0 64"]
pub mod fifo64;
#[doc = "FIFO65 (rw) register accessor: FIFO Memory Aperture0 65\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo65::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo65::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo65`]
module"]
pub type FIFO65 = crate::Reg<fifo65::FIFO65_SPEC>;
#[doc = "FIFO Memory Aperture0 65"]
pub mod fifo65;
#[doc = "FIFO66 (rw) register accessor: FIFO Memory Aperture0 66\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo66::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo66::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo66`]
module"]
pub type FIFO66 = crate::Reg<fifo66::FIFO66_SPEC>;
#[doc = "FIFO Memory Aperture0 66"]
pub mod fifo66;
#[doc = "FIFO67 (rw) register accessor: FIFO Memory Aperture0 67\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo67::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo67`]
module"]
pub type FIFO67 = crate::Reg<fifo67::FIFO67_SPEC>;
#[doc = "FIFO Memory Aperture0 67"]
pub mod fifo67;
#[doc = "FIFO68 (rw) register accessor: FIFO Memory Aperture0 68\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo68::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo68::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo68`]
module"]
pub type FIFO68 = crate::Reg<fifo68::FIFO68_SPEC>;
#[doc = "FIFO Memory Aperture0 68"]
pub mod fifo68;
#[doc = "FIFO69 (rw) register accessor: FIFO Memory Aperture0 69\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo69::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo69::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo69`]
module"]
pub type FIFO69 = crate::Reg<fifo69::FIFO69_SPEC>;
#[doc = "FIFO Memory Aperture0 69"]
pub mod fifo69;
#[doc = "FIFO70 (rw) register accessor: FIFO Memory Aperture0 70\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo70::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo70::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo70`]
module"]
pub type FIFO70 = crate::Reg<fifo70::FIFO70_SPEC>;
#[doc = "FIFO Memory Aperture0 70"]
pub mod fifo70;
#[doc = "FIFO71 (rw) register accessor: FIFO Memory Aperture0 71\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo71::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo71::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo71`]
module"]
pub type FIFO71 = crate::Reg<fifo71::FIFO71_SPEC>;
#[doc = "FIFO Memory Aperture0 71"]
pub mod fifo71;
#[doc = "FIFO72 (rw) register accessor: FIFO Memory Aperture0 72\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo72::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo72::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo72`]
module"]
pub type FIFO72 = crate::Reg<fifo72::FIFO72_SPEC>;
#[doc = "FIFO Memory Aperture0 72"]
pub mod fifo72;
#[doc = "FIFO73 (rw) register accessor: FIFO Memory Aperture0 73\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo73::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo73::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo73`]
module"]
pub type FIFO73 = crate::Reg<fifo73::FIFO73_SPEC>;
#[doc = "FIFO Memory Aperture0 73"]
pub mod fifo73;
#[doc = "FIFO74 (rw) register accessor: FIFO Memory Aperture0 74\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo74::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo74::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo74`]
module"]
pub type FIFO74 = crate::Reg<fifo74::FIFO74_SPEC>;
#[doc = "FIFO Memory Aperture0 74"]
pub mod fifo74;
#[doc = "FIFO75 (rw) register accessor: FIFO Memory Aperture0 75\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo75::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo75::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo75`]
module"]
pub type FIFO75 = crate::Reg<fifo75::FIFO75_SPEC>;
#[doc = "FIFO Memory Aperture0 75"]
pub mod fifo75;
#[doc = "FIFO76 (rw) register accessor: FIFO Memory Aperture0 76\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo76::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo76::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo76`]
module"]
pub type FIFO76 = crate::Reg<fifo76::FIFO76_SPEC>;
#[doc = "FIFO Memory Aperture0 76"]
pub mod fifo76;
#[doc = "FIFO77 (rw) register accessor: FIFO Memory Aperture0 77\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo77::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo77::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo77`]
module"]
pub type FIFO77 = crate::Reg<fifo77::FIFO77_SPEC>;
#[doc = "FIFO Memory Aperture0 77"]
pub mod fifo77;
#[doc = "FIFO78 (rw) register accessor: FIFO Memory Aperture0 78\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo78::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo78::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo78`]
module"]
pub type FIFO78 = crate::Reg<fifo78::FIFO78_SPEC>;
#[doc = "FIFO Memory Aperture0 78"]
pub mod fifo78;
#[doc = "FIFO79 (rw) register accessor: FIFO Memory Aperture0 79\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo79::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo79::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo79`]
module"]
pub type FIFO79 = crate::Reg<fifo79::FIFO79_SPEC>;
#[doc = "FIFO Memory Aperture0 79"]
pub mod fifo79;
#[doc = "FIFO80 (rw) register accessor: FIFO Memory Aperture0 80\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo80::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo80::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo80`]
module"]
pub type FIFO80 = crate::Reg<fifo80::FIFO80_SPEC>;
#[doc = "FIFO Memory Aperture0 80"]
pub mod fifo80;
#[doc = "FIFO81 (rw) register accessor: FIFO Memory Aperture0 81\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo81::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo81::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo81`]
module"]
pub type FIFO81 = crate::Reg<fifo81::FIFO81_SPEC>;
#[doc = "FIFO Memory Aperture0 81"]
pub mod fifo81;
#[doc = "FIFO82 (rw) register accessor: FIFO Memory Aperture0 82\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo82::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo82::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo82`]
module"]
pub type FIFO82 = crate::Reg<fifo82::FIFO82_SPEC>;
#[doc = "FIFO Memory Aperture0 82"]
pub mod fifo82;
#[doc = "FIFO83 (rw) register accessor: FIFO Memory Aperture0 83\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo83::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo83::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo83`]
module"]
pub type FIFO83 = crate::Reg<fifo83::FIFO83_SPEC>;
#[doc = "FIFO Memory Aperture0 83"]
pub mod fifo83;
#[doc = "FIFO84 (rw) register accessor: FIFO Memory Aperture0 84\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo84::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo84::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo84`]
module"]
pub type FIFO84 = crate::Reg<fifo84::FIFO84_SPEC>;
#[doc = "FIFO Memory Aperture0 84"]
pub mod fifo84;
#[doc = "FIFO85 (rw) register accessor: FIFO Memory Aperture0 85\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo85::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo85::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo85`]
module"]
pub type FIFO85 = crate::Reg<fifo85::FIFO85_SPEC>;
#[doc = "FIFO Memory Aperture0 85"]
pub mod fifo85;
#[doc = "FIFO86 (rw) register accessor: FIFO Memory Aperture0 86\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo86::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo86::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo86`]
module"]
pub type FIFO86 = crate::Reg<fifo86::FIFO86_SPEC>;
#[doc = "FIFO Memory Aperture0 86"]
pub mod fifo86;
#[doc = "FIFO87 (rw) register accessor: FIFO Memory Aperture0 87\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo87::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo87::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo87`]
module"]
pub type FIFO87 = crate::Reg<fifo87::FIFO87_SPEC>;
#[doc = "FIFO Memory Aperture0 87"]
pub mod fifo87;
#[doc = "FIFO88 (rw) register accessor: FIFO Memory Aperture0 88\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo88::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo88::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo88`]
module"]
pub type FIFO88 = crate::Reg<fifo88::FIFO88_SPEC>;
#[doc = "FIFO Memory Aperture0 88"]
pub mod fifo88;
#[doc = "FIFO89 (rw) register accessor: FIFO Memory Aperture0 89\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo89::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo89::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo89`]
module"]
pub type FIFO89 = crate::Reg<fifo89::FIFO89_SPEC>;
#[doc = "FIFO Memory Aperture0 89"]
pub mod fifo89;
#[doc = "FIFO90 (rw) register accessor: FIFO Memory Aperture0 90\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo90::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo90::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo90`]
module"]
pub type FIFO90 = crate::Reg<fifo90::FIFO90_SPEC>;
#[doc = "FIFO Memory Aperture0 90"]
pub mod fifo90;
#[doc = "FIFO91 (rw) register accessor: FIFO Memory Aperture0 91\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo91::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo91::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo91`]
module"]
pub type FIFO91 = crate::Reg<fifo91::FIFO91_SPEC>;
#[doc = "FIFO Memory Aperture0 91"]
pub mod fifo91;
#[doc = "FIFO92 (rw) register accessor: FIFO Memory Aperture0 92\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo92::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo92::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo92`]
module"]
pub type FIFO92 = crate::Reg<fifo92::FIFO92_SPEC>;
#[doc = "FIFO Memory Aperture0 92"]
pub mod fifo92;
#[doc = "FIFO93 (rw) register accessor: FIFO Memory Aperture0 93\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo93::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo93::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo93`]
module"]
pub type FIFO93 = crate::Reg<fifo93::FIFO93_SPEC>;
#[doc = "FIFO Memory Aperture0 93"]
pub mod fifo93;
#[doc = "FIFO94 (rw) register accessor: FIFO Memory Aperture0 94\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo94::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo94::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo94`]
module"]
pub type FIFO94 = crate::Reg<fifo94::FIFO94_SPEC>;
#[doc = "FIFO Memory Aperture0 94"]
pub mod fifo94;
#[doc = "FIFO95 (rw) register accessor: FIFO Memory Aperture0 95\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo95::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo95::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo95`]
module"]
pub type FIFO95 = crate::Reg<fifo95::FIFO95_SPEC>;
#[doc = "FIFO Memory Aperture0 95"]
pub mod fifo95;
#[doc = "FIFO96 (rw) register accessor: FIFO Memory Aperture0 96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo96::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo96::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo96`]
module"]
pub type FIFO96 = crate::Reg<fifo96::FIFO96_SPEC>;
#[doc = "FIFO Memory Aperture0 96"]
pub mod fifo96;
#[doc = "FIFO97 (rw) register accessor: FIFO Memory Aperture0 97\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo97::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo97::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo97`]
module"]
pub type FIFO97 = crate::Reg<fifo97::FIFO97_SPEC>;
#[doc = "FIFO Memory Aperture0 97"]
pub mod fifo97;
#[doc = "FIFO98 (rw) register accessor: FIFO Memory Aperture0 98\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo98::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo98::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo98`]
module"]
pub type FIFO98 = crate::Reg<fifo98::FIFO98_SPEC>;
#[doc = "FIFO Memory Aperture0 98"]
pub mod fifo98;
#[doc = "FIFO99 (rw) register accessor: FIFO Memory Aperture0 99\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo99::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo99::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo99`]
module"]
pub type FIFO99 = crate::Reg<fifo99::FIFO99_SPEC>;
#[doc = "FIFO Memory Aperture0 99"]
pub mod fifo99;
#[doc = "FIFO100 (rw) register accessor: FIFO Memory Aperture0 100\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo100::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo100::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo100`]
module"]
pub type FIFO100 = crate::Reg<fifo100::FIFO100_SPEC>;
#[doc = "FIFO Memory Aperture0 100"]
pub mod fifo100;
#[doc = "FIFO101 (rw) register accessor: FIFO Memory Aperture0 101\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo101::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo101::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo101`]
module"]
pub type FIFO101 = crate::Reg<fifo101::FIFO101_SPEC>;
#[doc = "FIFO Memory Aperture0 101"]
pub mod fifo101;
#[doc = "FIFO102 (rw) register accessor: FIFO Memory Aperture0 102\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo102::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo102::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo102`]
module"]
pub type FIFO102 = crate::Reg<fifo102::FIFO102_SPEC>;
#[doc = "FIFO Memory Aperture0 102"]
pub mod fifo102;
#[doc = "FIFO103 (rw) register accessor: FIFO Memory Aperture0 103\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo103::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo103::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo103`]
module"]
pub type FIFO103 = crate::Reg<fifo103::FIFO103_SPEC>;
#[doc = "FIFO Memory Aperture0 103"]
pub mod fifo103;
#[doc = "FIFO104 (rw) register accessor: FIFO Memory Aperture0 104\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo104::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo104::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo104`]
module"]
pub type FIFO104 = crate::Reg<fifo104::FIFO104_SPEC>;
#[doc = "FIFO Memory Aperture0 104"]
pub mod fifo104;
#[doc = "FIFO105 (rw) register accessor: FIFO Memory Aperture0 105\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo105::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo105::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo105`]
module"]
pub type FIFO105 = crate::Reg<fifo105::FIFO105_SPEC>;
#[doc = "FIFO Memory Aperture0 105"]
pub mod fifo105;
#[doc = "FIFO106 (rw) register accessor: FIFO Memory Aperture0 106\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo106::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo106::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo106`]
module"]
pub type FIFO106 = crate::Reg<fifo106::FIFO106_SPEC>;
#[doc = "FIFO Memory Aperture0 106"]
pub mod fifo106;
#[doc = "FIFO107 (rw) register accessor: FIFO Memory Aperture0 107\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo107::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo107::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo107`]
module"]
pub type FIFO107 = crate::Reg<fifo107::FIFO107_SPEC>;
#[doc = "FIFO Memory Aperture0 107"]
pub mod fifo107;
#[doc = "FIFO108 (rw) register accessor: FIFO Memory Aperture0 108\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo108::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo108::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo108`]
module"]
pub type FIFO108 = crate::Reg<fifo108::FIFO108_SPEC>;
#[doc = "FIFO Memory Aperture0 108"]
pub mod fifo108;
#[doc = "FIFO109 (rw) register accessor: FIFO Memory Aperture0 109\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo109::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo109::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo109`]
module"]
pub type FIFO109 = crate::Reg<fifo109::FIFO109_SPEC>;
#[doc = "FIFO Memory Aperture0 109"]
pub mod fifo109;
#[doc = "FIFO110 (rw) register accessor: FIFO Memory Aperture0 110\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo110::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo110::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo110`]
module"]
pub type FIFO110 = crate::Reg<fifo110::FIFO110_SPEC>;
#[doc = "FIFO Memory Aperture0 110"]
pub mod fifo110;
#[doc = "FIFO111 (rw) register accessor: FIFO Memory Aperture0 111\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo111::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo111::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo111`]
module"]
pub type FIFO111 = crate::Reg<fifo111::FIFO111_SPEC>;
#[doc = "FIFO Memory Aperture0 111"]
pub mod fifo111;
#[doc = "FIFO112 (rw) register accessor: FIFO Memory Aperture0 112\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo112::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo112::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo112`]
module"]
pub type FIFO112 = crate::Reg<fifo112::FIFO112_SPEC>;
#[doc = "FIFO Memory Aperture0 112"]
pub mod fifo112;
#[doc = "FIFO113 (rw) register accessor: FIFO Memory Aperture0 113\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo113::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo113::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo113`]
module"]
pub type FIFO113 = crate::Reg<fifo113::FIFO113_SPEC>;
#[doc = "FIFO Memory Aperture0 113"]
pub mod fifo113;
#[doc = "FIFO114 (rw) register accessor: FIFO Memory Aperture0 114\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo114::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo114::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo114`]
module"]
pub type FIFO114 = crate::Reg<fifo114::FIFO114_SPEC>;
#[doc = "FIFO Memory Aperture0 114"]
pub mod fifo114;
#[doc = "FIFO115 (rw) register accessor: FIFO Memory Aperture0 115\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo115::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo115::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo115`]
module"]
pub type FIFO115 = crate::Reg<fifo115::FIFO115_SPEC>;
#[doc = "FIFO Memory Aperture0 115"]
pub mod fifo115;
#[doc = "FIFO116 (rw) register accessor: FIFO Memory Aperture0 116\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo116::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo116::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo116`]
module"]
pub type FIFO116 = crate::Reg<fifo116::FIFO116_SPEC>;
#[doc = "FIFO Memory Aperture0 116"]
pub mod fifo116;
#[doc = "FIFO117 (rw) register accessor: FIFO Memory Aperture0 117\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo117::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo117::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo117`]
module"]
pub type FIFO117 = crate::Reg<fifo117::FIFO117_SPEC>;
#[doc = "FIFO Memory Aperture0 117"]
pub mod fifo117;
#[doc = "FIFO118 (rw) register accessor: FIFO Memory Aperture0 118\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo118::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo118::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo118`]
module"]
pub type FIFO118 = crate::Reg<fifo118::FIFO118_SPEC>;
#[doc = "FIFO Memory Aperture0 118"]
pub mod fifo118;
#[doc = "FIFO119 (rw) register accessor: FIFO Memory Aperture0 119\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo119::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo119::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo119`]
module"]
pub type FIFO119 = crate::Reg<fifo119::FIFO119_SPEC>;
#[doc = "FIFO Memory Aperture0 119"]
pub mod fifo119;
#[doc = "FIFO120 (rw) register accessor: FIFO Memory Aperture0 120\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo120::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo120::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo120`]
module"]
pub type FIFO120 = crate::Reg<fifo120::FIFO120_SPEC>;
#[doc = "FIFO Memory Aperture0 120"]
pub mod fifo120;
#[doc = "FIFO121 (rw) register accessor: FIFO Memory Aperture0 121\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo121::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo121::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo121`]
module"]
pub type FIFO121 = crate::Reg<fifo121::FIFO121_SPEC>;
#[doc = "FIFO Memory Aperture0 121"]
pub mod fifo121;
#[doc = "FIFO122 (rw) register accessor: FIFO Memory Aperture0 122\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo122::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo122::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo122`]
module"]
pub type FIFO122 = crate::Reg<fifo122::FIFO122_SPEC>;
#[doc = "FIFO Memory Aperture0 122"]
pub mod fifo122;
#[doc = "FIFO123 (rw) register accessor: FIFO Memory Aperture0 123\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo123::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo123::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo123`]
module"]
pub type FIFO123 = crate::Reg<fifo123::FIFO123_SPEC>;
#[doc = "FIFO Memory Aperture0 123"]
pub mod fifo123;
#[doc = "FIFO124 (rw) register accessor: FIFO Memory Aperture0 124\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo124::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo124::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo124`]
module"]
pub type FIFO124 = crate::Reg<fifo124::FIFO124_SPEC>;
#[doc = "FIFO Memory Aperture0 124"]
pub mod fifo124;
#[doc = "FIFO125 (rw) register accessor: FIFO Memory Aperture0 125\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo125::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo125::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo125`]
module"]
pub type FIFO125 = crate::Reg<fifo125::FIFO125_SPEC>;
#[doc = "FIFO Memory Aperture0 125"]
pub mod fifo125;
#[doc = "FIFO126 (rw) register accessor: FIFO Memory Aperture0 126\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo126::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo126::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo126`]
module"]
pub type FIFO126 = crate::Reg<fifo126::FIFO126_SPEC>;
#[doc = "FIFO Memory Aperture0 126"]
pub mod fifo126;
#[doc = "FIFO127 (rw) register accessor: FIFO Memory Aperture0 127\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo127::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo127::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo127`]
module"]
pub type FIFO127 = crate::Reg<fifo127::FIFO127_SPEC>;
#[doc = "FIFO Memory Aperture0 127"]
pub mod fifo127;
#[doc = "FIFO128 (rw) register accessor: FIFO Memory Aperture0 128\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo128::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo128::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo128`]
module"]
pub type FIFO128 = crate::Reg<fifo128::FIFO128_SPEC>;
#[doc = "FIFO Memory Aperture0 128"]
pub mod fifo128;
#[doc = "FIFO129 (rw) register accessor: FIFO Memory Aperture0 129\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo129::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo129::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo129`]
module"]
pub type FIFO129 = crate::Reg<fifo129::FIFO129_SPEC>;
#[doc = "FIFO Memory Aperture0 129"]
pub mod fifo129;
#[doc = "FIFO130 (rw) register accessor: FIFO Memory Aperture0 130\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo130::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo130::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo130`]
module"]
pub type FIFO130 = crate::Reg<fifo130::FIFO130_SPEC>;
#[doc = "FIFO Memory Aperture0 130"]
pub mod fifo130;
#[doc = "FIFO131 (rw) register accessor: FIFO Memory Aperture0 131\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo131::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo131::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo131`]
module"]
pub type FIFO131 = crate::Reg<fifo131::FIFO131_SPEC>;
#[doc = "FIFO Memory Aperture0 131"]
pub mod fifo131;
#[doc = "FIFO132 (rw) register accessor: FIFO Memory Aperture0 132\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo132::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo132::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo132`]
module"]
pub type FIFO132 = crate::Reg<fifo132::FIFO132_SPEC>;
#[doc = "FIFO Memory Aperture0 132"]
pub mod fifo132;
#[doc = "FIFO133 (rw) register accessor: FIFO Memory Aperture0 133\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo133::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo133::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo133`]
module"]
pub type FIFO133 = crate::Reg<fifo133::FIFO133_SPEC>;
#[doc = "FIFO Memory Aperture0 133"]
pub mod fifo133;
#[doc = "FIFO134 (rw) register accessor: FIFO Memory Aperture0 134\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo134::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo134::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo134`]
module"]
pub type FIFO134 = crate::Reg<fifo134::FIFO134_SPEC>;
#[doc = "FIFO Memory Aperture0 134"]
pub mod fifo134;
#[doc = "FIFO135 (rw) register accessor: FIFO Memory Aperture0 135\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo135::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo135::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo135`]
module"]
pub type FIFO135 = crate::Reg<fifo135::FIFO135_SPEC>;
#[doc = "FIFO Memory Aperture0 135"]
pub mod fifo135;
#[doc = "FIFO136 (rw) register accessor: FIFO Memory Aperture0 136\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo136::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo136::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo136`]
module"]
pub type FIFO136 = crate::Reg<fifo136::FIFO136_SPEC>;
#[doc = "FIFO Memory Aperture0 136"]
pub mod fifo136;
#[doc = "FIFO137 (rw) register accessor: FIFO Memory Aperture0 137\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo137::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo137::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo137`]
module"]
pub type FIFO137 = crate::Reg<fifo137::FIFO137_SPEC>;
#[doc = "FIFO Memory Aperture0 137"]
pub mod fifo137;
#[doc = "FIFO138 (rw) register accessor: FIFO Memory Aperture0 138\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo138::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo138::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo138`]
module"]
pub type FIFO138 = crate::Reg<fifo138::FIFO138_SPEC>;
#[doc = "FIFO Memory Aperture0 138"]
pub mod fifo138;
#[doc = "FIFO139 (rw) register accessor: FIFO Memory Aperture0 139\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo139::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo139::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo139`]
module"]
pub type FIFO139 = crate::Reg<fifo139::FIFO139_SPEC>;
#[doc = "FIFO Memory Aperture0 139"]
pub mod fifo139;
#[doc = "FIFO140 (rw) register accessor: FIFO Memory Aperture0 140\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo140::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo140::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo140`]
module"]
pub type FIFO140 = crate::Reg<fifo140::FIFO140_SPEC>;
#[doc = "FIFO Memory Aperture0 140"]
pub mod fifo140;
#[doc = "FIFO141 (rw) register accessor: FIFO Memory Aperture0 141\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo141::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo141::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo141`]
module"]
pub type FIFO141 = crate::Reg<fifo141::FIFO141_SPEC>;
#[doc = "FIFO Memory Aperture0 141"]
pub mod fifo141;
#[doc = "FIFO142 (rw) register accessor: FIFO Memory Aperture0 142\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo142::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo142::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo142`]
module"]
pub type FIFO142 = crate::Reg<fifo142::FIFO142_SPEC>;
#[doc = "FIFO Memory Aperture0 142"]
pub mod fifo142;
#[doc = "FIFO143 (rw) register accessor: FIFO Memory Aperture0 143\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo143::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo143::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo143`]
module"]
pub type FIFO143 = crate::Reg<fifo143::FIFO143_SPEC>;
#[doc = "FIFO Memory Aperture0 143"]
pub mod fifo143;
#[doc = "FIFO144 (rw) register accessor: FIFO Memory Aperture0 144\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo144::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo144::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo144`]
module"]
pub type FIFO144 = crate::Reg<fifo144::FIFO144_SPEC>;
#[doc = "FIFO Memory Aperture0 144"]
pub mod fifo144;
#[doc = "FIFO145 (rw) register accessor: FIFO Memory Aperture0 145\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo145::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo145::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo145`]
module"]
pub type FIFO145 = crate::Reg<fifo145::FIFO145_SPEC>;
#[doc = "FIFO Memory Aperture0 145"]
pub mod fifo145;
#[doc = "FIFO146 (rw) register accessor: FIFO Memory Aperture0 146\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo146::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo146::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo146`]
module"]
pub type FIFO146 = crate::Reg<fifo146::FIFO146_SPEC>;
#[doc = "FIFO Memory Aperture0 146"]
pub mod fifo146;
#[doc = "FIFO147 (rw) register accessor: FIFO Memory Aperture0 147\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo147::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo147::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo147`]
module"]
pub type FIFO147 = crate::Reg<fifo147::FIFO147_SPEC>;
#[doc = "FIFO Memory Aperture0 147"]
pub mod fifo147;
#[doc = "FIFO148 (rw) register accessor: FIFO Memory Aperture0 148\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo148::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo148::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo148`]
module"]
pub type FIFO148 = crate::Reg<fifo148::FIFO148_SPEC>;
#[doc = "FIFO Memory Aperture0 148"]
pub mod fifo148;
#[doc = "FIFO149 (rw) register accessor: FIFO Memory Aperture0 149\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo149::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo149::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo149`]
module"]
pub type FIFO149 = crate::Reg<fifo149::FIFO149_SPEC>;
#[doc = "FIFO Memory Aperture0 149"]
pub mod fifo149;
#[doc = "FIFO150 (rw) register accessor: FIFO Memory Aperture0 150\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo150::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo150::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo150`]
module"]
pub type FIFO150 = crate::Reg<fifo150::FIFO150_SPEC>;
#[doc = "FIFO Memory Aperture0 150"]
pub mod fifo150;
#[doc = "FIFO151 (rw) register accessor: FIFO Memory Aperture0 151\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo151::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo151::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo151`]
module"]
pub type FIFO151 = crate::Reg<fifo151::FIFO151_SPEC>;
#[doc = "FIFO Memory Aperture0 151"]
pub mod fifo151;
#[doc = "FIFO152 (rw) register accessor: FIFO Memory Aperture0 152\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo152::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo152::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo152`]
module"]
pub type FIFO152 = crate::Reg<fifo152::FIFO152_SPEC>;
#[doc = "FIFO Memory Aperture0 152"]
pub mod fifo152;
#[doc = "FIFO153 (rw) register accessor: FIFO Memory Aperture0 153\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo153::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo153::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo153`]
module"]
pub type FIFO153 = crate::Reg<fifo153::FIFO153_SPEC>;
#[doc = "FIFO Memory Aperture0 153"]
pub mod fifo153;
#[doc = "FIFO154 (rw) register accessor: FIFO Memory Aperture0 154\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo154::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo154::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo154`]
module"]
pub type FIFO154 = crate::Reg<fifo154::FIFO154_SPEC>;
#[doc = "FIFO Memory Aperture0 154"]
pub mod fifo154;
#[doc = "FIFO155 (rw) register accessor: FIFO Memory Aperture0 155\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo155::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo155::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo155`]
module"]
pub type FIFO155 = crate::Reg<fifo155::FIFO155_SPEC>;
#[doc = "FIFO Memory Aperture0 155"]
pub mod fifo155;
#[doc = "FIFO156 (rw) register accessor: FIFO Memory Aperture0 156\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo156::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo156::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo156`]
module"]
pub type FIFO156 = crate::Reg<fifo156::FIFO156_SPEC>;
#[doc = "FIFO Memory Aperture0 156"]
pub mod fifo156;
#[doc = "FIFO157 (rw) register accessor: FIFO Memory Aperture0 157\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo157::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo157::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo157`]
module"]
pub type FIFO157 = crate::Reg<fifo157::FIFO157_SPEC>;
#[doc = "FIFO Memory Aperture0 157"]
pub mod fifo157;
#[doc = "FIFO158 (rw) register accessor: FIFO Memory Aperture0 158\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo158::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo158::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo158`]
module"]
pub type FIFO158 = crate::Reg<fifo158::FIFO158_SPEC>;
#[doc = "FIFO Memory Aperture0 158"]
pub mod fifo158;
#[doc = "FIFO159 (rw) register accessor: FIFO Memory Aperture0 159\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo159::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo159::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo159`]
module"]
pub type FIFO159 = crate::Reg<fifo159::FIFO159_SPEC>;
#[doc = "FIFO Memory Aperture0 159"]
pub mod fifo159;
#[doc = "FIFO160 (rw) register accessor: FIFO Memory Aperture0 160\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo160::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo160::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo160`]
module"]
pub type FIFO160 = crate::Reg<fifo160::FIFO160_SPEC>;
#[doc = "FIFO Memory Aperture0 160"]
pub mod fifo160;
#[doc = "FIFO161 (rw) register accessor: FIFO Memory Aperture0 161\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo161::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo161::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo161`]
module"]
pub type FIFO161 = crate::Reg<fifo161::FIFO161_SPEC>;
#[doc = "FIFO Memory Aperture0 161"]
pub mod fifo161;
#[doc = "FIFO162 (rw) register accessor: FIFO Memory Aperture0 162\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo162::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo162::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo162`]
module"]
pub type FIFO162 = crate::Reg<fifo162::FIFO162_SPEC>;
#[doc = "FIFO Memory Aperture0 162"]
pub mod fifo162;
#[doc = "FIFO163 (rw) register accessor: FIFO Memory Aperture0 163\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo163::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo163::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo163`]
module"]
pub type FIFO163 = crate::Reg<fifo163::FIFO163_SPEC>;
#[doc = "FIFO Memory Aperture0 163"]
pub mod fifo163;
#[doc = "FIFO164 (rw) register accessor: FIFO Memory Aperture0 164\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo164::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo164::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo164`]
module"]
pub type FIFO164 = crate::Reg<fifo164::FIFO164_SPEC>;
#[doc = "FIFO Memory Aperture0 164"]
pub mod fifo164;
#[doc = "FIFO165 (rw) register accessor: FIFO Memory Aperture0 165\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo165::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo165::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo165`]
module"]
pub type FIFO165 = crate::Reg<fifo165::FIFO165_SPEC>;
#[doc = "FIFO Memory Aperture0 165"]
pub mod fifo165;
#[doc = "FIFO166 (rw) register accessor: FIFO Memory Aperture0 166\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo166::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo166::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo166`]
module"]
pub type FIFO166 = crate::Reg<fifo166::FIFO166_SPEC>;
#[doc = "FIFO Memory Aperture0 166"]
pub mod fifo166;
#[doc = "FIFO167 (rw) register accessor: FIFO Memory Aperture0 167\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo167::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo167::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo167`]
module"]
pub type FIFO167 = crate::Reg<fifo167::FIFO167_SPEC>;
#[doc = "FIFO Memory Aperture0 167"]
pub mod fifo167;
#[doc = "FIFO168 (rw) register accessor: FIFO Memory Aperture0 168\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo168::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo168::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo168`]
module"]
pub type FIFO168 = crate::Reg<fifo168::FIFO168_SPEC>;
#[doc = "FIFO Memory Aperture0 168"]
pub mod fifo168;
#[doc = "FIFO169 (rw) register accessor: FIFO Memory Aperture0 169\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo169::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo169::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo169`]
module"]
pub type FIFO169 = crate::Reg<fifo169::FIFO169_SPEC>;
#[doc = "FIFO Memory Aperture0 169"]
pub mod fifo169;
#[doc = "FIFO170 (rw) register accessor: FIFO Memory Aperture0 170\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo170::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo170::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo170`]
module"]
pub type FIFO170 = crate::Reg<fifo170::FIFO170_SPEC>;
#[doc = "FIFO Memory Aperture0 170"]
pub mod fifo170;
#[doc = "FIFO171 (rw) register accessor: FIFO Memory Aperture0 171\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo171::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo171::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo171`]
module"]
pub type FIFO171 = crate::Reg<fifo171::FIFO171_SPEC>;
#[doc = "FIFO Memory Aperture0 171"]
pub mod fifo171;
#[doc = "FIFO172 (rw) register accessor: FIFO Memory Aperture0 172\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo172::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo172::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo172`]
module"]
pub type FIFO172 = crate::Reg<fifo172::FIFO172_SPEC>;
#[doc = "FIFO Memory Aperture0 172"]
pub mod fifo172;
#[doc = "FIFO173 (rw) register accessor: FIFO Memory Aperture0 173\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo173::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo173::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo173`]
module"]
pub type FIFO173 = crate::Reg<fifo173::FIFO173_SPEC>;
#[doc = "FIFO Memory Aperture0 173"]
pub mod fifo173;
#[doc = "FIFO174 (rw) register accessor: FIFO Memory Aperture0 174\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo174::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo174::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo174`]
module"]
pub type FIFO174 = crate::Reg<fifo174::FIFO174_SPEC>;
#[doc = "FIFO Memory Aperture0 174"]
pub mod fifo174;
#[doc = "FIFO175 (rw) register accessor: FIFO Memory Aperture0 175\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo175::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo175::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo175`]
module"]
pub type FIFO175 = crate::Reg<fifo175::FIFO175_SPEC>;
#[doc = "FIFO Memory Aperture0 175"]
pub mod fifo175;
#[doc = "FIFO176 (rw) register accessor: FIFO Memory Aperture0 176\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo176::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo176::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo176`]
module"]
pub type FIFO176 = crate::Reg<fifo176::FIFO176_SPEC>;
#[doc = "FIFO Memory Aperture0 176"]
pub mod fifo176;
#[doc = "FIFO177 (rw) register accessor: FIFO Memory Aperture0 177\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo177::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo177::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo177`]
module"]
pub type FIFO177 = crate::Reg<fifo177::FIFO177_SPEC>;
#[doc = "FIFO Memory Aperture0 177"]
pub mod fifo177;
#[doc = "FIFO178 (rw) register accessor: FIFO Memory Aperture0 178\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo178::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo178::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo178`]
module"]
pub type FIFO178 = crate::Reg<fifo178::FIFO178_SPEC>;
#[doc = "FIFO Memory Aperture0 178"]
pub mod fifo178;
#[doc = "FIFO179 (rw) register accessor: FIFO Memory Aperture0 179\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo179::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo179::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo179`]
module"]
pub type FIFO179 = crate::Reg<fifo179::FIFO179_SPEC>;
#[doc = "FIFO Memory Aperture0 179"]
pub mod fifo179;
#[doc = "FIFO180 (rw) register accessor: FIFO Memory Aperture0 180\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo180::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo180::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo180`]
module"]
pub type FIFO180 = crate::Reg<fifo180::FIFO180_SPEC>;
#[doc = "FIFO Memory Aperture0 180"]
pub mod fifo180;
#[doc = "FIFO181 (rw) register accessor: FIFO Memory Aperture0 181\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo181::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo181::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo181`]
module"]
pub type FIFO181 = crate::Reg<fifo181::FIFO181_SPEC>;
#[doc = "FIFO Memory Aperture0 181"]
pub mod fifo181;
#[doc = "FIFO182 (rw) register accessor: FIFO Memory Aperture0 182\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo182::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo182::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo182`]
module"]
pub type FIFO182 = crate::Reg<fifo182::FIFO182_SPEC>;
#[doc = "FIFO Memory Aperture0 182"]
pub mod fifo182;
#[doc = "FIFO183 (rw) register accessor: FIFO Memory Aperture0 183\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo183::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo183::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo183`]
module"]
pub type FIFO183 = crate::Reg<fifo183::FIFO183_SPEC>;
#[doc = "FIFO Memory Aperture0 183"]
pub mod fifo183;
#[doc = "FIFO184 (rw) register accessor: FIFO Memory Aperture0 184\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo184::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo184::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo184`]
module"]
pub type FIFO184 = crate::Reg<fifo184::FIFO184_SPEC>;
#[doc = "FIFO Memory Aperture0 184"]
pub mod fifo184;
#[doc = "FIFO185 (rw) register accessor: FIFO Memory Aperture0 185\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo185::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo185::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo185`]
module"]
pub type FIFO185 = crate::Reg<fifo185::FIFO185_SPEC>;
#[doc = "FIFO Memory Aperture0 185"]
pub mod fifo185;
#[doc = "FIFO186 (rw) register accessor: FIFO Memory Aperture0 186\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo186::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo186::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo186`]
module"]
pub type FIFO186 = crate::Reg<fifo186::FIFO186_SPEC>;
#[doc = "FIFO Memory Aperture0 186"]
pub mod fifo186;
#[doc = "FIFO187 (rw) register accessor: FIFO Memory Aperture0 187\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo187::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo187::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo187`]
module"]
pub type FIFO187 = crate::Reg<fifo187::FIFO187_SPEC>;
#[doc = "FIFO Memory Aperture0 187"]
pub mod fifo187;
#[doc = "FIFO188 (rw) register accessor: FIFO Memory Aperture0 188\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo188::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo188::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo188`]
module"]
pub type FIFO188 = crate::Reg<fifo188::FIFO188_SPEC>;
#[doc = "FIFO Memory Aperture0 188"]
pub mod fifo188;
#[doc = "FIFO189 (rw) register accessor: FIFO Memory Aperture0 189\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo189::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo189::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo189`]
module"]
pub type FIFO189 = crate::Reg<fifo189::FIFO189_SPEC>;
#[doc = "FIFO Memory Aperture0 189"]
pub mod fifo189;
#[doc = "FIFO190 (rw) register accessor: FIFO Memory Aperture0 190\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo190::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo190::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo190`]
module"]
pub type FIFO190 = crate::Reg<fifo190::FIFO190_SPEC>;
#[doc = "FIFO Memory Aperture0 190"]
pub mod fifo190;
#[doc = "FIFO191 (rw) register accessor: FIFO Memory Aperture0 191\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo191::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo191::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo191`]
module"]
pub type FIFO191 = crate::Reg<fifo191::FIFO191_SPEC>;
#[doc = "FIFO Memory Aperture0 191"]
pub mod fifo191;
#[doc = "FIFO192 (rw) register accessor: FIFO Memory Aperture0 192\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo192::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo192::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo192`]
module"]
pub type FIFO192 = crate::Reg<fifo192::FIFO192_SPEC>;
#[doc = "FIFO Memory Aperture0 192"]
pub mod fifo192;
#[doc = "FIFO193 (rw) register accessor: FIFO Memory Aperture0 193\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo193::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo193::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo193`]
module"]
pub type FIFO193 = crate::Reg<fifo193::FIFO193_SPEC>;
#[doc = "FIFO Memory Aperture0 193"]
pub mod fifo193;
#[doc = "FIFO194 (rw) register accessor: FIFO Memory Aperture0 194\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo194::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo194::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo194`]
module"]
pub type FIFO194 = crate::Reg<fifo194::FIFO194_SPEC>;
#[doc = "FIFO Memory Aperture0 194"]
pub mod fifo194;
#[doc = "FIFO195 (rw) register accessor: FIFO Memory Aperture0 195\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo195::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo195::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo195`]
module"]
pub type FIFO195 = crate::Reg<fifo195::FIFO195_SPEC>;
#[doc = "FIFO Memory Aperture0 195"]
pub mod fifo195;
#[doc = "FIFO196 (rw) register accessor: FIFO Memory Aperture0 196\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo196::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo196::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo196`]
module"]
pub type FIFO196 = crate::Reg<fifo196::FIFO196_SPEC>;
#[doc = "FIFO Memory Aperture0 196"]
pub mod fifo196;
#[doc = "FIFO197 (rw) register accessor: FIFO Memory Aperture0 197\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo197::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo197::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo197`]
module"]
pub type FIFO197 = crate::Reg<fifo197::FIFO197_SPEC>;
#[doc = "FIFO Memory Aperture0 197"]
pub mod fifo197;
#[doc = "FIFO198 (rw) register accessor: FIFO Memory Aperture0 198\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo198::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo198::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo198`]
module"]
pub type FIFO198 = crate::Reg<fifo198::FIFO198_SPEC>;
#[doc = "FIFO Memory Aperture0 198"]
pub mod fifo198;
#[doc = "FIFO199 (rw) register accessor: FIFO Memory Aperture0 199\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo199::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo199::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo199`]
module"]
pub type FIFO199 = crate::Reg<fifo199::FIFO199_SPEC>;
#[doc = "FIFO Memory Aperture0 199"]
pub mod fifo199;
#[doc = "FIFO200 (rw) register accessor: FIFO Memory Aperture0 200\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo200::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo200::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo200`]
module"]
pub type FIFO200 = crate::Reg<fifo200::FIFO200_SPEC>;
#[doc = "FIFO Memory Aperture0 200"]
pub mod fifo200;
#[doc = "FIFO201 (rw) register accessor: FIFO Memory Aperture0 201\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo201::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo201::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo201`]
module"]
pub type FIFO201 = crate::Reg<fifo201::FIFO201_SPEC>;
#[doc = "FIFO Memory Aperture0 201"]
pub mod fifo201;
#[doc = "FIFO202 (rw) register accessor: FIFO Memory Aperture0 202\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo202::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo202::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo202`]
module"]
pub type FIFO202 = crate::Reg<fifo202::FIFO202_SPEC>;
#[doc = "FIFO Memory Aperture0 202"]
pub mod fifo202;
#[doc = "FIFO203 (rw) register accessor: FIFO Memory Aperture0 203\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo203::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo203::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo203`]
module"]
pub type FIFO203 = crate::Reg<fifo203::FIFO203_SPEC>;
#[doc = "FIFO Memory Aperture0 203"]
pub mod fifo203;
#[doc = "FIFO204 (rw) register accessor: FIFO Memory Aperture0 204\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo204::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo204::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo204`]
module"]
pub type FIFO204 = crate::Reg<fifo204::FIFO204_SPEC>;
#[doc = "FIFO Memory Aperture0 204"]
pub mod fifo204;
#[doc = "FIFO205 (rw) register accessor: FIFO Memory Aperture0 205\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo205::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo205::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo205`]
module"]
pub type FIFO205 = crate::Reg<fifo205::FIFO205_SPEC>;
#[doc = "FIFO Memory Aperture0 205"]
pub mod fifo205;
#[doc = "FIFO206 (rw) register accessor: FIFO Memory Aperture0 206\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo206::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo206::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo206`]
module"]
pub type FIFO206 = crate::Reg<fifo206::FIFO206_SPEC>;
#[doc = "FIFO Memory Aperture0 206"]
pub mod fifo206;
#[doc = "FIFO207 (rw) register accessor: FIFO Memory Aperture0 207\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo207::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo207::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo207`]
module"]
pub type FIFO207 = crate::Reg<fifo207::FIFO207_SPEC>;
#[doc = "FIFO Memory Aperture0 207"]
pub mod fifo207;
#[doc = "FIFO208 (rw) register accessor: FIFO Memory Aperture0 208\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo208::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo208::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo208`]
module"]
pub type FIFO208 = crate::Reg<fifo208::FIFO208_SPEC>;
#[doc = "FIFO Memory Aperture0 208"]
pub mod fifo208;
#[doc = "FIFO209 (rw) register accessor: FIFO Memory Aperture0 209\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo209::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo209::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo209`]
module"]
pub type FIFO209 = crate::Reg<fifo209::FIFO209_SPEC>;
#[doc = "FIFO Memory Aperture0 209"]
pub mod fifo209;
#[doc = "FIFO210 (rw) register accessor: FIFO Memory Aperture0 210\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo210::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo210::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo210`]
module"]
pub type FIFO210 = crate::Reg<fifo210::FIFO210_SPEC>;
#[doc = "FIFO Memory Aperture0 210"]
pub mod fifo210;
#[doc = "FIFO211 (rw) register accessor: FIFO Memory Aperture0 211\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo211::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo211::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo211`]
module"]
pub type FIFO211 = crate::Reg<fifo211::FIFO211_SPEC>;
#[doc = "FIFO Memory Aperture0 211"]
pub mod fifo211;
#[doc = "FIFO212 (rw) register accessor: FIFO Memory Aperture0 212\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo212::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo212::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo212`]
module"]
pub type FIFO212 = crate::Reg<fifo212::FIFO212_SPEC>;
#[doc = "FIFO Memory Aperture0 212"]
pub mod fifo212;
#[doc = "FIFO213 (rw) register accessor: FIFO Memory Aperture0 213\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo213::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo213::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo213`]
module"]
pub type FIFO213 = crate::Reg<fifo213::FIFO213_SPEC>;
#[doc = "FIFO Memory Aperture0 213"]
pub mod fifo213;
#[doc = "FIFO214 (rw) register accessor: FIFO Memory Aperture0 214\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo214::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo214::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo214`]
module"]
pub type FIFO214 = crate::Reg<fifo214::FIFO214_SPEC>;
#[doc = "FIFO Memory Aperture0 214"]
pub mod fifo214;
#[doc = "FIFO215 (rw) register accessor: FIFO Memory Aperture0 215\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo215::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo215::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo215`]
module"]
pub type FIFO215 = crate::Reg<fifo215::FIFO215_SPEC>;
#[doc = "FIFO Memory Aperture0 215"]
pub mod fifo215;
#[doc = "FIFO216 (rw) register accessor: FIFO Memory Aperture0 216\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo216::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo216::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo216`]
module"]
pub type FIFO216 = crate::Reg<fifo216::FIFO216_SPEC>;
#[doc = "FIFO Memory Aperture0 216"]
pub mod fifo216;
#[doc = "FIFO217 (rw) register accessor: FIFO Memory Aperture0 217\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo217::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo217::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo217`]
module"]
pub type FIFO217 = crate::Reg<fifo217::FIFO217_SPEC>;
#[doc = "FIFO Memory Aperture0 217"]
pub mod fifo217;
#[doc = "FIFO218 (rw) register accessor: FIFO Memory Aperture0 218\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo218::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo218::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo218`]
module"]
pub type FIFO218 = crate::Reg<fifo218::FIFO218_SPEC>;
#[doc = "FIFO Memory Aperture0 218"]
pub mod fifo218;
#[doc = "FIFO219 (rw) register accessor: FIFO Memory Aperture0 219\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo219::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo219::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo219`]
module"]
pub type FIFO219 = crate::Reg<fifo219::FIFO219_SPEC>;
#[doc = "FIFO Memory Aperture0 219"]
pub mod fifo219;
#[doc = "FIFO220 (rw) register accessor: FIFO Memory Aperture0 220\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo220::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo220::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo220`]
module"]
pub type FIFO220 = crate::Reg<fifo220::FIFO220_SPEC>;
#[doc = "FIFO Memory Aperture0 220"]
pub mod fifo220;
#[doc = "FIFO221 (rw) register accessor: FIFO Memory Aperture0 221\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo221::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo221::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo221`]
module"]
pub type FIFO221 = crate::Reg<fifo221::FIFO221_SPEC>;
#[doc = "FIFO Memory Aperture0 221"]
pub mod fifo221;
#[doc = "FIFO222 (rw) register accessor: FIFO Memory Aperture0 222\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo222::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo222::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo222`]
module"]
pub type FIFO222 = crate::Reg<fifo222::FIFO222_SPEC>;
#[doc = "FIFO Memory Aperture0 222"]
pub mod fifo222;
#[doc = "FIFO223 (rw) register accessor: FIFO Memory Aperture0 223\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo223::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo223::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo223`]
module"]
pub type FIFO223 = crate::Reg<fifo223::FIFO223_SPEC>;
#[doc = "FIFO Memory Aperture0 223"]
pub mod fifo223;
#[doc = "FIFO224 (rw) register accessor: FIFO Memory Aperture0 224\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo224::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo224::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo224`]
module"]
pub type FIFO224 = crate::Reg<fifo224::FIFO224_SPEC>;
#[doc = "FIFO Memory Aperture0 224"]
pub mod fifo224;
#[doc = "FIFO225 (rw) register accessor: FIFO Memory Aperture0 225\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo225::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo225::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo225`]
module"]
pub type FIFO225 = crate::Reg<fifo225::FIFO225_SPEC>;
#[doc = "FIFO Memory Aperture0 225"]
pub mod fifo225;
#[doc = "FIFO226 (rw) register accessor: FIFO Memory Aperture0 226\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo226::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo226::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo226`]
module"]
pub type FIFO226 = crate::Reg<fifo226::FIFO226_SPEC>;
#[doc = "FIFO Memory Aperture0 226"]
pub mod fifo226;
#[doc = "FIFO227 (rw) register accessor: FIFO Memory Aperture0 227\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo227::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo227::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo227`]
module"]
pub type FIFO227 = crate::Reg<fifo227::FIFO227_SPEC>;
#[doc = "FIFO Memory Aperture0 227"]
pub mod fifo227;
#[doc = "FIFO228 (rw) register accessor: FIFO Memory Aperture0 228\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo228::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo228::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo228`]
module"]
pub type FIFO228 = crate::Reg<fifo228::FIFO228_SPEC>;
#[doc = "FIFO Memory Aperture0 228"]
pub mod fifo228;
#[doc = "FIFO229 (rw) register accessor: FIFO Memory Aperture0 229\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo229::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo229::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo229`]
module"]
pub type FIFO229 = crate::Reg<fifo229::FIFO229_SPEC>;
#[doc = "FIFO Memory Aperture0 229"]
pub mod fifo229;
#[doc = "FIFO230 (rw) register accessor: FIFO Memory Aperture0 230\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo230::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo230::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo230`]
module"]
pub type FIFO230 = crate::Reg<fifo230::FIFO230_SPEC>;
#[doc = "FIFO Memory Aperture0 230"]
pub mod fifo230;
#[doc = "FIFO231 (rw) register accessor: FIFO Memory Aperture0 231\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo231::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo231::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo231`]
module"]
pub type FIFO231 = crate::Reg<fifo231::FIFO231_SPEC>;
#[doc = "FIFO Memory Aperture0 231"]
pub mod fifo231;
#[doc = "FIFO232 (rw) register accessor: FIFO Memory Aperture0 232\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo232::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo232::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo232`]
module"]
pub type FIFO232 = crate::Reg<fifo232::FIFO232_SPEC>;
#[doc = "FIFO Memory Aperture0 232"]
pub mod fifo232;
#[doc = "FIFO233 (rw) register accessor: FIFO Memory Aperture0 233\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo233::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo233::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo233`]
module"]
pub type FIFO233 = crate::Reg<fifo233::FIFO233_SPEC>;
#[doc = "FIFO Memory Aperture0 233"]
pub mod fifo233;
#[doc = "FIFO234 (rw) register accessor: FIFO Memory Aperture0 234\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo234::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo234::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo234`]
module"]
pub type FIFO234 = crate::Reg<fifo234::FIFO234_SPEC>;
#[doc = "FIFO Memory Aperture0 234"]
pub mod fifo234;
#[doc = "FIFO235 (rw) register accessor: FIFO Memory Aperture0 235\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo235::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo235::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo235`]
module"]
pub type FIFO235 = crate::Reg<fifo235::FIFO235_SPEC>;
#[doc = "FIFO Memory Aperture0 235"]
pub mod fifo235;
#[doc = "FIFO236 (rw) register accessor: FIFO Memory Aperture0 236\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo236::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo236::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo236`]
module"]
pub type FIFO236 = crate::Reg<fifo236::FIFO236_SPEC>;
#[doc = "FIFO Memory Aperture0 236"]
pub mod fifo236;
#[doc = "FIFO237 (rw) register accessor: FIFO Memory Aperture0 237\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo237::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo237::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo237`]
module"]
pub type FIFO237 = crate::Reg<fifo237::FIFO237_SPEC>;
#[doc = "FIFO Memory Aperture0 237"]
pub mod fifo237;
#[doc = "FIFO238 (rw) register accessor: FIFO Memory Aperture0 238\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo238::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo238::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo238`]
module"]
pub type FIFO238 = crate::Reg<fifo238::FIFO238_SPEC>;
#[doc = "FIFO Memory Aperture0 238"]
pub mod fifo238;
#[doc = "FIFO239 (rw) register accessor: FIFO Memory Aperture0 239\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo239::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo239::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo239`]
module"]
pub type FIFO239 = crate::Reg<fifo239::FIFO239_SPEC>;
#[doc = "FIFO Memory Aperture0 239"]
pub mod fifo239;
#[doc = "FIFO240 (rw) register accessor: FIFO Memory Aperture0 240\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo240::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo240::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo240`]
module"]
pub type FIFO240 = crate::Reg<fifo240::FIFO240_SPEC>;
#[doc = "FIFO Memory Aperture0 240"]
pub mod fifo240;
#[doc = "FIFO241 (rw) register accessor: FIFO Memory Aperture0 241\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo241::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo241::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo241`]
module"]
pub type FIFO241 = crate::Reg<fifo241::FIFO241_SPEC>;
#[doc = "FIFO Memory Aperture0 241"]
pub mod fifo241;
#[doc = "FIFO242 (rw) register accessor: FIFO Memory Aperture0 242\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo242::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo242::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo242`]
module"]
pub type FIFO242 = crate::Reg<fifo242::FIFO242_SPEC>;
#[doc = "FIFO Memory Aperture0 242"]
pub mod fifo242;
#[doc = "FIFO243 (rw) register accessor: FIFO Memory Aperture0 243\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo243::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo243::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo243`]
module"]
pub type FIFO243 = crate::Reg<fifo243::FIFO243_SPEC>;
#[doc = "FIFO Memory Aperture0 243"]
pub mod fifo243;
#[doc = "FIFO244 (rw) register accessor: FIFO Memory Aperture0 244\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo244::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo244::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo244`]
module"]
pub type FIFO244 = crate::Reg<fifo244::FIFO244_SPEC>;
#[doc = "FIFO Memory Aperture0 244"]
pub mod fifo244;
#[doc = "FIFO245 (rw) register accessor: FIFO Memory Aperture0 245\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo245::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo245::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo245`]
module"]
pub type FIFO245 = crate::Reg<fifo245::FIFO245_SPEC>;
#[doc = "FIFO Memory Aperture0 245"]
pub mod fifo245;
#[doc = "FIFO246 (rw) register accessor: FIFO Memory Aperture0 246\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo246::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo246::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo246`]
module"]
pub type FIFO246 = crate::Reg<fifo246::FIFO246_SPEC>;
#[doc = "FIFO Memory Aperture0 246"]
pub mod fifo246;
#[doc = "FIFO247 (rw) register accessor: FIFO Memory Aperture0 247\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo247::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo247::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo247`]
module"]
pub type FIFO247 = crate::Reg<fifo247::FIFO247_SPEC>;
#[doc = "FIFO Memory Aperture0 247"]
pub mod fifo247;
#[doc = "FIFO248 (rw) register accessor: FIFO Memory Aperture0 248\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo248::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo248::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo248`]
module"]
pub type FIFO248 = crate::Reg<fifo248::FIFO248_SPEC>;
#[doc = "FIFO Memory Aperture0 248"]
pub mod fifo248;
#[doc = "FIFO249 (rw) register accessor: FIFO Memory Aperture0 249\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo249::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo249::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo249`]
module"]
pub type FIFO249 = crate::Reg<fifo249::FIFO249_SPEC>;
#[doc = "FIFO Memory Aperture0 249"]
pub mod fifo249;
#[doc = "FIFO250 (rw) register accessor: FIFO Memory Aperture0 250\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo250::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo250::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo250`]
module"]
pub type FIFO250 = crate::Reg<fifo250::FIFO250_SPEC>;
#[doc = "FIFO Memory Aperture0 250"]
pub mod fifo250;
#[doc = "FIFO251 (rw) register accessor: FIFO Memory Aperture0 251\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo251::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo251::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo251`]
module"]
pub type FIFO251 = crate::Reg<fifo251::FIFO251_SPEC>;
#[doc = "FIFO Memory Aperture0 251"]
pub mod fifo251;
#[doc = "FIFO252 (rw) register accessor: FIFO Memory Aperture0 252\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo252::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo252::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo252`]
module"]
pub type FIFO252 = crate::Reg<fifo252::FIFO252_SPEC>;
#[doc = "FIFO Memory Aperture0 252"]
pub mod fifo252;
#[doc = "FIFO253 (rw) register accessor: FIFO Memory Aperture0 253\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo253::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo253::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo253`]
module"]
pub type FIFO253 = crate::Reg<fifo253::FIFO253_SPEC>;
#[doc = "FIFO Memory Aperture0 253"]
pub mod fifo253;
#[doc = "FIFO254 (rw) register accessor: FIFO Memory Aperture0 254\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo254::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo254::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo254`]
module"]
pub type FIFO254 = crate::Reg<fifo254::FIFO254_SPEC>;
#[doc = "FIFO Memory Aperture0 254"]
pub mod fifo254;
#[doc = "FIFO255 (rw) register accessor: FIFO Memory Aperture0 255\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo255::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo255::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo255`]
module"]
pub type FIFO255 = crate::Reg<fifo255::FIFO255_SPEC>;
#[doc = "FIFO Memory Aperture0 255"]
pub mod fifo255;
