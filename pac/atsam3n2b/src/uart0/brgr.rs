#[doc = "Register `BRGR` reader"]
pub struct R(crate::R<BRGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRGR` writer"]
pub struct W(crate::W<BRGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRGR_SPEC>;
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
impl From<crate::W<BRGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CD` reader - Clock Divisor"]
pub struct CD_R(crate::FieldReader<u16, u16>);
impl CD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CD` writer - Clock Divisor"]
pub struct CD_W<'a> {
    w: &'a mut W,
}
impl<'a> CD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Clock Divisor"]
    #[inline(always)]
    pub fn cd(&self) -> CD_R {
        CD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock Divisor"]
    #[inline(always)]
    pub fn cd(&mut self) -> CD_W {
        CD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baud Rate Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brgr](index.html) module"]
pub struct BRGR_SPEC;
impl crate::RegisterSpec for BRGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brgr::R](R) reader structure"]
impl crate::Readable for BRGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brgr::W](W) writer structure"]
impl crate::Writable for BRGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BRGR to value 0"]
impl crate::Resettable for BRGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
