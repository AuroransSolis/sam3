#[doc = "Register `CUPD1` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<CUPD1_SPEC>);
#[doc = "Field `CUPD` writer - "]
pub type CUPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CUPD1_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cupd(&mut self) -> CUPD_W<0> {
        CUPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Update Register (ch_num = 1)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cupd1](index.html) module"]
pub struct CUPD1_SPEC;
impl crate::RegisterSpec for CUPD1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cupd1::W](W) writer structure"]
impl crate::Writable for CUPD1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
