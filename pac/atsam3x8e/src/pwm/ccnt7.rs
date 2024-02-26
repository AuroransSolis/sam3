#[doc = "Register `CCNT7` reader"]
pub type R = crate::R<Ccnt7Spec>;
#[doc = "Field `CNT` reader - Channel Counter Register"]
pub type CntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Channel Counter Register"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PWM Channel Counter Register (ch_num = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccnt7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccnt7Spec;
impl crate::RegisterSpec for Ccnt7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccnt7::R`](R) reader structure"]
impl crate::Readable for Ccnt7Spec {}
#[doc = "`reset()` method sets CCNT7 to value 0"]
impl crate::Resettable for Ccnt7Spec {
    const RESET_VALUE: u32 = 0;
}
