#[doc = "Register `RA2` reader"]
pub type R = crate::R<Ra2Spec>;
#[doc = "Register `RA2` writer"]
pub type W = crate::W<Ra2Spec>;
#[doc = "Field `RA` reader - Register A"]
pub type RaR = crate::FieldReader<u32>;
#[doc = "Field `RA` writer - Register A"]
pub type RaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register A"]
    #[inline(always)]
    pub fn ra(&self) -> RaR {
        RaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register A"]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RaW<Ra2Spec> {
        RaW::new(self, 0)
    }
}
#[doc = "Register A (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ra2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ra2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ra2Spec;
impl crate::RegisterSpec for Ra2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ra2::R`](R) reader structure"]
impl crate::Readable for Ra2Spec {}
#[doc = "`write(|w| ..)` method takes [`ra2::W`](W) writer structure"]
impl crate::Writable for Ra2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RA2 to value 0"]
impl crate::Resettable for Ra2Spec {
    const RESET_VALUE: u32 = 0;
}
