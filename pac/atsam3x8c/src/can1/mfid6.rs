#[doc = "Register `MFID6` reader"]
pub type R = crate::R<MFID6_SPEC>;
#[doc = "Field `MFID` reader - Family ID"]
pub type MFID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:28 - Family ID"]
    #[inline(always)]
    pub fn mfid(&self) -> MFID_R {
        MFID_R::new(self.bits & 0x1fff_ffff)
    }
}
#[doc = "Mailbox Family ID Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MFID6_SPEC;
impl crate::RegisterSpec for MFID6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mfid6::R`](R) reader structure"]
impl crate::Readable for MFID6_SPEC {}
#[doc = "`reset()` method sets MFID6 to value 0"]
impl crate::Resettable for MFID6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
