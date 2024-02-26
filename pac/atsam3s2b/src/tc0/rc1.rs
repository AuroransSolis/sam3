#[doc = "Register `RC1` reader"]
pub type R = crate::R<Rc1Spec>;
#[doc = "Register `RC1` writer"]
pub type W = crate::W<Rc1Spec>;
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
    pub fn rc(&mut self) -> RcW<Rc1Spec> {
        RcW::new(self, 0)
    }
}
#[doc = "Register C (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rc1Spec;
impl crate::RegisterSpec for Rc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc1::R`](R) reader structure"]
impl crate::Readable for Rc1Spec {}
#[doc = "`write(|w| ..)` method takes [`rc1::W`](W) writer structure"]
impl crate::Writable for Rc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RC1 to value 0"]
impl crate::Resettable for Rc1Spec {
    const RESET_VALUE: u32 = 0;
}
