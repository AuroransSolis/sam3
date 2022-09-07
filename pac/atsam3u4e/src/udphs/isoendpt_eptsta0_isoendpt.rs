#[doc = "Register `EPTSTA0_ISOENDPT` reader"]
pub struct R(crate::R<ISOENDPT_EPTSTA0_ISOENDPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISOENDPT_EPTSTA0_ISOENDPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISOENDPT_EPTSTA0_ISOENDPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISOENDPT_EPTSTA0_ISOENDPT_SPEC>) -> Self {
        R(reader)
    }
}
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
    #[doc = "2: Data2 (only for High Bandwidth Isochronous Endpoint)"]
    DATA2 = 2,
    #[doc = "3: MData (only for High Bandwidth Isochronous Endpoint)"]
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
#[doc = "Field `TXRDY_TRER` reader - TX Packet Ready/Transaction Error"]
pub type TXRDY_TRER_R = crate::BitReader<bool>;
#[doc = "Field `ERR_FL_ISO` reader - Error Flow"]
pub type ERR_FL_ISO_R = crate::BitReader<bool>;
#[doc = "Field `ERR_CRC_NTR` reader - CRC ISO Error/Number of Transaction Error"]
pub type ERR_CRC_NTR_R = crate::BitReader<bool>;
#[doc = "Field `ERR_FLUSH` reader - Bank Flush Error"]
pub type ERR_FLUSH_R = crate::BitReader<bool>;
#[doc = "Field `CURBK` reader - Current Bank"]
pub type CURBK_R = crate::FieldReader<u8, CURBK_A>;
#[doc = "Current Bank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CURBK_A {
    #[doc = "0: Bank 0 (or single bank)"]
    BANK0 = 0,
    #[doc = "1: Bank 1"]
    BANK1 = 1,
    #[doc = "2: Bank 2"]
    BANK2 = 2,
}
impl From<CURBK_A> for u8 {
    #[inline(always)]
    fn from(variant: CURBK_A) -> Self {
        variant as _
    }
}
impl CURBK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CURBK_A> {
        match self.bits {
            0 => Some(CURBK_A::BANK0),
            1 => Some(CURBK_A::BANK1),
            2 => Some(CURBK_A::BANK2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BANK0`"]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        *self == CURBK_A::BANK0
    }
    #[doc = "Checks if the value of the field is `BANK1`"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == CURBK_A::BANK1
    }
    #[doc = "Checks if the value of the field is `BANK2`"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == CURBK_A::BANK2
    }
}
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
    #[doc = "Bit 11 - TX Packet Ready/Transaction Error"]
    #[inline(always)]
    pub fn txrdy_trer(&self) -> TXRDY_TRER_R {
        TXRDY_TRER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Error Flow"]
    #[inline(always)]
    pub fn err_fl_iso(&self) -> ERR_FL_ISO_R {
        ERR_FL_ISO_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CRC ISO Error/Number of Transaction Error"]
    #[inline(always)]
    pub fn err_crc_ntr(&self) -> ERR_CRC_NTR_R {
        ERR_CRC_NTR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bank Flush Error"]
    #[inline(always)]
    pub fn err_flush(&self) -> ERR_FLUSH_R {
        ERR_FLUSH_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Current Bank"]
    #[inline(always)]
    pub fn curbk(&self) -> CURBK_R {
        CURBK_R::new(((self.bits >> 16) & 3) as u8)
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
#[doc = "UDPHS Endpoint Status Register (endpoint = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isoendpt_eptsta0_isoendpt](index.html) module"]
pub struct ISOENDPT_EPTSTA0_ISOENDPT_SPEC;
impl crate::RegisterSpec for ISOENDPT_EPTSTA0_ISOENDPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isoendpt_eptsta0_isoendpt::R](R) reader structure"]
impl crate::Readable for ISOENDPT_EPTSTA0_ISOENDPT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EPTSTA0_ISOENDPT to value 0x40"]
impl crate::Resettable for ISOENDPT_EPTSTA0_ISOENDPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
