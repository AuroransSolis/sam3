#[doc = "Register `MFID1` reader"]
pub type R = crate::R<Mfid1Spec>;
#[doc = "Field `MFID` reader - Family ID"]
pub type MfidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:28 - Family ID"]
    #[inline(always)]
    pub fn mfid(&self) -> MfidR {
        MfidR::new(self.bits & 0x1fff_ffff)
    }
}
#[doc = "Mailbox Family ID Register (MB = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`mfid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mfid1Spec;
impl crate::RegisterSpec for Mfid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mfid1::R`](R) reader structure"]
impl crate::Readable for Mfid1Spec {}
#[doc = "`reset()` method sets MFID1 to value 0"]
impl crate::Resettable for Mfid1Spec {
    const RESET_VALUE: u32 = 0;
}
