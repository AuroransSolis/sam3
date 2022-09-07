#[doc = "Register `FIDI` reader"]
pub struct R(crate::R<FIDI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIDI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIDI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIDI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIDI` writer"]
pub struct W(crate::W<FIDI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIDI_SPEC>;
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
impl From<crate::W<FIDI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIDI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FI_DI_RATIO` reader - FI Over DI Ratio Value"]
pub type FI_DI_RATIO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FI_DI_RATIO` writer - FI Over DI Ratio Value"]
pub type FI_DI_RATIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIDI_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - FI Over DI Ratio Value"]
    #[inline(always)]
    pub fn fi_di_ratio(&self) -> FI_DI_RATIO_R {
        FI_DI_RATIO_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - FI Over DI Ratio Value"]
    #[inline(always)]
    pub fn fi_di_ratio(&mut self) -> FI_DI_RATIO_W<0> {
        FI_DI_RATIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FI DI Ratio Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fidi](index.html) module"]
pub struct FIDI_SPEC;
impl crate::RegisterSpec for FIDI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fidi::R](R) reader structure"]
impl crate::Readable for FIDI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fidi::W](W) writer structure"]
impl crate::Writable for FIDI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIDI to value 0x0174"]
impl crate::Resettable for FIDI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0174
    }
}
