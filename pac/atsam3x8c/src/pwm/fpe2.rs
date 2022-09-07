#[doc = "Register `FPE2` reader"]
pub struct R(crate::R<FPE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPE2` writer"]
pub struct W(crate::W<FPE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FPE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPE4` reader - Fault Protection Enable for channel 4 (fault input bit varies from 0 to 5)"]
pub type FPE4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FPE4` writer - Fault Protection Enable for channel 4 (fault input bit varies from 0 to 5)"]
pub type FPE4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPE2_SPEC, u8, u8, 8, O>;
#[doc = "Field `FPE5` reader - Fault Protection Enable for channel 5 (fault input bit varies from 0 to 5)"]
pub type FPE5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FPE5` writer - Fault Protection Enable for channel 5 (fault input bit varies from 0 to 5)"]
pub type FPE5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPE2_SPEC, u8, u8, 8, O>;
#[doc = "Field `FPE6` reader - Fault Protection Enable for channel 6 (fault input bit varies from 0 to 5)"]
pub type FPE6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FPE6` writer - Fault Protection Enable for channel 6 (fault input bit varies from 0 to 5)"]
pub type FPE6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPE2_SPEC, u8, u8, 8, O>;
#[doc = "Field `FPE7` reader - Fault Protection Enable for channel 7 (fault input bit varies from 0 to 5)"]
pub type FPE7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FPE7` writer - Fault Protection Enable for channel 7 (fault input bit varies from 0 to 5)"]
pub type FPE7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPE2_SPEC, u8, u8, 8, O>;
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
    pub fn fpe4(&mut self) -> FPE4_W<0> {
        FPE4_W::new(self)
    }
    #[doc = "Bits 8:15 - Fault Protection Enable for channel 5 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe5(&mut self) -> FPE5_W<8> {
        FPE5_W::new(self)
    }
    #[doc = "Bits 16:23 - Fault Protection Enable for channel 6 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe6(&mut self) -> FPE6_W<16> {
        FPE6_W::new(self)
    }
    #[doc = "Bits 24:31 - Fault Protection Enable for channel 7 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe7(&mut self) -> FPE7_W<24> {
        FPE7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Fault Protection Enable Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpe2](index.html) module"]
pub struct FPE2_SPEC;
impl crate::RegisterSpec for FPE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpe2::R](R) reader structure"]
impl crate::Readable for FPE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpe2::W](W) writer structure"]
impl crate::Writable for FPE2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FPE2 to value 0"]
impl crate::Resettable for FPE2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
