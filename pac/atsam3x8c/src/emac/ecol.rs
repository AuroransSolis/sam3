#[doc = "Register `ECOL` reader"]
pub struct R(crate::R<ECOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECOL` writer"]
pub struct W(crate::W<ECOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECOL_SPEC>;
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
impl From<crate::W<ECOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECOL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXCOL` reader - Excessive Collisions"]
pub struct EXCOL_R(crate::FieldReader<u8, u8>);
impl EXCOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXCOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXCOL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXCOL` writer - Excessive Collisions"]
pub struct EXCOL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXCOL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Excessive Collisions"]
    #[inline(always)]
    pub fn excol(&self) -> EXCOL_R {
        EXCOL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Excessive Collisions"]
    #[inline(always)]
    pub fn excol(&mut self) -> EXCOL_W {
        EXCOL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Excessive Collisions Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecol](index.html) module"]
pub struct ECOL_SPEC;
impl crate::RegisterSpec for ECOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecol::R](R) reader structure"]
impl crate::Readable for ECOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecol::W](W) writer structure"]
impl crate::Writable for ECOL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECOL to value 0"]
impl crate::Resettable for ECOL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
