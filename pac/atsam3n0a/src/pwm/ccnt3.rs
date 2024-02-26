#[doc = "Register `CCNT3` reader"]
pub type R = crate::R<Ccnt3Spec>;
#[doc = "Field `CNT` reader - Channel Counter Register"]
pub type CntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Channel Counter Register"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits)
    }
}
#[doc = "PWM Channel Counter Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccnt3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccnt3Spec;
impl crate::RegisterSpec for Ccnt3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccnt3::R`](R) reader structure"]
impl crate::Readable for Ccnt3Spec {}
#[doc = "`reset()` method sets CCNT3 to value 0"]
impl crate::Resettable for Ccnt3Spec {
    const RESET_VALUE: u32 = 0;
}
