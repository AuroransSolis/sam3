#[doc = "Register `FTO` reader"]
pub struct R(crate::R<FTO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FTO` writer"]
pub struct W(crate::W<FTO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTO_SPEC>;
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
impl From<crate::W<FTO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTOK` reader - Frames Transmitted OK"]
pub type FTOK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FTOK` writer - Frames Transmitted OK"]
pub type FTOK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FTO_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Frames Transmitted OK"]
    #[inline(always)]
    pub fn ftok(&self) -> FTOK_R {
        FTOK_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Frames Transmitted OK"]
    #[inline(always)]
    pub fn ftok(&mut self) -> FTOK_W<0> {
        FTOK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frames Transmitted Ok Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fto](index.html) module"]
pub struct FTO_SPEC;
impl crate::RegisterSpec for FTO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fto::R](R) reader structure"]
impl crate::Readable for FTO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fto::W](W) writer structure"]
impl crate::Writable for FTO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FTO to value 0"]
impl crate::Resettable for FTO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
