#[doc = "Register `MFID4` reader"]
pub struct R(crate::R<MFID4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MFID4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MFID4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MFID4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MFID` reader - Family ID"]
pub type MFID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:28 - Family ID"]
    #[inline(always)]
    pub fn mfid(&self) -> MFID_R {
        MFID_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
#[doc = "Mailbox Family ID Register (MB = 4)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mfid4](index.html) module"]
pub struct MFID4_SPEC;
impl crate::RegisterSpec for MFID4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mfid4::R](R) reader structure"]
impl crate::Readable for MFID4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MFID4 to value 0"]
impl crate::Resettable for MFID4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
