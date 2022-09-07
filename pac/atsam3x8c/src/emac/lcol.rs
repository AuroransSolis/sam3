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
pub type LCOL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LCOL` writer - Late Collisions"]
pub type LCOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCOL_SPEC, u8, u8, 8, O>;
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
    pub fn lcol(&mut self) -> LCOL_W<0> {
        LCOL_W::new(self)
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
