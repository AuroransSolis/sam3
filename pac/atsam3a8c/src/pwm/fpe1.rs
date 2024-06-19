#[doc = "Register `FPE1` reader"]
pub type R = crate::R<Fpe1Spec>;
#[doc = "Register `FPE1` writer"]
pub type W = crate::W<Fpe1Spec>;
#[doc = "Field `FPE0` reader - Fault Protection Enable for channel 0 (fault input bit varies from 0 to 5)"]
pub type Fpe0R = crate::FieldReader;
#[doc = "Field `FPE0` writer - Fault Protection Enable for channel 0 (fault input bit varies from 0 to 5)"]
pub type Fpe0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FPE1` reader - Fault Protection Enable for channel 1 (fault input bit varies from 0 to 5)"]
pub type Fpe1R = crate::FieldReader;
#[doc = "Field `FPE1` writer - Fault Protection Enable for channel 1 (fault input bit varies from 0 to 5)"]
pub type Fpe1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FPE2` reader - Fault Protection Enable for channel 2 (fault input bit varies from 0 to 5)"]
pub type Fpe2R = crate::FieldReader;
#[doc = "Field `FPE2` writer - Fault Protection Enable for channel 2 (fault input bit varies from 0 to 5)"]
pub type Fpe2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FPE3` reader - Fault Protection Enable for channel 3 (fault input bit varies from 0 to 5)"]
pub type Fpe3R = crate::FieldReader;
#[doc = "Field `FPE3` writer - Fault Protection Enable for channel 3 (fault input bit varies from 0 to 5)"]
pub type Fpe3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Fault Protection Enable for channel 0 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe0(&self) -> Fpe0R {
        Fpe0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fault Protection Enable for channel 1 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe1(&self) -> Fpe1R {
        Fpe1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Fault Protection Enable for channel 2 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe2(&self) -> Fpe2R {
        Fpe2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Fault Protection Enable for channel 3 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe3(&self) -> Fpe3R {
        Fpe3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fault Protection Enable for channel 0 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    #[must_use]
    pub fn fpe0(&mut self) -> Fpe0W<Fpe1Spec> {
        Fpe0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Fault Protection Enable for channel 1 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    #[must_use]
    pub fn fpe1(&mut self) -> Fpe1W<Fpe1Spec> {
        Fpe1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Fault Protection Enable for channel 2 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    #[must_use]
    pub fn fpe2(&mut self) -> Fpe2W<Fpe1Spec> {
        Fpe2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Fault Protection Enable for channel 3 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    #[must_use]
    pub fn fpe3(&mut self) -> Fpe3W<Fpe1Spec> {
        Fpe3W::new(self, 24)
    }
}
#[doc = "PWM Fault Protection Enable Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fpe1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpe1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fpe1Spec;
impl crate::RegisterSpec for Fpe1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpe1::R`](R) reader structure"]
impl crate::Readable for Fpe1Spec {}
#[doc = "`write(|w| ..)` method takes [`fpe1::W`](W) writer structure"]
impl crate::Writable for Fpe1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPE1 to value 0"]
impl crate::Resettable for Fpe1Spec {
    const RESET_VALUE: u32 = 0;
}
