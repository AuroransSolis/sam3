#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    mr: Mr,
    dtor: Dtor,
    sdcr: Sdcr,
    argr: Argr,
    cmdr: Cmdr,
    blkr: Blkr,
    cstor: Cstor,
    rspr0: Rspr0,
    rspr1: Rspr1,
    rspr2: Rspr2,
    rspr3: Rspr3,
    rdr: Rdr,
    tdr: Tdr,
    _reserved14: [u8; 0x08],
    sr: Sr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    dma: Dma,
    cfg: Cfg,
    _reserved20: [u8; 0x8c],
    wpmr: Wpmr,
    wpsr: Wpsr,
    _reserved22: [u8; 0x0114],
    fifo0: Fifo0,
    fifo1: Fifo1,
    fifo2: Fifo2,
    fifo3: Fifo3,
    fifo4: Fifo4,
    fifo5: Fifo5,
    fifo6: Fifo6,
    fifo7: Fifo7,
    fifo8: Fifo8,
    fifo9: Fifo9,
    fifo10: Fifo10,
    fifo11: Fifo11,
    fifo12: Fifo12,
    fifo13: Fifo13,
    fifo14: Fifo14,
    fifo15: Fifo15,
    fifo16: Fifo16,
    fifo17: Fifo17,
    fifo18: Fifo18,
    fifo19: Fifo19,
    fifo20: Fifo20,
    fifo21: Fifo21,
    fifo22: Fifo22,
    fifo23: Fifo23,
    fifo24: Fifo24,
    fifo25: Fifo25,
    fifo26: Fifo26,
    fifo27: Fifo27,
    fifo28: Fifo28,
    fifo29: Fifo29,
    fifo30: Fifo30,
    fifo31: Fifo31,
    fifo32: Fifo32,
    fifo33: Fifo33,
    fifo34: Fifo34,
    fifo35: Fifo35,
    fifo36: Fifo36,
    fifo37: Fifo37,
    fifo38: Fifo38,
    fifo39: Fifo39,
    fifo40: Fifo40,
    fifo41: Fifo41,
    fifo42: Fifo42,
    fifo43: Fifo43,
    fifo44: Fifo44,
    fifo45: Fifo45,
    fifo46: Fifo46,
    fifo47: Fifo47,
    fifo48: Fifo48,
    fifo49: Fifo49,
    fifo50: Fifo50,
    fifo51: Fifo51,
    fifo52: Fifo52,
    fifo53: Fifo53,
    fifo54: Fifo54,
    fifo55: Fifo55,
    fifo56: Fifo56,
    fifo57: Fifo57,
    fifo58: Fifo58,
    fifo59: Fifo59,
    fifo60: Fifo60,
    fifo61: Fifo61,
    fifo62: Fifo62,
    fifo63: Fifo63,
    fifo64: Fifo64,
    fifo65: Fifo65,
    fifo66: Fifo66,
    fifo67: Fifo67,
    fifo68: Fifo68,
    fifo69: Fifo69,
    fifo70: Fifo70,
    fifo71: Fifo71,
    fifo72: Fifo72,
    fifo73: Fifo73,
    fifo74: Fifo74,
    fifo75: Fifo75,
    fifo76: Fifo76,
    fifo77: Fifo77,
    fifo78: Fifo78,
    fifo79: Fifo79,
    fifo80: Fifo80,
    fifo81: Fifo81,
    fifo82: Fifo82,
    fifo83: Fifo83,
    fifo84: Fifo84,
    fifo85: Fifo85,
    fifo86: Fifo86,
    fifo87: Fifo87,
    fifo88: Fifo88,
    fifo89: Fifo89,
    fifo90: Fifo90,
    fifo91: Fifo91,
    fifo92: Fifo92,
    fifo93: Fifo93,
    fifo94: Fifo94,
    fifo95: Fifo95,
    fifo96: Fifo96,
    fifo97: Fifo97,
    fifo98: Fifo98,
    fifo99: Fifo99,
    fifo100: Fifo100,
    fifo101: Fifo101,
    fifo102: Fifo102,
    fifo103: Fifo103,
    fifo104: Fifo104,
    fifo105: Fifo105,
    fifo106: Fifo106,
    fifo107: Fifo107,
    fifo108: Fifo108,
    fifo109: Fifo109,
    fifo110: Fifo110,
    fifo111: Fifo111,
    fifo112: Fifo112,
    fifo113: Fifo113,
    fifo114: Fifo114,
    fifo115: Fifo115,
    fifo116: Fifo116,
    fifo117: Fifo117,
    fifo118: Fifo118,
    fifo119: Fifo119,
    fifo120: Fifo120,
    fifo121: Fifo121,
    fifo122: Fifo122,
    fifo123: Fifo123,
    fifo124: Fifo124,
    fifo125: Fifo125,
    fifo126: Fifo126,
    fifo127: Fifo127,
    fifo128: Fifo128,
    fifo129: Fifo129,
    fifo130: Fifo130,
    fifo131: Fifo131,
    fifo132: Fifo132,
    fifo133: Fifo133,
    fifo134: Fifo134,
    fifo135: Fifo135,
    fifo136: Fifo136,
    fifo137: Fifo137,
    fifo138: Fifo138,
    fifo139: Fifo139,
    fifo140: Fifo140,
    fifo141: Fifo141,
    fifo142: Fifo142,
    fifo143: Fifo143,
    fifo144: Fifo144,
    fifo145: Fifo145,
    fifo146: Fifo146,
    fifo147: Fifo147,
    fifo148: Fifo148,
    fifo149: Fifo149,
    fifo150: Fifo150,
    fifo151: Fifo151,
    fifo152: Fifo152,
    fifo153: Fifo153,
    fifo154: Fifo154,
    fifo155: Fifo155,
    fifo156: Fifo156,
    fifo157: Fifo157,
    fifo158: Fifo158,
    fifo159: Fifo159,
    fifo160: Fifo160,
    fifo161: Fifo161,
    fifo162: Fifo162,
    fifo163: Fifo163,
    fifo164: Fifo164,
    fifo165: Fifo165,
    fifo166: Fifo166,
    fifo167: Fifo167,
    fifo168: Fifo168,
    fifo169: Fifo169,
    fifo170: Fifo170,
    fifo171: Fifo171,
    fifo172: Fifo172,
    fifo173: Fifo173,
    fifo174: Fifo174,
    fifo175: Fifo175,
    fifo176: Fifo176,
    fifo177: Fifo177,
    fifo178: Fifo178,
    fifo179: Fifo179,
    fifo180: Fifo180,
    fifo181: Fifo181,
    fifo182: Fifo182,
    fifo183: Fifo183,
    fifo184: Fifo184,
    fifo185: Fifo185,
    fifo186: Fifo186,
    fifo187: Fifo187,
    fifo188: Fifo188,
    fifo189: Fifo189,
    fifo190: Fifo190,
    fifo191: Fifo191,
    fifo192: Fifo192,
    fifo193: Fifo193,
    fifo194: Fifo194,
    fifo195: Fifo195,
    fifo196: Fifo196,
    fifo197: Fifo197,
    fifo198: Fifo198,
    fifo199: Fifo199,
    fifo200: Fifo200,
    fifo201: Fifo201,
    fifo202: Fifo202,
    fifo203: Fifo203,
    fifo204: Fifo204,
    fifo205: Fifo205,
    fifo206: Fifo206,
    fifo207: Fifo207,
    fifo208: Fifo208,
    fifo209: Fifo209,
    fifo210: Fifo210,
    fifo211: Fifo211,
    fifo212: Fifo212,
    fifo213: Fifo213,
    fifo214: Fifo214,
    fifo215: Fifo215,
    fifo216: Fifo216,
    fifo217: Fifo217,
    fifo218: Fifo218,
    fifo219: Fifo219,
    fifo220: Fifo220,
    fifo221: Fifo221,
    fifo222: Fifo222,
    fifo223: Fifo223,
    fifo224: Fifo224,
    fifo225: Fifo225,
    fifo226: Fifo226,
    fifo227: Fifo227,
    fifo228: Fifo228,
    fifo229: Fifo229,
    fifo230: Fifo230,
    fifo231: Fifo231,
    fifo232: Fifo232,
    fifo233: Fifo233,
    fifo234: Fifo234,
    fifo235: Fifo235,
    fifo236: Fifo236,
    fifo237: Fifo237,
    fifo238: Fifo238,
    fifo239: Fifo239,
    fifo240: Fifo240,
    fifo241: Fifo241,
    fifo242: Fifo242,
    fifo243: Fifo243,
    fifo244: Fifo244,
    fifo245: Fifo245,
    fifo246: Fifo246,
    fifo247: Fifo247,
    fifo248: Fifo248,
    fifo249: Fifo249,
    fifo250: Fifo250,
    fifo251: Fifo251,
    fifo252: Fifo252,
    fifo253: Fifo253,
    fifo254: Fifo254,
    fifo255: Fifo255,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &Mr {
        &self.mr
    }
    #[doc = "0x08 - Data Timeout Register"]
    #[inline(always)]
    pub const fn dtor(&self) -> &Dtor {
        &self.dtor
    }
    #[doc = "0x0c - SD/SDIO Card Register"]
    #[inline(always)]
    pub const fn sdcr(&self) -> &Sdcr {
        &self.sdcr
    }
    #[doc = "0x10 - Argument Register"]
    #[inline(always)]
    pub const fn argr(&self) -> &Argr {
        &self.argr
    }
    #[doc = "0x14 - Command Register"]
    #[inline(always)]
    pub const fn cmdr(&self) -> &Cmdr {
        &self.cmdr
    }
    #[doc = "0x18 - Block Register"]
    #[inline(always)]
    pub const fn blkr(&self) -> &Blkr {
        &self.blkr
    }
    #[doc = "0x1c - Completion Signal Timeout Register"]
    #[inline(always)]
    pub const fn cstor(&self) -> &Cstor {
        &self.cstor
    }
    #[doc = "0x20 - Response Register 0"]
    #[inline(always)]
    pub const fn rspr0(&self) -> &Rspr0 {
        &self.rspr0
    }
    #[doc = "0x24 - Response Register 1"]
    #[inline(always)]
    pub const fn rspr1(&self) -> &Rspr1 {
        &self.rspr1
    }
    #[doc = "0x28 - Response Register 2"]
    #[inline(always)]
    pub const fn rspr2(&self) -> &Rspr2 {
        &self.rspr2
    }
    #[doc = "0x2c - Response Register 3"]
    #[inline(always)]
    pub const fn rspr3(&self) -> &Rspr3 {
        &self.rspr3
    }
    #[doc = "0x30 - Receive Data Register"]
    #[inline(always)]
    pub const fn rdr(&self) -> &Rdr {
        &self.rdr
    }
    #[doc = "0x34 - Transmit Data Register"]
    #[inline(always)]
    pub const fn tdr(&self) -> &Tdr {
        &self.tdr
    }
    #[doc = "0x40 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x44 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x48 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x4c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x50 - DMA Configuration Register"]
    #[inline(always)]
    pub const fn dma(&self) -> &Dma {
        &self.dma
    }
    #[doc = "0x54 - Configuration Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0xe4 - Write Protection Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &Wpmr {
        &self.wpmr
    }
    #[doc = "0xe8 - Write Protection Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &Wpsr {
        &self.wpsr
    }
    #[doc = "0x200 - FIFO Memory Aperture0 0"]
    #[inline(always)]
    pub const fn fifo0(&self) -> &Fifo0 {
        &self.fifo0
    }
    #[doc = "0x204 - FIFO Memory Aperture0 1"]
    #[inline(always)]
    pub const fn fifo1(&self) -> &Fifo1 {
        &self.fifo1
    }
    #[doc = "0x208 - FIFO Memory Aperture0 2"]
    #[inline(always)]
    pub const fn fifo2(&self) -> &Fifo2 {
        &self.fifo2
    }
    #[doc = "0x20c - FIFO Memory Aperture0 3"]
    #[inline(always)]
    pub const fn fifo3(&self) -> &Fifo3 {
        &self.fifo3
    }
    #[doc = "0x210 - FIFO Memory Aperture0 4"]
    #[inline(always)]
    pub const fn fifo4(&self) -> &Fifo4 {
        &self.fifo4
    }
    #[doc = "0x214 - FIFO Memory Aperture0 5"]
    #[inline(always)]
    pub const fn fifo5(&self) -> &Fifo5 {
        &self.fifo5
    }
    #[doc = "0x218 - FIFO Memory Aperture0 6"]
    #[inline(always)]
    pub const fn fifo6(&self) -> &Fifo6 {
        &self.fifo6
    }
    #[doc = "0x21c - FIFO Memory Aperture0 7"]
    #[inline(always)]
    pub const fn fifo7(&self) -> &Fifo7 {
        &self.fifo7
    }
    #[doc = "0x220 - FIFO Memory Aperture0 8"]
    #[inline(always)]
    pub const fn fifo8(&self) -> &Fifo8 {
        &self.fifo8
    }
    #[doc = "0x224 - FIFO Memory Aperture0 9"]
    #[inline(always)]
    pub const fn fifo9(&self) -> &Fifo9 {
        &self.fifo9
    }
    #[doc = "0x228 - FIFO Memory Aperture0 10"]
    #[inline(always)]
    pub const fn fifo10(&self) -> &Fifo10 {
        &self.fifo10
    }
    #[doc = "0x22c - FIFO Memory Aperture0 11"]
    #[inline(always)]
    pub const fn fifo11(&self) -> &Fifo11 {
        &self.fifo11
    }
    #[doc = "0x230 - FIFO Memory Aperture0 12"]
    #[inline(always)]
    pub const fn fifo12(&self) -> &Fifo12 {
        &self.fifo12
    }
    #[doc = "0x234 - FIFO Memory Aperture0 13"]
    #[inline(always)]
    pub const fn fifo13(&self) -> &Fifo13 {
        &self.fifo13
    }
    #[doc = "0x238 - FIFO Memory Aperture0 14"]
    #[inline(always)]
    pub const fn fifo14(&self) -> &Fifo14 {
        &self.fifo14
    }
    #[doc = "0x23c - FIFO Memory Aperture0 15"]
    #[inline(always)]
    pub const fn fifo15(&self) -> &Fifo15 {
        &self.fifo15
    }
    #[doc = "0x240 - FIFO Memory Aperture0 16"]
    #[inline(always)]
    pub const fn fifo16(&self) -> &Fifo16 {
        &self.fifo16
    }
    #[doc = "0x244 - FIFO Memory Aperture0 17"]
    #[inline(always)]
    pub const fn fifo17(&self) -> &Fifo17 {
        &self.fifo17
    }
    #[doc = "0x248 - FIFO Memory Aperture0 18"]
    #[inline(always)]
    pub const fn fifo18(&self) -> &Fifo18 {
        &self.fifo18
    }
    #[doc = "0x24c - FIFO Memory Aperture0 19"]
    #[inline(always)]
    pub const fn fifo19(&self) -> &Fifo19 {
        &self.fifo19
    }
    #[doc = "0x250 - FIFO Memory Aperture0 20"]
    #[inline(always)]
    pub const fn fifo20(&self) -> &Fifo20 {
        &self.fifo20
    }
    #[doc = "0x254 - FIFO Memory Aperture0 21"]
    #[inline(always)]
    pub const fn fifo21(&self) -> &Fifo21 {
        &self.fifo21
    }
    #[doc = "0x258 - FIFO Memory Aperture0 22"]
    #[inline(always)]
    pub const fn fifo22(&self) -> &Fifo22 {
        &self.fifo22
    }
    #[doc = "0x25c - FIFO Memory Aperture0 23"]
    #[inline(always)]
    pub const fn fifo23(&self) -> &Fifo23 {
        &self.fifo23
    }
    #[doc = "0x260 - FIFO Memory Aperture0 24"]
    #[inline(always)]
    pub const fn fifo24(&self) -> &Fifo24 {
        &self.fifo24
    }
    #[doc = "0x264 - FIFO Memory Aperture0 25"]
    #[inline(always)]
    pub const fn fifo25(&self) -> &Fifo25 {
        &self.fifo25
    }
    #[doc = "0x268 - FIFO Memory Aperture0 26"]
    #[inline(always)]
    pub const fn fifo26(&self) -> &Fifo26 {
        &self.fifo26
    }
    #[doc = "0x26c - FIFO Memory Aperture0 27"]
    #[inline(always)]
    pub const fn fifo27(&self) -> &Fifo27 {
        &self.fifo27
    }
    #[doc = "0x270 - FIFO Memory Aperture0 28"]
    #[inline(always)]
    pub const fn fifo28(&self) -> &Fifo28 {
        &self.fifo28
    }
    #[doc = "0x274 - FIFO Memory Aperture0 29"]
    #[inline(always)]
    pub const fn fifo29(&self) -> &Fifo29 {
        &self.fifo29
    }
    #[doc = "0x278 - FIFO Memory Aperture0 30"]
    #[inline(always)]
    pub const fn fifo30(&self) -> &Fifo30 {
        &self.fifo30
    }
    #[doc = "0x27c - FIFO Memory Aperture0 31"]
    #[inline(always)]
    pub const fn fifo31(&self) -> &Fifo31 {
        &self.fifo31
    }
    #[doc = "0x280 - FIFO Memory Aperture0 32"]
    #[inline(always)]
    pub const fn fifo32(&self) -> &Fifo32 {
        &self.fifo32
    }
    #[doc = "0x284 - FIFO Memory Aperture0 33"]
    #[inline(always)]
    pub const fn fifo33(&self) -> &Fifo33 {
        &self.fifo33
    }
    #[doc = "0x288 - FIFO Memory Aperture0 34"]
    #[inline(always)]
    pub const fn fifo34(&self) -> &Fifo34 {
        &self.fifo34
    }
    #[doc = "0x28c - FIFO Memory Aperture0 35"]
    #[inline(always)]
    pub const fn fifo35(&self) -> &Fifo35 {
        &self.fifo35
    }
    #[doc = "0x290 - FIFO Memory Aperture0 36"]
    #[inline(always)]
    pub const fn fifo36(&self) -> &Fifo36 {
        &self.fifo36
    }
    #[doc = "0x294 - FIFO Memory Aperture0 37"]
    #[inline(always)]
    pub const fn fifo37(&self) -> &Fifo37 {
        &self.fifo37
    }
    #[doc = "0x298 - FIFO Memory Aperture0 38"]
    #[inline(always)]
    pub const fn fifo38(&self) -> &Fifo38 {
        &self.fifo38
    }
    #[doc = "0x29c - FIFO Memory Aperture0 39"]
    #[inline(always)]
    pub const fn fifo39(&self) -> &Fifo39 {
        &self.fifo39
    }
    #[doc = "0x2a0 - FIFO Memory Aperture0 40"]
    #[inline(always)]
    pub const fn fifo40(&self) -> &Fifo40 {
        &self.fifo40
    }
    #[doc = "0x2a4 - FIFO Memory Aperture0 41"]
    #[inline(always)]
    pub const fn fifo41(&self) -> &Fifo41 {
        &self.fifo41
    }
    #[doc = "0x2a8 - FIFO Memory Aperture0 42"]
    #[inline(always)]
    pub const fn fifo42(&self) -> &Fifo42 {
        &self.fifo42
    }
    #[doc = "0x2ac - FIFO Memory Aperture0 43"]
    #[inline(always)]
    pub const fn fifo43(&self) -> &Fifo43 {
        &self.fifo43
    }
    #[doc = "0x2b0 - FIFO Memory Aperture0 44"]
    #[inline(always)]
    pub const fn fifo44(&self) -> &Fifo44 {
        &self.fifo44
    }
    #[doc = "0x2b4 - FIFO Memory Aperture0 45"]
    #[inline(always)]
    pub const fn fifo45(&self) -> &Fifo45 {
        &self.fifo45
    }
    #[doc = "0x2b8 - FIFO Memory Aperture0 46"]
    #[inline(always)]
    pub const fn fifo46(&self) -> &Fifo46 {
        &self.fifo46
    }
    #[doc = "0x2bc - FIFO Memory Aperture0 47"]
    #[inline(always)]
    pub const fn fifo47(&self) -> &Fifo47 {
        &self.fifo47
    }
    #[doc = "0x2c0 - FIFO Memory Aperture0 48"]
    #[inline(always)]
    pub const fn fifo48(&self) -> &Fifo48 {
        &self.fifo48
    }
    #[doc = "0x2c4 - FIFO Memory Aperture0 49"]
    #[inline(always)]
    pub const fn fifo49(&self) -> &Fifo49 {
        &self.fifo49
    }
    #[doc = "0x2c8 - FIFO Memory Aperture0 50"]
    #[inline(always)]
    pub const fn fifo50(&self) -> &Fifo50 {
        &self.fifo50
    }
    #[doc = "0x2cc - FIFO Memory Aperture0 51"]
    #[inline(always)]
    pub const fn fifo51(&self) -> &Fifo51 {
        &self.fifo51
    }
    #[doc = "0x2d0 - FIFO Memory Aperture0 52"]
    #[inline(always)]
    pub const fn fifo52(&self) -> &Fifo52 {
        &self.fifo52
    }
    #[doc = "0x2d4 - FIFO Memory Aperture0 53"]
    #[inline(always)]
    pub const fn fifo53(&self) -> &Fifo53 {
        &self.fifo53
    }
    #[doc = "0x2d8 - FIFO Memory Aperture0 54"]
    #[inline(always)]
    pub const fn fifo54(&self) -> &Fifo54 {
        &self.fifo54
    }
    #[doc = "0x2dc - FIFO Memory Aperture0 55"]
    #[inline(always)]
    pub const fn fifo55(&self) -> &Fifo55 {
        &self.fifo55
    }
    #[doc = "0x2e0 - FIFO Memory Aperture0 56"]
    #[inline(always)]
    pub const fn fifo56(&self) -> &Fifo56 {
        &self.fifo56
    }
    #[doc = "0x2e4 - FIFO Memory Aperture0 57"]
    #[inline(always)]
    pub const fn fifo57(&self) -> &Fifo57 {
        &self.fifo57
    }
    #[doc = "0x2e8 - FIFO Memory Aperture0 58"]
    #[inline(always)]
    pub const fn fifo58(&self) -> &Fifo58 {
        &self.fifo58
    }
    #[doc = "0x2ec - FIFO Memory Aperture0 59"]
    #[inline(always)]
    pub const fn fifo59(&self) -> &Fifo59 {
        &self.fifo59
    }
    #[doc = "0x2f0 - FIFO Memory Aperture0 60"]
    #[inline(always)]
    pub const fn fifo60(&self) -> &Fifo60 {
        &self.fifo60
    }
    #[doc = "0x2f4 - FIFO Memory Aperture0 61"]
    #[inline(always)]
    pub const fn fifo61(&self) -> &Fifo61 {
        &self.fifo61
    }
    #[doc = "0x2f8 - FIFO Memory Aperture0 62"]
    #[inline(always)]
    pub const fn fifo62(&self) -> &Fifo62 {
        &self.fifo62
    }
    #[doc = "0x2fc - FIFO Memory Aperture0 63"]
    #[inline(always)]
    pub const fn fifo63(&self) -> &Fifo63 {
        &self.fifo63
    }
    #[doc = "0x300 - FIFO Memory Aperture0 64"]
    #[inline(always)]
    pub const fn fifo64(&self) -> &Fifo64 {
        &self.fifo64
    }
    #[doc = "0x304 - FIFO Memory Aperture0 65"]
    #[inline(always)]
    pub const fn fifo65(&self) -> &Fifo65 {
        &self.fifo65
    }
    #[doc = "0x308 - FIFO Memory Aperture0 66"]
    #[inline(always)]
    pub const fn fifo66(&self) -> &Fifo66 {
        &self.fifo66
    }
    #[doc = "0x30c - FIFO Memory Aperture0 67"]
    #[inline(always)]
    pub const fn fifo67(&self) -> &Fifo67 {
        &self.fifo67
    }
    #[doc = "0x310 - FIFO Memory Aperture0 68"]
    #[inline(always)]
    pub const fn fifo68(&self) -> &Fifo68 {
        &self.fifo68
    }
    #[doc = "0x314 - FIFO Memory Aperture0 69"]
    #[inline(always)]
    pub const fn fifo69(&self) -> &Fifo69 {
        &self.fifo69
    }
    #[doc = "0x318 - FIFO Memory Aperture0 70"]
    #[inline(always)]
    pub const fn fifo70(&self) -> &Fifo70 {
        &self.fifo70
    }
    #[doc = "0x31c - FIFO Memory Aperture0 71"]
    #[inline(always)]
    pub const fn fifo71(&self) -> &Fifo71 {
        &self.fifo71
    }
    #[doc = "0x320 - FIFO Memory Aperture0 72"]
    #[inline(always)]
    pub const fn fifo72(&self) -> &Fifo72 {
        &self.fifo72
    }
    #[doc = "0x324 - FIFO Memory Aperture0 73"]
    #[inline(always)]
    pub const fn fifo73(&self) -> &Fifo73 {
        &self.fifo73
    }
    #[doc = "0x328 - FIFO Memory Aperture0 74"]
    #[inline(always)]
    pub const fn fifo74(&self) -> &Fifo74 {
        &self.fifo74
    }
    #[doc = "0x32c - FIFO Memory Aperture0 75"]
    #[inline(always)]
    pub const fn fifo75(&self) -> &Fifo75 {
        &self.fifo75
    }
    #[doc = "0x330 - FIFO Memory Aperture0 76"]
    #[inline(always)]
    pub const fn fifo76(&self) -> &Fifo76 {
        &self.fifo76
    }
    #[doc = "0x334 - FIFO Memory Aperture0 77"]
    #[inline(always)]
    pub const fn fifo77(&self) -> &Fifo77 {
        &self.fifo77
    }
    #[doc = "0x338 - FIFO Memory Aperture0 78"]
    #[inline(always)]
    pub const fn fifo78(&self) -> &Fifo78 {
        &self.fifo78
    }
    #[doc = "0x33c - FIFO Memory Aperture0 79"]
    #[inline(always)]
    pub const fn fifo79(&self) -> &Fifo79 {
        &self.fifo79
    }
    #[doc = "0x340 - FIFO Memory Aperture0 80"]
    #[inline(always)]
    pub const fn fifo80(&self) -> &Fifo80 {
        &self.fifo80
    }
    #[doc = "0x344 - FIFO Memory Aperture0 81"]
    #[inline(always)]
    pub const fn fifo81(&self) -> &Fifo81 {
        &self.fifo81
    }
    #[doc = "0x348 - FIFO Memory Aperture0 82"]
    #[inline(always)]
    pub const fn fifo82(&self) -> &Fifo82 {
        &self.fifo82
    }
    #[doc = "0x34c - FIFO Memory Aperture0 83"]
    #[inline(always)]
    pub const fn fifo83(&self) -> &Fifo83 {
        &self.fifo83
    }
    #[doc = "0x350 - FIFO Memory Aperture0 84"]
    #[inline(always)]
    pub const fn fifo84(&self) -> &Fifo84 {
        &self.fifo84
    }
    #[doc = "0x354 - FIFO Memory Aperture0 85"]
    #[inline(always)]
    pub const fn fifo85(&self) -> &Fifo85 {
        &self.fifo85
    }
    #[doc = "0x358 - FIFO Memory Aperture0 86"]
    #[inline(always)]
    pub const fn fifo86(&self) -> &Fifo86 {
        &self.fifo86
    }
    #[doc = "0x35c - FIFO Memory Aperture0 87"]
    #[inline(always)]
    pub const fn fifo87(&self) -> &Fifo87 {
        &self.fifo87
    }
    #[doc = "0x360 - FIFO Memory Aperture0 88"]
    #[inline(always)]
    pub const fn fifo88(&self) -> &Fifo88 {
        &self.fifo88
    }
    #[doc = "0x364 - FIFO Memory Aperture0 89"]
    #[inline(always)]
    pub const fn fifo89(&self) -> &Fifo89 {
        &self.fifo89
    }
    #[doc = "0x368 - FIFO Memory Aperture0 90"]
    #[inline(always)]
    pub const fn fifo90(&self) -> &Fifo90 {
        &self.fifo90
    }
    #[doc = "0x36c - FIFO Memory Aperture0 91"]
    #[inline(always)]
    pub const fn fifo91(&self) -> &Fifo91 {
        &self.fifo91
    }
    #[doc = "0x370 - FIFO Memory Aperture0 92"]
    #[inline(always)]
    pub const fn fifo92(&self) -> &Fifo92 {
        &self.fifo92
    }
    #[doc = "0x374 - FIFO Memory Aperture0 93"]
    #[inline(always)]
    pub const fn fifo93(&self) -> &Fifo93 {
        &self.fifo93
    }
    #[doc = "0x378 - FIFO Memory Aperture0 94"]
    #[inline(always)]
    pub const fn fifo94(&self) -> &Fifo94 {
        &self.fifo94
    }
    #[doc = "0x37c - FIFO Memory Aperture0 95"]
    #[inline(always)]
    pub const fn fifo95(&self) -> &Fifo95 {
        &self.fifo95
    }
    #[doc = "0x380 - FIFO Memory Aperture0 96"]
    #[inline(always)]
    pub const fn fifo96(&self) -> &Fifo96 {
        &self.fifo96
    }
    #[doc = "0x384 - FIFO Memory Aperture0 97"]
    #[inline(always)]
    pub const fn fifo97(&self) -> &Fifo97 {
        &self.fifo97
    }
    #[doc = "0x388 - FIFO Memory Aperture0 98"]
    #[inline(always)]
    pub const fn fifo98(&self) -> &Fifo98 {
        &self.fifo98
    }
    #[doc = "0x38c - FIFO Memory Aperture0 99"]
    #[inline(always)]
    pub const fn fifo99(&self) -> &Fifo99 {
        &self.fifo99
    }
    #[doc = "0x390 - FIFO Memory Aperture0 100"]
    #[inline(always)]
    pub const fn fifo100(&self) -> &Fifo100 {
        &self.fifo100
    }
    #[doc = "0x394 - FIFO Memory Aperture0 101"]
    #[inline(always)]
    pub const fn fifo101(&self) -> &Fifo101 {
        &self.fifo101
    }
    #[doc = "0x398 - FIFO Memory Aperture0 102"]
    #[inline(always)]
    pub const fn fifo102(&self) -> &Fifo102 {
        &self.fifo102
    }
    #[doc = "0x39c - FIFO Memory Aperture0 103"]
    #[inline(always)]
    pub const fn fifo103(&self) -> &Fifo103 {
        &self.fifo103
    }
    #[doc = "0x3a0 - FIFO Memory Aperture0 104"]
    #[inline(always)]
    pub const fn fifo104(&self) -> &Fifo104 {
        &self.fifo104
    }
    #[doc = "0x3a4 - FIFO Memory Aperture0 105"]
    #[inline(always)]
    pub const fn fifo105(&self) -> &Fifo105 {
        &self.fifo105
    }
    #[doc = "0x3a8 - FIFO Memory Aperture0 106"]
    #[inline(always)]
    pub const fn fifo106(&self) -> &Fifo106 {
        &self.fifo106
    }
    #[doc = "0x3ac - FIFO Memory Aperture0 107"]
    #[inline(always)]
    pub const fn fifo107(&self) -> &Fifo107 {
        &self.fifo107
    }
    #[doc = "0x3b0 - FIFO Memory Aperture0 108"]
    #[inline(always)]
    pub const fn fifo108(&self) -> &Fifo108 {
        &self.fifo108
    }
    #[doc = "0x3b4 - FIFO Memory Aperture0 109"]
    #[inline(always)]
    pub const fn fifo109(&self) -> &Fifo109 {
        &self.fifo109
    }
    #[doc = "0x3b8 - FIFO Memory Aperture0 110"]
    #[inline(always)]
    pub const fn fifo110(&self) -> &Fifo110 {
        &self.fifo110
    }
    #[doc = "0x3bc - FIFO Memory Aperture0 111"]
    #[inline(always)]
    pub const fn fifo111(&self) -> &Fifo111 {
        &self.fifo111
    }
    #[doc = "0x3c0 - FIFO Memory Aperture0 112"]
    #[inline(always)]
    pub const fn fifo112(&self) -> &Fifo112 {
        &self.fifo112
    }
    #[doc = "0x3c4 - FIFO Memory Aperture0 113"]
    #[inline(always)]
    pub const fn fifo113(&self) -> &Fifo113 {
        &self.fifo113
    }
    #[doc = "0x3c8 - FIFO Memory Aperture0 114"]
    #[inline(always)]
    pub const fn fifo114(&self) -> &Fifo114 {
        &self.fifo114
    }
    #[doc = "0x3cc - FIFO Memory Aperture0 115"]
    #[inline(always)]
    pub const fn fifo115(&self) -> &Fifo115 {
        &self.fifo115
    }
    #[doc = "0x3d0 - FIFO Memory Aperture0 116"]
    #[inline(always)]
    pub const fn fifo116(&self) -> &Fifo116 {
        &self.fifo116
    }
    #[doc = "0x3d4 - FIFO Memory Aperture0 117"]
    #[inline(always)]
    pub const fn fifo117(&self) -> &Fifo117 {
        &self.fifo117
    }
    #[doc = "0x3d8 - FIFO Memory Aperture0 118"]
    #[inline(always)]
    pub const fn fifo118(&self) -> &Fifo118 {
        &self.fifo118
    }
    #[doc = "0x3dc - FIFO Memory Aperture0 119"]
    #[inline(always)]
    pub const fn fifo119(&self) -> &Fifo119 {
        &self.fifo119
    }
    #[doc = "0x3e0 - FIFO Memory Aperture0 120"]
    #[inline(always)]
    pub const fn fifo120(&self) -> &Fifo120 {
        &self.fifo120
    }
    #[doc = "0x3e4 - FIFO Memory Aperture0 121"]
    #[inline(always)]
    pub const fn fifo121(&self) -> &Fifo121 {
        &self.fifo121
    }
    #[doc = "0x3e8 - FIFO Memory Aperture0 122"]
    #[inline(always)]
    pub const fn fifo122(&self) -> &Fifo122 {
        &self.fifo122
    }
    #[doc = "0x3ec - FIFO Memory Aperture0 123"]
    #[inline(always)]
    pub const fn fifo123(&self) -> &Fifo123 {
        &self.fifo123
    }
    #[doc = "0x3f0 - FIFO Memory Aperture0 124"]
    #[inline(always)]
    pub const fn fifo124(&self) -> &Fifo124 {
        &self.fifo124
    }
    #[doc = "0x3f4 - FIFO Memory Aperture0 125"]
    #[inline(always)]
    pub const fn fifo125(&self) -> &Fifo125 {
        &self.fifo125
    }
    #[doc = "0x3f8 - FIFO Memory Aperture0 126"]
    #[inline(always)]
    pub const fn fifo126(&self) -> &Fifo126 {
        &self.fifo126
    }
    #[doc = "0x3fc - FIFO Memory Aperture0 127"]
    #[inline(always)]
    pub const fn fifo127(&self) -> &Fifo127 {
        &self.fifo127
    }
    #[doc = "0x400 - FIFO Memory Aperture0 128"]
    #[inline(always)]
    pub const fn fifo128(&self) -> &Fifo128 {
        &self.fifo128
    }
    #[doc = "0x404 - FIFO Memory Aperture0 129"]
    #[inline(always)]
    pub const fn fifo129(&self) -> &Fifo129 {
        &self.fifo129
    }
    #[doc = "0x408 - FIFO Memory Aperture0 130"]
    #[inline(always)]
    pub const fn fifo130(&self) -> &Fifo130 {
        &self.fifo130
    }
    #[doc = "0x40c - FIFO Memory Aperture0 131"]
    #[inline(always)]
    pub const fn fifo131(&self) -> &Fifo131 {
        &self.fifo131
    }
    #[doc = "0x410 - FIFO Memory Aperture0 132"]
    #[inline(always)]
    pub const fn fifo132(&self) -> &Fifo132 {
        &self.fifo132
    }
    #[doc = "0x414 - FIFO Memory Aperture0 133"]
    #[inline(always)]
    pub const fn fifo133(&self) -> &Fifo133 {
        &self.fifo133
    }
    #[doc = "0x418 - FIFO Memory Aperture0 134"]
    #[inline(always)]
    pub const fn fifo134(&self) -> &Fifo134 {
        &self.fifo134
    }
    #[doc = "0x41c - FIFO Memory Aperture0 135"]
    #[inline(always)]
    pub const fn fifo135(&self) -> &Fifo135 {
        &self.fifo135
    }
    #[doc = "0x420 - FIFO Memory Aperture0 136"]
    #[inline(always)]
    pub const fn fifo136(&self) -> &Fifo136 {
        &self.fifo136
    }
    #[doc = "0x424 - FIFO Memory Aperture0 137"]
    #[inline(always)]
    pub const fn fifo137(&self) -> &Fifo137 {
        &self.fifo137
    }
    #[doc = "0x428 - FIFO Memory Aperture0 138"]
    #[inline(always)]
    pub const fn fifo138(&self) -> &Fifo138 {
        &self.fifo138
    }
    #[doc = "0x42c - FIFO Memory Aperture0 139"]
    #[inline(always)]
    pub const fn fifo139(&self) -> &Fifo139 {
        &self.fifo139
    }
    #[doc = "0x430 - FIFO Memory Aperture0 140"]
    #[inline(always)]
    pub const fn fifo140(&self) -> &Fifo140 {
        &self.fifo140
    }
    #[doc = "0x434 - FIFO Memory Aperture0 141"]
    #[inline(always)]
    pub const fn fifo141(&self) -> &Fifo141 {
        &self.fifo141
    }
    #[doc = "0x438 - FIFO Memory Aperture0 142"]
    #[inline(always)]
    pub const fn fifo142(&self) -> &Fifo142 {
        &self.fifo142
    }
    #[doc = "0x43c - FIFO Memory Aperture0 143"]
    #[inline(always)]
    pub const fn fifo143(&self) -> &Fifo143 {
        &self.fifo143
    }
    #[doc = "0x440 - FIFO Memory Aperture0 144"]
    #[inline(always)]
    pub const fn fifo144(&self) -> &Fifo144 {
        &self.fifo144
    }
    #[doc = "0x444 - FIFO Memory Aperture0 145"]
    #[inline(always)]
    pub const fn fifo145(&self) -> &Fifo145 {
        &self.fifo145
    }
    #[doc = "0x448 - FIFO Memory Aperture0 146"]
    #[inline(always)]
    pub const fn fifo146(&self) -> &Fifo146 {
        &self.fifo146
    }
    #[doc = "0x44c - FIFO Memory Aperture0 147"]
    #[inline(always)]
    pub const fn fifo147(&self) -> &Fifo147 {
        &self.fifo147
    }
    #[doc = "0x450 - FIFO Memory Aperture0 148"]
    #[inline(always)]
    pub const fn fifo148(&self) -> &Fifo148 {
        &self.fifo148
    }
    #[doc = "0x454 - FIFO Memory Aperture0 149"]
    #[inline(always)]
    pub const fn fifo149(&self) -> &Fifo149 {
        &self.fifo149
    }
    #[doc = "0x458 - FIFO Memory Aperture0 150"]
    #[inline(always)]
    pub const fn fifo150(&self) -> &Fifo150 {
        &self.fifo150
    }
    #[doc = "0x45c - FIFO Memory Aperture0 151"]
    #[inline(always)]
    pub const fn fifo151(&self) -> &Fifo151 {
        &self.fifo151
    }
    #[doc = "0x460 - FIFO Memory Aperture0 152"]
    #[inline(always)]
    pub const fn fifo152(&self) -> &Fifo152 {
        &self.fifo152
    }
    #[doc = "0x464 - FIFO Memory Aperture0 153"]
    #[inline(always)]
    pub const fn fifo153(&self) -> &Fifo153 {
        &self.fifo153
    }
    #[doc = "0x468 - FIFO Memory Aperture0 154"]
    #[inline(always)]
    pub const fn fifo154(&self) -> &Fifo154 {
        &self.fifo154
    }
    #[doc = "0x46c - FIFO Memory Aperture0 155"]
    #[inline(always)]
    pub const fn fifo155(&self) -> &Fifo155 {
        &self.fifo155
    }
    #[doc = "0x470 - FIFO Memory Aperture0 156"]
    #[inline(always)]
    pub const fn fifo156(&self) -> &Fifo156 {
        &self.fifo156
    }
    #[doc = "0x474 - FIFO Memory Aperture0 157"]
    #[inline(always)]
    pub const fn fifo157(&self) -> &Fifo157 {
        &self.fifo157
    }
    #[doc = "0x478 - FIFO Memory Aperture0 158"]
    #[inline(always)]
    pub const fn fifo158(&self) -> &Fifo158 {
        &self.fifo158
    }
    #[doc = "0x47c - FIFO Memory Aperture0 159"]
    #[inline(always)]
    pub const fn fifo159(&self) -> &Fifo159 {
        &self.fifo159
    }
    #[doc = "0x480 - FIFO Memory Aperture0 160"]
    #[inline(always)]
    pub const fn fifo160(&self) -> &Fifo160 {
        &self.fifo160
    }
    #[doc = "0x484 - FIFO Memory Aperture0 161"]
    #[inline(always)]
    pub const fn fifo161(&self) -> &Fifo161 {
        &self.fifo161
    }
    #[doc = "0x488 - FIFO Memory Aperture0 162"]
    #[inline(always)]
    pub const fn fifo162(&self) -> &Fifo162 {
        &self.fifo162
    }
    #[doc = "0x48c - FIFO Memory Aperture0 163"]
    #[inline(always)]
    pub const fn fifo163(&self) -> &Fifo163 {
        &self.fifo163
    }
    #[doc = "0x490 - FIFO Memory Aperture0 164"]
    #[inline(always)]
    pub const fn fifo164(&self) -> &Fifo164 {
        &self.fifo164
    }
    #[doc = "0x494 - FIFO Memory Aperture0 165"]
    #[inline(always)]
    pub const fn fifo165(&self) -> &Fifo165 {
        &self.fifo165
    }
    #[doc = "0x498 - FIFO Memory Aperture0 166"]
    #[inline(always)]
    pub const fn fifo166(&self) -> &Fifo166 {
        &self.fifo166
    }
    #[doc = "0x49c - FIFO Memory Aperture0 167"]
    #[inline(always)]
    pub const fn fifo167(&self) -> &Fifo167 {
        &self.fifo167
    }
    #[doc = "0x4a0 - FIFO Memory Aperture0 168"]
    #[inline(always)]
    pub const fn fifo168(&self) -> &Fifo168 {
        &self.fifo168
    }
    #[doc = "0x4a4 - FIFO Memory Aperture0 169"]
    #[inline(always)]
    pub const fn fifo169(&self) -> &Fifo169 {
        &self.fifo169
    }
    #[doc = "0x4a8 - FIFO Memory Aperture0 170"]
    #[inline(always)]
    pub const fn fifo170(&self) -> &Fifo170 {
        &self.fifo170
    }
    #[doc = "0x4ac - FIFO Memory Aperture0 171"]
    #[inline(always)]
    pub const fn fifo171(&self) -> &Fifo171 {
        &self.fifo171
    }
    #[doc = "0x4b0 - FIFO Memory Aperture0 172"]
    #[inline(always)]
    pub const fn fifo172(&self) -> &Fifo172 {
        &self.fifo172
    }
    #[doc = "0x4b4 - FIFO Memory Aperture0 173"]
    #[inline(always)]
    pub const fn fifo173(&self) -> &Fifo173 {
        &self.fifo173
    }
    #[doc = "0x4b8 - FIFO Memory Aperture0 174"]
    #[inline(always)]
    pub const fn fifo174(&self) -> &Fifo174 {
        &self.fifo174
    }
    #[doc = "0x4bc - FIFO Memory Aperture0 175"]
    #[inline(always)]
    pub const fn fifo175(&self) -> &Fifo175 {
        &self.fifo175
    }
    #[doc = "0x4c0 - FIFO Memory Aperture0 176"]
    #[inline(always)]
    pub const fn fifo176(&self) -> &Fifo176 {
        &self.fifo176
    }
    #[doc = "0x4c4 - FIFO Memory Aperture0 177"]
    #[inline(always)]
    pub const fn fifo177(&self) -> &Fifo177 {
        &self.fifo177
    }
    #[doc = "0x4c8 - FIFO Memory Aperture0 178"]
    #[inline(always)]
    pub const fn fifo178(&self) -> &Fifo178 {
        &self.fifo178
    }
    #[doc = "0x4cc - FIFO Memory Aperture0 179"]
    #[inline(always)]
    pub const fn fifo179(&self) -> &Fifo179 {
        &self.fifo179
    }
    #[doc = "0x4d0 - FIFO Memory Aperture0 180"]
    #[inline(always)]
    pub const fn fifo180(&self) -> &Fifo180 {
        &self.fifo180
    }
    #[doc = "0x4d4 - FIFO Memory Aperture0 181"]
    #[inline(always)]
    pub const fn fifo181(&self) -> &Fifo181 {
        &self.fifo181
    }
    #[doc = "0x4d8 - FIFO Memory Aperture0 182"]
    #[inline(always)]
    pub const fn fifo182(&self) -> &Fifo182 {
        &self.fifo182
    }
    #[doc = "0x4dc - FIFO Memory Aperture0 183"]
    #[inline(always)]
    pub const fn fifo183(&self) -> &Fifo183 {
        &self.fifo183
    }
    #[doc = "0x4e0 - FIFO Memory Aperture0 184"]
    #[inline(always)]
    pub const fn fifo184(&self) -> &Fifo184 {
        &self.fifo184
    }
    #[doc = "0x4e4 - FIFO Memory Aperture0 185"]
    #[inline(always)]
    pub const fn fifo185(&self) -> &Fifo185 {
        &self.fifo185
    }
    #[doc = "0x4e8 - FIFO Memory Aperture0 186"]
    #[inline(always)]
    pub const fn fifo186(&self) -> &Fifo186 {
        &self.fifo186
    }
    #[doc = "0x4ec - FIFO Memory Aperture0 187"]
    #[inline(always)]
    pub const fn fifo187(&self) -> &Fifo187 {
        &self.fifo187
    }
    #[doc = "0x4f0 - FIFO Memory Aperture0 188"]
    #[inline(always)]
    pub const fn fifo188(&self) -> &Fifo188 {
        &self.fifo188
    }
    #[doc = "0x4f4 - FIFO Memory Aperture0 189"]
    #[inline(always)]
    pub const fn fifo189(&self) -> &Fifo189 {
        &self.fifo189
    }
    #[doc = "0x4f8 - FIFO Memory Aperture0 190"]
    #[inline(always)]
    pub const fn fifo190(&self) -> &Fifo190 {
        &self.fifo190
    }
    #[doc = "0x4fc - FIFO Memory Aperture0 191"]
    #[inline(always)]
    pub const fn fifo191(&self) -> &Fifo191 {
        &self.fifo191
    }
    #[doc = "0x500 - FIFO Memory Aperture0 192"]
    #[inline(always)]
    pub const fn fifo192(&self) -> &Fifo192 {
        &self.fifo192
    }
    #[doc = "0x504 - FIFO Memory Aperture0 193"]
    #[inline(always)]
    pub const fn fifo193(&self) -> &Fifo193 {
        &self.fifo193
    }
    #[doc = "0x508 - FIFO Memory Aperture0 194"]
    #[inline(always)]
    pub const fn fifo194(&self) -> &Fifo194 {
        &self.fifo194
    }
    #[doc = "0x50c - FIFO Memory Aperture0 195"]
    #[inline(always)]
    pub const fn fifo195(&self) -> &Fifo195 {
        &self.fifo195
    }
    #[doc = "0x510 - FIFO Memory Aperture0 196"]
    #[inline(always)]
    pub const fn fifo196(&self) -> &Fifo196 {
        &self.fifo196
    }
    #[doc = "0x514 - FIFO Memory Aperture0 197"]
    #[inline(always)]
    pub const fn fifo197(&self) -> &Fifo197 {
        &self.fifo197
    }
    #[doc = "0x518 - FIFO Memory Aperture0 198"]
    #[inline(always)]
    pub const fn fifo198(&self) -> &Fifo198 {
        &self.fifo198
    }
    #[doc = "0x51c - FIFO Memory Aperture0 199"]
    #[inline(always)]
    pub const fn fifo199(&self) -> &Fifo199 {
        &self.fifo199
    }
    #[doc = "0x520 - FIFO Memory Aperture0 200"]
    #[inline(always)]
    pub const fn fifo200(&self) -> &Fifo200 {
        &self.fifo200
    }
    #[doc = "0x524 - FIFO Memory Aperture0 201"]
    #[inline(always)]
    pub const fn fifo201(&self) -> &Fifo201 {
        &self.fifo201
    }
    #[doc = "0x528 - FIFO Memory Aperture0 202"]
    #[inline(always)]
    pub const fn fifo202(&self) -> &Fifo202 {
        &self.fifo202
    }
    #[doc = "0x52c - FIFO Memory Aperture0 203"]
    #[inline(always)]
    pub const fn fifo203(&self) -> &Fifo203 {
        &self.fifo203
    }
    #[doc = "0x530 - FIFO Memory Aperture0 204"]
    #[inline(always)]
    pub const fn fifo204(&self) -> &Fifo204 {
        &self.fifo204
    }
    #[doc = "0x534 - FIFO Memory Aperture0 205"]
    #[inline(always)]
    pub const fn fifo205(&self) -> &Fifo205 {
        &self.fifo205
    }
    #[doc = "0x538 - FIFO Memory Aperture0 206"]
    #[inline(always)]
    pub const fn fifo206(&self) -> &Fifo206 {
        &self.fifo206
    }
    #[doc = "0x53c - FIFO Memory Aperture0 207"]
    #[inline(always)]
    pub const fn fifo207(&self) -> &Fifo207 {
        &self.fifo207
    }
    #[doc = "0x540 - FIFO Memory Aperture0 208"]
    #[inline(always)]
    pub const fn fifo208(&self) -> &Fifo208 {
        &self.fifo208
    }
    #[doc = "0x544 - FIFO Memory Aperture0 209"]
    #[inline(always)]
    pub const fn fifo209(&self) -> &Fifo209 {
        &self.fifo209
    }
    #[doc = "0x548 - FIFO Memory Aperture0 210"]
    #[inline(always)]
    pub const fn fifo210(&self) -> &Fifo210 {
        &self.fifo210
    }
    #[doc = "0x54c - FIFO Memory Aperture0 211"]
    #[inline(always)]
    pub const fn fifo211(&self) -> &Fifo211 {
        &self.fifo211
    }
    #[doc = "0x550 - FIFO Memory Aperture0 212"]
    #[inline(always)]
    pub const fn fifo212(&self) -> &Fifo212 {
        &self.fifo212
    }
    #[doc = "0x554 - FIFO Memory Aperture0 213"]
    #[inline(always)]
    pub const fn fifo213(&self) -> &Fifo213 {
        &self.fifo213
    }
    #[doc = "0x558 - FIFO Memory Aperture0 214"]
    #[inline(always)]
    pub const fn fifo214(&self) -> &Fifo214 {
        &self.fifo214
    }
    #[doc = "0x55c - FIFO Memory Aperture0 215"]
    #[inline(always)]
    pub const fn fifo215(&self) -> &Fifo215 {
        &self.fifo215
    }
    #[doc = "0x560 - FIFO Memory Aperture0 216"]
    #[inline(always)]
    pub const fn fifo216(&self) -> &Fifo216 {
        &self.fifo216
    }
    #[doc = "0x564 - FIFO Memory Aperture0 217"]
    #[inline(always)]
    pub const fn fifo217(&self) -> &Fifo217 {
        &self.fifo217
    }
    #[doc = "0x568 - FIFO Memory Aperture0 218"]
    #[inline(always)]
    pub const fn fifo218(&self) -> &Fifo218 {
        &self.fifo218
    }
    #[doc = "0x56c - FIFO Memory Aperture0 219"]
    #[inline(always)]
    pub const fn fifo219(&self) -> &Fifo219 {
        &self.fifo219
    }
    #[doc = "0x570 - FIFO Memory Aperture0 220"]
    #[inline(always)]
    pub const fn fifo220(&self) -> &Fifo220 {
        &self.fifo220
    }
    #[doc = "0x574 - FIFO Memory Aperture0 221"]
    #[inline(always)]
    pub const fn fifo221(&self) -> &Fifo221 {
        &self.fifo221
    }
    #[doc = "0x578 - FIFO Memory Aperture0 222"]
    #[inline(always)]
    pub const fn fifo222(&self) -> &Fifo222 {
        &self.fifo222
    }
    #[doc = "0x57c - FIFO Memory Aperture0 223"]
    #[inline(always)]
    pub const fn fifo223(&self) -> &Fifo223 {
        &self.fifo223
    }
    #[doc = "0x580 - FIFO Memory Aperture0 224"]
    #[inline(always)]
    pub const fn fifo224(&self) -> &Fifo224 {
        &self.fifo224
    }
    #[doc = "0x584 - FIFO Memory Aperture0 225"]
    #[inline(always)]
    pub const fn fifo225(&self) -> &Fifo225 {
        &self.fifo225
    }
    #[doc = "0x588 - FIFO Memory Aperture0 226"]
    #[inline(always)]
    pub const fn fifo226(&self) -> &Fifo226 {
        &self.fifo226
    }
    #[doc = "0x58c - FIFO Memory Aperture0 227"]
    #[inline(always)]
    pub const fn fifo227(&self) -> &Fifo227 {
        &self.fifo227
    }
    #[doc = "0x590 - FIFO Memory Aperture0 228"]
    #[inline(always)]
    pub const fn fifo228(&self) -> &Fifo228 {
        &self.fifo228
    }
    #[doc = "0x594 - FIFO Memory Aperture0 229"]
    #[inline(always)]
    pub const fn fifo229(&self) -> &Fifo229 {
        &self.fifo229
    }
    #[doc = "0x598 - FIFO Memory Aperture0 230"]
    #[inline(always)]
    pub const fn fifo230(&self) -> &Fifo230 {
        &self.fifo230
    }
    #[doc = "0x59c - FIFO Memory Aperture0 231"]
    #[inline(always)]
    pub const fn fifo231(&self) -> &Fifo231 {
        &self.fifo231
    }
    #[doc = "0x5a0 - FIFO Memory Aperture0 232"]
    #[inline(always)]
    pub const fn fifo232(&self) -> &Fifo232 {
        &self.fifo232
    }
    #[doc = "0x5a4 - FIFO Memory Aperture0 233"]
    #[inline(always)]
    pub const fn fifo233(&self) -> &Fifo233 {
        &self.fifo233
    }
    #[doc = "0x5a8 - FIFO Memory Aperture0 234"]
    #[inline(always)]
    pub const fn fifo234(&self) -> &Fifo234 {
        &self.fifo234
    }
    #[doc = "0x5ac - FIFO Memory Aperture0 235"]
    #[inline(always)]
    pub const fn fifo235(&self) -> &Fifo235 {
        &self.fifo235
    }
    #[doc = "0x5b0 - FIFO Memory Aperture0 236"]
    #[inline(always)]
    pub const fn fifo236(&self) -> &Fifo236 {
        &self.fifo236
    }
    #[doc = "0x5b4 - FIFO Memory Aperture0 237"]
    #[inline(always)]
    pub const fn fifo237(&self) -> &Fifo237 {
        &self.fifo237
    }
    #[doc = "0x5b8 - FIFO Memory Aperture0 238"]
    #[inline(always)]
    pub const fn fifo238(&self) -> &Fifo238 {
        &self.fifo238
    }
    #[doc = "0x5bc - FIFO Memory Aperture0 239"]
    #[inline(always)]
    pub const fn fifo239(&self) -> &Fifo239 {
        &self.fifo239
    }
    #[doc = "0x5c0 - FIFO Memory Aperture0 240"]
    #[inline(always)]
    pub const fn fifo240(&self) -> &Fifo240 {
        &self.fifo240
    }
    #[doc = "0x5c4 - FIFO Memory Aperture0 241"]
    #[inline(always)]
    pub const fn fifo241(&self) -> &Fifo241 {
        &self.fifo241
    }
    #[doc = "0x5c8 - FIFO Memory Aperture0 242"]
    #[inline(always)]
    pub const fn fifo242(&self) -> &Fifo242 {
        &self.fifo242
    }
    #[doc = "0x5cc - FIFO Memory Aperture0 243"]
    #[inline(always)]
    pub const fn fifo243(&self) -> &Fifo243 {
        &self.fifo243
    }
    #[doc = "0x5d0 - FIFO Memory Aperture0 244"]
    #[inline(always)]
    pub const fn fifo244(&self) -> &Fifo244 {
        &self.fifo244
    }
    #[doc = "0x5d4 - FIFO Memory Aperture0 245"]
    #[inline(always)]
    pub const fn fifo245(&self) -> &Fifo245 {
        &self.fifo245
    }
    #[doc = "0x5d8 - FIFO Memory Aperture0 246"]
    #[inline(always)]
    pub const fn fifo246(&self) -> &Fifo246 {
        &self.fifo246
    }
    #[doc = "0x5dc - FIFO Memory Aperture0 247"]
    #[inline(always)]
    pub const fn fifo247(&self) -> &Fifo247 {
        &self.fifo247
    }
    #[doc = "0x5e0 - FIFO Memory Aperture0 248"]
    #[inline(always)]
    pub const fn fifo248(&self) -> &Fifo248 {
        &self.fifo248
    }
    #[doc = "0x5e4 - FIFO Memory Aperture0 249"]
    #[inline(always)]
    pub const fn fifo249(&self) -> &Fifo249 {
        &self.fifo249
    }
    #[doc = "0x5e8 - FIFO Memory Aperture0 250"]
    #[inline(always)]
    pub const fn fifo250(&self) -> &Fifo250 {
        &self.fifo250
    }
    #[doc = "0x5ec - FIFO Memory Aperture0 251"]
    #[inline(always)]
    pub const fn fifo251(&self) -> &Fifo251 {
        &self.fifo251
    }
    #[doc = "0x5f0 - FIFO Memory Aperture0 252"]
    #[inline(always)]
    pub const fn fifo252(&self) -> &Fifo252 {
        &self.fifo252
    }
    #[doc = "0x5f4 - FIFO Memory Aperture0 253"]
    #[inline(always)]
    pub const fn fifo253(&self) -> &Fifo253 {
        &self.fifo253
    }
    #[doc = "0x5f8 - FIFO Memory Aperture0 254"]
    #[inline(always)]
    pub const fn fifo254(&self) -> &Fifo254 {
        &self.fifo254
    }
    #[doc = "0x5fc - FIFO Memory Aperture0 255"]
    #[inline(always)]
    pub const fn fifo255(&self) -> &Fifo255 {
        &self.fifo255
    }
}
#[doc = "CR (w) register accessor: Control Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MR (rw) register accessor: Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`]
module"]
#[doc(alias = "MR")]
pub type Mr = crate::Reg<mr::MrSpec>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "DTOR (rw) register accessor: Data Timeout Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtor`]
module"]
#[doc(alias = "DTOR")]
pub type Dtor = crate::Reg<dtor::DtorSpec>;
#[doc = "Data Timeout Register"]
pub mod dtor;
#[doc = "SDCR (rw) register accessor: SD/SDIO Card Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdcr`]
module"]
#[doc(alias = "SDCR")]
pub type Sdcr = crate::Reg<sdcr::SdcrSpec>;
#[doc = "SD/SDIO Card Register"]
pub mod sdcr;
#[doc = "ARGR (rw) register accessor: Argument Register\n\nYou can [`read`](crate::Reg::read) this register and get [`argr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`argr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@argr`]
module"]
#[doc(alias = "ARGR")]
pub type Argr = crate::Reg<argr::ArgrSpec>;
#[doc = "Argument Register"]
pub mod argr;
#[doc = "CMDR (w) register accessor: Command Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdr`]
module"]
#[doc(alias = "CMDR")]
pub type Cmdr = crate::Reg<cmdr::CmdrSpec>;
#[doc = "Command Register"]
pub mod cmdr;
#[doc = "BLKR (rw) register accessor: Block Register\n\nYou can [`read`](crate::Reg::read) this register and get [`blkr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blkr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blkr`]
module"]
#[doc(alias = "BLKR")]
pub type Blkr = crate::Reg<blkr::BlkrSpec>;
#[doc = "Block Register"]
pub mod blkr;
#[doc = "CSTOR (rw) register accessor: Completion Signal Timeout Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cstor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cstor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cstor`]
module"]
#[doc(alias = "CSTOR")]
pub type Cstor = crate::Reg<cstor::CstorSpec>;
#[doc = "Completion Signal Timeout Register"]
pub mod cstor;
#[doc = "RSPR0 (r) register accessor: Response Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rspr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rspr0`]
module"]
#[doc(alias = "RSPR0")]
pub type Rspr0 = crate::Reg<rspr0::Rspr0Spec>;
#[doc = "Response Register 0"]
pub mod rspr0;
#[doc = "RSPR1 (r) register accessor: Response Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rspr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rspr1`]
module"]
#[doc(alias = "RSPR1")]
pub type Rspr1 = crate::Reg<rspr1::Rspr1Spec>;
#[doc = "Response Register 1"]
pub mod rspr1;
#[doc = "RSPR2 (r) register accessor: Response Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rspr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rspr2`]
module"]
#[doc(alias = "RSPR2")]
pub type Rspr2 = crate::Reg<rspr2::Rspr2Spec>;
#[doc = "Response Register 2"]
pub mod rspr2;
#[doc = "RSPR3 (r) register accessor: Response Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`rspr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rspr3`]
module"]
#[doc(alias = "RSPR3")]
pub type Rspr3 = crate::Reg<rspr3::Rspr3Spec>;
#[doc = "Response Register 3"]
pub mod rspr3;
#[doc = "RDR (r) register accessor: Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdr`]
module"]
#[doc(alias = "RDR")]
pub type Rdr = crate::Reg<rdr::RdrSpec>;
#[doc = "Receive Data Register"]
pub mod rdr;
#[doc = "TDR (w) register accessor: Transmit Data Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr`]
module"]
#[doc(alias = "TDR")]
pub type Tdr = crate::Reg<tdr::TdrSpec>;
#[doc = "Transmit Data Register"]
pub mod tdr;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register"]
pub mod sr;
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
#[doc = "DMA (rw) register accessor: DMA Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma`]
module"]
#[doc(alias = "DMA")]
pub type Dma = crate::Reg<dma::DmaSpec>;
#[doc = "DMA Configuration Register"]
pub mod dma;
#[doc = "CFG (rw) register accessor: Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`]
module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "WPMR (rw) register accessor: Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`]
module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`]
module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
#[doc = "FIFO0 (rw) register accessor: FIFO Memory Aperture0 0\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo0`]
module"]
#[doc(alias = "FIFO0")]
pub type Fifo0 = crate::Reg<fifo0::Fifo0Spec>;
#[doc = "FIFO Memory Aperture0 0"]
pub mod fifo0;
#[doc = "FIFO1 (rw) register accessor: FIFO Memory Aperture0 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo1::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo1`]
module"]
#[doc(alias = "FIFO1")]
pub type Fifo1 = crate::Reg<fifo1::Fifo1Spec>;
#[doc = "FIFO Memory Aperture0 1"]
pub mod fifo1;
#[doc = "FIFO2 (rw) register accessor: FIFO Memory Aperture0 2\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo2::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo2`]
module"]
#[doc(alias = "FIFO2")]
pub type Fifo2 = crate::Reg<fifo2::Fifo2Spec>;
#[doc = "FIFO Memory Aperture0 2"]
pub mod fifo2;
#[doc = "FIFO3 (rw) register accessor: FIFO Memory Aperture0 3\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo3::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo3`]
module"]
#[doc(alias = "FIFO3")]
pub type Fifo3 = crate::Reg<fifo3::Fifo3Spec>;
#[doc = "FIFO Memory Aperture0 3"]
pub mod fifo3;
#[doc = "FIFO4 (rw) register accessor: FIFO Memory Aperture0 4\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo4::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo4`]
module"]
#[doc(alias = "FIFO4")]
pub type Fifo4 = crate::Reg<fifo4::Fifo4Spec>;
#[doc = "FIFO Memory Aperture0 4"]
pub mod fifo4;
#[doc = "FIFO5 (rw) register accessor: FIFO Memory Aperture0 5\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo5::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo5`]
module"]
#[doc(alias = "FIFO5")]
pub type Fifo5 = crate::Reg<fifo5::Fifo5Spec>;
#[doc = "FIFO Memory Aperture0 5"]
pub mod fifo5;
#[doc = "FIFO6 (rw) register accessor: FIFO Memory Aperture0 6\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo6::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo6`]
module"]
#[doc(alias = "FIFO6")]
pub type Fifo6 = crate::Reg<fifo6::Fifo6Spec>;
#[doc = "FIFO Memory Aperture0 6"]
pub mod fifo6;
#[doc = "FIFO7 (rw) register accessor: FIFO Memory Aperture0 7\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo7::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo7`]
module"]
#[doc(alias = "FIFO7")]
pub type Fifo7 = crate::Reg<fifo7::Fifo7Spec>;
#[doc = "FIFO Memory Aperture0 7"]
pub mod fifo7;
#[doc = "FIFO8 (rw) register accessor: FIFO Memory Aperture0 8\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo8::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo8`]
module"]
#[doc(alias = "FIFO8")]
pub type Fifo8 = crate::Reg<fifo8::Fifo8Spec>;
#[doc = "FIFO Memory Aperture0 8"]
pub mod fifo8;
#[doc = "FIFO9 (rw) register accessor: FIFO Memory Aperture0 9\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo9::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo9`]
module"]
#[doc(alias = "FIFO9")]
pub type Fifo9 = crate::Reg<fifo9::Fifo9Spec>;
#[doc = "FIFO Memory Aperture0 9"]
pub mod fifo9;
#[doc = "FIFO10 (rw) register accessor: FIFO Memory Aperture0 10\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo10::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo10`]
module"]
#[doc(alias = "FIFO10")]
pub type Fifo10 = crate::Reg<fifo10::Fifo10Spec>;
#[doc = "FIFO Memory Aperture0 10"]
pub mod fifo10;
#[doc = "FIFO11 (rw) register accessor: FIFO Memory Aperture0 11\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo11::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo11`]
module"]
#[doc(alias = "FIFO11")]
pub type Fifo11 = crate::Reg<fifo11::Fifo11Spec>;
#[doc = "FIFO Memory Aperture0 11"]
pub mod fifo11;
#[doc = "FIFO12 (rw) register accessor: FIFO Memory Aperture0 12\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo12::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo12`]
module"]
#[doc(alias = "FIFO12")]
pub type Fifo12 = crate::Reg<fifo12::Fifo12Spec>;
#[doc = "FIFO Memory Aperture0 12"]
pub mod fifo12;
#[doc = "FIFO13 (rw) register accessor: FIFO Memory Aperture0 13\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo13::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo13`]
module"]
#[doc(alias = "FIFO13")]
pub type Fifo13 = crate::Reg<fifo13::Fifo13Spec>;
#[doc = "FIFO Memory Aperture0 13"]
pub mod fifo13;
#[doc = "FIFO14 (rw) register accessor: FIFO Memory Aperture0 14\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo14::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo14`]
module"]
#[doc(alias = "FIFO14")]
pub type Fifo14 = crate::Reg<fifo14::Fifo14Spec>;
#[doc = "FIFO Memory Aperture0 14"]
pub mod fifo14;
#[doc = "FIFO15 (rw) register accessor: FIFO Memory Aperture0 15\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo15::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo15`]
module"]
#[doc(alias = "FIFO15")]
pub type Fifo15 = crate::Reg<fifo15::Fifo15Spec>;
#[doc = "FIFO Memory Aperture0 15"]
pub mod fifo15;
#[doc = "FIFO16 (rw) register accessor: FIFO Memory Aperture0 16\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo16::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo16`]
module"]
#[doc(alias = "FIFO16")]
pub type Fifo16 = crate::Reg<fifo16::Fifo16Spec>;
#[doc = "FIFO Memory Aperture0 16"]
pub mod fifo16;
#[doc = "FIFO17 (rw) register accessor: FIFO Memory Aperture0 17\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo17::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo17`]
module"]
#[doc(alias = "FIFO17")]
pub type Fifo17 = crate::Reg<fifo17::Fifo17Spec>;
#[doc = "FIFO Memory Aperture0 17"]
pub mod fifo17;
#[doc = "FIFO18 (rw) register accessor: FIFO Memory Aperture0 18\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo18::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo18`]
module"]
#[doc(alias = "FIFO18")]
pub type Fifo18 = crate::Reg<fifo18::Fifo18Spec>;
#[doc = "FIFO Memory Aperture0 18"]
pub mod fifo18;
#[doc = "FIFO19 (rw) register accessor: FIFO Memory Aperture0 19\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo19::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo19`]
module"]
#[doc(alias = "FIFO19")]
pub type Fifo19 = crate::Reg<fifo19::Fifo19Spec>;
#[doc = "FIFO Memory Aperture0 19"]
pub mod fifo19;
#[doc = "FIFO20 (rw) register accessor: FIFO Memory Aperture0 20\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo20::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo20`]
module"]
#[doc(alias = "FIFO20")]
pub type Fifo20 = crate::Reg<fifo20::Fifo20Spec>;
#[doc = "FIFO Memory Aperture0 20"]
pub mod fifo20;
#[doc = "FIFO21 (rw) register accessor: FIFO Memory Aperture0 21\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo21::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo21`]
module"]
#[doc(alias = "FIFO21")]
pub type Fifo21 = crate::Reg<fifo21::Fifo21Spec>;
#[doc = "FIFO Memory Aperture0 21"]
pub mod fifo21;
#[doc = "FIFO22 (rw) register accessor: FIFO Memory Aperture0 22\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo22::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo22`]
module"]
#[doc(alias = "FIFO22")]
pub type Fifo22 = crate::Reg<fifo22::Fifo22Spec>;
#[doc = "FIFO Memory Aperture0 22"]
pub mod fifo22;
#[doc = "FIFO23 (rw) register accessor: FIFO Memory Aperture0 23\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo23::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo23`]
module"]
#[doc(alias = "FIFO23")]
pub type Fifo23 = crate::Reg<fifo23::Fifo23Spec>;
#[doc = "FIFO Memory Aperture0 23"]
pub mod fifo23;
#[doc = "FIFO24 (rw) register accessor: FIFO Memory Aperture0 24\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo24::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo24`]
module"]
#[doc(alias = "FIFO24")]
pub type Fifo24 = crate::Reg<fifo24::Fifo24Spec>;
#[doc = "FIFO Memory Aperture0 24"]
pub mod fifo24;
#[doc = "FIFO25 (rw) register accessor: FIFO Memory Aperture0 25\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo25::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo25`]
module"]
#[doc(alias = "FIFO25")]
pub type Fifo25 = crate::Reg<fifo25::Fifo25Spec>;
#[doc = "FIFO Memory Aperture0 25"]
pub mod fifo25;
#[doc = "FIFO26 (rw) register accessor: FIFO Memory Aperture0 26\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo26::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo26`]
module"]
#[doc(alias = "FIFO26")]
pub type Fifo26 = crate::Reg<fifo26::Fifo26Spec>;
#[doc = "FIFO Memory Aperture0 26"]
pub mod fifo26;
#[doc = "FIFO27 (rw) register accessor: FIFO Memory Aperture0 27\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo27::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo27`]
module"]
#[doc(alias = "FIFO27")]
pub type Fifo27 = crate::Reg<fifo27::Fifo27Spec>;
#[doc = "FIFO Memory Aperture0 27"]
pub mod fifo27;
#[doc = "FIFO28 (rw) register accessor: FIFO Memory Aperture0 28\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo28::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo28`]
module"]
#[doc(alias = "FIFO28")]
pub type Fifo28 = crate::Reg<fifo28::Fifo28Spec>;
#[doc = "FIFO Memory Aperture0 28"]
pub mod fifo28;
#[doc = "FIFO29 (rw) register accessor: FIFO Memory Aperture0 29\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo29::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo29`]
module"]
#[doc(alias = "FIFO29")]
pub type Fifo29 = crate::Reg<fifo29::Fifo29Spec>;
#[doc = "FIFO Memory Aperture0 29"]
pub mod fifo29;
#[doc = "FIFO30 (rw) register accessor: FIFO Memory Aperture0 30\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo30::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo30`]
module"]
#[doc(alias = "FIFO30")]
pub type Fifo30 = crate::Reg<fifo30::Fifo30Spec>;
#[doc = "FIFO Memory Aperture0 30"]
pub mod fifo30;
#[doc = "FIFO31 (rw) register accessor: FIFO Memory Aperture0 31\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo31::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo31`]
module"]
#[doc(alias = "FIFO31")]
pub type Fifo31 = crate::Reg<fifo31::Fifo31Spec>;
#[doc = "FIFO Memory Aperture0 31"]
pub mod fifo31;
#[doc = "FIFO32 (rw) register accessor: FIFO Memory Aperture0 32\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo32::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo32`]
module"]
#[doc(alias = "FIFO32")]
pub type Fifo32 = crate::Reg<fifo32::Fifo32Spec>;
#[doc = "FIFO Memory Aperture0 32"]
pub mod fifo32;
#[doc = "FIFO33 (rw) register accessor: FIFO Memory Aperture0 33\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo33::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo33`]
module"]
#[doc(alias = "FIFO33")]
pub type Fifo33 = crate::Reg<fifo33::Fifo33Spec>;
#[doc = "FIFO Memory Aperture0 33"]
pub mod fifo33;
#[doc = "FIFO34 (rw) register accessor: FIFO Memory Aperture0 34\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo34::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo34`]
module"]
#[doc(alias = "FIFO34")]
pub type Fifo34 = crate::Reg<fifo34::Fifo34Spec>;
#[doc = "FIFO Memory Aperture0 34"]
pub mod fifo34;
#[doc = "FIFO35 (rw) register accessor: FIFO Memory Aperture0 35\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo35::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo35`]
module"]
#[doc(alias = "FIFO35")]
pub type Fifo35 = crate::Reg<fifo35::Fifo35Spec>;
#[doc = "FIFO Memory Aperture0 35"]
pub mod fifo35;
#[doc = "FIFO36 (rw) register accessor: FIFO Memory Aperture0 36\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo36::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo36`]
module"]
#[doc(alias = "FIFO36")]
pub type Fifo36 = crate::Reg<fifo36::Fifo36Spec>;
#[doc = "FIFO Memory Aperture0 36"]
pub mod fifo36;
#[doc = "FIFO37 (rw) register accessor: FIFO Memory Aperture0 37\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo37::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo37`]
module"]
#[doc(alias = "FIFO37")]
pub type Fifo37 = crate::Reg<fifo37::Fifo37Spec>;
#[doc = "FIFO Memory Aperture0 37"]
pub mod fifo37;
#[doc = "FIFO38 (rw) register accessor: FIFO Memory Aperture0 38\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo38::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo38`]
module"]
#[doc(alias = "FIFO38")]
pub type Fifo38 = crate::Reg<fifo38::Fifo38Spec>;
#[doc = "FIFO Memory Aperture0 38"]
pub mod fifo38;
#[doc = "FIFO39 (rw) register accessor: FIFO Memory Aperture0 39\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo39::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo39`]
module"]
#[doc(alias = "FIFO39")]
pub type Fifo39 = crate::Reg<fifo39::Fifo39Spec>;
#[doc = "FIFO Memory Aperture0 39"]
pub mod fifo39;
#[doc = "FIFO40 (rw) register accessor: FIFO Memory Aperture0 40\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo40::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo40`]
module"]
#[doc(alias = "FIFO40")]
pub type Fifo40 = crate::Reg<fifo40::Fifo40Spec>;
#[doc = "FIFO Memory Aperture0 40"]
pub mod fifo40;
#[doc = "FIFO41 (rw) register accessor: FIFO Memory Aperture0 41\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo41::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo41`]
module"]
#[doc(alias = "FIFO41")]
pub type Fifo41 = crate::Reg<fifo41::Fifo41Spec>;
#[doc = "FIFO Memory Aperture0 41"]
pub mod fifo41;
#[doc = "FIFO42 (rw) register accessor: FIFO Memory Aperture0 42\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo42::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo42`]
module"]
#[doc(alias = "FIFO42")]
pub type Fifo42 = crate::Reg<fifo42::Fifo42Spec>;
#[doc = "FIFO Memory Aperture0 42"]
pub mod fifo42;
#[doc = "FIFO43 (rw) register accessor: FIFO Memory Aperture0 43\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo43::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo43`]
module"]
#[doc(alias = "FIFO43")]
pub type Fifo43 = crate::Reg<fifo43::Fifo43Spec>;
#[doc = "FIFO Memory Aperture0 43"]
pub mod fifo43;
#[doc = "FIFO44 (rw) register accessor: FIFO Memory Aperture0 44\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo44::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo44`]
module"]
#[doc(alias = "FIFO44")]
pub type Fifo44 = crate::Reg<fifo44::Fifo44Spec>;
#[doc = "FIFO Memory Aperture0 44"]
pub mod fifo44;
#[doc = "FIFO45 (rw) register accessor: FIFO Memory Aperture0 45\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo45::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo45`]
module"]
#[doc(alias = "FIFO45")]
pub type Fifo45 = crate::Reg<fifo45::Fifo45Spec>;
#[doc = "FIFO Memory Aperture0 45"]
pub mod fifo45;
#[doc = "FIFO46 (rw) register accessor: FIFO Memory Aperture0 46\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo46::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo46`]
module"]
#[doc(alias = "FIFO46")]
pub type Fifo46 = crate::Reg<fifo46::Fifo46Spec>;
#[doc = "FIFO Memory Aperture0 46"]
pub mod fifo46;
#[doc = "FIFO47 (rw) register accessor: FIFO Memory Aperture0 47\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo47::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo47`]
module"]
#[doc(alias = "FIFO47")]
pub type Fifo47 = crate::Reg<fifo47::Fifo47Spec>;
#[doc = "FIFO Memory Aperture0 47"]
pub mod fifo47;
#[doc = "FIFO48 (rw) register accessor: FIFO Memory Aperture0 48\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo48::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo48`]
module"]
#[doc(alias = "FIFO48")]
pub type Fifo48 = crate::Reg<fifo48::Fifo48Spec>;
#[doc = "FIFO Memory Aperture0 48"]
pub mod fifo48;
#[doc = "FIFO49 (rw) register accessor: FIFO Memory Aperture0 49\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo49::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo49`]
module"]
#[doc(alias = "FIFO49")]
pub type Fifo49 = crate::Reg<fifo49::Fifo49Spec>;
#[doc = "FIFO Memory Aperture0 49"]
pub mod fifo49;
#[doc = "FIFO50 (rw) register accessor: FIFO Memory Aperture0 50\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo50::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo50`]
module"]
#[doc(alias = "FIFO50")]
pub type Fifo50 = crate::Reg<fifo50::Fifo50Spec>;
#[doc = "FIFO Memory Aperture0 50"]
pub mod fifo50;
#[doc = "FIFO51 (rw) register accessor: FIFO Memory Aperture0 51\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo51::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo51`]
module"]
#[doc(alias = "FIFO51")]
pub type Fifo51 = crate::Reg<fifo51::Fifo51Spec>;
#[doc = "FIFO Memory Aperture0 51"]
pub mod fifo51;
#[doc = "FIFO52 (rw) register accessor: FIFO Memory Aperture0 52\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo52::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo52`]
module"]
#[doc(alias = "FIFO52")]
pub type Fifo52 = crate::Reg<fifo52::Fifo52Spec>;
#[doc = "FIFO Memory Aperture0 52"]
pub mod fifo52;
#[doc = "FIFO53 (rw) register accessor: FIFO Memory Aperture0 53\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo53::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo53`]
module"]
#[doc(alias = "FIFO53")]
pub type Fifo53 = crate::Reg<fifo53::Fifo53Spec>;
#[doc = "FIFO Memory Aperture0 53"]
pub mod fifo53;
#[doc = "FIFO54 (rw) register accessor: FIFO Memory Aperture0 54\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo54::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo54`]
module"]
#[doc(alias = "FIFO54")]
pub type Fifo54 = crate::Reg<fifo54::Fifo54Spec>;
#[doc = "FIFO Memory Aperture0 54"]
pub mod fifo54;
#[doc = "FIFO55 (rw) register accessor: FIFO Memory Aperture0 55\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo55::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo55`]
module"]
#[doc(alias = "FIFO55")]
pub type Fifo55 = crate::Reg<fifo55::Fifo55Spec>;
#[doc = "FIFO Memory Aperture0 55"]
pub mod fifo55;
#[doc = "FIFO56 (rw) register accessor: FIFO Memory Aperture0 56\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo56::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo56`]
module"]
#[doc(alias = "FIFO56")]
pub type Fifo56 = crate::Reg<fifo56::Fifo56Spec>;
#[doc = "FIFO Memory Aperture0 56"]
pub mod fifo56;
#[doc = "FIFO57 (rw) register accessor: FIFO Memory Aperture0 57\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo57::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo57`]
module"]
#[doc(alias = "FIFO57")]
pub type Fifo57 = crate::Reg<fifo57::Fifo57Spec>;
#[doc = "FIFO Memory Aperture0 57"]
pub mod fifo57;
#[doc = "FIFO58 (rw) register accessor: FIFO Memory Aperture0 58\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo58::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo58`]
module"]
#[doc(alias = "FIFO58")]
pub type Fifo58 = crate::Reg<fifo58::Fifo58Spec>;
#[doc = "FIFO Memory Aperture0 58"]
pub mod fifo58;
#[doc = "FIFO59 (rw) register accessor: FIFO Memory Aperture0 59\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo59::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo59`]
module"]
#[doc(alias = "FIFO59")]
pub type Fifo59 = crate::Reg<fifo59::Fifo59Spec>;
#[doc = "FIFO Memory Aperture0 59"]
pub mod fifo59;
#[doc = "FIFO60 (rw) register accessor: FIFO Memory Aperture0 60\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo60::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo60`]
module"]
#[doc(alias = "FIFO60")]
pub type Fifo60 = crate::Reg<fifo60::Fifo60Spec>;
#[doc = "FIFO Memory Aperture0 60"]
pub mod fifo60;
#[doc = "FIFO61 (rw) register accessor: FIFO Memory Aperture0 61\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo61::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo61`]
module"]
#[doc(alias = "FIFO61")]
pub type Fifo61 = crate::Reg<fifo61::Fifo61Spec>;
#[doc = "FIFO Memory Aperture0 61"]
pub mod fifo61;
#[doc = "FIFO62 (rw) register accessor: FIFO Memory Aperture0 62\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo62::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo62`]
module"]
#[doc(alias = "FIFO62")]
pub type Fifo62 = crate::Reg<fifo62::Fifo62Spec>;
#[doc = "FIFO Memory Aperture0 62"]
pub mod fifo62;
#[doc = "FIFO63 (rw) register accessor: FIFO Memory Aperture0 63\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo63::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo63`]
module"]
#[doc(alias = "FIFO63")]
pub type Fifo63 = crate::Reg<fifo63::Fifo63Spec>;
#[doc = "FIFO Memory Aperture0 63"]
pub mod fifo63;
#[doc = "FIFO64 (rw) register accessor: FIFO Memory Aperture0 64\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo64::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo64`]
module"]
#[doc(alias = "FIFO64")]
pub type Fifo64 = crate::Reg<fifo64::Fifo64Spec>;
#[doc = "FIFO Memory Aperture0 64"]
pub mod fifo64;
#[doc = "FIFO65 (rw) register accessor: FIFO Memory Aperture0 65\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo65::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo65::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo65`]
module"]
#[doc(alias = "FIFO65")]
pub type Fifo65 = crate::Reg<fifo65::Fifo65Spec>;
#[doc = "FIFO Memory Aperture0 65"]
pub mod fifo65;
#[doc = "FIFO66 (rw) register accessor: FIFO Memory Aperture0 66\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo66::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo66::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo66`]
module"]
#[doc(alias = "FIFO66")]
pub type Fifo66 = crate::Reg<fifo66::Fifo66Spec>;
#[doc = "FIFO Memory Aperture0 66"]
pub mod fifo66;
#[doc = "FIFO67 (rw) register accessor: FIFO Memory Aperture0 67\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo67::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo67`]
module"]
#[doc(alias = "FIFO67")]
pub type Fifo67 = crate::Reg<fifo67::Fifo67Spec>;
#[doc = "FIFO Memory Aperture0 67"]
pub mod fifo67;
#[doc = "FIFO68 (rw) register accessor: FIFO Memory Aperture0 68\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo68::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo68`]
module"]
#[doc(alias = "FIFO68")]
pub type Fifo68 = crate::Reg<fifo68::Fifo68Spec>;
#[doc = "FIFO Memory Aperture0 68"]
pub mod fifo68;
#[doc = "FIFO69 (rw) register accessor: FIFO Memory Aperture0 69\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo69::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo69::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo69`]
module"]
#[doc(alias = "FIFO69")]
pub type Fifo69 = crate::Reg<fifo69::Fifo69Spec>;
#[doc = "FIFO Memory Aperture0 69"]
pub mod fifo69;
#[doc = "FIFO70 (rw) register accessor: FIFO Memory Aperture0 70\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo70::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo70`]
module"]
#[doc(alias = "FIFO70")]
pub type Fifo70 = crate::Reg<fifo70::Fifo70Spec>;
#[doc = "FIFO Memory Aperture0 70"]
pub mod fifo70;
#[doc = "FIFO71 (rw) register accessor: FIFO Memory Aperture0 71\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo71::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo71::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo71`]
module"]
#[doc(alias = "FIFO71")]
pub type Fifo71 = crate::Reg<fifo71::Fifo71Spec>;
#[doc = "FIFO Memory Aperture0 71"]
pub mod fifo71;
#[doc = "FIFO72 (rw) register accessor: FIFO Memory Aperture0 72\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo72::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo72::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo72`]
module"]
#[doc(alias = "FIFO72")]
pub type Fifo72 = crate::Reg<fifo72::Fifo72Spec>;
#[doc = "FIFO Memory Aperture0 72"]
pub mod fifo72;
#[doc = "FIFO73 (rw) register accessor: FIFO Memory Aperture0 73\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo73::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo73::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo73`]
module"]
#[doc(alias = "FIFO73")]
pub type Fifo73 = crate::Reg<fifo73::Fifo73Spec>;
#[doc = "FIFO Memory Aperture0 73"]
pub mod fifo73;
#[doc = "FIFO74 (rw) register accessor: FIFO Memory Aperture0 74\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo74::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo74`]
module"]
#[doc(alias = "FIFO74")]
pub type Fifo74 = crate::Reg<fifo74::Fifo74Spec>;
#[doc = "FIFO Memory Aperture0 74"]
pub mod fifo74;
#[doc = "FIFO75 (rw) register accessor: FIFO Memory Aperture0 75\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo75::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo75::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo75`]
module"]
#[doc(alias = "FIFO75")]
pub type Fifo75 = crate::Reg<fifo75::Fifo75Spec>;
#[doc = "FIFO Memory Aperture0 75"]
pub mod fifo75;
#[doc = "FIFO76 (rw) register accessor: FIFO Memory Aperture0 76\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo76::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo76::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo76`]
module"]
#[doc(alias = "FIFO76")]
pub type Fifo76 = crate::Reg<fifo76::Fifo76Spec>;
#[doc = "FIFO Memory Aperture0 76"]
pub mod fifo76;
#[doc = "FIFO77 (rw) register accessor: FIFO Memory Aperture0 77\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo77::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo77::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo77`]
module"]
#[doc(alias = "FIFO77")]
pub type Fifo77 = crate::Reg<fifo77::Fifo77Spec>;
#[doc = "FIFO Memory Aperture0 77"]
pub mod fifo77;
#[doc = "FIFO78 (rw) register accessor: FIFO Memory Aperture0 78\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo78::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo78`]
module"]
#[doc(alias = "FIFO78")]
pub type Fifo78 = crate::Reg<fifo78::Fifo78Spec>;
#[doc = "FIFO Memory Aperture0 78"]
pub mod fifo78;
#[doc = "FIFO79 (rw) register accessor: FIFO Memory Aperture0 79\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo79::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo79::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo79`]
module"]
#[doc(alias = "FIFO79")]
pub type Fifo79 = crate::Reg<fifo79::Fifo79Spec>;
#[doc = "FIFO Memory Aperture0 79"]
pub mod fifo79;
#[doc = "FIFO80 (rw) register accessor: FIFO Memory Aperture0 80\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo80::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo80`]
module"]
#[doc(alias = "FIFO80")]
pub type Fifo80 = crate::Reg<fifo80::Fifo80Spec>;
#[doc = "FIFO Memory Aperture0 80"]
pub mod fifo80;
#[doc = "FIFO81 (rw) register accessor: FIFO Memory Aperture0 81\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo81::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo81::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo81`]
module"]
#[doc(alias = "FIFO81")]
pub type Fifo81 = crate::Reg<fifo81::Fifo81Spec>;
#[doc = "FIFO Memory Aperture0 81"]
pub mod fifo81;
#[doc = "FIFO82 (rw) register accessor: FIFO Memory Aperture0 82\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo82::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo82::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo82`]
module"]
#[doc(alias = "FIFO82")]
pub type Fifo82 = crate::Reg<fifo82::Fifo82Spec>;
#[doc = "FIFO Memory Aperture0 82"]
pub mod fifo82;
#[doc = "FIFO83 (rw) register accessor: FIFO Memory Aperture0 83\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo83::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo83::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo83`]
module"]
#[doc(alias = "FIFO83")]
pub type Fifo83 = crate::Reg<fifo83::Fifo83Spec>;
#[doc = "FIFO Memory Aperture0 83"]
pub mod fifo83;
#[doc = "FIFO84 (rw) register accessor: FIFO Memory Aperture0 84\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo84::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo84::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo84`]
module"]
#[doc(alias = "FIFO84")]
pub type Fifo84 = crate::Reg<fifo84::Fifo84Spec>;
#[doc = "FIFO Memory Aperture0 84"]
pub mod fifo84;
#[doc = "FIFO85 (rw) register accessor: FIFO Memory Aperture0 85\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo85::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo85::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo85`]
module"]
#[doc(alias = "FIFO85")]
pub type Fifo85 = crate::Reg<fifo85::Fifo85Spec>;
#[doc = "FIFO Memory Aperture0 85"]
pub mod fifo85;
#[doc = "FIFO86 (rw) register accessor: FIFO Memory Aperture0 86\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo86::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo86::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo86`]
module"]
#[doc(alias = "FIFO86")]
pub type Fifo86 = crate::Reg<fifo86::Fifo86Spec>;
#[doc = "FIFO Memory Aperture0 86"]
pub mod fifo86;
#[doc = "FIFO87 (rw) register accessor: FIFO Memory Aperture0 87\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo87::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo87::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo87`]
module"]
#[doc(alias = "FIFO87")]
pub type Fifo87 = crate::Reg<fifo87::Fifo87Spec>;
#[doc = "FIFO Memory Aperture0 87"]
pub mod fifo87;
#[doc = "FIFO88 (rw) register accessor: FIFO Memory Aperture0 88\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo88::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo88`]
module"]
#[doc(alias = "FIFO88")]
pub type Fifo88 = crate::Reg<fifo88::Fifo88Spec>;
#[doc = "FIFO Memory Aperture0 88"]
pub mod fifo88;
#[doc = "FIFO89 (rw) register accessor: FIFO Memory Aperture0 89\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo89::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo89::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo89`]
module"]
#[doc(alias = "FIFO89")]
pub type Fifo89 = crate::Reg<fifo89::Fifo89Spec>;
#[doc = "FIFO Memory Aperture0 89"]
pub mod fifo89;
#[doc = "FIFO90 (rw) register accessor: FIFO Memory Aperture0 90\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo90::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo90`]
module"]
#[doc(alias = "FIFO90")]
pub type Fifo90 = crate::Reg<fifo90::Fifo90Spec>;
#[doc = "FIFO Memory Aperture0 90"]
pub mod fifo90;
#[doc = "FIFO91 (rw) register accessor: FIFO Memory Aperture0 91\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo91::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo91::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo91`]
module"]
#[doc(alias = "FIFO91")]
pub type Fifo91 = crate::Reg<fifo91::Fifo91Spec>;
#[doc = "FIFO Memory Aperture0 91"]
pub mod fifo91;
#[doc = "FIFO92 (rw) register accessor: FIFO Memory Aperture0 92\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo92::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo92::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo92`]
module"]
#[doc(alias = "FIFO92")]
pub type Fifo92 = crate::Reg<fifo92::Fifo92Spec>;
#[doc = "FIFO Memory Aperture0 92"]
pub mod fifo92;
#[doc = "FIFO93 (rw) register accessor: FIFO Memory Aperture0 93\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo93::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo93::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo93`]
module"]
#[doc(alias = "FIFO93")]
pub type Fifo93 = crate::Reg<fifo93::Fifo93Spec>;
#[doc = "FIFO Memory Aperture0 93"]
pub mod fifo93;
#[doc = "FIFO94 (rw) register accessor: FIFO Memory Aperture0 94\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo94::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo94::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo94`]
module"]
#[doc(alias = "FIFO94")]
pub type Fifo94 = crate::Reg<fifo94::Fifo94Spec>;
#[doc = "FIFO Memory Aperture0 94"]
pub mod fifo94;
#[doc = "FIFO95 (rw) register accessor: FIFO Memory Aperture0 95\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo95::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo95::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo95`]
module"]
#[doc(alias = "FIFO95")]
pub type Fifo95 = crate::Reg<fifo95::Fifo95Spec>;
#[doc = "FIFO Memory Aperture0 95"]
pub mod fifo95;
#[doc = "FIFO96 (rw) register accessor: FIFO Memory Aperture0 96\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo96::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo96::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo96`]
module"]
#[doc(alias = "FIFO96")]
pub type Fifo96 = crate::Reg<fifo96::Fifo96Spec>;
#[doc = "FIFO Memory Aperture0 96"]
pub mod fifo96;
#[doc = "FIFO97 (rw) register accessor: FIFO Memory Aperture0 97\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo97::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo97::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo97`]
module"]
#[doc(alias = "FIFO97")]
pub type Fifo97 = crate::Reg<fifo97::Fifo97Spec>;
#[doc = "FIFO Memory Aperture0 97"]
pub mod fifo97;
#[doc = "FIFO98 (rw) register accessor: FIFO Memory Aperture0 98\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo98::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo98`]
module"]
#[doc(alias = "FIFO98")]
pub type Fifo98 = crate::Reg<fifo98::Fifo98Spec>;
#[doc = "FIFO Memory Aperture0 98"]
pub mod fifo98;
#[doc = "FIFO99 (rw) register accessor: FIFO Memory Aperture0 99\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo99::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo99::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo99`]
module"]
#[doc(alias = "FIFO99")]
pub type Fifo99 = crate::Reg<fifo99::Fifo99Spec>;
#[doc = "FIFO Memory Aperture0 99"]
pub mod fifo99;
#[doc = "FIFO100 (rw) register accessor: FIFO Memory Aperture0 100\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo100::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo100`]
module"]
#[doc(alias = "FIFO100")]
pub type Fifo100 = crate::Reg<fifo100::Fifo100Spec>;
#[doc = "FIFO Memory Aperture0 100"]
pub mod fifo100;
#[doc = "FIFO101 (rw) register accessor: FIFO Memory Aperture0 101\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo101::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo101::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo101`]
module"]
#[doc(alias = "FIFO101")]
pub type Fifo101 = crate::Reg<fifo101::Fifo101Spec>;
#[doc = "FIFO Memory Aperture0 101"]
pub mod fifo101;
#[doc = "FIFO102 (rw) register accessor: FIFO Memory Aperture0 102\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo102::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo102::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo102`]
module"]
#[doc(alias = "FIFO102")]
pub type Fifo102 = crate::Reg<fifo102::Fifo102Spec>;
#[doc = "FIFO Memory Aperture0 102"]
pub mod fifo102;
#[doc = "FIFO103 (rw) register accessor: FIFO Memory Aperture0 103\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo103::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo103::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo103`]
module"]
#[doc(alias = "FIFO103")]
pub type Fifo103 = crate::Reg<fifo103::Fifo103Spec>;
#[doc = "FIFO Memory Aperture0 103"]
pub mod fifo103;
#[doc = "FIFO104 (rw) register accessor: FIFO Memory Aperture0 104\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo104::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo104`]
module"]
#[doc(alias = "FIFO104")]
pub type Fifo104 = crate::Reg<fifo104::Fifo104Spec>;
#[doc = "FIFO Memory Aperture0 104"]
pub mod fifo104;
#[doc = "FIFO105 (rw) register accessor: FIFO Memory Aperture0 105\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo105::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo105::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo105`]
module"]
#[doc(alias = "FIFO105")]
pub type Fifo105 = crate::Reg<fifo105::Fifo105Spec>;
#[doc = "FIFO Memory Aperture0 105"]
pub mod fifo105;
#[doc = "FIFO106 (rw) register accessor: FIFO Memory Aperture0 106\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo106::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo106::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo106`]
module"]
#[doc(alias = "FIFO106")]
pub type Fifo106 = crate::Reg<fifo106::Fifo106Spec>;
#[doc = "FIFO Memory Aperture0 106"]
pub mod fifo106;
#[doc = "FIFO107 (rw) register accessor: FIFO Memory Aperture0 107\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo107::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo107::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo107`]
module"]
#[doc(alias = "FIFO107")]
pub type Fifo107 = crate::Reg<fifo107::Fifo107Spec>;
#[doc = "FIFO Memory Aperture0 107"]
pub mod fifo107;
#[doc = "FIFO108 (rw) register accessor: FIFO Memory Aperture0 108\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo108::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo108`]
module"]
#[doc(alias = "FIFO108")]
pub type Fifo108 = crate::Reg<fifo108::Fifo108Spec>;
#[doc = "FIFO Memory Aperture0 108"]
pub mod fifo108;
#[doc = "FIFO109 (rw) register accessor: FIFO Memory Aperture0 109\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo109::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo109::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo109`]
module"]
#[doc(alias = "FIFO109")]
pub type Fifo109 = crate::Reg<fifo109::Fifo109Spec>;
#[doc = "FIFO Memory Aperture0 109"]
pub mod fifo109;
#[doc = "FIFO110 (rw) register accessor: FIFO Memory Aperture0 110\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo110::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo110`]
module"]
#[doc(alias = "FIFO110")]
pub type Fifo110 = crate::Reg<fifo110::Fifo110Spec>;
#[doc = "FIFO Memory Aperture0 110"]
pub mod fifo110;
#[doc = "FIFO111 (rw) register accessor: FIFO Memory Aperture0 111\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo111::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo111::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo111`]
module"]
#[doc(alias = "FIFO111")]
pub type Fifo111 = crate::Reg<fifo111::Fifo111Spec>;
#[doc = "FIFO Memory Aperture0 111"]
pub mod fifo111;
#[doc = "FIFO112 (rw) register accessor: FIFO Memory Aperture0 112\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo112::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo112::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo112`]
module"]
#[doc(alias = "FIFO112")]
pub type Fifo112 = crate::Reg<fifo112::Fifo112Spec>;
#[doc = "FIFO Memory Aperture0 112"]
pub mod fifo112;
#[doc = "FIFO113 (rw) register accessor: FIFO Memory Aperture0 113\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo113::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo113::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo113`]
module"]
#[doc(alias = "FIFO113")]
pub type Fifo113 = crate::Reg<fifo113::Fifo113Spec>;
#[doc = "FIFO Memory Aperture0 113"]
pub mod fifo113;
#[doc = "FIFO114 (rw) register accessor: FIFO Memory Aperture0 114\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo114::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo114`]
module"]
#[doc(alias = "FIFO114")]
pub type Fifo114 = crate::Reg<fifo114::Fifo114Spec>;
#[doc = "FIFO Memory Aperture0 114"]
pub mod fifo114;
#[doc = "FIFO115 (rw) register accessor: FIFO Memory Aperture0 115\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo115::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo115::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo115`]
module"]
#[doc(alias = "FIFO115")]
pub type Fifo115 = crate::Reg<fifo115::Fifo115Spec>;
#[doc = "FIFO Memory Aperture0 115"]
pub mod fifo115;
#[doc = "FIFO116 (rw) register accessor: FIFO Memory Aperture0 116\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo116::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo116::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo116`]
module"]
#[doc(alias = "FIFO116")]
pub type Fifo116 = crate::Reg<fifo116::Fifo116Spec>;
#[doc = "FIFO Memory Aperture0 116"]
pub mod fifo116;
#[doc = "FIFO117 (rw) register accessor: FIFO Memory Aperture0 117\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo117::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo117::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo117`]
module"]
#[doc(alias = "FIFO117")]
pub type Fifo117 = crate::Reg<fifo117::Fifo117Spec>;
#[doc = "FIFO Memory Aperture0 117"]
pub mod fifo117;
#[doc = "FIFO118 (rw) register accessor: FIFO Memory Aperture0 118\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo118::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo118`]
module"]
#[doc(alias = "FIFO118")]
pub type Fifo118 = crate::Reg<fifo118::Fifo118Spec>;
#[doc = "FIFO Memory Aperture0 118"]
pub mod fifo118;
#[doc = "FIFO119 (rw) register accessor: FIFO Memory Aperture0 119\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo119::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo119::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo119`]
module"]
#[doc(alias = "FIFO119")]
pub type Fifo119 = crate::Reg<fifo119::Fifo119Spec>;
#[doc = "FIFO Memory Aperture0 119"]
pub mod fifo119;
#[doc = "FIFO120 (rw) register accessor: FIFO Memory Aperture0 120\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo120::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo120`]
module"]
#[doc(alias = "FIFO120")]
pub type Fifo120 = crate::Reg<fifo120::Fifo120Spec>;
#[doc = "FIFO Memory Aperture0 120"]
pub mod fifo120;
#[doc = "FIFO121 (rw) register accessor: FIFO Memory Aperture0 121\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo121::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo121::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo121`]
module"]
#[doc(alias = "FIFO121")]
pub type Fifo121 = crate::Reg<fifo121::Fifo121Spec>;
#[doc = "FIFO Memory Aperture0 121"]
pub mod fifo121;
#[doc = "FIFO122 (rw) register accessor: FIFO Memory Aperture0 122\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo122::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo122::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo122`]
module"]
#[doc(alias = "FIFO122")]
pub type Fifo122 = crate::Reg<fifo122::Fifo122Spec>;
#[doc = "FIFO Memory Aperture0 122"]
pub mod fifo122;
#[doc = "FIFO123 (rw) register accessor: FIFO Memory Aperture0 123\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo123::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo123::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo123`]
module"]
#[doc(alias = "FIFO123")]
pub type Fifo123 = crate::Reg<fifo123::Fifo123Spec>;
#[doc = "FIFO Memory Aperture0 123"]
pub mod fifo123;
#[doc = "FIFO124 (rw) register accessor: FIFO Memory Aperture0 124\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo124::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo124::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo124`]
module"]
#[doc(alias = "FIFO124")]
pub type Fifo124 = crate::Reg<fifo124::Fifo124Spec>;
#[doc = "FIFO Memory Aperture0 124"]
pub mod fifo124;
#[doc = "FIFO125 (rw) register accessor: FIFO Memory Aperture0 125\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo125::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo125::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo125`]
module"]
#[doc(alias = "FIFO125")]
pub type Fifo125 = crate::Reg<fifo125::Fifo125Spec>;
#[doc = "FIFO Memory Aperture0 125"]
pub mod fifo125;
#[doc = "FIFO126 (rw) register accessor: FIFO Memory Aperture0 126\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo126::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo126::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo126`]
module"]
#[doc(alias = "FIFO126")]
pub type Fifo126 = crate::Reg<fifo126::Fifo126Spec>;
#[doc = "FIFO Memory Aperture0 126"]
pub mod fifo126;
#[doc = "FIFO127 (rw) register accessor: FIFO Memory Aperture0 127\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo127::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo127::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo127`]
module"]
#[doc(alias = "FIFO127")]
pub type Fifo127 = crate::Reg<fifo127::Fifo127Spec>;
#[doc = "FIFO Memory Aperture0 127"]
pub mod fifo127;
#[doc = "FIFO128 (rw) register accessor: FIFO Memory Aperture0 128\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo128::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo128`]
module"]
#[doc(alias = "FIFO128")]
pub type Fifo128 = crate::Reg<fifo128::Fifo128Spec>;
#[doc = "FIFO Memory Aperture0 128"]
pub mod fifo128;
#[doc = "FIFO129 (rw) register accessor: FIFO Memory Aperture0 129\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo129::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo129::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo129`]
module"]
#[doc(alias = "FIFO129")]
pub type Fifo129 = crate::Reg<fifo129::Fifo129Spec>;
#[doc = "FIFO Memory Aperture0 129"]
pub mod fifo129;
#[doc = "FIFO130 (rw) register accessor: FIFO Memory Aperture0 130\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo130::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo130::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo130`]
module"]
#[doc(alias = "FIFO130")]
pub type Fifo130 = crate::Reg<fifo130::Fifo130Spec>;
#[doc = "FIFO Memory Aperture0 130"]
pub mod fifo130;
#[doc = "FIFO131 (rw) register accessor: FIFO Memory Aperture0 131\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo131::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo131::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo131`]
module"]
#[doc(alias = "FIFO131")]
pub type Fifo131 = crate::Reg<fifo131::Fifo131Spec>;
#[doc = "FIFO Memory Aperture0 131"]
pub mod fifo131;
#[doc = "FIFO132 (rw) register accessor: FIFO Memory Aperture0 132\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo132::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo132::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo132`]
module"]
#[doc(alias = "FIFO132")]
pub type Fifo132 = crate::Reg<fifo132::Fifo132Spec>;
#[doc = "FIFO Memory Aperture0 132"]
pub mod fifo132;
#[doc = "FIFO133 (rw) register accessor: FIFO Memory Aperture0 133\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo133::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo133::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo133`]
module"]
#[doc(alias = "FIFO133")]
pub type Fifo133 = crate::Reg<fifo133::Fifo133Spec>;
#[doc = "FIFO Memory Aperture0 133"]
pub mod fifo133;
#[doc = "FIFO134 (rw) register accessor: FIFO Memory Aperture0 134\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo134::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo134::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo134`]
module"]
#[doc(alias = "FIFO134")]
pub type Fifo134 = crate::Reg<fifo134::Fifo134Spec>;
#[doc = "FIFO Memory Aperture0 134"]
pub mod fifo134;
#[doc = "FIFO135 (rw) register accessor: FIFO Memory Aperture0 135\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo135::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo135::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo135`]
module"]
#[doc(alias = "FIFO135")]
pub type Fifo135 = crate::Reg<fifo135::Fifo135Spec>;
#[doc = "FIFO Memory Aperture0 135"]
pub mod fifo135;
#[doc = "FIFO136 (rw) register accessor: FIFO Memory Aperture0 136\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo136::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo136::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo136`]
module"]
#[doc(alias = "FIFO136")]
pub type Fifo136 = crate::Reg<fifo136::Fifo136Spec>;
#[doc = "FIFO Memory Aperture0 136"]
pub mod fifo136;
#[doc = "FIFO137 (rw) register accessor: FIFO Memory Aperture0 137\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo137::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo137::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo137`]
module"]
#[doc(alias = "FIFO137")]
pub type Fifo137 = crate::Reg<fifo137::Fifo137Spec>;
#[doc = "FIFO Memory Aperture0 137"]
pub mod fifo137;
#[doc = "FIFO138 (rw) register accessor: FIFO Memory Aperture0 138\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo138::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo138::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo138`]
module"]
#[doc(alias = "FIFO138")]
pub type Fifo138 = crate::Reg<fifo138::Fifo138Spec>;
#[doc = "FIFO Memory Aperture0 138"]
pub mod fifo138;
#[doc = "FIFO139 (rw) register accessor: FIFO Memory Aperture0 139\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo139::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo139::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo139`]
module"]
#[doc(alias = "FIFO139")]
pub type Fifo139 = crate::Reg<fifo139::Fifo139Spec>;
#[doc = "FIFO Memory Aperture0 139"]
pub mod fifo139;
#[doc = "FIFO140 (rw) register accessor: FIFO Memory Aperture0 140\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo140::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo140::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo140`]
module"]
#[doc(alias = "FIFO140")]
pub type Fifo140 = crate::Reg<fifo140::Fifo140Spec>;
#[doc = "FIFO Memory Aperture0 140"]
pub mod fifo140;
#[doc = "FIFO141 (rw) register accessor: FIFO Memory Aperture0 141\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo141::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo141::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo141`]
module"]
#[doc(alias = "FIFO141")]
pub type Fifo141 = crate::Reg<fifo141::Fifo141Spec>;
#[doc = "FIFO Memory Aperture0 141"]
pub mod fifo141;
#[doc = "FIFO142 (rw) register accessor: FIFO Memory Aperture0 142\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo142::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo142::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo142`]
module"]
#[doc(alias = "FIFO142")]
pub type Fifo142 = crate::Reg<fifo142::Fifo142Spec>;
#[doc = "FIFO Memory Aperture0 142"]
pub mod fifo142;
#[doc = "FIFO143 (rw) register accessor: FIFO Memory Aperture0 143\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo143::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo143::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo143`]
module"]
#[doc(alias = "FIFO143")]
pub type Fifo143 = crate::Reg<fifo143::Fifo143Spec>;
#[doc = "FIFO Memory Aperture0 143"]
pub mod fifo143;
#[doc = "FIFO144 (rw) register accessor: FIFO Memory Aperture0 144\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo144::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo144::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo144`]
module"]
#[doc(alias = "FIFO144")]
pub type Fifo144 = crate::Reg<fifo144::Fifo144Spec>;
#[doc = "FIFO Memory Aperture0 144"]
pub mod fifo144;
#[doc = "FIFO145 (rw) register accessor: FIFO Memory Aperture0 145\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo145::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo145::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo145`]
module"]
#[doc(alias = "FIFO145")]
pub type Fifo145 = crate::Reg<fifo145::Fifo145Spec>;
#[doc = "FIFO Memory Aperture0 145"]
pub mod fifo145;
#[doc = "FIFO146 (rw) register accessor: FIFO Memory Aperture0 146\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo146::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo146::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo146`]
module"]
#[doc(alias = "FIFO146")]
pub type Fifo146 = crate::Reg<fifo146::Fifo146Spec>;
#[doc = "FIFO Memory Aperture0 146"]
pub mod fifo146;
#[doc = "FIFO147 (rw) register accessor: FIFO Memory Aperture0 147\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo147::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo147::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo147`]
module"]
#[doc(alias = "FIFO147")]
pub type Fifo147 = crate::Reg<fifo147::Fifo147Spec>;
#[doc = "FIFO Memory Aperture0 147"]
pub mod fifo147;
#[doc = "FIFO148 (rw) register accessor: FIFO Memory Aperture0 148\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo148::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo148::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo148`]
module"]
#[doc(alias = "FIFO148")]
pub type Fifo148 = crate::Reg<fifo148::Fifo148Spec>;
#[doc = "FIFO Memory Aperture0 148"]
pub mod fifo148;
#[doc = "FIFO149 (rw) register accessor: FIFO Memory Aperture0 149\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo149::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo149::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo149`]
module"]
#[doc(alias = "FIFO149")]
pub type Fifo149 = crate::Reg<fifo149::Fifo149Spec>;
#[doc = "FIFO Memory Aperture0 149"]
pub mod fifo149;
#[doc = "FIFO150 (rw) register accessor: FIFO Memory Aperture0 150\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo150::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo150::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo150`]
module"]
#[doc(alias = "FIFO150")]
pub type Fifo150 = crate::Reg<fifo150::Fifo150Spec>;
#[doc = "FIFO Memory Aperture0 150"]
pub mod fifo150;
#[doc = "FIFO151 (rw) register accessor: FIFO Memory Aperture0 151\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo151::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo151::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo151`]
module"]
#[doc(alias = "FIFO151")]
pub type Fifo151 = crate::Reg<fifo151::Fifo151Spec>;
#[doc = "FIFO Memory Aperture0 151"]
pub mod fifo151;
#[doc = "FIFO152 (rw) register accessor: FIFO Memory Aperture0 152\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo152::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo152::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo152`]
module"]
#[doc(alias = "FIFO152")]
pub type Fifo152 = crate::Reg<fifo152::Fifo152Spec>;
#[doc = "FIFO Memory Aperture0 152"]
pub mod fifo152;
#[doc = "FIFO153 (rw) register accessor: FIFO Memory Aperture0 153\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo153::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo153::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo153`]
module"]
#[doc(alias = "FIFO153")]
pub type Fifo153 = crate::Reg<fifo153::Fifo153Spec>;
#[doc = "FIFO Memory Aperture0 153"]
pub mod fifo153;
#[doc = "FIFO154 (rw) register accessor: FIFO Memory Aperture0 154\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo154::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo154::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo154`]
module"]
#[doc(alias = "FIFO154")]
pub type Fifo154 = crate::Reg<fifo154::Fifo154Spec>;
#[doc = "FIFO Memory Aperture0 154"]
pub mod fifo154;
#[doc = "FIFO155 (rw) register accessor: FIFO Memory Aperture0 155\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo155::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo155::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo155`]
module"]
#[doc(alias = "FIFO155")]
pub type Fifo155 = crate::Reg<fifo155::Fifo155Spec>;
#[doc = "FIFO Memory Aperture0 155"]
pub mod fifo155;
#[doc = "FIFO156 (rw) register accessor: FIFO Memory Aperture0 156\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo156::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo156::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo156`]
module"]
#[doc(alias = "FIFO156")]
pub type Fifo156 = crate::Reg<fifo156::Fifo156Spec>;
#[doc = "FIFO Memory Aperture0 156"]
pub mod fifo156;
#[doc = "FIFO157 (rw) register accessor: FIFO Memory Aperture0 157\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo157::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo157::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo157`]
module"]
#[doc(alias = "FIFO157")]
pub type Fifo157 = crate::Reg<fifo157::Fifo157Spec>;
#[doc = "FIFO Memory Aperture0 157"]
pub mod fifo157;
#[doc = "FIFO158 (rw) register accessor: FIFO Memory Aperture0 158\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo158::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo158::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo158`]
module"]
#[doc(alias = "FIFO158")]
pub type Fifo158 = crate::Reg<fifo158::Fifo158Spec>;
#[doc = "FIFO Memory Aperture0 158"]
pub mod fifo158;
#[doc = "FIFO159 (rw) register accessor: FIFO Memory Aperture0 159\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo159::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo159::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo159`]
module"]
#[doc(alias = "FIFO159")]
pub type Fifo159 = crate::Reg<fifo159::Fifo159Spec>;
#[doc = "FIFO Memory Aperture0 159"]
pub mod fifo159;
#[doc = "FIFO160 (rw) register accessor: FIFO Memory Aperture0 160\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo160::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo160::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo160`]
module"]
#[doc(alias = "FIFO160")]
pub type Fifo160 = crate::Reg<fifo160::Fifo160Spec>;
#[doc = "FIFO Memory Aperture0 160"]
pub mod fifo160;
#[doc = "FIFO161 (rw) register accessor: FIFO Memory Aperture0 161\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo161::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo161::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo161`]
module"]
#[doc(alias = "FIFO161")]
pub type Fifo161 = crate::Reg<fifo161::Fifo161Spec>;
#[doc = "FIFO Memory Aperture0 161"]
pub mod fifo161;
#[doc = "FIFO162 (rw) register accessor: FIFO Memory Aperture0 162\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo162::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo162::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo162`]
module"]
#[doc(alias = "FIFO162")]
pub type Fifo162 = crate::Reg<fifo162::Fifo162Spec>;
#[doc = "FIFO Memory Aperture0 162"]
pub mod fifo162;
#[doc = "FIFO163 (rw) register accessor: FIFO Memory Aperture0 163\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo163::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo163::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo163`]
module"]
#[doc(alias = "FIFO163")]
pub type Fifo163 = crate::Reg<fifo163::Fifo163Spec>;
#[doc = "FIFO Memory Aperture0 163"]
pub mod fifo163;
#[doc = "FIFO164 (rw) register accessor: FIFO Memory Aperture0 164\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo164::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo164::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo164`]
module"]
#[doc(alias = "FIFO164")]
pub type Fifo164 = crate::Reg<fifo164::Fifo164Spec>;
#[doc = "FIFO Memory Aperture0 164"]
pub mod fifo164;
#[doc = "FIFO165 (rw) register accessor: FIFO Memory Aperture0 165\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo165::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo165::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo165`]
module"]
#[doc(alias = "FIFO165")]
pub type Fifo165 = crate::Reg<fifo165::Fifo165Spec>;
#[doc = "FIFO Memory Aperture0 165"]
pub mod fifo165;
#[doc = "FIFO166 (rw) register accessor: FIFO Memory Aperture0 166\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo166::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo166::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo166`]
module"]
#[doc(alias = "FIFO166")]
pub type Fifo166 = crate::Reg<fifo166::Fifo166Spec>;
#[doc = "FIFO Memory Aperture0 166"]
pub mod fifo166;
#[doc = "FIFO167 (rw) register accessor: FIFO Memory Aperture0 167\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo167::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo167::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo167`]
module"]
#[doc(alias = "FIFO167")]
pub type Fifo167 = crate::Reg<fifo167::Fifo167Spec>;
#[doc = "FIFO Memory Aperture0 167"]
pub mod fifo167;
#[doc = "FIFO168 (rw) register accessor: FIFO Memory Aperture0 168\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo168::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo168::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo168`]
module"]
#[doc(alias = "FIFO168")]
pub type Fifo168 = crate::Reg<fifo168::Fifo168Spec>;
#[doc = "FIFO Memory Aperture0 168"]
pub mod fifo168;
#[doc = "FIFO169 (rw) register accessor: FIFO Memory Aperture0 169\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo169::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo169::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo169`]
module"]
#[doc(alias = "FIFO169")]
pub type Fifo169 = crate::Reg<fifo169::Fifo169Spec>;
#[doc = "FIFO Memory Aperture0 169"]
pub mod fifo169;
#[doc = "FIFO170 (rw) register accessor: FIFO Memory Aperture0 170\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo170::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo170::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo170`]
module"]
#[doc(alias = "FIFO170")]
pub type Fifo170 = crate::Reg<fifo170::Fifo170Spec>;
#[doc = "FIFO Memory Aperture0 170"]
pub mod fifo170;
#[doc = "FIFO171 (rw) register accessor: FIFO Memory Aperture0 171\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo171::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo171::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo171`]
module"]
#[doc(alias = "FIFO171")]
pub type Fifo171 = crate::Reg<fifo171::Fifo171Spec>;
#[doc = "FIFO Memory Aperture0 171"]
pub mod fifo171;
#[doc = "FIFO172 (rw) register accessor: FIFO Memory Aperture0 172\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo172::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo172::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo172`]
module"]
#[doc(alias = "FIFO172")]
pub type Fifo172 = crate::Reg<fifo172::Fifo172Spec>;
#[doc = "FIFO Memory Aperture0 172"]
pub mod fifo172;
#[doc = "FIFO173 (rw) register accessor: FIFO Memory Aperture0 173\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo173::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo173::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo173`]
module"]
#[doc(alias = "FIFO173")]
pub type Fifo173 = crate::Reg<fifo173::Fifo173Spec>;
#[doc = "FIFO Memory Aperture0 173"]
pub mod fifo173;
#[doc = "FIFO174 (rw) register accessor: FIFO Memory Aperture0 174\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo174::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo174::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo174`]
module"]
#[doc(alias = "FIFO174")]
pub type Fifo174 = crate::Reg<fifo174::Fifo174Spec>;
#[doc = "FIFO Memory Aperture0 174"]
pub mod fifo174;
#[doc = "FIFO175 (rw) register accessor: FIFO Memory Aperture0 175\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo175::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo175::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo175`]
module"]
#[doc(alias = "FIFO175")]
pub type Fifo175 = crate::Reg<fifo175::Fifo175Spec>;
#[doc = "FIFO Memory Aperture0 175"]
pub mod fifo175;
#[doc = "FIFO176 (rw) register accessor: FIFO Memory Aperture0 176\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo176::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo176::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo176`]
module"]
#[doc(alias = "FIFO176")]
pub type Fifo176 = crate::Reg<fifo176::Fifo176Spec>;
#[doc = "FIFO Memory Aperture0 176"]
pub mod fifo176;
#[doc = "FIFO177 (rw) register accessor: FIFO Memory Aperture0 177\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo177::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo177::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo177`]
module"]
#[doc(alias = "FIFO177")]
pub type Fifo177 = crate::Reg<fifo177::Fifo177Spec>;
#[doc = "FIFO Memory Aperture0 177"]
pub mod fifo177;
#[doc = "FIFO178 (rw) register accessor: FIFO Memory Aperture0 178\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo178::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo178::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo178`]
module"]
#[doc(alias = "FIFO178")]
pub type Fifo178 = crate::Reg<fifo178::Fifo178Spec>;
#[doc = "FIFO Memory Aperture0 178"]
pub mod fifo178;
#[doc = "FIFO179 (rw) register accessor: FIFO Memory Aperture0 179\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo179::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo179::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo179`]
module"]
#[doc(alias = "FIFO179")]
pub type Fifo179 = crate::Reg<fifo179::Fifo179Spec>;
#[doc = "FIFO Memory Aperture0 179"]
pub mod fifo179;
#[doc = "FIFO180 (rw) register accessor: FIFO Memory Aperture0 180\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo180::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo180::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo180`]
module"]
#[doc(alias = "FIFO180")]
pub type Fifo180 = crate::Reg<fifo180::Fifo180Spec>;
#[doc = "FIFO Memory Aperture0 180"]
pub mod fifo180;
#[doc = "FIFO181 (rw) register accessor: FIFO Memory Aperture0 181\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo181::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo181::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo181`]
module"]
#[doc(alias = "FIFO181")]
pub type Fifo181 = crate::Reg<fifo181::Fifo181Spec>;
#[doc = "FIFO Memory Aperture0 181"]
pub mod fifo181;
#[doc = "FIFO182 (rw) register accessor: FIFO Memory Aperture0 182\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo182::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo182::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo182`]
module"]
#[doc(alias = "FIFO182")]
pub type Fifo182 = crate::Reg<fifo182::Fifo182Spec>;
#[doc = "FIFO Memory Aperture0 182"]
pub mod fifo182;
#[doc = "FIFO183 (rw) register accessor: FIFO Memory Aperture0 183\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo183::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo183::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo183`]
module"]
#[doc(alias = "FIFO183")]
pub type Fifo183 = crate::Reg<fifo183::Fifo183Spec>;
#[doc = "FIFO Memory Aperture0 183"]
pub mod fifo183;
#[doc = "FIFO184 (rw) register accessor: FIFO Memory Aperture0 184\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo184::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo184::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo184`]
module"]
#[doc(alias = "FIFO184")]
pub type Fifo184 = crate::Reg<fifo184::Fifo184Spec>;
#[doc = "FIFO Memory Aperture0 184"]
pub mod fifo184;
#[doc = "FIFO185 (rw) register accessor: FIFO Memory Aperture0 185\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo185::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo185::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo185`]
module"]
#[doc(alias = "FIFO185")]
pub type Fifo185 = crate::Reg<fifo185::Fifo185Spec>;
#[doc = "FIFO Memory Aperture0 185"]
pub mod fifo185;
#[doc = "FIFO186 (rw) register accessor: FIFO Memory Aperture0 186\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo186::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo186::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo186`]
module"]
#[doc(alias = "FIFO186")]
pub type Fifo186 = crate::Reg<fifo186::Fifo186Spec>;
#[doc = "FIFO Memory Aperture0 186"]
pub mod fifo186;
#[doc = "FIFO187 (rw) register accessor: FIFO Memory Aperture0 187\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo187::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo187::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo187`]
module"]
#[doc(alias = "FIFO187")]
pub type Fifo187 = crate::Reg<fifo187::Fifo187Spec>;
#[doc = "FIFO Memory Aperture0 187"]
pub mod fifo187;
#[doc = "FIFO188 (rw) register accessor: FIFO Memory Aperture0 188\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo188::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo188::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo188`]
module"]
#[doc(alias = "FIFO188")]
pub type Fifo188 = crate::Reg<fifo188::Fifo188Spec>;
#[doc = "FIFO Memory Aperture0 188"]
pub mod fifo188;
#[doc = "FIFO189 (rw) register accessor: FIFO Memory Aperture0 189\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo189::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo189::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo189`]
module"]
#[doc(alias = "FIFO189")]
pub type Fifo189 = crate::Reg<fifo189::Fifo189Spec>;
#[doc = "FIFO Memory Aperture0 189"]
pub mod fifo189;
#[doc = "FIFO190 (rw) register accessor: FIFO Memory Aperture0 190\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo190::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo190::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo190`]
module"]
#[doc(alias = "FIFO190")]
pub type Fifo190 = crate::Reg<fifo190::Fifo190Spec>;
#[doc = "FIFO Memory Aperture0 190"]
pub mod fifo190;
#[doc = "FIFO191 (rw) register accessor: FIFO Memory Aperture0 191\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo191::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo191::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo191`]
module"]
#[doc(alias = "FIFO191")]
pub type Fifo191 = crate::Reg<fifo191::Fifo191Spec>;
#[doc = "FIFO Memory Aperture0 191"]
pub mod fifo191;
#[doc = "FIFO192 (rw) register accessor: FIFO Memory Aperture0 192\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo192::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo192::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo192`]
module"]
#[doc(alias = "FIFO192")]
pub type Fifo192 = crate::Reg<fifo192::Fifo192Spec>;
#[doc = "FIFO Memory Aperture0 192"]
pub mod fifo192;
#[doc = "FIFO193 (rw) register accessor: FIFO Memory Aperture0 193\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo193::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo193::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo193`]
module"]
#[doc(alias = "FIFO193")]
pub type Fifo193 = crate::Reg<fifo193::Fifo193Spec>;
#[doc = "FIFO Memory Aperture0 193"]
pub mod fifo193;
#[doc = "FIFO194 (rw) register accessor: FIFO Memory Aperture0 194\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo194::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo194::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo194`]
module"]
#[doc(alias = "FIFO194")]
pub type Fifo194 = crate::Reg<fifo194::Fifo194Spec>;
#[doc = "FIFO Memory Aperture0 194"]
pub mod fifo194;
#[doc = "FIFO195 (rw) register accessor: FIFO Memory Aperture0 195\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo195::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo195::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo195`]
module"]
#[doc(alias = "FIFO195")]
pub type Fifo195 = crate::Reg<fifo195::Fifo195Spec>;
#[doc = "FIFO Memory Aperture0 195"]
pub mod fifo195;
#[doc = "FIFO196 (rw) register accessor: FIFO Memory Aperture0 196\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo196::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo196::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo196`]
module"]
#[doc(alias = "FIFO196")]
pub type Fifo196 = crate::Reg<fifo196::Fifo196Spec>;
#[doc = "FIFO Memory Aperture0 196"]
pub mod fifo196;
#[doc = "FIFO197 (rw) register accessor: FIFO Memory Aperture0 197\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo197::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo197::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo197`]
module"]
#[doc(alias = "FIFO197")]
pub type Fifo197 = crate::Reg<fifo197::Fifo197Spec>;
#[doc = "FIFO Memory Aperture0 197"]
pub mod fifo197;
#[doc = "FIFO198 (rw) register accessor: FIFO Memory Aperture0 198\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo198::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo198::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo198`]
module"]
#[doc(alias = "FIFO198")]
pub type Fifo198 = crate::Reg<fifo198::Fifo198Spec>;
#[doc = "FIFO Memory Aperture0 198"]
pub mod fifo198;
#[doc = "FIFO199 (rw) register accessor: FIFO Memory Aperture0 199\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo199::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo199::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo199`]
module"]
#[doc(alias = "FIFO199")]
pub type Fifo199 = crate::Reg<fifo199::Fifo199Spec>;
#[doc = "FIFO Memory Aperture0 199"]
pub mod fifo199;
#[doc = "FIFO200 (rw) register accessor: FIFO Memory Aperture0 200\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo200::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo200::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo200`]
module"]
#[doc(alias = "FIFO200")]
pub type Fifo200 = crate::Reg<fifo200::Fifo200Spec>;
#[doc = "FIFO Memory Aperture0 200"]
pub mod fifo200;
#[doc = "FIFO201 (rw) register accessor: FIFO Memory Aperture0 201\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo201::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo201::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo201`]
module"]
#[doc(alias = "FIFO201")]
pub type Fifo201 = crate::Reg<fifo201::Fifo201Spec>;
#[doc = "FIFO Memory Aperture0 201"]
pub mod fifo201;
#[doc = "FIFO202 (rw) register accessor: FIFO Memory Aperture0 202\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo202::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo202::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo202`]
module"]
#[doc(alias = "FIFO202")]
pub type Fifo202 = crate::Reg<fifo202::Fifo202Spec>;
#[doc = "FIFO Memory Aperture0 202"]
pub mod fifo202;
#[doc = "FIFO203 (rw) register accessor: FIFO Memory Aperture0 203\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo203::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo203::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo203`]
module"]
#[doc(alias = "FIFO203")]
pub type Fifo203 = crate::Reg<fifo203::Fifo203Spec>;
#[doc = "FIFO Memory Aperture0 203"]
pub mod fifo203;
#[doc = "FIFO204 (rw) register accessor: FIFO Memory Aperture0 204\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo204::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo204::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo204`]
module"]
#[doc(alias = "FIFO204")]
pub type Fifo204 = crate::Reg<fifo204::Fifo204Spec>;
#[doc = "FIFO Memory Aperture0 204"]
pub mod fifo204;
#[doc = "FIFO205 (rw) register accessor: FIFO Memory Aperture0 205\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo205::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo205::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo205`]
module"]
#[doc(alias = "FIFO205")]
pub type Fifo205 = crate::Reg<fifo205::Fifo205Spec>;
#[doc = "FIFO Memory Aperture0 205"]
pub mod fifo205;
#[doc = "FIFO206 (rw) register accessor: FIFO Memory Aperture0 206\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo206::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo206::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo206`]
module"]
#[doc(alias = "FIFO206")]
pub type Fifo206 = crate::Reg<fifo206::Fifo206Spec>;
#[doc = "FIFO Memory Aperture0 206"]
pub mod fifo206;
#[doc = "FIFO207 (rw) register accessor: FIFO Memory Aperture0 207\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo207::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo207::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo207`]
module"]
#[doc(alias = "FIFO207")]
pub type Fifo207 = crate::Reg<fifo207::Fifo207Spec>;
#[doc = "FIFO Memory Aperture0 207"]
pub mod fifo207;
#[doc = "FIFO208 (rw) register accessor: FIFO Memory Aperture0 208\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo208::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo208::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo208`]
module"]
#[doc(alias = "FIFO208")]
pub type Fifo208 = crate::Reg<fifo208::Fifo208Spec>;
#[doc = "FIFO Memory Aperture0 208"]
pub mod fifo208;
#[doc = "FIFO209 (rw) register accessor: FIFO Memory Aperture0 209\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo209::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo209::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo209`]
module"]
#[doc(alias = "FIFO209")]
pub type Fifo209 = crate::Reg<fifo209::Fifo209Spec>;
#[doc = "FIFO Memory Aperture0 209"]
pub mod fifo209;
#[doc = "FIFO210 (rw) register accessor: FIFO Memory Aperture0 210\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo210::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo210::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo210`]
module"]
#[doc(alias = "FIFO210")]
pub type Fifo210 = crate::Reg<fifo210::Fifo210Spec>;
#[doc = "FIFO Memory Aperture0 210"]
pub mod fifo210;
#[doc = "FIFO211 (rw) register accessor: FIFO Memory Aperture0 211\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo211::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo211::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo211`]
module"]
#[doc(alias = "FIFO211")]
pub type Fifo211 = crate::Reg<fifo211::Fifo211Spec>;
#[doc = "FIFO Memory Aperture0 211"]
pub mod fifo211;
#[doc = "FIFO212 (rw) register accessor: FIFO Memory Aperture0 212\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo212::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo212::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo212`]
module"]
#[doc(alias = "FIFO212")]
pub type Fifo212 = crate::Reg<fifo212::Fifo212Spec>;
#[doc = "FIFO Memory Aperture0 212"]
pub mod fifo212;
#[doc = "FIFO213 (rw) register accessor: FIFO Memory Aperture0 213\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo213::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo213::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo213`]
module"]
#[doc(alias = "FIFO213")]
pub type Fifo213 = crate::Reg<fifo213::Fifo213Spec>;
#[doc = "FIFO Memory Aperture0 213"]
pub mod fifo213;
#[doc = "FIFO214 (rw) register accessor: FIFO Memory Aperture0 214\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo214::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo214::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo214`]
module"]
#[doc(alias = "FIFO214")]
pub type Fifo214 = crate::Reg<fifo214::Fifo214Spec>;
#[doc = "FIFO Memory Aperture0 214"]
pub mod fifo214;
#[doc = "FIFO215 (rw) register accessor: FIFO Memory Aperture0 215\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo215::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo215::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo215`]
module"]
#[doc(alias = "FIFO215")]
pub type Fifo215 = crate::Reg<fifo215::Fifo215Spec>;
#[doc = "FIFO Memory Aperture0 215"]
pub mod fifo215;
#[doc = "FIFO216 (rw) register accessor: FIFO Memory Aperture0 216\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo216::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo216::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo216`]
module"]
#[doc(alias = "FIFO216")]
pub type Fifo216 = crate::Reg<fifo216::Fifo216Spec>;
#[doc = "FIFO Memory Aperture0 216"]
pub mod fifo216;
#[doc = "FIFO217 (rw) register accessor: FIFO Memory Aperture0 217\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo217::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo217::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo217`]
module"]
#[doc(alias = "FIFO217")]
pub type Fifo217 = crate::Reg<fifo217::Fifo217Spec>;
#[doc = "FIFO Memory Aperture0 217"]
pub mod fifo217;
#[doc = "FIFO218 (rw) register accessor: FIFO Memory Aperture0 218\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo218::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo218::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo218`]
module"]
#[doc(alias = "FIFO218")]
pub type Fifo218 = crate::Reg<fifo218::Fifo218Spec>;
#[doc = "FIFO Memory Aperture0 218"]
pub mod fifo218;
#[doc = "FIFO219 (rw) register accessor: FIFO Memory Aperture0 219\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo219::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo219::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo219`]
module"]
#[doc(alias = "FIFO219")]
pub type Fifo219 = crate::Reg<fifo219::Fifo219Spec>;
#[doc = "FIFO Memory Aperture0 219"]
pub mod fifo219;
#[doc = "FIFO220 (rw) register accessor: FIFO Memory Aperture0 220\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo220::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo220::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo220`]
module"]
#[doc(alias = "FIFO220")]
pub type Fifo220 = crate::Reg<fifo220::Fifo220Spec>;
#[doc = "FIFO Memory Aperture0 220"]
pub mod fifo220;
#[doc = "FIFO221 (rw) register accessor: FIFO Memory Aperture0 221\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo221::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo221::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo221`]
module"]
#[doc(alias = "FIFO221")]
pub type Fifo221 = crate::Reg<fifo221::Fifo221Spec>;
#[doc = "FIFO Memory Aperture0 221"]
pub mod fifo221;
#[doc = "FIFO222 (rw) register accessor: FIFO Memory Aperture0 222\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo222::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo222::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo222`]
module"]
#[doc(alias = "FIFO222")]
pub type Fifo222 = crate::Reg<fifo222::Fifo222Spec>;
#[doc = "FIFO Memory Aperture0 222"]
pub mod fifo222;
#[doc = "FIFO223 (rw) register accessor: FIFO Memory Aperture0 223\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo223::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo223::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo223`]
module"]
#[doc(alias = "FIFO223")]
pub type Fifo223 = crate::Reg<fifo223::Fifo223Spec>;
#[doc = "FIFO Memory Aperture0 223"]
pub mod fifo223;
#[doc = "FIFO224 (rw) register accessor: FIFO Memory Aperture0 224\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo224::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo224::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo224`]
module"]
#[doc(alias = "FIFO224")]
pub type Fifo224 = crate::Reg<fifo224::Fifo224Spec>;
#[doc = "FIFO Memory Aperture0 224"]
pub mod fifo224;
#[doc = "FIFO225 (rw) register accessor: FIFO Memory Aperture0 225\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo225::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo225::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo225`]
module"]
#[doc(alias = "FIFO225")]
pub type Fifo225 = crate::Reg<fifo225::Fifo225Spec>;
#[doc = "FIFO Memory Aperture0 225"]
pub mod fifo225;
#[doc = "FIFO226 (rw) register accessor: FIFO Memory Aperture0 226\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo226::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo226::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo226`]
module"]
#[doc(alias = "FIFO226")]
pub type Fifo226 = crate::Reg<fifo226::Fifo226Spec>;
#[doc = "FIFO Memory Aperture0 226"]
pub mod fifo226;
#[doc = "FIFO227 (rw) register accessor: FIFO Memory Aperture0 227\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo227::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo227::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo227`]
module"]
#[doc(alias = "FIFO227")]
pub type Fifo227 = crate::Reg<fifo227::Fifo227Spec>;
#[doc = "FIFO Memory Aperture0 227"]
pub mod fifo227;
#[doc = "FIFO228 (rw) register accessor: FIFO Memory Aperture0 228\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo228::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo228::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo228`]
module"]
#[doc(alias = "FIFO228")]
pub type Fifo228 = crate::Reg<fifo228::Fifo228Spec>;
#[doc = "FIFO Memory Aperture0 228"]
pub mod fifo228;
#[doc = "FIFO229 (rw) register accessor: FIFO Memory Aperture0 229\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo229::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo229::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo229`]
module"]
#[doc(alias = "FIFO229")]
pub type Fifo229 = crate::Reg<fifo229::Fifo229Spec>;
#[doc = "FIFO Memory Aperture0 229"]
pub mod fifo229;
#[doc = "FIFO230 (rw) register accessor: FIFO Memory Aperture0 230\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo230::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo230::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo230`]
module"]
#[doc(alias = "FIFO230")]
pub type Fifo230 = crate::Reg<fifo230::Fifo230Spec>;
#[doc = "FIFO Memory Aperture0 230"]
pub mod fifo230;
#[doc = "FIFO231 (rw) register accessor: FIFO Memory Aperture0 231\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo231::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo231::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo231`]
module"]
#[doc(alias = "FIFO231")]
pub type Fifo231 = crate::Reg<fifo231::Fifo231Spec>;
#[doc = "FIFO Memory Aperture0 231"]
pub mod fifo231;
#[doc = "FIFO232 (rw) register accessor: FIFO Memory Aperture0 232\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo232::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo232::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo232`]
module"]
#[doc(alias = "FIFO232")]
pub type Fifo232 = crate::Reg<fifo232::Fifo232Spec>;
#[doc = "FIFO Memory Aperture0 232"]
pub mod fifo232;
#[doc = "FIFO233 (rw) register accessor: FIFO Memory Aperture0 233\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo233::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo233::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo233`]
module"]
#[doc(alias = "FIFO233")]
pub type Fifo233 = crate::Reg<fifo233::Fifo233Spec>;
#[doc = "FIFO Memory Aperture0 233"]
pub mod fifo233;
#[doc = "FIFO234 (rw) register accessor: FIFO Memory Aperture0 234\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo234::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo234::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo234`]
module"]
#[doc(alias = "FIFO234")]
pub type Fifo234 = crate::Reg<fifo234::Fifo234Spec>;
#[doc = "FIFO Memory Aperture0 234"]
pub mod fifo234;
#[doc = "FIFO235 (rw) register accessor: FIFO Memory Aperture0 235\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo235::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo235::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo235`]
module"]
#[doc(alias = "FIFO235")]
pub type Fifo235 = crate::Reg<fifo235::Fifo235Spec>;
#[doc = "FIFO Memory Aperture0 235"]
pub mod fifo235;
#[doc = "FIFO236 (rw) register accessor: FIFO Memory Aperture0 236\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo236::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo236::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo236`]
module"]
#[doc(alias = "FIFO236")]
pub type Fifo236 = crate::Reg<fifo236::Fifo236Spec>;
#[doc = "FIFO Memory Aperture0 236"]
pub mod fifo236;
#[doc = "FIFO237 (rw) register accessor: FIFO Memory Aperture0 237\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo237::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo237::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo237`]
module"]
#[doc(alias = "FIFO237")]
pub type Fifo237 = crate::Reg<fifo237::Fifo237Spec>;
#[doc = "FIFO Memory Aperture0 237"]
pub mod fifo237;
#[doc = "FIFO238 (rw) register accessor: FIFO Memory Aperture0 238\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo238::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo238::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo238`]
module"]
#[doc(alias = "FIFO238")]
pub type Fifo238 = crate::Reg<fifo238::Fifo238Spec>;
#[doc = "FIFO Memory Aperture0 238"]
pub mod fifo238;
#[doc = "FIFO239 (rw) register accessor: FIFO Memory Aperture0 239\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo239::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo239::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo239`]
module"]
#[doc(alias = "FIFO239")]
pub type Fifo239 = crate::Reg<fifo239::Fifo239Spec>;
#[doc = "FIFO Memory Aperture0 239"]
pub mod fifo239;
#[doc = "FIFO240 (rw) register accessor: FIFO Memory Aperture0 240\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo240::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo240::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo240`]
module"]
#[doc(alias = "FIFO240")]
pub type Fifo240 = crate::Reg<fifo240::Fifo240Spec>;
#[doc = "FIFO Memory Aperture0 240"]
pub mod fifo240;
#[doc = "FIFO241 (rw) register accessor: FIFO Memory Aperture0 241\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo241::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo241::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo241`]
module"]
#[doc(alias = "FIFO241")]
pub type Fifo241 = crate::Reg<fifo241::Fifo241Spec>;
#[doc = "FIFO Memory Aperture0 241"]
pub mod fifo241;
#[doc = "FIFO242 (rw) register accessor: FIFO Memory Aperture0 242\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo242::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo242::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo242`]
module"]
#[doc(alias = "FIFO242")]
pub type Fifo242 = crate::Reg<fifo242::Fifo242Spec>;
#[doc = "FIFO Memory Aperture0 242"]
pub mod fifo242;
#[doc = "FIFO243 (rw) register accessor: FIFO Memory Aperture0 243\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo243::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo243::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo243`]
module"]
#[doc(alias = "FIFO243")]
pub type Fifo243 = crate::Reg<fifo243::Fifo243Spec>;
#[doc = "FIFO Memory Aperture0 243"]
pub mod fifo243;
#[doc = "FIFO244 (rw) register accessor: FIFO Memory Aperture0 244\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo244::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo244::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo244`]
module"]
#[doc(alias = "FIFO244")]
pub type Fifo244 = crate::Reg<fifo244::Fifo244Spec>;
#[doc = "FIFO Memory Aperture0 244"]
pub mod fifo244;
#[doc = "FIFO245 (rw) register accessor: FIFO Memory Aperture0 245\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo245::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo245::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo245`]
module"]
#[doc(alias = "FIFO245")]
pub type Fifo245 = crate::Reg<fifo245::Fifo245Spec>;
#[doc = "FIFO Memory Aperture0 245"]
pub mod fifo245;
#[doc = "FIFO246 (rw) register accessor: FIFO Memory Aperture0 246\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo246::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo246::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo246`]
module"]
#[doc(alias = "FIFO246")]
pub type Fifo246 = crate::Reg<fifo246::Fifo246Spec>;
#[doc = "FIFO Memory Aperture0 246"]
pub mod fifo246;
#[doc = "FIFO247 (rw) register accessor: FIFO Memory Aperture0 247\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo247::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo247::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo247`]
module"]
#[doc(alias = "FIFO247")]
pub type Fifo247 = crate::Reg<fifo247::Fifo247Spec>;
#[doc = "FIFO Memory Aperture0 247"]
pub mod fifo247;
#[doc = "FIFO248 (rw) register accessor: FIFO Memory Aperture0 248\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo248::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo248::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo248`]
module"]
#[doc(alias = "FIFO248")]
pub type Fifo248 = crate::Reg<fifo248::Fifo248Spec>;
#[doc = "FIFO Memory Aperture0 248"]
pub mod fifo248;
#[doc = "FIFO249 (rw) register accessor: FIFO Memory Aperture0 249\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo249::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo249::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo249`]
module"]
#[doc(alias = "FIFO249")]
pub type Fifo249 = crate::Reg<fifo249::Fifo249Spec>;
#[doc = "FIFO Memory Aperture0 249"]
pub mod fifo249;
#[doc = "FIFO250 (rw) register accessor: FIFO Memory Aperture0 250\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo250::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo250::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo250`]
module"]
#[doc(alias = "FIFO250")]
pub type Fifo250 = crate::Reg<fifo250::Fifo250Spec>;
#[doc = "FIFO Memory Aperture0 250"]
pub mod fifo250;
#[doc = "FIFO251 (rw) register accessor: FIFO Memory Aperture0 251\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo251::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo251::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo251`]
module"]
#[doc(alias = "FIFO251")]
pub type Fifo251 = crate::Reg<fifo251::Fifo251Spec>;
#[doc = "FIFO Memory Aperture0 251"]
pub mod fifo251;
#[doc = "FIFO252 (rw) register accessor: FIFO Memory Aperture0 252\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo252::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo252::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo252`]
module"]
#[doc(alias = "FIFO252")]
pub type Fifo252 = crate::Reg<fifo252::Fifo252Spec>;
#[doc = "FIFO Memory Aperture0 252"]
pub mod fifo252;
#[doc = "FIFO253 (rw) register accessor: FIFO Memory Aperture0 253\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo253::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo253::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo253`]
module"]
#[doc(alias = "FIFO253")]
pub type Fifo253 = crate::Reg<fifo253::Fifo253Spec>;
#[doc = "FIFO Memory Aperture0 253"]
pub mod fifo253;
#[doc = "FIFO254 (rw) register accessor: FIFO Memory Aperture0 254\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo254::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo254::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo254`]
module"]
#[doc(alias = "FIFO254")]
pub type Fifo254 = crate::Reg<fifo254::Fifo254Spec>;
#[doc = "FIFO Memory Aperture0 254"]
pub mod fifo254;
#[doc = "FIFO255 (rw) register accessor: FIFO Memory Aperture0 255\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo255::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo255::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo255`]
module"]
#[doc(alias = "FIFO255")]
pub type Fifo255 = crate::Reg<fifo255::Fifo255Spec>;
#[doc = "FIFO Memory Aperture0 255"]
pub mod fifo255;
