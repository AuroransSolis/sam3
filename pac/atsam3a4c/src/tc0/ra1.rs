#[doc = "Register `RA1` reader"]
pub struct R(crate::R<RA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RA1` writer"]
pub struct W(crate::W<RA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RA1_SPEC>;
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
impl From<crate::W<RA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RA` reader - Register A"]
pub struct RA_R(crate::FieldReader<u32, u32>);
impl RA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RA` writer - Register A"]
pub struct RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Register A"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register A"]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W {
        RA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register A (channel = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ra1](index.html) module"]
pub struct RA1_SPEC;
impl crate::RegisterSpec for RA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ra1::R](R) reader structure"]
impl crate::Readable for RA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ra1::W](W) writer structure"]
impl crate::Writable for RA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RA1 to value 0"]
impl crate::Resettable for RA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
