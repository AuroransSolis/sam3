#[doc = "Register `EPTSTA1` reader"]
pub struct R(crate::R<EPTSTA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPTSTA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPTSTA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPTSTA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRCESTALL` reader - Stall Handshake Request"]
pub struct FRCESTALL_R(crate::FieldReader<bool, bool>);
impl FRCESTALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRCESTALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRCESTALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Field `TOGGLESQ_STA` reader - Toggle Sequencing"]
pub struct TOGGLESQ_STA_R(crate::FieldReader<u8, TOGGLESQ_STA_A>);
impl TOGGLESQ_STA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOGGLESQ_STA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == TOGGLESQ_STA_A::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        **self == TOGGLESQ_STA_A::DATA1
    }
    #[doc = "Checks if the value of the field is `DATA2`"]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        **self == TOGGLESQ_STA_A::DATA2
    }
    #[doc = "Checks if the value of the field is `MDATA`"]
    #[inline(always)]
    pub fn is_mdata(&self) -> bool {
        **self == TOGGLESQ_STA_A::MDATA
    }
}
impl core::ops::Deref for TOGGLESQ_STA_R {
    type Target = crate::FieldReader<u8, TOGGLESQ_STA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_OVFLW` reader - Overflow Error"]
pub struct ERR_OVFLW_R(crate::FieldReader<bool, bool>);
impl ERR_OVFLW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR_OVFLW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_OVFLW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRDY_TXKL` reader - Received OUT Data/KILL Bank"]
pub struct RXRDY_TXKL_R(crate::FieldReader<bool, bool>);
impl RXRDY_TXKL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXRDY_TXKL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRDY_TXKL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_COMPLT` reader - Transmitted IN Data Complete"]
pub struct TX_COMPLT_R(crate::FieldReader<bool, bool>);
impl TX_COMPLT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_COMPLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_COMPLT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRDY` reader - TX Packet Ready"]
pub struct TXRDY_R(crate::FieldReader<bool, bool>);
impl TXRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_SETUP` reader - Received SETUP"]
pub struct RX_SETUP_R(crate::FieldReader<bool, bool>);
impl RX_SETUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_SETUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL_SNT` reader - Stall Sent"]
pub struct STALL_SNT_R(crate::FieldReader<bool, bool>);
impl STALL_SNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STALL_SNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALL_SNT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAK_IN` reader - NAK IN"]
pub struct NAK_IN_R(crate::FieldReader<bool, bool>);
impl NAK_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NAK_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAK_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAK_OUT` reader - NAK OUT"]
pub struct NAK_OUT_R(crate::FieldReader<bool, bool>);
impl NAK_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NAK_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAK_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CURBK_CTLDIR` reader - Current Bank/Control Direction"]
pub struct CURBK_CTLDIR_R(crate::FieldReader<u8, u8>);
impl CURBK_CTLDIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CURBK_CTLDIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURBK_CTLDIR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Field `BUSY_BANK_STA` reader - Busy Bank Number"]
pub struct BUSY_BANK_STA_R(crate::FieldReader<u8, BUSY_BANK_STA_A>);
impl BUSY_BANK_STA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BUSY_BANK_STA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == BUSY_BANK_STA_A::_1BUSYBANK
    }
    #[doc = "Checks if the value of the field is `_2BUSYBANKS`"]
    #[inline(always)]
    pub fn is_2busybanks(&self) -> bool {
        **self == BUSY_BANK_STA_A::_2BUSYBANKS
    }
    #[doc = "Checks if the value of the field is `_3BUSYBANKS`"]
    #[inline(always)]
    pub fn is_3busybanks(&self) -> bool {
        **self == BUSY_BANK_STA_A::_3BUSYBANKS
    }
}
impl core::ops::Deref for BUSY_BANK_STA_R {
    type Target = crate::FieldReader<u8, BUSY_BANK_STA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE_COUNT` reader - UDPHS Byte Count"]
pub struct BYTE_COUNT_R(crate::FieldReader<u16, u16>);
impl BYTE_COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BYTE_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE_COUNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHRT_PCKT` reader - Short Packet"]
pub struct SHRT_PCKT_R(crate::FieldReader<bool, bool>);
impl SHRT_PCKT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SHRT_PCKT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHRT_PCKT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 5 - Stall Handshake Request"]
    #[inline(always)]
    pub fn frcestall(&self) -> FRCESTALL_R {
        FRCESTALL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Toggle Sequencing"]
    #[inline(always)]
    pub fn togglesq_sta(&self) -> TOGGLESQ_STA_R {
        TOGGLESQ_STA_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Overflow Error"]
    #[inline(always)]
    pub fn err_ovflw(&self) -> ERR_OVFLW_R {
        ERR_OVFLW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Received OUT Data/KILL Bank"]
    #[inline(always)]
    pub fn rxrdy_txkl(&self) -> RXRDY_TXKL_R {
        RXRDY_TXKL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete"]
    #[inline(always)]
    pub fn tx_complt(&self) -> TX_COMPLT_R {
        TX_COMPLT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TX Packet Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Received SETUP"]
    #[inline(always)]
    pub fn rx_setup(&self) -> RX_SETUP_R {
        RX_SETUP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Stall Sent"]
    #[inline(always)]
    pub fn stall_snt(&self) -> STALL_SNT_R {
        STALL_SNT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - NAK IN"]
    #[inline(always)]
    pub fn nak_in(&self) -> NAK_IN_R {
        NAK_IN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - NAK OUT"]
    #[inline(always)]
    pub fn nak_out(&self) -> NAK_OUT_R {
        NAK_OUT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Current Bank/Control Direction"]
    #[inline(always)]
    pub fn curbk_ctldir(&self) -> CURBK_CTLDIR_R {
        CURBK_CTLDIR_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Busy Bank Number"]
    #[inline(always)]
    pub fn busy_bank_sta(&self) -> BUSY_BANK_STA_R {
        BUSY_BANK_STA_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:30 - UDPHS Byte Count"]
    #[inline(always)]
    pub fn byte_count(&self) -> BYTE_COUNT_R {
        BYTE_COUNT_R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - Short Packet"]
    #[inline(always)]
    pub fn shrt_pckt(&self) -> SHRT_PCKT_R {
        SHRT_PCKT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "UDPHS Endpoint Status Register (endpoint = 1)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eptsta1](index.html) module"]
pub struct EPTSTA1_SPEC;
impl crate::RegisterSpec for EPTSTA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eptsta1::R](R) reader structure"]
impl crate::Readable for EPTSTA1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EPTSTA1 to value 0x40"]
impl crate::Resettable for EPTSTA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
