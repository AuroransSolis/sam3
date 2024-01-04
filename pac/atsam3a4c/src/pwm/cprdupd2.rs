#[doc = "Register `CPRDUPD2` writer"]
pub type W = crate::W<CPRDUPD2_SPEC>;
#[doc = "Field `CPRDUPD` writer - Channel Period Update"]
pub type CPRDUPD_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl W {
    #[doc = "Bits 0:23 - Channel Period Update"]
    #[inline(always)]
    #[must_use]
    pub fn cprdupd(&mut self) -> CPRDUPD_W<CPRDUPD2_SPEC> {
        CPRDUPD_W::new(self, 0)
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
#[doc = "PWM Channel Period Update Register (ch_num = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprdupd2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPRDUPD2_SPEC;
impl crate::RegisterSpec for CPRDUPD2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cprdupd2::W`](W) writer structure"]
impl crate::Writable for CPRDUPD2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
