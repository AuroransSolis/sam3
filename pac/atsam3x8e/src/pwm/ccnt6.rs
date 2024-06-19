#[doc = "Register `CCNT6` reader"]
pub type R = crate::R<Ccnt6Spec>;
#[doc = "Field `CNT` reader - Channel Counter Register"]
pub type CntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Channel Counter Register"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PWM Channel Counter Register (ch_num = 6)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccnt6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccnt6Spec;
impl crate::RegisterSpec for Ccnt6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccnt6::R`](R) reader structure"]
impl crate::Readable for Ccnt6Spec {}
#[doc = "`reset()` method sets CCNT6 to value 0"]
impl crate::Resettable for Ccnt6Spec {
    const RESET_VALUE: u32 = 0;
}
