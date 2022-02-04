#[doc = "Register `SEQR[%s]` reader"]
pub struct R(crate::R<SEQR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQR[%s]` writer"]
pub struct W(crate::W<SEQR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQR_SPEC>;
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
impl From<crate::W<SEQR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQR_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Sequence Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqr](index.html) module"]
pub struct SEQR_SPEC;
impl crate::RegisterSpec for SEQR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seqr::R](R) reader structure"]
impl crate::Readable for SEQR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seqr::W](W) writer structure"]
impl crate::Writable for SEQR_SPEC {
    type Writer = W;
}
