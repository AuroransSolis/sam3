#[doc = "Register `RB0` reader"]
pub type R = crate::R<Rb0Spec>;
#[doc = "Register `RB0` writer"]
pub type W = crate::W<Rb0Spec>;
#[doc = "Field `RB` reader - Register B"]
pub type RbR = crate::FieldReader<u32>;
#[doc = "Field `RB` writer - Register B"]
pub type RbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register B"]
    #[inline(always)]
    pub fn rb(&self) -> RbR {
        RbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register B"]
    #[inline(always)]
    #[must_use]
    pub fn rb(&mut self) -> RbW<Rb0Spec> {
        RbW::new(self, 0)
    }
}
#[doc = "Register B (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rb0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rb0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rb0Spec;
impl crate::RegisterSpec for Rb0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rb0::R`](R) reader structure"]
impl crate::Readable for Rb0Spec {}
#[doc = "`write(|w| ..)` method takes [`rb0::W`](W) writer structure"]
impl crate::Writable for Rb0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RB0 to value 0"]
impl crate::Resettable for Rb0Spec {
    const RESET_VALUE: u32 = 0;
}
