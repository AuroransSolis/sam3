#[doc = "Register `RB2` reader"]
pub type R = crate::R<Rb2Spec>;
#[doc = "Register `RB2` writer"]
pub type W = crate::W<Rb2Spec>;
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
    pub fn rb(&mut self) -> RbW<Rb2Spec> {
        RbW::new(self, 0)
    }
}
#[doc = "Register B (channel = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`rb2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rb2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rb2Spec;
impl crate::RegisterSpec for Rb2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rb2::R`](R) reader structure"]
impl crate::Readable for Rb2Spec {}
#[doc = "`write(|w| ..)` method takes [`rb2::W`](W) writer structure"]
impl crate::Writable for Rb2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RB2 to value 0"]
impl crate::Resettable for Rb2Spec {
    const RESET_VALUE: u32 = 0;
}
