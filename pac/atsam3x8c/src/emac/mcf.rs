#[doc = "Register `MCF` reader"]
pub struct R(crate::R<MCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCF` writer"]
pub struct W(crate::W<MCF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCF_SPEC>;
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
impl From<crate::W<MCF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCF` reader - Multicollision Frames"]
pub struct MCF_R(crate::FieldReader<u16, u16>);
impl MCF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCF` writer - Multicollision Frames"]
pub struct MCF_W<'a> {
    w: &'a mut W,
}
impl<'a> MCF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Multicollision Frames"]
    #[inline(always)]
    pub fn mcf(&self) -> MCF_R {
        MCF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Multicollision Frames"]
    #[inline(always)]
    pub fn mcf(&mut self) -> MCF_W {
        MCF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Multiple Collision Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcf](index.html) module"]
pub struct MCF_SPEC;
impl crate::RegisterSpec for MCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcf::R](R) reader structure"]
impl crate::Readable for MCF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcf::W](W) writer structure"]
impl crate::Writable for MCF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCF to value 0"]
impl crate::Resettable for MCF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
