#[doc = "Register `MFID3` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<MFID3_SPEC>);
#[doc = "Field `MFID` reader - Family ID"]
pub type MFID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:28 - Family ID"]
    #[inline(always)]
    pub fn mfid(&self) -> MFID_R {
        MFID_R::new(self.bits & 0x1fff_ffff)
    }
}
#[doc = "Mailbox Family ID Register (MB = 3)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mfid3](index.html) module"]
pub struct MFID3_SPEC;
impl crate::RegisterSpec for MFID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mfid3::R](R) reader structure"]
impl crate::Readable for MFID3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MFID3 to value 0"]
impl crate::Resettable for MFID3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
