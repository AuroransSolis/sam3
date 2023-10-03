#[doc = "Register `CDTYUPD4` writer"]
pub type W = crate::W<CDTYUPD4_SPEC>;
#[doc = "Field `CDTYUPD` writer - Channel Duty-Cycle Update"]
pub type CDTYUPD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl W {
    #[doc = "Bits 0:23 - Channel Duty-Cycle Update"]
    #[inline(always)]
    #[must_use]
    pub fn cdtyupd(&mut self) -> CDTYUPD_W<CDTYUPD4_SPEC, 0> {
        CDTYUPD_W::new(self)
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
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 4)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdtyupd4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDTYUPD4_SPEC;
impl crate::RegisterSpec for CDTYUPD4_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cdtyupd4::W`](W) writer structure"]
impl crate::Writable for CDTYUPD4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
