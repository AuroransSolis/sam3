#[doc = "Register `MCF` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<MCF_SPEC>);
#[doc = "Register `MCF` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<MCF_SPEC>);
#[doc = "Field `MCF` reader - Multicollision Frames"]
pub type MCF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MCF` writer - Multicollision Frames"]
pub type MCF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCF_SPEC, u16, u16, 16, O>;
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
    #[must_use]
    pub fn mcf(&mut self) -> MCF_W<0> {
        MCF_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCF to value 0"]
impl crate::Resettable for MCF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
