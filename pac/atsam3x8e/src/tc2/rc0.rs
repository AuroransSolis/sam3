#[doc = "Register `RC0` reader"]
pub type R = crate::R<Rc0Spec>;
#[doc = "Register `RC0` writer"]
pub type W = crate::W<Rc0Spec>;
#[doc = "Field `RC` reader - Register C"]
pub type RcR = crate::FieldReader<u32>;
#[doc = "Field `RC` writer - Register C"]
pub type RcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register C"]
    #[inline(always)]
    pub fn rc(&self) -> RcR {
        RcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register C"]
    #[inline(always)]
    #[must_use]
    pub fn rc(&mut self) -> RcW<Rc0Spec> {
        RcW::new(self, 0)
    }
}
#[doc = "Register C (channel = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`rc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rc0Spec;
impl crate::RegisterSpec for Rc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc0::R`](R) reader structure"]
impl crate::Readable for Rc0Spec {}
#[doc = "`write(|w| ..)` method takes [`rc0::W`](W) writer structure"]
impl crate::Writable for Rc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RC0 to value 0"]
impl crate::Resettable for Rc0Spec {
    const RESET_VALUE: u32 = 0;
}
