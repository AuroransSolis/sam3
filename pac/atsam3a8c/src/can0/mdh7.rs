#[doc = "Register `MDH7` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<MDH7_SPEC>);
#[doc = "Register `MDH7` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<MDH7_SPEC>);
#[doc = "Field `MDH` reader - Message Data High Value"]
pub type MDH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MDH` writer - Message Data High Value"]
pub type MDH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDH7_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Message Data High Value"]
    #[inline(always)]
    pub fn mdh(&self) -> MDH_R {
        MDH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Data High Value"]
    #[inline(always)]
    #[must_use]
    pub fn mdh(&mut self) -> MDH_W<0> {
        MDH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mailbox Data High Register (MB = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdh7](index.html) module"]
pub struct MDH7_SPEC;
impl crate::RegisterSpec for MDH7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdh7::R](R) reader structure"]
impl crate::Readable for MDH7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdh7::W](W) writer structure"]
impl crate::Writable for MDH7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDH7 to value 0"]
impl crate::Resettable for MDH7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
