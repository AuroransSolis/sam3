#[doc = "Register `EBCISR` reader"]
pub struct R(crate::R<EBCISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EBCISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EBCISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EBCISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BTC0` reader - Buffer Transfer Completed \\[5:0\\]"]
pub struct BTC0_R(crate::FieldReader<bool, bool>);
impl BTC0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BTC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTC1` reader - Buffer Transfer Completed \\[5:0\\]"]
pub struct BTC1_R(crate::FieldReader<bool, bool>);
impl BTC1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BTC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTC2` reader - Buffer Transfer Completed \\[5:0\\]"]
pub struct BTC2_R(crate::FieldReader<bool, bool>);
impl BTC2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BTC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTC3` reader - Buffer Transfer Completed \\[5:0\\]"]
pub struct BTC3_R(crate::FieldReader<bool, bool>);
impl BTC3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BTC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTC4` reader - Buffer Transfer Completed \\[5:0\\]"]
pub struct BTC4_R(crate::FieldReader<bool, bool>);
impl BTC4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BTC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTC5` reader - Buffer Transfer Completed \\[5:0\\]"]
pub struct BTC5_R(crate::FieldReader<bool, bool>);
impl BTC5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BTC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTC5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBTC0` reader - Chained Buffer Transfer Completed \\[5:0\\]"]
pub struct CBTC0_R(crate::FieldReader<bool, bool>);
impl CBTC0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBTC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBTC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBTC1` reader - Chained Buffer Transfer Completed \\[5:0\\]"]
pub struct CBTC1_R(crate::FieldReader<bool, bool>);
impl CBTC1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBTC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBTC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBTC2` reader - Chained Buffer Transfer Completed \\[5:0\\]"]
pub struct CBTC2_R(crate::FieldReader<bool, bool>);
impl CBTC2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBTC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBTC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBTC3` reader - Chained Buffer Transfer Completed \\[5:0\\]"]
pub struct CBTC3_R(crate::FieldReader<bool, bool>);
impl CBTC3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBTC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBTC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBTC4` reader - Chained Buffer Transfer Completed \\[5:0\\]"]
pub struct CBTC4_R(crate::FieldReader<bool, bool>);
impl CBTC4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBTC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBTC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBTC5` reader - Chained Buffer Transfer Completed \\[5:0\\]"]
pub struct CBTC5_R(crate::FieldReader<bool, bool>);
impl CBTC5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBTC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBTC5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR0` reader - Access Error \\[5:0\\]"]
pub struct ERR0_R(crate::FieldReader<bool, bool>);
impl ERR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR1` reader - Access Error \\[5:0\\]"]
pub struct ERR1_R(crate::FieldReader<bool, bool>);
impl ERR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR2` reader - Access Error \\[5:0\\]"]
pub struct ERR2_R(crate::FieldReader<bool, bool>);
impl ERR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR3` reader - Access Error \\[5:0\\]"]
pub struct ERR3_R(crate::FieldReader<bool, bool>);
impl ERR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR4` reader - Access Error \\[5:0\\]"]
pub struct ERR4_R(crate::FieldReader<bool, bool>);
impl ERR4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR5` reader - Access Error \\[5:0\\]"]
pub struct ERR5_R(crate::FieldReader<bool, bool>);
impl ERR5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc0(&self) -> BTC0_R {
        BTC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc1(&self) -> BTC1_R {
        BTC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc2(&self) -> BTC2_R {
        BTC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc3(&self) -> BTC3_R {
        BTC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc4(&self) -> BTC4_R {
        BTC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc5(&self) -> BTC5_R {
        BTC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc0(&self) -> CBTC0_R {
        CBTC0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc1(&self) -> CBTC1_R {
        CBTC1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc2(&self) -> CBTC2_R {
        CBTC2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc3(&self) -> CBTC3_R {
        CBTC3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc4(&self) -> CBTC4_R {
        CBTC4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc5(&self) -> CBTC5_R {
        CBTC5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err0(&self) -> ERR0_R {
        ERR0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err1(&self) -> ERR1_R {
        ERR1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err2(&self) -> ERR2_R {
        ERR2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err3(&self) -> ERR3_R {
        ERR3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err4(&self) -> ERR4_R {
        ERR4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err5(&self) -> ERR5_R {
        ERR5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebcisr](index.html) module"]
pub struct EBCISR_SPEC;
impl crate::RegisterSpec for EBCISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ebcisr::R](R) reader structure"]
impl crate::Readable for EBCISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EBCISR to value 0"]
impl crate::Resettable for EBCISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
