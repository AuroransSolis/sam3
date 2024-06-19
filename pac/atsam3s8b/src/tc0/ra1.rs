#[doc = "Register `RA1` reader"]
pub type R = crate::R<Ra1Spec>;
#[doc = "Register `RA1` writer"]
pub type W = crate::W<Ra1Spec>;
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
    pub fn ra(&mut self) -> RaW<Ra1Spec> {
        RaW::new(self, 0)
    }
}
#[doc = "Register A (channel = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ra1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ra1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ra1Spec;
impl crate::RegisterSpec for Ra1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ra1::R`](R) reader structure"]
impl crate::Readable for Ra1Spec {}
#[doc = "`write(|w| ..)` method takes [`ra1::W`](W) writer structure"]
impl crate::Writable for Ra1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RA1 to value 0"]
impl crate::Resettable for Ra1Spec {
    const RESET_VALUE: u32 = 0;
}
