#[doc = "Register `RB0` reader"]
pub struct R(crate::R<RB0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RB0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RB0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RB0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RB0` writer"]
pub struct W(crate::W<RB0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RB0_SPEC>;
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
impl From<crate::W<RB0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RB0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB` reader - Register B"]
pub type RB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RB` writer - Register B"]
pub type RB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RB0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Register B"]
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register B"]
    #[inline(always)]
    pub fn rb(&mut self) -> RB_W<0> {
        RB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register B (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rb0](index.html) module"]
pub struct RB0_SPEC;
impl crate::RegisterSpec for RB0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rb0::R](R) reader structure"]
impl crate::Readable for RB0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rb0::W](W) writer structure"]
impl crate::Writable for RB0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RB0 to value 0"]
impl crate::Resettable for RB0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
