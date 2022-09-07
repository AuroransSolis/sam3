#[doc = "Register `CUPD0` writer"]
pub struct W(crate::W<CUPD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CUPD0_SPEC>;
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
impl From<crate::W<CUPD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CUPD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CUPD` writer - "]
pub type CUPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CUPD0_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
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
#[doc = "PWM Channel Update Register (ch_num = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cupd0](index.html) module"]
pub struct CUPD0_SPEC;
impl crate::RegisterSpec for CUPD0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cupd0::W](W) writer structure"]
impl crate::Writable for CUPD0_SPEC {
    type Writer = W;
}
