#[doc = "Register `MFID4` reader"]
pub type R = crate::R<MFID4_SPEC>;
#[doc = "Field `MFID` reader - Family ID"]
pub type MFID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:28 - Family ID"]
    #[inline(always)]
    pub fn mfid(&self) -> MFID_R {
        MFID_R::new(self.bits & 0x1fff_ffff)
    }
}
#[doc = "Mailbox Family ID Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MFID4_SPEC;
impl crate::RegisterSpec for MFID4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mfid4::R`](R) reader structure"]
impl crate::Readable for MFID4_SPEC {}
#[doc = "`reset()` method sets MFID4 to value 0"]
impl crate::Resettable for MFID4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
