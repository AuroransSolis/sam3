#[doc = "Register `CCNT1` reader"]
pub type R = crate::R<Ccnt1Spec>;
#[doc = "Field `CNT` reader - Channel Counter Register"]
pub type CntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Channel Counter Register"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits)
    }
}
#[doc = "PWM Channel Counter Register (ch_num = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccnt1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccnt1Spec;
impl crate::RegisterSpec for Ccnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccnt1::R`](R) reader structure"]
impl crate::Readable for Ccnt1Spec {}
#[doc = "`reset()` method sets CCNT1 to value 0"]
impl crate::Resettable for Ccnt1Spec {
    const RESET_VALUE: u32 = 0;
}
