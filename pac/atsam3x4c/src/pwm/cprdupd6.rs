#[doc = "Register `CPRDUPD6` writer"]
pub type W = crate::W<CPRDUPD6_SPEC>;
#[doc = "Field `CPRDUPD` writer - Channel Period Update"]
pub type CPRDUPD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl W {
    #[doc = "Bits 0:23 - Channel Period Update"]
    #[inline(always)]
    #[must_use]
    pub fn cprdupd(&mut self) -> CPRDUPD_W<CPRDUPD6_SPEC, 0> {
        CPRDUPD_W::new(self)
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
#[doc = "PWM Channel Period Update Register (ch_num = 6)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprdupd6::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPRDUPD6_SPEC;
impl crate::RegisterSpec for CPRDUPD6_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cprdupd6::W`](W) writer structure"]
impl crate::Writable for CPRDUPD6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
