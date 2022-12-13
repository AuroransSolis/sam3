#[doc = "Register `MDL3` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<MDL3_SPEC>);
#[doc = "Register `MDL3` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<MDL3_SPEC>);
#[doc = "Field `MDL` reader - Message Data Low Value"]
pub type MDL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MDL` writer - Message Data Low Value"]
pub type MDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDL3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Message Data Low Value"]
    #[inline(always)]
    pub fn mdl(&self) -> MDL_R {
        MDL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Data Low Value"]
    #[inline(always)]
    #[must_use]
    pub fn mdl(&mut self) -> MDL_W<0> {
        MDL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mailbox Data Low Register (MB = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdl3](index.html) module"]
pub struct MDL3_SPEC;
impl crate::RegisterSpec for MDL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdl3::R](R) reader structure"]
impl crate::Readable for MDL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdl3::W](W) writer structure"]
impl crate::Writable for MDL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDL3 to value 0"]
impl crate::Resettable for MDL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
