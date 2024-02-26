#[doc = "Register `CCNT2` reader"]
pub type R = crate::R<Ccnt2Spec>;
#[doc = "Field `CNT` reader - Channel Counter Register"]
pub type CntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Channel Counter Register"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits)
    }
}
#[doc = "PWM Channel Counter Register (ch_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccnt2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccnt2Spec;
impl crate::RegisterSpec for Ccnt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccnt2::R`](R) reader structure"]
impl crate::Readable for Ccnt2Spec {}
#[doc = "`reset()` method sets CCNT2 to value 0"]
impl crate::Resettable for Ccnt2Spec {
    const RESET_VALUE: u32 = 0;
}
