#[doc = "Register `CDTY2` reader"]
pub type R = crate::R<Cdty2Spec>;
#[doc = "Register `CDTY2` writer"]
pub type W = crate::W<Cdty2Spec>;
#[doc = "Field `CDTY` reader - Channel Duty-Cycle"]
pub type CdtyR = crate::FieldReader<u32>;
#[doc = "Field `CDTY` writer - Channel Duty-Cycle"]
pub type CdtyW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Channel Duty-Cycle"]
    #[inline(always)]
    pub fn cdty(&self) -> CdtyR {
        CdtyR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Duty-Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn cdty(&mut self) -> CdtyW<Cdty2Spec> {
        CdtyW::new(self, 0)
    }
}
#[doc = "PWM Channel Duty Cycle Register (ch_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdty2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdty2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cdty2Spec;
impl crate::RegisterSpec for Cdty2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdty2::R`](R) reader structure"]
impl crate::Readable for Cdty2Spec {}
#[doc = "`write(|w| ..)` method takes [`cdty2::W`](W) writer structure"]
impl crate::Writable for Cdty2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CDTY2 to value 0"]
impl crate::Resettable for Cdty2Spec {
    const RESET_VALUE: u32 = 0;
}
