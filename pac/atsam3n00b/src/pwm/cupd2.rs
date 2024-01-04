#[doc = "Register `CUPD2` writer"]
pub type W = crate::W<CUPD2_SPEC>;
#[doc = "Field `CUPD` writer - "]
pub type CUPD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cupd(&mut self) -> CUPD_W<CUPD2_SPEC> {
        CUPD_W::new(self, 0)
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
#[doc = "PWM Channel Update Register (ch_num = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cupd2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CUPD2_SPEC;
impl crate::RegisterSpec for CUPD2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cupd2::W`](W) writer structure"]
impl crate::Writable for CUPD2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
