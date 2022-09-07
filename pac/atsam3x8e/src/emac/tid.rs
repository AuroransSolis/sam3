#[doc = "Register `TID` reader"]
pub struct R(crate::R<TID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TID` writer"]
pub struct W(crate::W<TID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TID_SPEC>;
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
impl From<crate::W<TID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TID` reader - Type ID checking"]
pub type TID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TID` writer - Type ID checking"]
pub type TID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TID_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Type ID checking"]
    #[inline(always)]
    pub fn tid(&self) -> TID_R {
        TID_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID checking"]
    #[inline(always)]
    pub fn tid(&mut self) -> TID_W<0> {
        TID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Type ID Checking Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tid](index.html) module"]
pub struct TID_SPEC;
impl crate::RegisterSpec for TID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tid::R](R) reader structure"]
impl crate::Readable for TID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tid::W](W) writer structure"]
impl crate::Writable for TID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TID to value 0"]
impl crate::Resettable for TID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
