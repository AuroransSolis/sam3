#[doc = "Register `MFID5` reader"]
pub type R = crate::R<MFID5_SPEC>;
#[doc = "Field `MFID` reader - Family ID"]
pub type MFID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:28 - Family ID"]
    #[inline(always)]
    pub fn mfid(&self) -> MFID_R {
        MFID_R::new(self.bits & 0x1fff_ffff)
    }
}
#[doc = "Mailbox Family ID Register (MB = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MFID5_SPEC;
impl crate::RegisterSpec for MFID5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mfid5::R`](R) reader structure"]
impl crate::Readable for MFID5_SPEC {}
#[doc = "`reset()` method sets MFID5 to value 0"]
impl crate::Resettable for MFID5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
