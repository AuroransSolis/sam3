#[doc = "Register `ALE` reader"]
pub struct R(crate::R<ALE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALE` writer"]
pub struct W(crate::W<ALE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALE_SPEC>;
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
impl From<crate::W<ALE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALE` reader - Alignment Errors"]
pub struct ALE_R(crate::FieldReader<u8, u8>);
impl ALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALE` writer - Alignment Errors"]
pub struct ALE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Alignment Errors"]
    #[inline(always)]
    pub fn ale(&self) -> ALE_R {
        ALE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Alignment Errors"]
    #[inline(always)]
    pub fn ale(&mut self) -> ALE_W {
        ALE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alignment Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ale](index.html) module"]
pub struct ALE_SPEC;
impl crate::RegisterSpec for ALE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ale::R](R) reader structure"]
impl crate::Readable for ALE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ale::W](W) writer structure"]
impl crate::Writable for ALE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALE to value 0"]
impl crate::Resettable for ALE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
