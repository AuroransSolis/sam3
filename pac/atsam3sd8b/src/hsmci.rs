#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cr: CR,
    mr: MR,
    dtor: DTOR,
    sdcr: SDCR,
    argr: ARGR,
    cmdr: CMDR,
    blkr: BLKR,
    cstor: CSTOR,
    rspr0: RSPR0,
    rspr1: RSPR1,
    rspr2: RSPR2,
    rspr3: RSPR3,
    rdr: RDR,
    tdr: TDR,
    _reserved14: [u8; 0x08],
    sr: SR,
    ier: IER,
    idr: IDR,
    imr: IMR,
    _reserved18: [u8; 0x04],
    cfg: CFG,
    _reserved19: [u8; 0x8c],
    wpmr: WPMR,
    wpsr: WPSR,
    _reserved21: [u8; 0x14],
    rpr: RPR,
    rcr: RCR,
    tpr: TPR,
    tcr: TCR,
    rnpr: RNPR,
    rncr: RNCR,
    tnpr: TNPR,
    tncr: TNCR,
    ptcr: PTCR,
    ptsr: PTSR,
    _reserved31: [u8; 0xd8],
    fifo0: FIFO0,
    fifo1: FIFO1,
    fifo2: FIFO2,
    fifo3: FIFO3,
    fifo4: FIFO4,
    fifo5: FIFO5,
    fifo6: FIFO6,
    fifo7: FIFO7,
    fifo8: FIFO8,
    fifo9: FIFO9,
    fifo10: FIFO10,
    fifo11: FIFO11,
    fifo12: FIFO12,
    fifo13: FIFO13,
    fifo14: FIFO14,
    fifo15: FIFO15,
    fifo16: FIFO16,
    fifo17: FIFO17,
    fifo18: FIFO18,
    fifo19: FIFO19,
    fifo20: FIFO20,
    fifo21: FIFO21,
    fifo22: FIFO22,
    fifo23: FIFO23,
    fifo24: FIFO24,
    fifo25: FIFO25,
    fifo26: FIFO26,
    fifo27: FIFO27,
    fifo28: FIFO28,
    fifo29: FIFO29,
    fifo30: FIFO30,
    fifo31: FIFO31,
    fifo32: FIFO32,
    fifo33: FIFO33,
    fifo34: FIFO34,
    fifo35: FIFO35,
    fifo36: FIFO36,
    fifo37: FIFO37,
    fifo38: FIFO38,
    fifo39: FIFO39,
    fifo40: FIFO40,
    fifo41: FIFO41,
    fifo42: FIFO42,
    fifo43: FIFO43,
    fifo44: FIFO44,
    fifo45: FIFO45,
    fifo46: FIFO46,
    fifo47: FIFO47,
    fifo48: FIFO48,
    fifo49: FIFO49,
    fifo50: FIFO50,
    fifo51: FIFO51,
    fifo52: FIFO52,
    fifo53: FIFO53,
    fifo54: FIFO54,
    fifo55: FIFO55,
    fifo56: FIFO56,
    fifo57: FIFO57,
    fifo58: FIFO58,
    fifo59: FIFO59,
    fifo60: FIFO60,
    fifo61: FIFO61,
    fifo62: FIFO62,
    fifo63: FIFO63,
    fifo64: FIFO64,
    fifo65: FIFO65,
    fifo66: FIFO66,
    fifo67: FIFO67,
    fifo68: FIFO68,
    fifo69: FIFO69,
    fifo70: FIFO70,
    fifo71: FIFO71,
    fifo72: FIFO72,
    fifo73: FIFO73,
    fifo74: FIFO74,
    fifo75: FIFO75,
    fifo76: FIFO76,
    fifo77: FIFO77,
    fifo78: FIFO78,
    fifo79: FIFO79,
    fifo80: FIFO80,
    fifo81: FIFO81,
    fifo82: FIFO82,
    fifo83: FIFO83,
    fifo84: FIFO84,
    fifo85: FIFO85,
    fifo86: FIFO86,
    fifo87: FIFO87,
    fifo88: FIFO88,
    fifo89: FIFO89,
    fifo90: FIFO90,
    fifo91: FIFO91,
    fifo92: FIFO92,
    fifo93: FIFO93,
    fifo94: FIFO94,
    fifo95: FIFO95,
    fifo96: FIFO96,
    fifo97: FIFO97,
    fifo98: FIFO98,
    fifo99: FIFO99,
    fifo100: FIFO100,
    fifo101: FIFO101,
    fifo102: FIFO102,
    fifo103: FIFO103,
    fifo104: FIFO104,
    fifo105: FIFO105,
    fifo106: FIFO106,
    fifo107: FIFO107,
    fifo108: FIFO108,
    fifo109: FIFO109,
    fifo110: FIFO110,
    fifo111: FIFO111,
    fifo112: FIFO112,
    fifo113: FIFO113,
    fifo114: FIFO114,
    fifo115: FIFO115,
    fifo116: FIFO116,
    fifo117: FIFO117,
    fifo118: FIFO118,
    fifo119: FIFO119,
    fifo120: FIFO120,
    fifo121: FIFO121,
    fifo122: FIFO122,
    fifo123: FIFO123,
    fifo124: FIFO124,
    fifo125: FIFO125,
    fifo126: FIFO126,
    fifo127: FIFO127,
    fifo128: FIFO128,
    fifo129: FIFO129,
    fifo130: FIFO130,
    fifo131: FIFO131,
    fifo132: FIFO132,
    fifo133: FIFO133,
    fifo134: FIFO134,
    fifo135: FIFO135,
    fifo136: FIFO136,
    fifo137: FIFO137,
    fifo138: FIFO138,
    fifo139: FIFO139,
    fifo140: FIFO140,
    fifo141: FIFO141,
    fifo142: FIFO142,
    fifo143: FIFO143,
    fifo144: FIFO144,
    fifo145: FIFO145,
    fifo146: FIFO146,
    fifo147: FIFO147,
    fifo148: FIFO148,
    fifo149: FIFO149,
    fifo150: FIFO150,
    fifo151: FIFO151,
    fifo152: FIFO152,
    fifo153: FIFO153,
    fifo154: FIFO154,
    fifo155: FIFO155,
    fifo156: FIFO156,
    fifo157: FIFO157,
    fifo158: FIFO158,
    fifo159: FIFO159,
    fifo160: FIFO160,
    fifo161: FIFO161,
    fifo162: FIFO162,
    fifo163: FIFO163,
    fifo164: FIFO164,
    fifo165: FIFO165,
    fifo166: FIFO166,
    fifo167: FIFO167,
    fifo168: FIFO168,
    fifo169: FIFO169,
    fifo170: FIFO170,
    fifo171: FIFO171,
    fifo172: FIFO172,
    fifo173: FIFO173,
    fifo174: FIFO174,
    fifo175: FIFO175,
    fifo176: FIFO176,
    fifo177: FIFO177,
    fifo178: FIFO178,
    fifo179: FIFO179,
    fifo180: FIFO180,
    fifo181: FIFO181,
    fifo182: FIFO182,
    fifo183: FIFO183,
    fifo184: FIFO184,
    fifo185: FIFO185,
    fifo186: FIFO186,
    fifo187: FIFO187,
    fifo188: FIFO188,
    fifo189: FIFO189,
    fifo190: FIFO190,
    fifo191: FIFO191,
    fifo192: FIFO192,
    fifo193: FIFO193,
    fifo194: FIFO194,
    fifo195: FIFO195,
    fifo196: FIFO196,
    fifo197: FIFO197,
    fifo198: FIFO198,
    fifo199: FIFO199,
    fifo200: FIFO200,
    fifo201: FIFO201,
    fifo202: FIFO202,
    fifo203: FIFO203,
    fifo204: FIFO204,
    fifo205: FIFO205,
    fifo206: FIFO206,
    fifo207: FIFO207,
    fifo208: FIFO208,
    fifo209: FIFO209,
    fifo210: FIFO210,
    fifo211: FIFO211,
    fifo212: FIFO212,
    fifo213: FIFO213,
    fifo214: FIFO214,
    fifo215: FIFO215,
    fifo216: FIFO216,
    fifo217: FIFO217,
    fifo218: FIFO218,
    fifo219: FIFO219,
    fifo220: FIFO220,
    fifo221: FIFO221,
    fifo222: FIFO222,
    fifo223: FIFO223,
    fifo224: FIFO224,
    fifo225: FIFO225,
    fifo226: FIFO226,
    fifo227: FIFO227,
    fifo228: FIFO228,
    fifo229: FIFO229,
    fifo230: FIFO230,
    fifo231: FIFO231,
    fifo232: FIFO232,
    fifo233: FIFO233,
    fifo234: FIFO234,
    fifo235: FIFO235,
    fifo236: FIFO236,
    fifo237: FIFO237,
    fifo238: FIFO238,
    fifo239: FIFO239,
    fifo240: FIFO240,
    fifo241: FIFO241,
    fifo242: FIFO242,
    fifo243: FIFO243,
    fifo244: FIFO244,
    fifo245: FIFO245,
    fifo246: FIFO246,
    fifo247: FIFO247,
    fifo248: FIFO248,
    fifo249: FIFO249,
    fifo250: FIFO250,
    fifo251: FIFO251,
    fifo252: FIFO252,
    fifo253: FIFO253,
    fifo254: FIFO254,
    fifo255: FIFO255,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &MR {
        &self.mr
    }
    #[doc = "0x08 - Data Timeout Register"]
    #[inline(always)]
    pub const fn dtor(&self) -> &DTOR {
        &self.dtor
    }
    #[doc = "0x0c - SD/SDIO Card Register"]
    #[inline(always)]
    pub const fn sdcr(&self) -> &SDCR {
        &self.sdcr
    }
    #[doc = "0x10 - Argument Register"]
    #[inline(always)]
    pub const fn argr(&self) -> &ARGR {
        &self.argr
    }
    #[doc = "0x14 - Command Register"]
    #[inline(always)]
    pub const fn cmdr(&self) -> &CMDR {
        &self.cmdr
    }
    #[doc = "0x18 - Block Register"]
    #[inline(always)]
    pub const fn blkr(&self) -> &BLKR {
        &self.blkr
    }
    #[doc = "0x1c - Completion Signal Timeout Register"]
    #[inline(always)]
    pub const fn cstor(&self) -> &CSTOR {
        &self.cstor
    }
    #[doc = "0x20 - Response Register 0"]
    #[inline(always)]
    pub const fn rspr0(&self) -> &RSPR0 {
        &self.rspr0
    }
    #[doc = "0x24 - Response Register 1"]
    #[inline(always)]
    pub const fn rspr1(&self) -> &RSPR1 {
        &self.rspr1
    }
    #[doc = "0x28 - Response Register 2"]
    #[inline(always)]
    pub const fn rspr2(&self) -> &RSPR2 {
        &self.rspr2
    }
    #[doc = "0x2c - Response Register 3"]
    #[inline(always)]
    pub const fn rspr3(&self) -> &RSPR3 {
        &self.rspr3
    }
    #[doc = "0x30 - Receive Data Register"]
    #[inline(always)]
    pub const fn rdr(&self) -> &RDR {
        &self.rdr
    }
    #[doc = "0x34 - Transmit Data Register"]
    #[inline(always)]
    pub const fn tdr(&self) -> &TDR {
        &self.tdr
    }
    #[doc = "0x40 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x44 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x48 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    #[doc = "0x4c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    #[doc = "0x54 - Configuration Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &CFG {
        &self.cfg
    }
    #[doc = "0xe4 - Write Protection Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &WPMR {
        &self.wpmr
    }
    #[doc = "0xe8 - Write Protection Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &WPSR {
        &self.wpsr
    }
    #[doc = "0x100 - Receive Pointer Register"]
    #[inline(always)]
    pub const fn rpr(&self) -> &RPR {
        &self.rpr
    }
    #[doc = "0x104 - Receive Counter Register"]
    #[inline(always)]
    pub const fn rcr(&self) -> &RCR {
        &self.rcr
    }
    #[doc = "0x108 - Transmit Pointer Register"]
    #[inline(always)]
    pub const fn tpr(&self) -> &TPR {
        &self.tpr
    }
    #[doc = "0x10c - Transmit Counter Register"]
    #[inline(always)]
    pub const fn tcr(&self) -> &TCR {
        &self.tcr
    }
    #[doc = "0x110 - Receive Next Pointer Register"]
    #[inline(always)]
    pub const fn rnpr(&self) -> &RNPR {
        &self.rnpr
    }
    #[doc = "0x114 - Receive Next Counter Register"]
    #[inline(always)]
    pub const fn rncr(&self) -> &RNCR {
        &self.rncr
    }
    #[doc = "0x118 - Transmit Next Pointer Register"]
    #[inline(always)]
    pub const fn tnpr(&self) -> &TNPR {
        &self.tnpr
    }
    #[doc = "0x11c - Transmit Next Counter Register"]
    #[inline(always)]
    pub const fn tncr(&self) -> &TNCR {
        &self.tncr
    }
    #[doc = "0x120 - Transfer Control Register"]
    #[inline(always)]
    pub const fn ptcr(&self) -> &PTCR {
        &self.ptcr
    }
    #[doc = "0x124 - Transfer Status Register"]
    #[inline(always)]
    pub const fn ptsr(&self) -> &PTSR {
        &self.ptsr
    }
    #[doc = "0x200 - FIFO Memory Aperture0 0"]
    #[inline(always)]
    pub const fn fifo0(&self) -> &FIFO0 {
        &self.fifo0
    }
    #[doc = "0x204 - FIFO Memory Aperture0 1"]
    #[inline(always)]
    pub const fn fifo1(&self) -> &FIFO1 {
        &self.fifo1
    }
    #[doc = "0x208 - FIFO Memory Aperture0 2"]
    #[inline(always)]
    pub const fn fifo2(&self) -> &FIFO2 {
        &self.fifo2
    }
    #[doc = "0x20c - FIFO Memory Aperture0 3"]
    #[inline(always)]
    pub const fn fifo3(&self) -> &FIFO3 {
        &self.fifo3
    }
    #[doc = "0x210 - FIFO Memory Aperture0 4"]
    #[inline(always)]
    pub const fn fifo4(&self) -> &FIFO4 {
        &self.fifo4
    }
    #[doc = "0x214 - FIFO Memory Aperture0 5"]
    #[inline(always)]
    pub const fn fifo5(&self) -> &FIFO5 {
        &self.fifo5
    }
    #[doc = "0x218 - FIFO Memory Aperture0 6"]
    #[inline(always)]
    pub const fn fifo6(&self) -> &FIFO6 {
        &self.fifo6
    }
    #[doc = "0x21c - FIFO Memory Aperture0 7"]
    #[inline(always)]
    pub const fn fifo7(&self) -> &FIFO7 {
        &self.fifo7
    }
    #[doc = "0x220 - FIFO Memory Aperture0 8"]
    #[inline(always)]
    pub const fn fifo8(&self) -> &FIFO8 {
        &self.fifo8
    }
    #[doc = "0x224 - FIFO Memory Aperture0 9"]
    #[inline(always)]
    pub const fn fifo9(&self) -> &FIFO9 {
        &self.fifo9
    }
    #[doc = "0x228 - FIFO Memory Aperture0 10"]
    #[inline(always)]
    pub const fn fifo10(&self) -> &FIFO10 {
        &self.fifo10
    }
    #[doc = "0x22c - FIFO Memory Aperture0 11"]
    #[inline(always)]
    pub const fn fifo11(&self) -> &FIFO11 {
        &self.fifo11
    }
    #[doc = "0x230 - FIFO Memory Aperture0 12"]
    #[inline(always)]
    pub const fn fifo12(&self) -> &FIFO12 {
        &self.fifo12
    }
    #[doc = "0x234 - FIFO Memory Aperture0 13"]
    #[inline(always)]
    pub const fn fifo13(&self) -> &FIFO13 {
        &self.fifo13
    }
    #[doc = "0x238 - FIFO Memory Aperture0 14"]
    #[inline(always)]
    pub const fn fifo14(&self) -> &FIFO14 {
        &self.fifo14
    }
    #[doc = "0x23c - FIFO Memory Aperture0 15"]
    #[inline(always)]
    pub const fn fifo15(&self) -> &FIFO15 {
        &self.fifo15
    }
    #[doc = "0x240 - FIFO Memory Aperture0 16"]
    #[inline(always)]
    pub const fn fifo16(&self) -> &FIFO16 {
        &self.fifo16
    }
    #[doc = "0x244 - FIFO Memory Aperture0 17"]
    #[inline(always)]
    pub const fn fifo17(&self) -> &FIFO17 {
        &self.fifo17
    }
    #[doc = "0x248 - FIFO Memory Aperture0 18"]
    #[inline(always)]
    pub const fn fifo18(&self) -> &FIFO18 {
        &self.fifo18
    }
    #[doc = "0x24c - FIFO Memory Aperture0 19"]
    #[inline(always)]
    pub const fn fifo19(&self) -> &FIFO19 {
        &self.fifo19
    }
    #[doc = "0x250 - FIFO Memory Aperture0 20"]
    #[inline(always)]
    pub const fn fifo20(&self) -> &FIFO20 {
        &self.fifo20
    }
    #[doc = "0x254 - FIFO Memory Aperture0 21"]
    #[inline(always)]
    pub const fn fifo21(&self) -> &FIFO21 {
        &self.fifo21
    }
    #[doc = "0x258 - FIFO Memory Aperture0 22"]
    #[inline(always)]
    pub const fn fifo22(&self) -> &FIFO22 {
        &self.fifo22
    }
    #[doc = "0x25c - FIFO Memory Aperture0 23"]
    #[inline(always)]
    pub const fn fifo23(&self) -> &FIFO23 {
        &self.fifo23
    }
    #[doc = "0x260 - FIFO Memory Aperture0 24"]
    #[inline(always)]
    pub const fn fifo24(&self) -> &FIFO24 {
        &self.fifo24
    }
    #[doc = "0x264 - FIFO Memory Aperture0 25"]
    #[inline(always)]
    pub const fn fifo25(&self) -> &FIFO25 {
        &self.fifo25
    }
    #[doc = "0x268 - FIFO Memory Aperture0 26"]
    #[inline(always)]
    pub const fn fifo26(&self) -> &FIFO26 {
        &self.fifo26
    }
    #[doc = "0x26c - FIFO Memory Aperture0 27"]
    #[inline(always)]
    pub const fn fifo27(&self) -> &FIFO27 {
        &self.fifo27
    }
    #[doc = "0x270 - FIFO Memory Aperture0 28"]
    #[inline(always)]
    pub const fn fifo28(&self) -> &FIFO28 {
        &self.fifo28
    }
    #[doc = "0x274 - FIFO Memory Aperture0 29"]
    #[inline(always)]
    pub const fn fifo29(&self) -> &FIFO29 {
        &self.fifo29
    }
    #[doc = "0x278 - FIFO Memory Aperture0 30"]
    #[inline(always)]
    pub const fn fifo30(&self) -> &FIFO30 {
        &self.fifo30
    }
    #[doc = "0x27c - FIFO Memory Aperture0 31"]
    #[inline(always)]
    pub const fn fifo31(&self) -> &FIFO31 {
        &self.fifo31
    }
    #[doc = "0x280 - FIFO Memory Aperture0 32"]
    #[inline(always)]
    pub const fn fifo32(&self) -> &FIFO32 {
        &self.fifo32
    }
    #[doc = "0x284 - FIFO Memory Aperture0 33"]
    #[inline(always)]
    pub const fn fifo33(&self) -> &FIFO33 {
        &self.fifo33
    }
    #[doc = "0x288 - FIFO Memory Aperture0 34"]
    #[inline(always)]
    pub const fn fifo34(&self) -> &FIFO34 {
        &self.fifo34
    }
    #[doc = "0x28c - FIFO Memory Aperture0 35"]
    #[inline(always)]
    pub const fn fifo35(&self) -> &FIFO35 {
        &self.fifo35
    }
    #[doc = "0x290 - FIFO Memory Aperture0 36"]
    #[inline(always)]
    pub const fn fifo36(&self) -> &FIFO36 {
        &self.fifo36
    }
    #[doc = "0x294 - FIFO Memory Aperture0 37"]
    #[inline(always)]
    pub const fn fifo37(&self) -> &FIFO37 {
        &self.fifo37
    }
    #[doc = "0x298 - FIFO Memory Aperture0 38"]
    #[inline(always)]
    pub const fn fifo38(&self) -> &FIFO38 {
        &self.fifo38
    }
    #[doc = "0x29c - FIFO Memory Aperture0 39"]
    #[inline(always)]
    pub const fn fifo39(&self) -> &FIFO39 {
        &self.fifo39
    }
    #[doc = "0x2a0 - FIFO Memory Aperture0 40"]
    #[inline(always)]
    pub const fn fifo40(&self) -> &FIFO40 {
        &self.fifo40
    }
    #[doc = "0x2a4 - FIFO Memory Aperture0 41"]
    #[inline(always)]
    pub const fn fifo41(&self) -> &FIFO41 {
        &self.fifo41
    }
    #[doc = "0x2a8 - FIFO Memory Aperture0 42"]
    #[inline(always)]
    pub const fn fifo42(&self) -> &FIFO42 {
        &self.fifo42
    }
    #[doc = "0x2ac - FIFO Memory Aperture0 43"]
    #[inline(always)]
    pub const fn fifo43(&self) -> &FIFO43 {
        &self.fifo43
    }
    #[doc = "0x2b0 - FIFO Memory Aperture0 44"]
    #[inline(always)]
    pub const fn fifo44(&self) -> &FIFO44 {
        &self.fifo44
    }
    #[doc = "0x2b4 - FIFO Memory Aperture0 45"]
    #[inline(always)]
    pub const fn fifo45(&self) -> &FIFO45 {
        &self.fifo45
    }
    #[doc = "0x2b8 - FIFO Memory Aperture0 46"]
    #[inline(always)]
    pub const fn fifo46(&self) -> &FIFO46 {
        &self.fifo46
    }
    #[doc = "0x2bc - FIFO Memory Aperture0 47"]
    #[inline(always)]
    pub const fn fifo47(&self) -> &FIFO47 {
        &self.fifo47
    }
    #[doc = "0x2c0 - FIFO Memory Aperture0 48"]
    #[inline(always)]
    pub const fn fifo48(&self) -> &FIFO48 {
        &self.fifo48
    }
    #[doc = "0x2c4 - FIFO Memory Aperture0 49"]
    #[inline(always)]
    pub const fn fifo49(&self) -> &FIFO49 {
        &self.fifo49
    }
    #[doc = "0x2c8 - FIFO Memory Aperture0 50"]
    #[inline(always)]
    pub const fn fifo50(&self) -> &FIFO50 {
        &self.fifo50
    }
    #[doc = "0x2cc - FIFO Memory Aperture0 51"]
    #[inline(always)]
    pub const fn fifo51(&self) -> &FIFO51 {
        &self.fifo51
    }
    #[doc = "0x2d0 - FIFO Memory Aperture0 52"]
    #[inline(always)]
    pub const fn fifo52(&self) -> &FIFO52 {
        &self.fifo52
    }
    #[doc = "0x2d4 - FIFO Memory Aperture0 53"]
    #[inline(always)]
    pub const fn fifo53(&self) -> &FIFO53 {
        &self.fifo53
    }
    #[doc = "0x2d8 - FIFO Memory Aperture0 54"]
    #[inline(always)]
    pub const fn fifo54(&self) -> &FIFO54 {
        &self.fifo54
    }
    #[doc = "0x2dc - FIFO Memory Aperture0 55"]
    #[inline(always)]
    pub const fn fifo55(&self) -> &FIFO55 {
        &self.fifo55
    }
    #[doc = "0x2e0 - FIFO Memory Aperture0 56"]
    #[inline(always)]
    pub const fn fifo56(&self) -> &FIFO56 {
        &self.fifo56
    }
    #[doc = "0x2e4 - FIFO Memory Aperture0 57"]
    #[inline(always)]
    pub const fn fifo57(&self) -> &FIFO57 {
        &self.fifo57
    }
    #[doc = "0x2e8 - FIFO Memory Aperture0 58"]
    #[inline(always)]
    pub const fn fifo58(&self) -> &FIFO58 {
        &self.fifo58
    }
    #[doc = "0x2ec - FIFO Memory Aperture0 59"]
    #[inline(always)]
    pub const fn fifo59(&self) -> &FIFO59 {
        &self.fifo59
    }
    #[doc = "0x2f0 - FIFO Memory Aperture0 60"]
    #[inline(always)]
    pub const fn fifo60(&self) -> &FIFO60 {
        &self.fifo60
    }
    #[doc = "0x2f4 - FIFO Memory Aperture0 61"]
    #[inline(always)]
    pub const fn fifo61(&self) -> &FIFO61 {
        &self.fifo61
    }
    #[doc = "0x2f8 - FIFO Memory Aperture0 62"]
    #[inline(always)]
    pub const fn fifo62(&self) -> &FIFO62 {
        &self.fifo62
    }
    #[doc = "0x2fc - FIFO Memory Aperture0 63"]
    #[inline(always)]
    pub const fn fifo63(&self) -> &FIFO63 {
        &self.fifo63
    }
    #[doc = "0x300 - FIFO Memory Aperture0 64"]
    #[inline(always)]
    pub const fn fifo64(&self) -> &FIFO64 {
        &self.fifo64
    }
    #[doc = "0x304 - FIFO Memory Aperture0 65"]
    #[inline(always)]
    pub const fn fifo65(&self) -> &FIFO65 {
        &self.fifo65
    }
    #[doc = "0x308 - FIFO Memory Aperture0 66"]
    #[inline(always)]
    pub const fn fifo66(&self) -> &FIFO66 {
        &self.fifo66
    }
    #[doc = "0x30c - FIFO Memory Aperture0 67"]
    #[inline(always)]
    pub const fn fifo67(&self) -> &FIFO67 {
        &self.fifo67
    }
    #[doc = "0x310 - FIFO Memory Aperture0 68"]
    #[inline(always)]
    pub const fn fifo68(&self) -> &FIFO68 {
        &self.fifo68
    }
    #[doc = "0x314 - FIFO Memory Aperture0 69"]
    #[inline(always)]
    pub const fn fifo69(&self) -> &FIFO69 {
        &self.fifo69
    }
    #[doc = "0x318 - FIFO Memory Aperture0 70"]
    #[inline(always)]
    pub const fn fifo70(&self) -> &FIFO70 {
        &self.fifo70
    }
    #[doc = "0x31c - FIFO Memory Aperture0 71"]
    #[inline(always)]
    pub const fn fifo71(&self) -> &FIFO71 {
        &self.fifo71
    }
    #[doc = "0x320 - FIFO Memory Aperture0 72"]
    #[inline(always)]
    pub const fn fifo72(&self) -> &FIFO72 {
        &self.fifo72
    }
    #[doc = "0x324 - FIFO Memory Aperture0 73"]
    #[inline(always)]
    pub const fn fifo73(&self) -> &FIFO73 {
        &self.fifo73
    }
    #[doc = "0x328 - FIFO Memory Aperture0 74"]
    #[inline(always)]
    pub const fn fifo74(&self) -> &FIFO74 {
        &self.fifo74
    }
    #[doc = "0x32c - FIFO Memory Aperture0 75"]
    #[inline(always)]
    pub const fn fifo75(&self) -> &FIFO75 {
        &self.fifo75
    }
    #[doc = "0x330 - FIFO Memory Aperture0 76"]
    #[inline(always)]
    pub const fn fifo76(&self) -> &FIFO76 {
        &self.fifo76
    }
    #[doc = "0x334 - FIFO Memory Aperture0 77"]
    #[inline(always)]
    pub const fn fifo77(&self) -> &FIFO77 {
        &self.fifo77
    }
    #[doc = "0x338 - FIFO Memory Aperture0 78"]
    #[inline(always)]
    pub const fn fifo78(&self) -> &FIFO78 {
        &self.fifo78
    }
    #[doc = "0x33c - FIFO Memory Aperture0 79"]
    #[inline(always)]
    pub const fn fifo79(&self) -> &FIFO79 {
        &self.fifo79
    }
    #[doc = "0x340 - FIFO Memory Aperture0 80"]
    #[inline(always)]
    pub const fn fifo80(&self) -> &FIFO80 {
        &self.fifo80
    }
    #[doc = "0x344 - FIFO Memory Aperture0 81"]
    #[inline(always)]
    pub const fn fifo81(&self) -> &FIFO81 {
        &self.fifo81
    }
    #[doc = "0x348 - FIFO Memory Aperture0 82"]
    #[inline(always)]
    pub const fn fifo82(&self) -> &FIFO82 {
        &self.fifo82
    }
    #[doc = "0x34c - FIFO Memory Aperture0 83"]
    #[inline(always)]
    pub const fn fifo83(&self) -> &FIFO83 {
        &self.fifo83
    }
    #[doc = "0x350 - FIFO Memory Aperture0 84"]
    #[inline(always)]
    pub const fn fifo84(&self) -> &FIFO84 {
        &self.fifo84
    }
    #[doc = "0x354 - FIFO Memory Aperture0 85"]
    #[inline(always)]
    pub const fn fifo85(&self) -> &FIFO85 {
        &self.fifo85
    }
    #[doc = "0x358 - FIFO Memory Aperture0 86"]
    #[inline(always)]
    pub const fn fifo86(&self) -> &FIFO86 {
        &self.fifo86
    }
    #[doc = "0x35c - FIFO Memory Aperture0 87"]
    #[inline(always)]
    pub const fn fifo87(&self) -> &FIFO87 {
        &self.fifo87
    }
    #[doc = "0x360 - FIFO Memory Aperture0 88"]
    #[inline(always)]
    pub const fn fifo88(&self) -> &FIFO88 {
        &self.fifo88
    }
    #[doc = "0x364 - FIFO Memory Aperture0 89"]
    #[inline(always)]
    pub const fn fifo89(&self) -> &FIFO89 {
        &self.fifo89
    }
    #[doc = "0x368 - FIFO Memory Aperture0 90"]
    #[inline(always)]
    pub const fn fifo90(&self) -> &FIFO90 {
        &self.fifo90
    }
    #[doc = "0x36c - FIFO Memory Aperture0 91"]
    #[inline(always)]
    pub const fn fifo91(&self) -> &FIFO91 {
        &self.fifo91
    }
    #[doc = "0x370 - FIFO Memory Aperture0 92"]
    #[inline(always)]
    pub const fn fifo92(&self) -> &FIFO92 {
        &self.fifo92
    }
    #[doc = "0x374 - FIFO Memory Aperture0 93"]
    #[inline(always)]
    pub const fn fifo93(&self) -> &FIFO93 {
        &self.fifo93
    }
    #[doc = "0x378 - FIFO Memory Aperture0 94"]
    #[inline(always)]
    pub const fn fifo94(&self) -> &FIFO94 {
        &self.fifo94
    }
    #[doc = "0x37c - FIFO Memory Aperture0 95"]
    #[inline(always)]
    pub const fn fifo95(&self) -> &FIFO95 {
        &self.fifo95
    }
    #[doc = "0x380 - FIFO Memory Aperture0 96"]
    #[inline(always)]
    pub const fn fifo96(&self) -> &FIFO96 {
        &self.fifo96
    }
    #[doc = "0x384 - FIFO Memory Aperture0 97"]
    #[inline(always)]
    pub const fn fifo97(&self) -> &FIFO97 {
        &self.fifo97
    }
    #[doc = "0x388 - FIFO Memory Aperture0 98"]
    #[inline(always)]
    pub const fn fifo98(&self) -> &FIFO98 {
        &self.fifo98
    }
    #[doc = "0x38c - FIFO Memory Aperture0 99"]
    #[inline(always)]
    pub const fn fifo99(&self) -> &FIFO99 {
        &self.fifo99
    }
    #[doc = "0x390 - FIFO Memory Aperture0 100"]
    #[inline(always)]
    pub const fn fifo100(&self) -> &FIFO100 {
        &self.fifo100
    }
    #[doc = "0x394 - FIFO Memory Aperture0 101"]
    #[inline(always)]
    pub const fn fifo101(&self) -> &FIFO101 {
        &self.fifo101
    }
    #[doc = "0x398 - FIFO Memory Aperture0 102"]
    #[inline(always)]
    pub const fn fifo102(&self) -> &FIFO102 {
        &self.fifo102
    }
    #[doc = "0x39c - FIFO Memory Aperture0 103"]
    #[inline(always)]
    pub const fn fifo103(&self) -> &FIFO103 {
        &self.fifo103
    }
    #[doc = "0x3a0 - FIFO Memory Aperture0 104"]
    #[inline(always)]
    pub const fn fifo104(&self) -> &FIFO104 {
        &self.fifo104
    }
    #[doc = "0x3a4 - FIFO Memory Aperture0 105"]
    #[inline(always)]
    pub const fn fifo105(&self) -> &FIFO105 {
        &self.fifo105
    }
    #[doc = "0x3a8 - FIFO Memory Aperture0 106"]
    #[inline(always)]
    pub const fn fifo106(&self) -> &FIFO106 {
        &self.fifo106
    }
    #[doc = "0x3ac - FIFO Memory Aperture0 107"]
    #[inline(always)]
    pub const fn fifo107(&self) -> &FIFO107 {
        &self.fifo107
    }
    #[doc = "0x3b0 - FIFO Memory Aperture0 108"]
    #[inline(always)]
    pub const fn fifo108(&self) -> &FIFO108 {
        &self.fifo108
    }
    #[doc = "0x3b4 - FIFO Memory Aperture0 109"]
    #[inline(always)]
    pub const fn fifo109(&self) -> &FIFO109 {
        &self.fifo109
    }
    #[doc = "0x3b8 - FIFO Memory Aperture0 110"]
    #[inline(always)]
    pub const fn fifo110(&self) -> &FIFO110 {
        &self.fifo110
    }
    #[doc = "0x3bc - FIFO Memory Aperture0 111"]
    #[inline(always)]
    pub const fn fifo111(&self) -> &FIFO111 {
        &self.fifo111
    }
    #[doc = "0x3c0 - FIFO Memory Aperture0 112"]
    #[inline(always)]
    pub const fn fifo112(&self) -> &FIFO112 {
        &self.fifo112
    }
    #[doc = "0x3c4 - FIFO Memory Aperture0 113"]
    #[inline(always)]
    pub const fn fifo113(&self) -> &FIFO113 {
        &self.fifo113
    }
    #[doc = "0x3c8 - FIFO Memory Aperture0 114"]
    #[inline(always)]
    pub const fn fifo114(&self) -> &FIFO114 {
        &self.fifo114
    }
    #[doc = "0x3cc - FIFO Memory Aperture0 115"]
    #[inline(always)]
    pub const fn fifo115(&self) -> &FIFO115 {
        &self.fifo115
    }
    #[doc = "0x3d0 - FIFO Memory Aperture0 116"]
    #[inline(always)]
    pub const fn fifo116(&self) -> &FIFO116 {
        &self.fifo116
    }
    #[doc = "0x3d4 - FIFO Memory Aperture0 117"]
    #[inline(always)]
    pub const fn fifo117(&self) -> &FIFO117 {
        &self.fifo117
    }
    #[doc = "0x3d8 - FIFO Memory Aperture0 118"]
    #[inline(always)]
    pub const fn fifo118(&self) -> &FIFO118 {
        &self.fifo118
    }
    #[doc = "0x3dc - FIFO Memory Aperture0 119"]
    #[inline(always)]
    pub const fn fifo119(&self) -> &FIFO119 {
        &self.fifo119
    }
    #[doc = "0x3e0 - FIFO Memory Aperture0 120"]
    #[inline(always)]
    pub const fn fifo120(&self) -> &FIFO120 {
        &self.fifo120
    }
    #[doc = "0x3e4 - FIFO Memory Aperture0 121"]
    #[inline(always)]
    pub const fn fifo121(&self) -> &FIFO121 {
        &self.fifo121
    }
    #[doc = "0x3e8 - FIFO Memory Aperture0 122"]
    #[inline(always)]
    pub const fn fifo122(&self) -> &FIFO122 {
        &self.fifo122
    }
    #[doc = "0x3ec - FIFO Memory Aperture0 123"]
    #[inline(always)]
    pub const fn fifo123(&self) -> &FIFO123 {
        &self.fifo123
    }
    #[doc = "0x3f0 - FIFO Memory Aperture0 124"]
    #[inline(always)]
    pub const fn fifo124(&self) -> &FIFO124 {
        &self.fifo124
    }
    #[doc = "0x3f4 - FIFO Memory Aperture0 125"]
    #[inline(always)]
    pub const fn fifo125(&self) -> &FIFO125 {
        &self.fifo125
    }
    #[doc = "0x3f8 - FIFO Memory Aperture0 126"]
    #[inline(always)]
    pub const fn fifo126(&self) -> &FIFO126 {
        &self.fifo126
    }
    #[doc = "0x3fc - FIFO Memory Aperture0 127"]
    #[inline(always)]
    pub const fn fifo127(&self) -> &FIFO127 {
        &self.fifo127
    }
    #[doc = "0x400 - FIFO Memory Aperture0 128"]
    #[inline(always)]
    pub const fn fifo128(&self) -> &FIFO128 {
        &self.fifo128
    }
    #[doc = "0x404 - FIFO Memory Aperture0 129"]
    #[inline(always)]
    pub const fn fifo129(&self) -> &FIFO129 {
        &self.fifo129
    }
    #[doc = "0x408 - FIFO Memory Aperture0 130"]
    #[inline(always)]
    pub const fn fifo130(&self) -> &FIFO130 {
        &self.fifo130
    }
    #[doc = "0x40c - FIFO Memory Aperture0 131"]
    #[inline(always)]
    pub const fn fifo131(&self) -> &FIFO131 {
        &self.fifo131
    }
    #[doc = "0x410 - FIFO Memory Aperture0 132"]
    #[inline(always)]
    pub const fn fifo132(&self) -> &FIFO132 {
        &self.fifo132
    }
    #[doc = "0x414 - FIFO Memory Aperture0 133"]
    #[inline(always)]
    pub const fn fifo133(&self) -> &FIFO133 {
        &self.fifo133
    }
    #[doc = "0x418 - FIFO Memory Aperture0 134"]
    #[inline(always)]
    pub const fn fifo134(&self) -> &FIFO134 {
        &self.fifo134
    }
    #[doc = "0x41c - FIFO Memory Aperture0 135"]
    #[inline(always)]
    pub const fn fifo135(&self) -> &FIFO135 {
        &self.fifo135
    }
    #[doc = "0x420 - FIFO Memory Aperture0 136"]
    #[inline(always)]
    pub const fn fifo136(&self) -> &FIFO136 {
        &self.fifo136
    }
    #[doc = "0x424 - FIFO Memory Aperture0 137"]
    #[inline(always)]
    pub const fn fifo137(&self) -> &FIFO137 {
        &self.fifo137
    }
    #[doc = "0x428 - FIFO Memory Aperture0 138"]
    #[inline(always)]
    pub const fn fifo138(&self) -> &FIFO138 {
        &self.fifo138
    }
    #[doc = "0x42c - FIFO Memory Aperture0 139"]
    #[inline(always)]
    pub const fn fifo139(&self) -> &FIFO139 {
        &self.fifo139
    }
    #[doc = "0x430 - FIFO Memory Aperture0 140"]
    #[inline(always)]
    pub const fn fifo140(&self) -> &FIFO140 {
        &self.fifo140
    }
    #[doc = "0x434 - FIFO Memory Aperture0 141"]
    #[inline(always)]
    pub const fn fifo141(&self) -> &FIFO141 {
        &self.fifo141
    }
    #[doc = "0x438 - FIFO Memory Aperture0 142"]
    #[inline(always)]
    pub const fn fifo142(&self) -> &FIFO142 {
        &self.fifo142
    }
    #[doc = "0x43c - FIFO Memory Aperture0 143"]
    #[inline(always)]
    pub const fn fifo143(&self) -> &FIFO143 {
        &self.fifo143
    }
    #[doc = "0x440 - FIFO Memory Aperture0 144"]
    #[inline(always)]
    pub const fn fifo144(&self) -> &FIFO144 {
        &self.fifo144
    }
    #[doc = "0x444 - FIFO Memory Aperture0 145"]
    #[inline(always)]
    pub const fn fifo145(&self) -> &FIFO145 {
        &self.fifo145
    }
    #[doc = "0x448 - FIFO Memory Aperture0 146"]
    #[inline(always)]
    pub const fn fifo146(&self) -> &FIFO146 {
        &self.fifo146
    }
    #[doc = "0x44c - FIFO Memory Aperture0 147"]
    #[inline(always)]
    pub const fn fifo147(&self) -> &FIFO147 {
        &self.fifo147
    }
    #[doc = "0x450 - FIFO Memory Aperture0 148"]
    #[inline(always)]
    pub const fn fifo148(&self) -> &FIFO148 {
        &self.fifo148
    }
    #[doc = "0x454 - FIFO Memory Aperture0 149"]
    #[inline(always)]
    pub const fn fifo149(&self) -> &FIFO149 {
        &self.fifo149
    }
    #[doc = "0x458 - FIFO Memory Aperture0 150"]
    #[inline(always)]
    pub const fn fifo150(&self) -> &FIFO150 {
        &self.fifo150
    }
    #[doc = "0x45c - FIFO Memory Aperture0 151"]
    #[inline(always)]
    pub const fn fifo151(&self) -> &FIFO151 {
        &self.fifo151
    }
    #[doc = "0x460 - FIFO Memory Aperture0 152"]
    #[inline(always)]
    pub const fn fifo152(&self) -> &FIFO152 {
        &self.fifo152
    }
    #[doc = "0x464 - FIFO Memory Aperture0 153"]
    #[inline(always)]
    pub const fn fifo153(&self) -> &FIFO153 {
        &self.fifo153
    }
    #[doc = "0x468 - FIFO Memory Aperture0 154"]
    #[inline(always)]
    pub const fn fifo154(&self) -> &FIFO154 {
        &self.fifo154
    }
    #[doc = "0x46c - FIFO Memory Aperture0 155"]
    #[inline(always)]
    pub const fn fifo155(&self) -> &FIFO155 {
        &self.fifo155
    }
    #[doc = "0x470 - FIFO Memory Aperture0 156"]
    #[inline(always)]
    pub const fn fifo156(&self) -> &FIFO156 {
        &self.fifo156
    }
    #[doc = "0x474 - FIFO Memory Aperture0 157"]
    #[inline(always)]
    pub const fn fifo157(&self) -> &FIFO157 {
        &self.fifo157
    }
    #[doc = "0x478 - FIFO Memory Aperture0 158"]
    #[inline(always)]
    pub const fn fifo158(&self) -> &FIFO158 {
        &self.fifo158
    }
    #[doc = "0x47c - FIFO Memory Aperture0 159"]
    #[inline(always)]
    pub const fn fifo159(&self) -> &FIFO159 {
        &self.fifo159
    }
    #[doc = "0x480 - FIFO Memory Aperture0 160"]
    #[inline(always)]
    pub const fn fifo160(&self) -> &FIFO160 {
        &self.fifo160
    }
    #[doc = "0x484 - FIFO Memory Aperture0 161"]
    #[inline(always)]
    pub const fn fifo161(&self) -> &FIFO161 {
        &self.fifo161
    }
    #[doc = "0x488 - FIFO Memory Aperture0 162"]
    #[inline(always)]
    pub const fn fifo162(&self) -> &FIFO162 {
        &self.fifo162
    }
    #[doc = "0x48c - FIFO Memory Aperture0 163"]
    #[inline(always)]
    pub const fn fifo163(&self) -> &FIFO163 {
        &self.fifo163
    }
    #[doc = "0x490 - FIFO Memory Aperture0 164"]
    #[inline(always)]
    pub const fn fifo164(&self) -> &FIFO164 {
        &self.fifo164
    }
    #[doc = "0x494 - FIFO Memory Aperture0 165"]
    #[inline(always)]
    pub const fn fifo165(&self) -> &FIFO165 {
        &self.fifo165
    }
    #[doc = "0x498 - FIFO Memory Aperture0 166"]
    #[inline(always)]
    pub const fn fifo166(&self) -> &FIFO166 {
        &self.fifo166
    }
    #[doc = "0x49c - FIFO Memory Aperture0 167"]
    #[inline(always)]
    pub const fn fifo167(&self) -> &FIFO167 {
        &self.fifo167
    }
    #[doc = "0x4a0 - FIFO Memory Aperture0 168"]
    #[inline(always)]
    pub const fn fifo168(&self) -> &FIFO168 {
        &self.fifo168
    }
    #[doc = "0x4a4 - FIFO Memory Aperture0 169"]
    #[inline(always)]
    pub const fn fifo169(&self) -> &FIFO169 {
        &self.fifo169
    }
    #[doc = "0x4a8 - FIFO Memory Aperture0 170"]
    #[inline(always)]
    pub const fn fifo170(&self) -> &FIFO170 {
        &self.fifo170
    }
    #[doc = "0x4ac - FIFO Memory Aperture0 171"]
    #[inline(always)]
    pub const fn fifo171(&self) -> &FIFO171 {
        &self.fifo171
    }
    #[doc = "0x4b0 - FIFO Memory Aperture0 172"]
    #[inline(always)]
    pub const fn fifo172(&self) -> &FIFO172 {
        &self.fifo172
    }
    #[doc = "0x4b4 - FIFO Memory Aperture0 173"]
    #[inline(always)]
    pub const fn fifo173(&self) -> &FIFO173 {
        &self.fifo173
    }
    #[doc = "0x4b8 - FIFO Memory Aperture0 174"]
    #[inline(always)]
    pub const fn fifo174(&self) -> &FIFO174 {
        &self.fifo174
    }
    #[doc = "0x4bc - FIFO Memory Aperture0 175"]
    #[inline(always)]
    pub const fn fifo175(&self) -> &FIFO175 {
        &self.fifo175
    }
    #[doc = "0x4c0 - FIFO Memory Aperture0 176"]
    #[inline(always)]
    pub const fn fifo176(&self) -> &FIFO176 {
        &self.fifo176
    }
    #[doc = "0x4c4 - FIFO Memory Aperture0 177"]
    #[inline(always)]
    pub const fn fifo177(&self) -> &FIFO177 {
        &self.fifo177
    }
    #[doc = "0x4c8 - FIFO Memory Aperture0 178"]
    #[inline(always)]
    pub const fn fifo178(&self) -> &FIFO178 {
        &self.fifo178
    }
    #[doc = "0x4cc - FIFO Memory Aperture0 179"]
    #[inline(always)]
    pub const fn fifo179(&self) -> &FIFO179 {
        &self.fifo179
    }
    #[doc = "0x4d0 - FIFO Memory Aperture0 180"]
    #[inline(always)]
    pub const fn fifo180(&self) -> &FIFO180 {
        &self.fifo180
    }
    #[doc = "0x4d4 - FIFO Memory Aperture0 181"]
    #[inline(always)]
    pub const fn fifo181(&self) -> &FIFO181 {
        &self.fifo181
    }
    #[doc = "0x4d8 - FIFO Memory Aperture0 182"]
    #[inline(always)]
    pub const fn fifo182(&self) -> &FIFO182 {
        &self.fifo182
    }
    #[doc = "0x4dc - FIFO Memory Aperture0 183"]
    #[inline(always)]
    pub const fn fifo183(&self) -> &FIFO183 {
        &self.fifo183
    }
    #[doc = "0x4e0 - FIFO Memory Aperture0 184"]
    #[inline(always)]
    pub const fn fifo184(&self) -> &FIFO184 {
        &self.fifo184
    }
    #[doc = "0x4e4 - FIFO Memory Aperture0 185"]
    #[inline(always)]
    pub const fn fifo185(&self) -> &FIFO185 {
        &self.fifo185
    }
    #[doc = "0x4e8 - FIFO Memory Aperture0 186"]
    #[inline(always)]
    pub const fn fifo186(&self) -> &FIFO186 {
        &self.fifo186
    }
    #[doc = "0x4ec - FIFO Memory Aperture0 187"]
    #[inline(always)]
    pub const fn fifo187(&self) -> &FIFO187 {
        &self.fifo187
    }
    #[doc = "0x4f0 - FIFO Memory Aperture0 188"]
    #[inline(always)]
    pub const fn fifo188(&self) -> &FIFO188 {
        &self.fifo188
    }
    #[doc = "0x4f4 - FIFO Memory Aperture0 189"]
    #[inline(always)]
    pub const fn fifo189(&self) -> &FIFO189 {
        &self.fifo189
    }
    #[doc = "0x4f8 - FIFO Memory Aperture0 190"]
    #[inline(always)]
    pub const fn fifo190(&self) -> &FIFO190 {
        &self.fifo190
    }
    #[doc = "0x4fc - FIFO Memory Aperture0 191"]
    #[inline(always)]
    pub const fn fifo191(&self) -> &FIFO191 {
        &self.fifo191
    }
    #[doc = "0x500 - FIFO Memory Aperture0 192"]
    #[inline(always)]
    pub const fn fifo192(&self) -> &FIFO192 {
        &self.fifo192
    }
    #[doc = "0x504 - FIFO Memory Aperture0 193"]
    #[inline(always)]
    pub const fn fifo193(&self) -> &FIFO193 {
        &self.fifo193
    }
    #[doc = "0x508 - FIFO Memory Aperture0 194"]
    #[inline(always)]
    pub const fn fifo194(&self) -> &FIFO194 {
        &self.fifo194
    }
    #[doc = "0x50c - FIFO Memory Aperture0 195"]
    #[inline(always)]
    pub const fn fifo195(&self) -> &FIFO195 {
        &self.fifo195
    }
    #[doc = "0x510 - FIFO Memory Aperture0 196"]
    #[inline(always)]
    pub const fn fifo196(&self) -> &FIFO196 {
        &self.fifo196
    }
    #[doc = "0x514 - FIFO Memory Aperture0 197"]
    #[inline(always)]
    pub const fn fifo197(&self) -> &FIFO197 {
        &self.fifo197
    }
    #[doc = "0x518 - FIFO Memory Aperture0 198"]
    #[inline(always)]
    pub const fn fifo198(&self) -> &FIFO198 {
        &self.fifo198
    }
    #[doc = "0x51c - FIFO Memory Aperture0 199"]
    #[inline(always)]
    pub const fn fifo199(&self) -> &FIFO199 {
        &self.fifo199
    }
    #[doc = "0x520 - FIFO Memory Aperture0 200"]
    #[inline(always)]
    pub const fn fifo200(&self) -> &FIFO200 {
        &self.fifo200
    }
    #[doc = "0x524 - FIFO Memory Aperture0 201"]
    #[inline(always)]
    pub const fn fifo201(&self) -> &FIFO201 {
        &self.fifo201
    }
    #[doc = "0x528 - FIFO Memory Aperture0 202"]
    #[inline(always)]
    pub const fn fifo202(&self) -> &FIFO202 {
        &self.fifo202
    }
    #[doc = "0x52c - FIFO Memory Aperture0 203"]
    #[inline(always)]
    pub const fn fifo203(&self) -> &FIFO203 {
        &self.fifo203
    }
    #[doc = "0x530 - FIFO Memory Aperture0 204"]
    #[inline(always)]
    pub const fn fifo204(&self) -> &FIFO204 {
        &self.fifo204
    }
    #[doc = "0x534 - FIFO Memory Aperture0 205"]
    #[inline(always)]
    pub const fn fifo205(&self) -> &FIFO205 {
        &self.fifo205
    }
    #[doc = "0x538 - FIFO Memory Aperture0 206"]
    #[inline(always)]
    pub const fn fifo206(&self) -> &FIFO206 {
        &self.fifo206
    }
    #[doc = "0x53c - FIFO Memory Aperture0 207"]
    #[inline(always)]
    pub const fn fifo207(&self) -> &FIFO207 {
        &self.fifo207
    }
    #[doc = "0x540 - FIFO Memory Aperture0 208"]
    #[inline(always)]
    pub const fn fifo208(&self) -> &FIFO208 {
        &self.fifo208
    }
    #[doc = "0x544 - FIFO Memory Aperture0 209"]
    #[inline(always)]
    pub const fn fifo209(&self) -> &FIFO209 {
        &self.fifo209
    }
    #[doc = "0x548 - FIFO Memory Aperture0 210"]
    #[inline(always)]
    pub const fn fifo210(&self) -> &FIFO210 {
        &self.fifo210
    }
    #[doc = "0x54c - FIFO Memory Aperture0 211"]
    #[inline(always)]
    pub const fn fifo211(&self) -> &FIFO211 {
        &self.fifo211
    }
    #[doc = "0x550 - FIFO Memory Aperture0 212"]
    #[inline(always)]
    pub const fn fifo212(&self) -> &FIFO212 {
        &self.fifo212
    }
    #[doc = "0x554 - FIFO Memory Aperture0 213"]
    #[inline(always)]
    pub const fn fifo213(&self) -> &FIFO213 {
        &self.fifo213
    }
    #[doc = "0x558 - FIFO Memory Aperture0 214"]
    #[inline(always)]
    pub const fn fifo214(&self) -> &FIFO214 {
        &self.fifo214
    }
    #[doc = "0x55c - FIFO Memory Aperture0 215"]
    #[inline(always)]
    pub const fn fifo215(&self) -> &FIFO215 {
        &self.fifo215
    }
    #[doc = "0x560 - FIFO Memory Aperture0 216"]
    #[inline(always)]
    pub const fn fifo216(&self) -> &FIFO216 {
        &self.fifo216
    }
    #[doc = "0x564 - FIFO Memory Aperture0 217"]
    #[inline(always)]
    pub const fn fifo217(&self) -> &FIFO217 {
        &self.fifo217
    }
    #[doc = "0x568 - FIFO Memory Aperture0 218"]
    #[inline(always)]
    pub const fn fifo218(&self) -> &FIFO218 {
        &self.fifo218
    }
    #[doc = "0x56c - FIFO Memory Aperture0 219"]
    #[inline(always)]
    pub const fn fifo219(&self) -> &FIFO219 {
        &self.fifo219
    }
    #[doc = "0x570 - FIFO Memory Aperture0 220"]
    #[inline(always)]
    pub const fn fifo220(&self) -> &FIFO220 {
        &self.fifo220
    }
    #[doc = "0x574 - FIFO Memory Aperture0 221"]
    #[inline(always)]
    pub const fn fifo221(&self) -> &FIFO221 {
        &self.fifo221
    }
    #[doc = "0x578 - FIFO Memory Aperture0 222"]
    #[inline(always)]
    pub const fn fifo222(&self) -> &FIFO222 {
        &self.fifo222
    }
    #[doc = "0x57c - FIFO Memory Aperture0 223"]
    #[inline(always)]
    pub const fn fifo223(&self) -> &FIFO223 {
        &self.fifo223
    }
    #[doc = "0x580 - FIFO Memory Aperture0 224"]
    #[inline(always)]
    pub const fn fifo224(&self) -> &FIFO224 {
        &self.fifo224
    }
    #[doc = "0x584 - FIFO Memory Aperture0 225"]
    #[inline(always)]
    pub const fn fifo225(&self) -> &FIFO225 {
        &self.fifo225
    }
    #[doc = "0x588 - FIFO Memory Aperture0 226"]
    #[inline(always)]
    pub const fn fifo226(&self) -> &FIFO226 {
        &self.fifo226
    }
    #[doc = "0x58c - FIFO Memory Aperture0 227"]
    #[inline(always)]
    pub const fn fifo227(&self) -> &FIFO227 {
        &self.fifo227
    }
    #[doc = "0x590 - FIFO Memory Aperture0 228"]
    #[inline(always)]
    pub const fn fifo228(&self) -> &FIFO228 {
        &self.fifo228
    }
    #[doc = "0x594 - FIFO Memory Aperture0 229"]
    #[inline(always)]
    pub const fn fifo229(&self) -> &FIFO229 {
        &self.fifo229
    }
    #[doc = "0x598 - FIFO Memory Aperture0 230"]
    #[inline(always)]
    pub const fn fifo230(&self) -> &FIFO230 {
        &self.fifo230
    }
    #[doc = "0x59c - FIFO Memory Aperture0 231"]
    #[inline(always)]
    pub const fn fifo231(&self) -> &FIFO231 {
        &self.fifo231
    }
    #[doc = "0x5a0 - FIFO Memory Aperture0 232"]
    #[inline(always)]
    pub const fn fifo232(&self) -> &FIFO232 {
        &self.fifo232
    }
    #[doc = "0x5a4 - FIFO Memory Aperture0 233"]
    #[inline(always)]
    pub const fn fifo233(&self) -> &FIFO233 {
        &self.fifo233
    }
    #[doc = "0x5a8 - FIFO Memory Aperture0 234"]
    #[inline(always)]
    pub const fn fifo234(&self) -> &FIFO234 {
        &self.fifo234
    }
    #[doc = "0x5ac - FIFO Memory Aperture0 235"]
    #[inline(always)]
    pub const fn fifo235(&self) -> &FIFO235 {
        &self.fifo235
    }
    #[doc = "0x5b0 - FIFO Memory Aperture0 236"]
    #[inline(always)]
    pub const fn fifo236(&self) -> &FIFO236 {
        &self.fifo236
    }
    #[doc = "0x5b4 - FIFO Memory Aperture0 237"]
    #[inline(always)]
    pub const fn fifo237(&self) -> &FIFO237 {
        &self.fifo237
    }
    #[doc = "0x5b8 - FIFO Memory Aperture0 238"]
    #[inline(always)]
    pub const fn fifo238(&self) -> &FIFO238 {
        &self.fifo238
    }
    #[doc = "0x5bc - FIFO Memory Aperture0 239"]
    #[inline(always)]
    pub const fn fifo239(&self) -> &FIFO239 {
        &self.fifo239
    }
    #[doc = "0x5c0 - FIFO Memory Aperture0 240"]
    #[inline(always)]
    pub const fn fifo240(&self) -> &FIFO240 {
        &self.fifo240
    }
    #[doc = "0x5c4 - FIFO Memory Aperture0 241"]
    #[inline(always)]
    pub const fn fifo241(&self) -> &FIFO241 {
        &self.fifo241
    }
    #[doc = "0x5c8 - FIFO Memory Aperture0 242"]
    #[inline(always)]
    pub const fn fifo242(&self) -> &FIFO242 {
        &self.fifo242
    }
    #[doc = "0x5cc - FIFO Memory Aperture0 243"]
    #[inline(always)]
    pub const fn fifo243(&self) -> &FIFO243 {
        &self.fifo243
    }
    #[doc = "0x5d0 - FIFO Memory Aperture0 244"]
    #[inline(always)]
    pub const fn fifo244(&self) -> &FIFO244 {
        &self.fifo244
    }
    #[doc = "0x5d4 - FIFO Memory Aperture0 245"]
    #[inline(always)]
    pub const fn fifo245(&self) -> &FIFO245 {
        &self.fifo245
    }
    #[doc = "0x5d8 - FIFO Memory Aperture0 246"]
    #[inline(always)]
    pub const fn fifo246(&self) -> &FIFO246 {
        &self.fifo246
    }
    #[doc = "0x5dc - FIFO Memory Aperture0 247"]
    #[inline(always)]
    pub const fn fifo247(&self) -> &FIFO247 {
        &self.fifo247
    }
    #[doc = "0x5e0 - FIFO Memory Aperture0 248"]
    #[inline(always)]
    pub const fn fifo248(&self) -> &FIFO248 {
        &self.fifo248
    }
    #[doc = "0x5e4 - FIFO Memory Aperture0 249"]
    #[inline(always)]
    pub const fn fifo249(&self) -> &FIFO249 {
        &self.fifo249
    }
    #[doc = "0x5e8 - FIFO Memory Aperture0 250"]
    #[inline(always)]
    pub const fn fifo250(&self) -> &FIFO250 {
        &self.fifo250
    }
    #[doc = "0x5ec - FIFO Memory Aperture0 251"]
    #[inline(always)]
    pub const fn fifo251(&self) -> &FIFO251 {
        &self.fifo251
    }
    #[doc = "0x5f0 - FIFO Memory Aperture0 252"]
    #[inline(always)]
    pub const fn fifo252(&self) -> &FIFO252 {
        &self.fifo252
    }
    #[doc = "0x5f4 - FIFO Memory Aperture0 253"]
    #[inline(always)]
    pub const fn fifo253(&self) -> &FIFO253 {
        &self.fifo253
    }
    #[doc = "0x5f8 - FIFO Memory Aperture0 254"]
    #[inline(always)]
    pub const fn fifo254(&self) -> &FIFO254 {
        &self.fifo254
    }
    #[doc = "0x5fc - FIFO Memory Aperture0 255"]
    #[inline(always)]
    pub const fn fifo255(&self) -> &FIFO255 {
        &self.fifo255
    }
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
#[doc = "RPR (rw) register accessor: Receive Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpr`]
module"]
pub type RPR = crate::Reg<rpr::RPR_SPEC>;
#[doc = "Receive Pointer Register"]
pub mod rpr;
#[doc = "RCR (rw) register accessor: Receive Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr`]
module"]
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
#[doc = "Receive Counter Register"]
pub mod rcr;
#[doc = "TPR (rw) register accessor: Transmit Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpr`]
module"]
pub type TPR = crate::Reg<tpr::TPR_SPEC>;
#[doc = "Transmit Pointer Register"]
pub mod tpr;
#[doc = "TCR (rw) register accessor: Transmit Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr`]
module"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Transmit Counter Register"]
pub mod tcr;
#[doc = "RNPR (rw) register accessor: Receive Next Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rnpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnpr`]
module"]
pub type RNPR = crate::Reg<rnpr::RNPR_SPEC>;
#[doc = "Receive Next Pointer Register"]
pub mod rnpr;
#[doc = "RNCR (rw) register accessor: Receive Next Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rncr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rncr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rncr`]
module"]
pub type RNCR = crate::Reg<rncr::RNCR_SPEC>;
#[doc = "Receive Next Counter Register"]
pub mod rncr;
#[doc = "TNPR (rw) register accessor: Transmit Next Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tnpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tnpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tnpr`]
module"]
pub type TNPR = crate::Reg<tnpr::TNPR_SPEC>;
#[doc = "Transmit Next Pointer Register"]
pub mod tnpr;
#[doc = "TNCR (rw) register accessor: Transmit Next Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tncr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tncr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tncr`]
module"]
pub type TNCR = crate::Reg<tncr::TNCR_SPEC>;
#[doc = "Transmit Next Counter Register"]
pub mod tncr;
#[doc = "PTCR (w) register accessor: Transfer Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptcr`]
module"]
pub type PTCR = crate::Reg<ptcr::PTCR_SPEC>;
#[doc = "Transfer Control Register"]
pub mod ptcr;
#[doc = "PTSR (r) register accessor: Transfer Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptsr`]
module"]
pub type PTSR = crate::Reg<ptsr::PTSR_SPEC>;
#[doc = "Transfer Status Register"]
pub mod ptsr;
