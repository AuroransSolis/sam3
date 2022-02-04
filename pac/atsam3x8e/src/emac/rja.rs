#[doc = "Register `RJA` reader"]
pub struct R(crate::R<RJA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RJA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RJA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RJA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RJA` writer"]
pub struct W(crate::W<RJA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RJA_SPEC>;
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
impl From<crate::W<RJA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RJA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RJB` reader - Receive Jabbers"]
pub struct RJB_R(crate::FieldReader<u8, u8>);
impl RJB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RJB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RJB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RJB` writer - Receive Jabbers"]
pub struct RJB_W<'a> {
    w: &'a mut W,
}
impl<'a> RJB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive Jabbers"]
    #[inline(always)]
    pub fn rjb(&self) -> RJB_R {
        RJB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Jabbers"]
    #[inline(always)]
    pub fn rjb(&mut self) -> RJB_W {
        RJB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Jabbers Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rja](index.html) module"]
pub struct RJA_SPEC;
impl crate::RegisterSpec for RJA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rja::R](R) reader structure"]
impl crate::Readable for RJA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rja::W](W) writer structure"]
impl crate::Writable for RJA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RJA to value 0"]
impl crate::Resettable for RJA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
