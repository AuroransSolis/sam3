#[doc = "Register `CCNT5` reader"]
pub type R = crate::R<Ccnt5Spec>;
#[doc = "Field `CNT` reader - Channel Counter Register"]
pub type CntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Channel Counter Register"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PWM Channel Counter Register (ch_num = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccnt5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccnt5Spec;
impl crate::RegisterSpec for Ccnt5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccnt5::R`](R) reader structure"]
impl crate::Readable for Ccnt5Spec {}
#[doc = "`reset()` method sets CCNT5 to value 0"]
impl crate::Resettable for Ccnt5Spec {
    const RESET_VALUE: u32 = 0;
}
