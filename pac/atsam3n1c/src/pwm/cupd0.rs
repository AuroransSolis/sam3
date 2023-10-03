#[doc = "Register `CUPD0` writer"]
pub type W = crate::W<CUPD0_SPEC>;
#[doc = "Field `CUPD` writer - "]
pub type CUPD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cupd(&mut self) -> CUPD_W<CUPD0_SPEC, 0> {
        CUPD_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM Channel Update Register (ch_num = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cupd0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CUPD0_SPEC;
impl crate::RegisterSpec for CUPD0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cupd0::W`](W) writer structure"]
impl crate::Writable for CUPD0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
