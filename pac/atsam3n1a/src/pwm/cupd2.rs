#[doc = "Register `CUPD2` writer"]
pub type W = crate::W<CUPD2_SPEC>;
#[doc = "Field `CUPD` writer - "]
pub type CUPD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cupd(&mut self) -> CUPD_W<CUPD2_SPEC, 0> {
        CUPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM Channel Update Register (ch_num = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cupd2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CUPD2_SPEC;
impl crate::RegisterSpec for CUPD2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cupd2::W`](W) writer structure"]
impl crate::Writable for CUPD2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
