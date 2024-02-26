#[doc = "Register `MFID3` reader"]
pub type R = crate::R<Mfid3Spec>;
#[doc = "Field `MFID` reader - Family ID"]
pub type MfidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:28 - Family ID"]
    #[inline(always)]
    pub fn mfid(&self) -> MfidR {
        MfidR::new(self.bits & 0x1fff_ffff)
    }
}
#[doc = "Mailbox Family ID Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfid3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mfid3Spec;
impl crate::RegisterSpec for Mfid3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mfid3::R`](R) reader structure"]
impl crate::Readable for Mfid3Spec {}
#[doc = "`reset()` method sets MFID3 to value 0"]
impl crate::Resettable for Mfid3Spec {
    const RESET_VALUE: u32 = 0;
}
