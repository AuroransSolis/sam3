#[doc = "Register `CDTY3` reader"]
pub type R = crate::R<Cdty3Spec>;
#[doc = "Register `CDTY3` writer"]
pub type W = crate::W<Cdty3Spec>;
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
    pub fn cdty(&mut self) -> CdtyW<Cdty3Spec> {
        CdtyW::new(self, 0)
    }
}
#[doc = "PWM Channel Duty Cycle Register (ch_num = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`cdty3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdty3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cdty3Spec;
impl crate::RegisterSpec for Cdty3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdty3::R`](R) reader structure"]
impl crate::Readable for Cdty3Spec {}
#[doc = "`write(|w| ..)` method takes [`cdty3::W`](W) writer structure"]
impl crate::Writable for Cdty3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CDTY3 to value 0"]
impl crate::Resettable for Cdty3Spec {
    const RESET_VALUE: u32 = 0;
}
