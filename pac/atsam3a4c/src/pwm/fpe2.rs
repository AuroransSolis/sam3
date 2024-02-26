#[doc = "Register `FPE2` reader"]
pub type R = crate::R<Fpe2Spec>;
#[doc = "Register `FPE2` writer"]
pub type W = crate::W<Fpe2Spec>;
#[doc = "Field `FPE4` reader - Fault Protection Enable for channel 4 (fault input bit varies from 0 to 5)"]
pub type Fpe4R = crate::FieldReader;
#[doc = "Field `FPE4` writer - Fault Protection Enable for channel 4 (fault input bit varies from 0 to 5)"]
pub type Fpe4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FPE5` reader - Fault Protection Enable for channel 5 (fault input bit varies from 0 to 5)"]
pub type Fpe5R = crate::FieldReader;
#[doc = "Field `FPE5` writer - Fault Protection Enable for channel 5 (fault input bit varies from 0 to 5)"]
pub type Fpe5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FPE6` reader - Fault Protection Enable for channel 6 (fault input bit varies from 0 to 5)"]
pub type Fpe6R = crate::FieldReader;
#[doc = "Field `FPE6` writer - Fault Protection Enable for channel 6 (fault input bit varies from 0 to 5)"]
pub type Fpe6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FPE7` reader - Fault Protection Enable for channel 7 (fault input bit varies from 0 to 5)"]
pub type Fpe7R = crate::FieldReader;
#[doc = "Field `FPE7` writer - Fault Protection Enable for channel 7 (fault input bit varies from 0 to 5)"]
pub type Fpe7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Fault Protection Enable for channel 4 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe4(&self) -> Fpe4R {
        Fpe4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fault Protection Enable for channel 5 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe5(&self) -> Fpe5R {
        Fpe5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Fault Protection Enable for channel 6 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe6(&self) -> Fpe6R {
        Fpe6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Fault Protection Enable for channel 7 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe7(&self) -> Fpe7R {
        Fpe7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fault Protection Enable for channel 4 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    #[must_use]
    pub fn fpe4(&mut self) -> Fpe4W<Fpe2Spec> {
        Fpe4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Fault Protection Enable for channel 5 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    #[must_use]
    pub fn fpe5(&mut self) -> Fpe5W<Fpe2Spec> {
        Fpe5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Fault Protection Enable for channel 6 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    #[must_use]
    pub fn fpe6(&mut self) -> Fpe6W<Fpe2Spec> {
        Fpe6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Fault Protection Enable for channel 7 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    #[must_use]
    pub fn fpe7(&mut self) -> Fpe7W<Fpe2Spec> {
        Fpe7W::new(self, 24)
    }
}
#[doc = "PWM Fault Protection Enable Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpe2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpe2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fpe2Spec;
impl crate::RegisterSpec for Fpe2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpe2::R`](R) reader structure"]
impl crate::Readable for Fpe2Spec {}
#[doc = "`write(|w| ..)` method takes [`fpe2::W`](W) writer structure"]
impl crate::Writable for Fpe2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPE2 to value 0"]
impl crate::Resettable for Fpe2Spec {
    const RESET_VALUE: u32 = 0;
}
