#[doc = "Register `ECC_SR1` reader"]
pub struct R(crate::R<ECC_SR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC_SR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC_SR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC_SR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RECERR0` reader - Recoverable Error"]
pub struct RECERR0_R(crate::FieldReader<bool, bool>);
impl RECERR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECERR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECERR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCERR0` reader - ECC Error"]
pub struct ECCERR0_R(crate::FieldReader<bool, bool>);
impl ECCERR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECCERR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCERR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULERR0` reader - Multiple Error"]
pub struct MULERR0_R(crate::FieldReader<bool, bool>);
impl MULERR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MULERR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULERR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECERR1` reader - Recoverable Error in the page between the 256th and the 511th bytes or the 512nd and the 1023rd bytes"]
pub struct RECERR1_R(crate::FieldReader<bool, bool>);
impl RECERR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECERR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECERR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCERR1` reader - ECC Error in the page between the 256th and the 511th bytes or between the 512nd and the 1023rd bytes"]
pub struct ECCERR1_R(crate::FieldReader<bool, bool>);
impl ECCERR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECCERR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCERR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULERR1` reader - Multiple Error in the page between the 256th and the 511th bytes or between the 512nd and the 1023rd bytes"]
pub struct MULERR1_R(crate::FieldReader<bool, bool>);
impl MULERR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MULERR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULERR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECERR2` reader - Recoverable Error in the page between the 512nd and the 767th bytes or between the 1024th and the 1535th bytes"]
pub struct RECERR2_R(crate::FieldReader<bool, bool>);
impl RECERR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECERR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECERR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCERR2` reader - ECC Error in the page between the 512nd and the 767th bytes or between the 1024th and the 1535th bytes"]
pub struct ECCERR2_R(crate::FieldReader<bool, bool>);
impl ECCERR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECCERR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCERR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULERR2` reader - Multiple Error in the page between the 512nd and the 767th bytes or between the 1024th and the 1535th bytes"]
pub struct MULERR2_R(crate::FieldReader<bool, bool>);
impl MULERR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MULERR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULERR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECERR3` reader - Recoverable Error in the page between the 768th and the 1023rd bytes or between the 1536th and the 2047th bytes"]
pub struct RECERR3_R(crate::FieldReader<bool, bool>);
impl RECERR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECERR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECERR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCERR3` reader - ECC Error in the page between the 768th and the 1023rd bytes or between the 1536th and the 2047th bytes"]
pub struct ECCERR3_R(crate::FieldReader<bool, bool>);
impl ECCERR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECCERR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCERR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULERR3` reader - Multiple Error in the page between the 768th and the 1023rd bytes or between the 1536th and the 2047th bytes"]
pub struct MULERR3_R(crate::FieldReader<bool, bool>);
impl MULERR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MULERR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULERR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECERR4` reader - Recoverable Error in the page between the 1024th and the 1279th bytes or between the 2048th and the 2559th bytes"]
pub struct RECERR4_R(crate::FieldReader<bool, bool>);
impl RECERR4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECERR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECERR4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCERR4` reader - ECC Error in the page between the 1024th and the 1279th bytes or between the 2048th and the 2559th bytes"]
pub struct ECCERR4_R(crate::FieldReader<bool, bool>);
impl ECCERR4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECCERR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCERR4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULERR4` reader - Multiple Error in the page between the 1024th and the 1279th bytes or between the 2048th and the 2559th bytes"]
pub struct MULERR4_R(crate::FieldReader<bool, bool>);
impl MULERR4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MULERR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULERR4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECERR5` reader - Recoverable Error in the page between the 1280th and the 1535th bytes or between the 2560th and the 3071st bytes"]
pub struct RECERR5_R(crate::FieldReader<bool, bool>);
impl RECERR5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECERR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECERR5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCERR5` reader - ECC Error in the page between the 1280th and the 1535th bytes or between the 2560th and the 3071st bytes"]
pub struct ECCERR5_R(crate::FieldReader<bool, bool>);
impl ECCERR5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECCERR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCERR5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULERR5` reader - Multiple Error in the page between the 1280th and the 1535th bytes or between the 2560th and the 3071st bytes"]
pub struct MULERR5_R(crate::FieldReader<bool, bool>);
impl MULERR5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MULERR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULERR5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECERR6` reader - Recoverable Error in the page between the 1536th and the 1791st bytes or between the 3072nd and the 3583rd bytes"]
pub struct RECERR6_R(crate::FieldReader<bool, bool>);
impl RECERR6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECERR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECERR6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCERR6` reader - ECC Error in the page between the 1536th and the 1791st bytes or between the 3072nd and the 3583rd bytes"]
pub struct ECCERR6_R(crate::FieldReader<bool, bool>);
impl ECCERR6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECCERR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCERR6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULERR6` reader - Multiple Error in the page between the 1536th and the 1791st bytes or between the 3072nd and the 3583rd bytes"]
pub struct MULERR6_R(crate::FieldReader<bool, bool>);
impl MULERR6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MULERR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULERR6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECERR7` reader - Recoverable Error in the page between the 1792nd and the 2047th bytes or between the 3584th and the 4095th bytes"]
pub struct RECERR7_R(crate::FieldReader<bool, bool>);
impl RECERR7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECERR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECERR7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCERR7` reader - ECC Error in the page between the 1792nd and the 2047th bytes or between the 3584th and the 4095th bytes"]
pub struct ECCERR7_R(crate::FieldReader<bool, bool>);
impl ECCERR7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECCERR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCERR7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULERR7` reader - Multiple Error in the page between the 1792nd and the 2047th bytes or between the 3584th and the 4095th bytes"]
pub struct MULERR7_R(crate::FieldReader<bool, bool>);
impl MULERR7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MULERR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULERR7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Recoverable Error"]
    #[inline(always)]
    pub fn recerr0(&self) -> RECERR0_R {
        RECERR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ECC Error"]
    #[inline(always)]
    pub fn eccerr0(&self) -> ECCERR0_R {
        ECCERR0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Multiple Error"]
    #[inline(always)]
    pub fn mulerr0(&self) -> MULERR0_R {
        MULERR0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Recoverable Error in the page between the 256th and the 511th bytes or the 512nd and the 1023rd bytes"]
    #[inline(always)]
    pub fn recerr1(&self) -> RECERR1_R {
        RECERR1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ECC Error in the page between the 256th and the 511th bytes or between the 512nd and the 1023rd bytes"]
    #[inline(always)]
    pub fn eccerr1(&self) -> ECCERR1_R {
        ECCERR1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Multiple Error in the page between the 256th and the 511th bytes or between the 512nd and the 1023rd bytes"]
    #[inline(always)]
    pub fn mulerr1(&self) -> MULERR1_R {
        MULERR1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Recoverable Error in the page between the 512nd and the 767th bytes or between the 1024th and the 1535th bytes"]
    #[inline(always)]
    pub fn recerr2(&self) -> RECERR2_R {
        RECERR2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ECC Error in the page between the 512nd and the 767th bytes or between the 1024th and the 1535th bytes"]
    #[inline(always)]
    pub fn eccerr2(&self) -> ECCERR2_R {
        ECCERR2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Multiple Error in the page between the 512nd and the 767th bytes or between the 1024th and the 1535th bytes"]
    #[inline(always)]
    pub fn mulerr2(&self) -> MULERR2_R {
        MULERR2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Recoverable Error in the page between the 768th and the 1023rd bytes or between the 1536th and the 2047th bytes"]
    #[inline(always)]
    pub fn recerr3(&self) -> RECERR3_R {
        RECERR3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ECC Error in the page between the 768th and the 1023rd bytes or between the 1536th and the 2047th bytes"]
    #[inline(always)]
    pub fn eccerr3(&self) -> ECCERR3_R {
        ECCERR3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Multiple Error in the page between the 768th and the 1023rd bytes or between the 1536th and the 2047th bytes"]
    #[inline(always)]
    pub fn mulerr3(&self) -> MULERR3_R {
        MULERR3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Recoverable Error in the page between the 1024th and the 1279th bytes or between the 2048th and the 2559th bytes"]
    #[inline(always)]
    pub fn recerr4(&self) -> RECERR4_R {
        RECERR4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ECC Error in the page between the 1024th and the 1279th bytes or between the 2048th and the 2559th bytes"]
    #[inline(always)]
    pub fn eccerr4(&self) -> ECCERR4_R {
        ECCERR4_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Multiple Error in the page between the 1024th and the 1279th bytes or between the 2048th and the 2559th bytes"]
    #[inline(always)]
    pub fn mulerr4(&self) -> MULERR4_R {
        MULERR4_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Recoverable Error in the page between the 1280th and the 1535th bytes or between the 2560th and the 3071st bytes"]
    #[inline(always)]
    pub fn recerr5(&self) -> RECERR5_R {
        RECERR5_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ECC Error in the page between the 1280th and the 1535th bytes or between the 2560th and the 3071st bytes"]
    #[inline(always)]
    pub fn eccerr5(&self) -> ECCERR5_R {
        ECCERR5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Multiple Error in the page between the 1280th and the 1535th bytes or between the 2560th and the 3071st bytes"]
    #[inline(always)]
    pub fn mulerr5(&self) -> MULERR5_R {
        MULERR5_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Recoverable Error in the page between the 1536th and the 1791st bytes or between the 3072nd and the 3583rd bytes"]
    #[inline(always)]
    pub fn recerr6(&self) -> RECERR6_R {
        RECERR6_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - ECC Error in the page between the 1536th and the 1791st bytes or between the 3072nd and the 3583rd bytes"]
    #[inline(always)]
    pub fn eccerr6(&self) -> ECCERR6_R {
        ECCERR6_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Multiple Error in the page between the 1536th and the 1791st bytes or between the 3072nd and the 3583rd bytes"]
    #[inline(always)]
    pub fn mulerr6(&self) -> MULERR6_R {
        MULERR6_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Recoverable Error in the page between the 1792nd and the 2047th bytes or between the 3584th and the 4095th bytes"]
    #[inline(always)]
    pub fn recerr7(&self) -> RECERR7_R {
        RECERR7_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ECC Error in the page between the 1792nd and the 2047th bytes or between the 3584th and the 4095th bytes"]
    #[inline(always)]
    pub fn eccerr7(&self) -> ECCERR7_R {
        ECCERR7_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Multiple Error in the page between the 1792nd and the 2047th bytes or between the 3584th and the 4095th bytes"]
    #[inline(always)]
    pub fn mulerr7(&self) -> MULERR7_R {
        MULERR7_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
#[doc = "SMC ECC Status 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_sr1](index.html) module"]
pub struct ECC_SR1_SPEC;
impl crate::RegisterSpec for ECC_SR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecc_sr1::R](R) reader structure"]
impl crate::Readable for ECC_SR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ECC_SR1 to value 0"]
impl crate::Resettable for ECC_SR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
