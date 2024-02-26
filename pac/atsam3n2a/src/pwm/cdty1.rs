#[doc = "Register `CDTY1` reader"]
pub type R = crate::R<Cdty1Spec>;
#[doc = "Register `CDTY1` writer"]
pub type W = crate::W<Cdty1Spec>;
#[doc = "Field `CDTY` reader - Channel Duty Cycle"]
pub type CdtyR = crate::FieldReader<u32>;
#[doc = "Field `CDTY` writer - Channel Duty Cycle"]
pub type CdtyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel Duty Cycle"]
    #[inline(always)]
    pub fn cdty(&self) -> CdtyR {
        CdtyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel Duty Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn cdty(&mut self) -> CdtyW<Cdty1Spec> {
        CdtyW::new(self, 0)
    }
}
#[doc = "PWM Channel Duty Cycle Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdty1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdty1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cdty1Spec;
impl crate::RegisterSpec for Cdty1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdty1::R`](R) reader structure"]
impl crate::Readable for Cdty1Spec {}
#[doc = "`write(|w| ..)` method takes [`cdty1::W`](W) writer structure"]
impl crate::Writable for Cdty1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CDTY1 to value 0"]
impl crate::Resettable for Cdty1Spec {
    const RESET_VALUE: u32 = 0;
}
