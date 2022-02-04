#[doc = "Register `USF` reader"]
pub struct R(crate::R<USF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USF` writer"]
pub struct W(crate::W<USF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USF_SPEC>;
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
impl From<crate::W<USF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USF` reader - Undersize frames"]
pub struct USF_R(crate::FieldReader<u8, u8>);
impl USF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USF` writer - Undersize frames"]
pub struct USF_W<'a> {
    w: &'a mut W,
}
impl<'a> USF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Undersize frames"]
    #[inline(always)]
    pub fn usf(&self) -> USF_R {
        USF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Undersize frames"]
    #[inline(always)]
    pub fn usf(&mut self) -> USF_W {
        USF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Undersize Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usf](index.html) module"]
pub struct USF_SPEC;
impl crate::RegisterSpec for USF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usf::R](R) reader structure"]
impl crate::Readable for USF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usf::W](W) writer structure"]
impl crate::Writable for USF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USF to value 0"]
impl crate::Resettable for USF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
