#[doc = "Register `FPE2` reader"]
pub type R = crate::R<FPE2_SPEC>;
#[doc = "Register `FPE2` writer"]
pub type W = crate::W<FPE2_SPEC>;
#[doc = "Field `FPE4` reader - Fault Protection Enable for channel 4 (fault input bit varies from 0 to 5)"]
pub type FPE4_R = crate::FieldReader;
#[doc = "Field `FPE4` writer - Fault Protection Enable for channel 4 (fault input bit varies from 0 to 5)"]
pub type FPE4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `FPE5` reader - Fault Protection Enable for channel 5 (fault input bit varies from 0 to 5)"]
pub type FPE5_R = crate::FieldReader;
#[doc = "Field `FPE5` writer - Fault Protection Enable for channel 5 (fault input bit varies from 0 to 5)"]
pub type FPE5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `FPE6` reader - Fault Protection Enable for channel 6 (fault input bit varies from 0 to 5)"]
pub type FPE6_R = crate::FieldReader;
#[doc = "Field `FPE6` writer - Fault Protection Enable for channel 6 (fault input bit varies from 0 to 5)"]
pub type FPE6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `FPE7` reader - Fault Protection Enable for channel 7 (fault input bit varies from 0 to 5)"]
pub type FPE7_R = crate::FieldReader;
#[doc = "Field `FPE7` writer - Fault Protection Enable for channel 7 (fault input bit varies from 0 to 5)"]
pub type FPE7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Fault Protection Enable for channel 4 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe4(&self) -> FPE4_R {
        FPE4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fault Protection Enable for channel 5 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe5(&self) -> FPE5_R {
        FPE5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Fault Protection Enable for channel 6 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe6(&self) -> FPE6_R {
        FPE6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Fault Protection Enable for channel 7 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe7(&self) -> FPE7_R {
        FPE7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fault Protection Enable for channel 4 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    #[must_use]
    pub fn fpe4(&mut self) -> FPE4_W<FPE2_SPEC, 0> {
        FPE4_W::new(self)
    }
    #[doc = "Bits 8:15 - Fault Protection Enable for channel 5 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    #[must_use]
    pub fn fpe5(&mut self) -> FPE5_W<FPE2_SPEC, 8> {
        FPE5_W::new(self)
    }
    #[doc = "Bits 16:23 - Fault Protection Enable for channel 6 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    #[must_use]
    pub fn fpe6(&mut self) -> FPE6_W<FPE2_SPEC, 16> {
        FPE6_W::new(self)
    }
    #[doc = "Bits 24:31 - Fault Protection Enable for channel 7 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    #[must_use]
    pub fn fpe7(&mut self) -> FPE7_W<FPE2_SPEC, 24> {
        FPE7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM Fault Protection Enable Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpe2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpe2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPE2_SPEC;
impl crate::RegisterSpec for FPE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpe2::R`](R) reader structure"]
impl crate::Readable for FPE2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fpe2::W`](W) writer structure"]
impl crate::Writable for FPE2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPE2 to value 0"]
impl crate::Resettable for FPE2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
