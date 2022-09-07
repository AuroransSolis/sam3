#[doc = "Register `EPTSTA4` reader"]
pub struct R(crate::R<EPTSTA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPTSTA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPTSTA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPTSTA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRCESTALL` reader - Stall Handshake Request"]
pub type FRCESTALL_R = crate::BitReader<bool>;
#[doc = "Field `TOGGLESQ_STA` reader - Toggle Sequencing"]
pub type TOGGLESQ_STA_R = crate::FieldReader<u8, TOGGLESQ_STA_A>;
#[doc = "Toggle Sequencing\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TOGGLESQ_STA_A {
    #[doc = "0: DATA0"]
    DATA0 = 0,
    #[doc = "1: DATA1"]
    DATA1 = 1,
    #[doc = "2: Reserved for High Bandwidth Isochronous Endpoint"]
    DATA2 = 2,
    #[doc = "3: Reserved for High Bandwidth Isochronous Endpoint"]
    MDATA = 3,
}
impl From<TOGGLESQ_STA_A> for u8 {
    #[inline(always)]
    fn from(variant: TOGGLESQ_STA_A) -> Self {
        variant as _
    }
}
impl TOGGLESQ_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOGGLESQ_STA_A {
        match self.bits {
            0 => TOGGLESQ_STA_A::DATA0,
            1 => TOGGLESQ_STA_A::DATA1,
            2 => TOGGLESQ_STA_A::DATA2,
            3 => TOGGLESQ_STA_A::MDATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == TOGGLESQ_STA_A::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == TOGGLESQ_STA_A::DATA1
    }
    #[doc = "Checks if the value of the field is `DATA2`"]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == TOGGLESQ_STA_A::DATA2
    }
    #[doc = "Checks if the value of the field is `MDATA`"]
    #[inline(always)]
    pub fn is_mdata(&self) -> bool {
        *self == TOGGLESQ_STA_A::MDATA
    }
}
#[doc = "Field `ERR_OVFLW` reader - Overflow Error"]
pub type ERR_OVFLW_R = crate::BitReader<bool>;
#[doc = "Field `RXRDY_TXKL` reader - Received OUT Data/KILL Bank"]
pub type RXRDY_TXKL_R = crate::BitReader<bool>;
#[doc = "Field `TX_COMPLT` reader - Transmitted IN Data Complete"]
pub type TX_COMPLT_R = crate::BitReader<bool>;
#[doc = "Field `TXRDY` reader - TX Packet Ready"]
pub type TXRDY_R = crate::BitReader<bool>;
#[doc = "Field `RX_SETUP` reader - Received SETUP"]
pub type RX_SETUP_R = crate::BitReader<bool>;
#[doc = "Field `STALL_SNT` reader - Stall Sent"]
pub type STALL_SNT_R = crate::BitReader<bool>;
#[doc = "Field `NAK_IN` reader - NAK IN"]
pub type NAK_IN_R = crate::BitReader<bool>;
#[doc = "Field `NAK_OUT` reader - NAK OUT"]
pub type NAK_OUT_R = crate::BitReader<bool>;
#[doc = "Field `CURBK_CTLDIR` reader - Current Bank/Control Direction"]
pub type CURBK_CTLDIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUSY_BANK_STA` reader - Busy Bank Number"]
pub type BUSY_BANK_STA_R = crate::FieldReader<u8, BUSY_BANK_STA_A>;
#[doc = "Busy Bank Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BUSY_BANK_STA_A {
    #[doc = "0: 1 busy bank"]
    _1BUSYBANK = 0,
    #[doc = "1: 2 busy banks"]
    _2BUSYBANKS = 1,
    #[doc = "2: 3 busy banks"]
    _3BUSYBANKS = 2,
}
impl From<BUSY_BANK_STA_A> for u8 {
    #[inline(always)]
    fn from(variant: BUSY_BANK_STA_A) -> Self {
        variant as _
    }
}
impl BUSY_BANK_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BUSY_BANK_STA_A> {
        match self.bits {
            0 => Some(BUSY_BANK_STA_A::_1BUSYBANK),
            1 => Some(BUSY_BANK_STA_A::_2BUSYBANKS),
            2 => Some(BUSY_BANK_STA_A::_3BUSYBANKS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1BUSYBANK`"]
    #[inline(always)]
    pub fn is_1busybank(&self) -> bool {
        *self == BUSY_BANK_STA_A::_1BUSYBANK
    }
    #[doc = "Checks if the value of the field is `_2BUSYBANKS`"]
    #[inline(always)]
    pub fn is_2busybanks(&self) -> bool {
        *self == BUSY_BANK_STA_A::_2BUSYBANKS
    }
    #[doc = "Checks if the value of the field is `_3BUSYBANKS`"]
    #[inline(always)]
    pub fn is_3busybanks(&self) -> bool {
        *self == BUSY_BANK_STA_A::_3BUSYBANKS
    }
}
#[doc = "Field `BYTE_COUNT` reader - UDPHS Byte Count"]
pub type BYTE_COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SHRT_PCKT` reader - Short Packet"]
pub type SHRT_PCKT_R = crate::BitReader<bool>;
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
#[doc = "UDPHS Endpoint Status Register (endpoint = 4)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eptsta4](index.html) module"]
pub struct EPTSTA4_SPEC;
impl crate::RegisterSpec for EPTSTA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eptsta4::R](R) reader structure"]
impl crate::Readable for EPTSTA4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EPTSTA4 to value 0x40"]
impl crate::Resettable for EPTSTA4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
