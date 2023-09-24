#[doc = "Register `EPTSTA3` reader"]
pub type R = crate::R<EPTSTA3_SPEC>;
#[doc = "Field `FRCESTALL` reader - Stall Handshake Request"]
pub type FRCESTALL_R = crate::BitReader;
#[doc = "Field `TOGGLESQ_STA` reader - Toggle Sequencing"]
pub type TOGGLESQ_STA_R = crate::FieldReader<TOGGLESQ_STA_A>;
#[doc = "Toggle Sequencing\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TOGGLESQ_STA_A {
    #[doc = "0: DATA0"]
    Data0 = 0,
    #[doc = "1: DATA1"]
    Data1 = 1,
    #[doc = "2: Reserved for High Bandwidth Isochronous Endpoint"]
    Data2 = 2,
    #[doc = "3: Reserved for High Bandwidth Isochronous Endpoint"]
    Mdata = 3,
}
impl From<TOGGLESQ_STA_A> for u8 {
    #[inline(always)]
    fn from(variant: TOGGLESQ_STA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TOGGLESQ_STA_A {
    type Ux = u8;
}
impl TOGGLESQ_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOGGLESQ_STA_A {
        match self.bits {
            0 => TOGGLESQ_STA_A::Data0,
            1 => TOGGLESQ_STA_A::Data1,
            2 => TOGGLESQ_STA_A::Data2,
            3 => TOGGLESQ_STA_A::Mdata,
            _ => unreachable!(),
        }
    }
    #[doc = "DATA0"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == TOGGLESQ_STA_A::Data0
    }
    #[doc = "DATA1"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == TOGGLESQ_STA_A::Data1
    }
    #[doc = "Reserved for High Bandwidth Isochronous Endpoint"]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == TOGGLESQ_STA_A::Data2
    }
    #[doc = "Reserved for High Bandwidth Isochronous Endpoint"]
    #[inline(always)]
    pub fn is_mdata(&self) -> bool {
        *self == TOGGLESQ_STA_A::Mdata
    }
}
#[doc = "Field `ERR_OVFLW` reader - Overflow Error"]
pub type ERR_OVFLW_R = crate::BitReader;
#[doc = "Field `RXRDY_TXKL` reader - Received OUT Data/KILL Bank"]
pub type RXRDY_TXKL_R = crate::BitReader;
#[doc = "Field `TX_COMPLT` reader - Transmitted IN Data Complete"]
pub type TX_COMPLT_R = crate::BitReader;
#[doc = "Field `TXRDY` reader - TX Packet Ready"]
pub type TXRDY_R = crate::BitReader;
#[doc = "Field `RX_SETUP` reader - Received SETUP"]
pub type RX_SETUP_R = crate::BitReader;
#[doc = "Field `STALL_SNT` reader - Stall Sent"]
pub type STALL_SNT_R = crate::BitReader;
#[doc = "Field `NAK_IN` reader - NAK IN"]
pub type NAK_IN_R = crate::BitReader;
#[doc = "Field `NAK_OUT` reader - NAK OUT"]
pub type NAK_OUT_R = crate::BitReader;
#[doc = "Field `CURBK_CTLDIR` reader - Current Bank/Control Direction"]
pub type CURBK_CTLDIR_R = crate::FieldReader;
#[doc = "Field `BUSY_BANK_STA` reader - Busy Bank Number"]
pub type BUSY_BANK_STA_R = crate::FieldReader<BUSY_BANK_STA_A>;
#[doc = "Busy Bank Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BUSY_BANK_STA_A {
    #[doc = "0: 1 busy bank"]
    _1busybank = 0,
    #[doc = "1: 2 busy banks"]
    _2busybanks = 1,
    #[doc = "2: 3 busy banks"]
    _3busybanks = 2,
}
impl From<BUSY_BANK_STA_A> for u8 {
    #[inline(always)]
    fn from(variant: BUSY_BANK_STA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BUSY_BANK_STA_A {
    type Ux = u8;
}
impl BUSY_BANK_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BUSY_BANK_STA_A> {
        match self.bits {
            0 => Some(BUSY_BANK_STA_A::_1busybank),
            1 => Some(BUSY_BANK_STA_A::_2busybanks),
            2 => Some(BUSY_BANK_STA_A::_3busybanks),
            _ => None,
        }
    }
    #[doc = "1 busy bank"]
    #[inline(always)]
    pub fn is_1busybank(&self) -> bool {
        *self == BUSY_BANK_STA_A::_1busybank
    }
    #[doc = "2 busy banks"]
    #[inline(always)]
    pub fn is_2busybanks(&self) -> bool {
        *self == BUSY_BANK_STA_A::_2busybanks
    }
    #[doc = "3 busy banks"]
    #[inline(always)]
    pub fn is_3busybanks(&self) -> bool {
        *self == BUSY_BANK_STA_A::_3busybanks
    }
}
#[doc = "Field `BYTE_COUNT` reader - UDPHS Byte Count"]
pub type BYTE_COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `SHRT_PCKT` reader - Short Packet"]
pub type SHRT_PCKT_R = crate::BitReader;
impl R {
    #[doc = "Bit 5 - Stall Handshake Request"]
    #[inline(always)]
    pub fn frcestall(&self) -> FRCESTALL_R {
        FRCESTALL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Toggle Sequencing"]
    #[inline(always)]
    pub fn togglesq_sta(&self) -> TOGGLESQ_STA_R {
        TOGGLESQ_STA_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Overflow Error"]
    #[inline(always)]
    pub fn err_ovflw(&self) -> ERR_OVFLW_R {
        ERR_OVFLW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Received OUT Data/KILL Bank"]
    #[inline(always)]
    pub fn rxrdy_txkl(&self) -> RXRDY_TXKL_R {
        RXRDY_TXKL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete"]
    #[inline(always)]
    pub fn tx_complt(&self) -> TX_COMPLT_R {
        TX_COMPLT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TX Packet Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Received SETUP"]
    #[inline(always)]
    pub fn rx_setup(&self) -> RX_SETUP_R {
        RX_SETUP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Stall Sent"]
    #[inline(always)]
    pub fn stall_snt(&self) -> STALL_SNT_R {
        STALL_SNT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NAK IN"]
    #[inline(always)]
    pub fn nak_in(&self) -> NAK_IN_R {
        NAK_IN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NAK OUT"]
    #[inline(always)]
    pub fn nak_out(&self) -> NAK_OUT_R {
        NAK_OUT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Current Bank/Control Direction"]
    #[inline(always)]
    pub fn curbk_ctldir(&self) -> CURBK_CTLDIR_R {
        CURBK_CTLDIR_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Busy Bank Number"]
    #[inline(always)]
    pub fn busy_bank_sta(&self) -> BUSY_BANK_STA_R {
        BUSY_BANK_STA_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:30 - UDPHS Byte Count"]
    #[inline(always)]
    pub fn byte_count(&self) -> BYTE_COUNT_R {
        BYTE_COUNT_R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - Short Packet"]
    #[inline(always)]
    pub fn shrt_pckt(&self) -> SHRT_PCKT_R {
        SHRT_PCKT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "UDPHS Endpoint Status Register (endpoint = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptsta3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPTSTA3_SPEC;
impl crate::RegisterSpec for EPTSTA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eptsta3::R`](R) reader structure"]
impl crate::Readable for EPTSTA3_SPEC {}
#[doc = "`reset()` method sets EPTSTA3 to value 0x40"]
impl crate::Resettable for EPTSTA3_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
