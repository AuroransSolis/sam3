#[doc = "Register `LCOL` reader"]
pub struct R(crate::R<LCOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCOL` writer"]
pub struct W(crate::W<LCOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCOL_SPEC>;
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
impl From<crate::W<LCOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCOL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCOL` reader - Late Collisions"]
pub struct LCOL_R(crate::FieldReader<u8, u8>);
impl LCOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LCOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCOL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCOL` writer - Late Collisions"]
pub struct LCOL_W<'a> {
    w: &'a mut W,
}
impl<'a> LCOL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Late Collisions"]
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Late Collisions"]
    #[inline(always)]
    pub fn lcol(&mut self) -> LCOL_W {
        LCOL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Late Collisions Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcol](index.html) module"]
pub struct LCOL_SPEC;
impl crate::RegisterSpec for LCOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcol::R](R) reader structure"]
impl crate::Readable for LCOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcol::W](W) writer structure"]
impl crate::Writable for LCOL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCOL to value 0"]
impl crate::Resettable for LCOL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
