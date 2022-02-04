#[doc = "Register `MFID6` reader"]
pub struct R(crate::R<MFID6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MFID6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MFID6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MFID6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MFID` reader - Family ID"]
pub struct MFID_R(crate::FieldReader<u32, u32>);
impl MFID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MFID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MFID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:28 - Family ID"]
    #[inline(always)]
    pub fn mfid(&self) -> MFID_R {
        MFID_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
#[doc = "Mailbox Family ID Register (MB = 6)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mfid6](index.html) module"]
pub struct MFID6_SPEC;
impl crate::RegisterSpec for MFID6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mfid6::R](R) reader structure"]
impl crate::Readable for MFID6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MFID6 to value 0"]
impl crate::Resettable for MFID6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
